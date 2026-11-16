/* AlphaRust GUI — application d'une page, sans dépendance, sans injection HTML.
 * Tout texte dynamique passe par textContent (helper h()), donc rien n'est interprété comme du HTML. */
'use strict';

// ---------------------------------------------------------------------------
// Utilitaires DOM
// ---------------------------------------------------------------------------
const PROPS = new Set(['value', 'checked', 'disabled', 'selected', 'hidden', 'open', 'indeterminate']);

function h(tag, props, ...kids) {
  const el = document.createElement(tag);
  for (const [k, v] of Object.entries(props || {})) {
    if (v == null || v === false) continue;
    if (k === 'class') el.className = v;
    else if (k === 'text') el.textContent = v;
    else if (k.startsWith('on') && typeof v === 'function') el.addEventListener(k.slice(2), v);
    else if (PROPS.has(k)) el[k] = v;
    else el.setAttribute(k, v === true ? '' : String(v));
  }
  append(el, kids);
  return el;
}

function append(el, kids) {
  for (const k of kids.flat(Infinity)) {
    if (k == null || k === false) continue;
    el.append(k instanceof Node ? k : document.createTextNode(String(k)));
  }
  return el;
}

// alpharust-ignore: no-hardcoded-urls
const SVG_NS = 'http://www.w3.org/2000/svg';
const ICONS = {
  dashboard: 'M3 3h7v9H3zM14 3h7v5h-7zM14 12h7v9h-7zM3 16h7v5H3z',
  generate: 'M4 4h16v16H4zM12 8v8M8 12h8',
  projects: 'M3 6a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z',
  lint: 'M12 3l8 3v6c0 5-3.5 8-8 9-4.5-1-8-4-8-9V6zM8.5 12l2.5 2.5L16 9.5',
  rules: 'M5 4h11a3 3 0 0 1 3 3v13H8a3 3 0 0 1-3-3zM5 17a3 3 0 0 1 3-3h11',
  checkers: 'M9 3v3M15 3v3M9 18v3M15 18v3M3 9h3M3 15h3M18 9h3M18 15h3M6 6h12v12H6zM10 10h4v4h-4z',
  doctor: 'M3 12h4l2-6 4 12 2-6h5',
};

function icon(name) {
  const svg = document.createElementNS(SVG_NS, 'svg');
  svg.setAttribute('viewBox', '0 0 24 24');
  svg.setAttribute('fill', 'none');
  svg.setAttribute('stroke', 'currentColor');
  svg.setAttribute('stroke-width', '1.8');
  svg.setAttribute('stroke-linecap', 'round');
  svg.setAttribute('stroke-linejoin', 'round');
  svg.setAttribute('aria-hidden', 'true');
  const p = document.createElementNS(SVG_NS, 'path');
  p.setAttribute('d', ICONS[name] || '');
  svg.append(p);
  return svg;
}

function clear(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
  return el;
}

const fmtMs = (ms) => (ms == null ? '' : ms < 1000 ? `${ms} ms` : `${(ms / 1000).toFixed(1)} s`);
const plural = (n, one, many) => `${n} ${n > 1 ? many || one + 's' : one}`;
const SEV_LABEL = { error: 'Erreur', warning: 'Avertissement', info: 'Info' };
const SEV_ORDER = { error: 0, warning: 1, info: 2 };

function sevBadge(s) {
  return h('span', { class: `badge ${s}` }, SEV_LABEL[s] || s);
}

function toast(msg, kind) {
  const t = h('div', { class: `toast ${kind || ''}` }, msg);
  $('#toasts').append(t);
  setTimeout(() => t.remove(), kind === 'err' ? 8000 : 4000);
}

function $(sel, root) {
  return (root || document).querySelector(sel);
}

function debounce(fn, ms) {
  let t;
  return (...a) => {
    clearTimeout(t);
    t = setTimeout(() => fn(...a), ms);
  };
}

function shellQuote(s) {
  return /^[A-Za-z0-9_@%+=:,./-]+$/.test(s) ? s : `'${s.replace(/'/g, `'\\''`)}'`;
}

async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
    toast('Copié dans le presse-papiers', 'ok');
  } catch {
    toast('Copie impossible : sélectionner le texte manuellement', 'err');
  }
}

function download(name, type, text) {
  const url = URL.createObjectURL(new Blob([text], { type }));
  const a = h('a', { href: url, download: name });
  document.body.append(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------
async function api(path, opts) {
  const init = { headers: { Accept: 'application/json' }, ...(opts || {}) };
  if (init.body && typeof init.body !== 'string') {
    init.body = JSON.stringify(init.body);
    init.headers['Content-Type'] = 'application/json';
  }
  let res;
  try {
    res = await fetch(path, init);
  } catch (e) {
    throw new Error('serveur injoignable : ' + e.message);
  }
  const text = await res.text();
  let data = null;
  try {
    data = text ? JSON.parse(text) : null;
  } catch {
    /* corps non JSON */
  }
  if (!res.ok) {
    const err = new Error((data && data.error) || `HTTP ${res.status}`);
    err.status = res.status;
    throw err;
  }
  return data;
}

// ---------------------------------------------------------------------------
// État global, thème, navigation
// ---------------------------------------------------------------------------
const S = { meta: null, projects: [], projectsDir: '' };
let cleanup = null;

function applyTheme(t) {
  document.documentElement.dataset.theme = t;
  $('#theme-label').textContent = t === 'dark' ? '☀ Thème clair' : '☾ Thème sombre';
  try {
    localStorage.setItem('alpharust.theme', t);
  } catch {
    /* stockage indisponible */
  }
}

function initTheme() {
  let t = null;
  try {
    t = localStorage.getItem('alpharust.theme');
  } catch {
    /* ignoré */
  }
  if (t !== 'dark' && t !== 'light') t = matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
  applyTheme(t);
  $('#theme-toggle').addEventListener('click', () =>
    applyTheme(document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark'),
  );
}

const NAV = [
  ['dashboard', 'Tableau de bord', 'dashboard'],
  ['generate', 'Générer un projet', 'generate'],
  ['projects', 'Projets', 'projects'],
  ['history', 'Historique', 'generate'],
  ['lint', 'Lint', 'lint'],
  ['rules', 'Règles', 'rules'],
  ['checkers', 'Checkers', 'checkers'],
  ['doctor', 'Outils', 'doctor'],
];

function renderNav(active) {
  const nav = clear($('#nav'));
  for (const [id, label, ic] of NAV) {
    let badge = null;
    if (id === 'rules' && S.meta) badge = h('span', { class: 'count' }, S.meta.rules);
    if (id === 'projects') badge = h('span', { class: 'count' }, S.projects.length);
    if (id === 'checkers' && S.meta) badge = h('span', { class: 'count' }, S.meta.checkers);
    nav.append(h('a', { href: `#/${id}`, 'aria-current': id === active ? 'page' : null }, icon(ic), label, badge));
  }
}

function parseHash() {
  const raw = location.hash.replace(/^#\/?/, '');
  const [path, query = ''] = raw.split('?');
  const parts = path.split('/').filter(Boolean).map(decodeURIComponent);
  return { name: parts[0] || 'dashboard', arg: parts.slice(1).join('/'), query: new URLSearchParams(query) };
}

const VIEWS = {
  dashboard: ['Tableau de bord', 'dashboard', viewDashboard],
  generate: ['Générer un projet', 'generate', viewGenerate],
  jobs: ['Génération', 'generate', viewJob],
  projects: ['Projets générés', 'projects', viewProjects],
  history: ['Historique', 'history', viewHistory],
  lint: ['Lint', 'lint', viewLint],
  rules: ['Règles', 'rules', viewRules],
  checkers: ['Checkers sémantiques', 'checkers', viewCheckers],
  doctor: ['Outils installés', 'doctor', viewDoctor],
};

let firstRoute = true;
async function route() {
  if (cleanup) {
    try {
      cleanup();
    } catch {
      /* ignoré */
    }
    cleanup = null;
  }
  $('#sidebar').classList.remove('open');
  $('#menu-toggle').setAttribute('aria-expanded', 'false');
  const r = parseHash();
  const view = VIEWS[r.name] || VIEWS.dashboard;
  const [title, active, fn] = view;
  document.title = `${title} · AlphaRust`;
  $('#page-title').textContent = title;
  clear($('#topbar-actions'));
  renderNav(active);
  const main = clear($('#main'));
  main.append(h('div', { class: 'skeleton', role: 'status', 'aria-label': 'Chargement' }));
  try {
    const result = await fn(main, r);
    if (typeof result === 'function') cleanup = result;
  } catch (e) {
    clear(main).append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, 'Erreur'), e.message));
  }
  if (!firstRoute) main.focus({ preventScroll: true });
  firstRoute = false;
}

async function refreshProjects() {
  try {
    const p = await api('/api/projects');
    S.projects = p.projects;
    S.projectsDir = p.dir;
  } catch {
    S.projects = [];
  }
}

// ---------------------------------------------------------------------------
// Composants partagés
// ---------------------------------------------------------------------------
function toolRow(t) {
  const state = t.ok ? 'ok' : t.required ? 'err' : 'warn';
  return h(
    'div',
    { class: 'tool' },
    h('span', { class: `dot ${state}`, title: t.ok ? 'OK' : t.required ? 'Manquant' : 'Recommandé' }),
    h('div', {}, h('strong', {}, t.label), ' ', h('span', { class: 'muted mono small' }, t.version || (t.found ? 'version inconnue' : 'introuvable'))),
    h('span', { class: 'muted small' }, t.min ? `≥ ${t.min}` : ''),
    t.ok ? null : h('p', { class: 'hint' }, t.hint),
  );
}

function statCard(num, label, href) {
  const body = [h('div', { class: 'num' }, num), h('div', { class: 'label' }, label)];
  return h('div', { class: 'card stat' }, href ? h('a', { href, class: 'stat-link' }, ...body) : body);
}

function frameworkBadge(id) {
  const f = S.meta && S.meta.frameworks.find((x) => x.id === id);
  return h('span', { class: 'badge accent' }, f ? f.label : id || 'inconnu');
}

function statusBadge(s) {
  const map = { running: ['accent', 'En cours'], succeeded: ['ok', 'Réussi'], failed: ['error', 'Échec'], cancelled: ['warning', 'Annulé'] };
  const [cls, label] = map[s] || ['', s];
  return h('span', { class: `badge ${cls}` }, label);
}

// ---------------------------------------------------------------------------
// Tableau de bord
// ---------------------------------------------------------------------------
async function viewDashboard(main) {
  const [doctor, jobs] = await Promise.all([api('/api/doctor'), api('/api/jobs')]);
  const m = S.meta;
  clear(main).append(
    h('p', { class: 'page-intro' }, 'Qualité de code par règles Markdown + générateurs de projets bâtis sur les outils officiels (create-astro, composer, wagtail start, cargo new), avec installation et vérification réelles.'),
    h(
      'div',
      { class: 'grid cols-4' },
      statCard(m.rules, 'règles actives', '#/rules'),
      statCard(m.profiles.length, 'profils', '#/lint'),
      statCard(m.checkers, 'checkers sémantiques', '#/checkers'),
      statCard(S.projects.length, 'projets générés', '#/projects'),
    ),
    h(
      'div',
      { class: 'grid cols-2', style: null },
      h(
        'section',
        { class: 'card', 'aria-labelledby': 'h-tools' },
        h('div', { class: 'card-head' }, h('h2', { id: 'h-tools' }, 'Chaînes d’outils'), h('a', { href: '#/doctor', class: 'btn small' }, 'Détails')),
        h(
          'div',
          { class: 'card-body stack' },
          doctor.map((d) =>
            h(
              'div',
              { class: 'row between' },
              h('div', {}, h('strong', {}, d.label), h('div', { class: 'muted small' }, d.tools.filter((t) => t.required).map((t) => t.label).join(' · '))),
              h('span', { class: `badge ${d.ready ? 'ok' : 'warning'}` }, d.ready ? 'Prêt' : 'Outils manquants'),
            ),
          ),
        ),
      ),
      h(
        'section',
        { class: 'card', 'aria-labelledby': 'h-jobs' },
        h('div', { class: 'card-head' }, h('h2', { id: 'h-jobs' }, 'Générations récentes'), h('a', { href: '#/generate', class: 'btn primary small' }, 'Nouveau projet')),
        jobs.length === 0
          ? h('div', { class: 'empty' }, 'Aucune génération pour cette session.')
          : h(
              'div',
              { class: 'card-body stack' },
              jobs.slice(0, 6).map((j) =>
                h('a', { href: `#/jobs/${j.id}`, class: 'row between', style: null }, h('span', {}, h('strong', {}, j.name), ' ', frameworkBadge(j.framework)), statusBadge(j.status)),
              ),
            ),
      ),
    ),
    h(
      'section',
      { class: 'card', 'aria-labelledby': 'h-prof' },
      h('div', { class: 'card-head' }, h('h2', { id: 'h-prof' }, 'Profils de règles')),
      h(
        'div',
        { class: 'table-wrap' },
        h(
          'table',
          {},
          h('thead', {}, h('tr', {}, h('th', {}, 'Profil'), h('th', { class: 'nowrap' }, 'Règles'), h('th', {}, ''))),
          h(
            'tbody',
            {},
            m.profiles.map((p) =>
              h('tr', {}, h('td', {}, h('code', {}, p.name)), h('td', {}, p.rules), h('td', {}, h('a', { href: `#/lint?profile=${encodeURIComponent(p.name)}` }, 'Lancer le lint'))),
            ),
          ),
        ),
      ),
    ),
  );
  // le dashboard est une grille : espacement vertical
  main.classList.add('stack');
  return () => main.classList.remove('stack');
}

// ---------------------------------------------------------------------------
// Génération : formulaire + aperçu du plan
// ---------------------------------------------------------------------------
const GEN_KEY = 'alpharust.gen.v2';
const GEN_DEFAULT = () => ({
  framework: 'astro',
  name: '',
  install: true,
  verify: true,
  git: true,
  force: false,
  astro: {
    package_manager: 'npm', template: 'alpharust', typescript: 'strict', integrations: [], site: '', typecheck: true,
    ui: 'none', ui_components: [], database: 'none', db_url: '', auth: false, auth_plugins: [], i18n: [],
    theme_on: false, theme: { primary: '#4f46e5', secondary: '#7c3aed', accent: '#ec4899' }, css_variants: [], docker: false, ci: false,
  },
  laravel: { starter: 'none', dark: false, pest: false, database: 'sqlite', frontend: false },
  wagtail: { packages: '', migrate: true },
  rust: { kind: 'bin', edition: '', deps: [] },
});
const UI_FW = ['react', 'vue', 'svelte'];

function loadGen() {
  const d = GEN_DEFAULT();
  try {
    const s = JSON.parse(localStorage.getItem(GEN_KEY) || 'null');
    if (s && typeof s === 'object') {
      for (const k of ['framework', 'name', 'install', 'verify', 'git']) if (k in s) d[k] = s[k];
      for (const k of ['astro', 'laravel', 'wagtail', 'rust']) Object.assign(d[k], s[k] || {});
    }
  } catch {
    /* état invalide ignoré */
  }
  return d;
}

function buildRequest(G) {
  const base = { name: G.name.trim(), force: G.force, git: G.git, install: G.install, verify: G.verify, framework: G.framework };
  if (G.framework === 'astro') {
    const a = G.astro;
    const platform = a.template === 'alpharust';
    const withUi = platform ? a.ui : 'none';
    return {
      ...base, package_manager: a.package_manager, template: a.template, typescript: a.typescript,
      integrations: G.install ? a.integrations : [], site: a.site.trim() || null, typecheck: a.typecheck,
      ui: withUi, ui_components: withUi === 'starwind' ? a.ui_components : [],
      database: platform ? a.database : 'none', db_url: platform && a.database === 'postgres' ? a.db_url.trim() || null : null,
      auth: platform && a.auth, auth_plugins: platform && a.auth ? a.auth_plugins : [],
      i18n: platform ? a.i18n : [],
      theme: withUi === 'concordia' && a.theme_on ? a.theme : null, css_variants: withUi === 'concordia' ? a.css_variants : [],
      docker: platform && a.docker, ci: platform && a.ci,
    };
  }
  if (G.framework === 'laravel') return { ...base, ...G.laravel };
  if (G.framework === 'wagtail') return { ...base, packages: G.wagtail.packages.split(/[\s,]+/).filter(Boolean), migrate: G.wagtail.migrate };
  return { ...base, kind: G.rust.kind, edition: G.rust.edition || null, deps: G.rust.deps };
}

function cliFor(G) {
  const args = ['alpharust', 'generate', G.framework, shellQuote(G.name.trim() || '<nom>')];
  const add = (...a) => args.push(...a);
  if (!G.install) add('--no-install');
  if (!G.verify) add('--no-verify');
  if (!G.git) add('--no-git');
  if (G.force) add('--force');
  if (G.framework === 'astro') {
    const a = G.astro;
    if (a.package_manager !== 'npm') add('--pm', a.package_manager);
    if (a.template !== 'alpharust') add('--template', a.template);
    if (a.typescript !== 'strict') add('--ts', a.typescript);
    if (a.integrations.length && G.install) add('--integration', a.integrations.join(','));
    if (a.site.trim()) add('--site', shellQuote(a.site.trim()));
    if (!a.typecheck) add('--no-typecheck');
    if (a.template === 'alpharust') {
      if (a.ui !== 'none') add('--ui', a.ui);
      if (a.ui === 'starwind' && a.ui_components.length) add('--ui-component', a.ui_components.join(','));
      if (a.ui === 'concordia' && a.theme_on) add('--theme-primary', a.theme.primary, '--theme-secondary', a.theme.secondary, '--theme-accent', a.theme.accent);
      if (a.ui === 'concordia' && a.css_variants.length) add('--css-variant', a.css_variants.join(','));
      if (a.database !== 'none') add('--db', a.database);
      if (a.database === 'postgres' && a.db_url.trim()) add('--db-url', shellQuote(a.db_url.trim()));
      if (a.auth) add('--auth');
      if (a.auth && a.auth_plugins.length) add('--auth-plugin', a.auth_plugins.join(','));
      if (a.i18n.length) add('--i18n', a.i18n.join(','));
      if (a.docker) add('--docker');
      if (a.ci) add('--ci');
    }
  } else if (G.framework === 'laravel') {
    const l = G.laravel;
    if (l.starter !== 'none') add('--starter', l.starter);
    if (l.dark) add('--dark');
    if (l.pest) add('--pest');
    if (l.database !== 'sqlite') add('--db', l.database);
    if (l.frontend) add('--frontend');
  } else if (G.framework === 'wagtail') {
    const pk = G.wagtail.packages.split(/[\s,]+/).filter(Boolean);
    if (pk.length) add('--package', shellQuote(pk.join(',')));
    if (!G.wagtail.migrate) add('--no-migrate');
  } else {
    const r = G.rust;
    if (r.kind !== 'bin') add('--kind', r.kind);
    if (r.edition) add('--edition', r.edition);
    if (r.deps.length) add('--dep', r.deps.join(','));
  }
  return args.join(' ');
}

function radioGroup(name, value, options, onChange, key) {
  return h(
    'div',
    { class: 'segmented', role: 'radiogroup' },
    options.map((o) => {
      const id = typeof o === 'string' ? o : o.id;
      const label = typeof o === 'string' ? o : o.label;
      return h('label', {}, h('input', { type: 'radio', name, value: id, checked: id === value, 'data-k': `${key}:${id}`, onchange: () => onChange(id) }), label);
    }),
  );
}

function checkbox(label, desc, checked, onChange, opts) {
  const o = opts || {};
  return h(
    'label',
    { class: 'check' },
    h('input', { type: 'checkbox', checked, disabled: o.disabled, 'data-k': o.key, onchange: (e) => onChange(e.target.checked) }),
    h('span', {}, label, desc ? h('span', { class: 'desc' }, desc) : null),
  );
}

async function viewGenerate(main, route) {
  const G = loadGen();
  if (route.query.get('framework')) G.framework = route.query.get('framework');
  const fwSchema = (id) => S.meta.frameworks.find((f) => f.id === id);
  let planSeq = 0;
  let plan = null;
  let planError = null;
  let tab = 'plan';
  const form = h('form', { class: 'card', novalidate: true, 'aria-label': 'Options de génération' });
  const preview = h('div', { class: 'card preview-col' });
  form.addEventListener('submit', (e) => e.preventDefault());

  const persist = () => {
    try {
      localStorage.setItem(GEN_KEY, JSON.stringify(G));
    } catch {
      /* ignoré */
    }
  };

  const restoreFocus = (k) => {
    if (!k) return;
    const el = form.querySelector(`[data-k="${CSS.escape(k)}"]`);
    if (el) el.focus();
  };

  const change = (fn, opts) => {
    const k = document.activeElement && document.activeElement.dataset && document.activeElement.dataset.k;
    fn();
    persist();
    if (opts && opts.rebuild) {
      renderForm();
      restoreFocus(k);
    }
    schedulePlan();
  };

  function renderForm() {
    const fw = fwSchema(G.framework);
    const o = fw.options;
    clear(form);
    const nameInvalid = G.name && !/^[A-Za-z0-9][A-Za-z0-9._-]*$/.test(G.name.trim());
    append(form, [
      h('div', { class: 'card-head' }, h('h2', {}, '1. Projet')),
      h(
        'div',
        { class: 'card-body' },
        h(
          'fieldset',
          {},
          h('legend', {}, 'Framework'),
          h(
            'div',
            { class: 'framework-picker', role: 'radiogroup', 'aria-label': 'Framework' },
            S.meta.frameworks.map((f) =>
              h(
                'label',
                { class: 'radio' },
                h('input', { type: 'radio', name: 'framework', value: f.id, checked: f.id === G.framework, 'data-k': `fw:${f.id}`, onchange: () => change(() => (G.framework = f.id), { rebuild: true }) }),
                h('strong', {}, f.label),
                h('span', { class: 'desc' }, f.tool),
              ),
            ),
          ),
          h('p', { class: 'hint' }, fw.description),
        ),
        h(
          'div',
          { class: 'field' },
          h('label', { for: 'f-name' }, 'Nom du projet'),
          h('input', {
            id: 'f-name',
            type: 'text',
            value: G.name,
            placeholder: `mon-${G.framework}`,
            autocomplete: 'off',
            spellcheck: 'false',
            'aria-invalid': nameInvalid ? 'true' : null,
            'aria-describedby': 'f-name-hint',
            oninput: (e) => {
              G.name = e.target.value;
              persist();
              e.target.setAttribute('aria-invalid', G.name && !/^[A-Za-z0-9][A-Za-z0-9._-]*$/.test(G.name.trim()) ? 'true' : 'false');
              schedulePlan();
            },
          }),
          h('p', { class: 'hint', id: 'f-name-hint' }, 'Lettres, chiffres, « - », « _ » et « . ». Le projet sera créé dans ', h('code', {}, `${S.projectsDir || 'sites'}/<nom>`), '.'),
        ),
      ),
      h('div', { class: 'card-head' }, h('h2', {}, `2. Options ${fw.label}`)),
      h('div', { class: 'card-body' }, frameworkOptions(G, o, change)),
      h('div', { class: 'card-head' }, h('h2', {}, '3. Exécution')),
      h(
        'div',
        { class: 'card-body' },
        h(
          'div',
          { class: 'options' },
          checkbox('Installer les dépendances', 'npm/pnpm install, composer install, pip install…', G.install, (v) => change(() => (G.install = v), { rebuild: true }), { key: 'install' }),
          checkbox('Vérifier le résultat', 'check, build, tests, lint AlphaRust', G.verify, (v) => change(() => (G.verify = v)), { key: 'verify', disabled: !G.install }),
          checkbox('Dépôt Git + commit initial', null, G.git, (v) => change(() => (G.git = v)), { key: 'git' }),
          checkbox('Remplacer si existant', 'Uniquement un projet généré par AlphaRust', G.force, (v) => change(() => (G.force = v)), { key: 'force' }),
        ),
      ),
    ]);
  }

  function renderPreview() {
    clear(preview);
    preview.classList.remove('loading');
    preview.removeAttribute('aria-busy');
    const tabs = h(
      'div',
      { class: 'tabs', role: 'tablist', 'aria-label': 'Aperçu' },
      [
        ['plan', 'Commandes'],
        ['tools', 'Outils'],
        ['cli', 'Ligne de commande'],
      ].map(([id, label]) =>
        h('button', { type: 'button', role: 'tab', id: `tab-${id}`, 'aria-selected': tab === id ? 'true' : 'false', 'aria-controls': 'preview-panel', onclick: () => { tab = id; renderPreview(); } }, label),
      ),
    );
    const panel = h('div', { class: 'card-body', id: 'preview-panel', role: 'tabpanel', 'aria-labelledby': `tab-${tab}`, 'aria-live': 'polite' });
    if (!G.name.trim()) panel.append(h('div', { class: 'callout info' }, h('strong', {}, 'Saisir un nom de projet'), 'L’aperçu affiche les commandes officielles exactes qui seront exécutées.'));
    else if (planError) panel.append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, 'Configuration invalide'), planError));
    else if (!plan) panel.append(h('div', { class: 'skeleton' }));
    else if (tab === 'plan') panel.append(planView(plan));
    else if (tab === 'tools') panel.append(toolsView(plan));
    else panel.append(cliView(G));
    if (tab === 'cli' && (!plan || planError || !G.name.trim())) panel.append(cliView(G));

    const can = G.name.trim() && plan && !planError && plan.ready;
    const go = h('button', { type: 'button', class: 'btn primary', disabled: !can, onclick: start }, 'Générer le projet');
    if (plan && !plan.ready && !planError) go.title = 'Installer les outils manquants (onglet Outils)';
    append(preview, [h('div', { class: 'card-head' }, h('h2', {}, 'Aperçu')), tabs, panel, h('div', { class: 'card-body row between' }, h('span', { class: 'muted small' }, plan ? `${plan.steps.length} étapes → ${plan.target}` : ''), go)]);
  }

  async function runPlan() {
    const seq = ++planSeq;
    preview.classList.add('loading');
    preview.setAttribute('aria-busy', 'true');
    if (!G.name.trim()) {
      plan = null;
      planError = null;
      return renderPreview();
    }
    try {
      const p = await api('/api/plan', { method: 'POST', body: buildRequest(G) });
      if (seq !== planSeq) return;
      plan = p;
      planError = null;
    } catch (e) {
      if (seq !== planSeq) return;
      plan = null;
      planError = e.message;
    }
    renderPreview();
  }
  const schedulePlan = (() => {
    const d = debounce(runPlan, 300);
    return () => {
      preview.classList.add('loading');
      d();
    };
  })();

  async function start() {
    try {
      const j = await api('/api/jobs', { method: 'POST', body: buildRequest(G) });
      location.hash = `#/jobs/${j.id}`;
    } catch (e) {
      toast(e.message, 'err');
    }
  }

  clear(main).append(
    h('p', { class: 'page-intro' }, 'Chaque projet est créé par l’outil officiel du framework, puis les dépendances sont installées, une couche de modèles propre est ajoutée par-dessus, et le résultat est vérifié (types, build, tests, lint).'),
    h('div', { class: 'split' }, form, preview),
  );
  renderForm();
  renderPreview();
  runPlan();
  return () => {
    planSeq++;
  };
}


// ---------------------------------------------------------------------------
// Briques plateforme (Astro) : préréglages, UI, base de données, authentification, i18n, thème
// ---------------------------------------------------------------------------
function applyPreset(a, p) {
  const o = p.options;
  a.template = 'alpharust';
  a.ui = o.ui || 'none';
  a.database = o.database || 'none';
  a.auth = !!o.auth;
  a.auth_plugins = [...(o.auth_plugins || [])];
  a.i18n = [...(o.i18n || [])];
  a.docker = !!o.docker;
  a.ci = !!o.ci;
  a.theme_on = !!o.theme;
  if (o.theme) a.theme = { ...o.theme };
  for (const i of o.integrations || []) if (!a.integrations.includes(i)) a.integrations.push(i);
}

function presetBar(G, o, change) {
  if (!o.presets || !o.presets.length) return null;
  return h('fieldset', {}, h('legend', {}, 'Préréglages'),
    h('div', { class: 'preset-grid' }, o.presets.map((p) =>
      h('button', { type: 'button', class: 'preset', 'data-k': `preset:${p.id}`, title: p.flags, onclick: () => { G.install = true; change(() => applyPreset(G.astro, p), { rebuild: true }); } },
        h('strong', {}, p.label), h('span', { class: 'desc' }, p.description)))),
    h('p', { class: 'hint' }, 'Un préréglage remplit les briques ci-dessous ; vous pouvez ensuite les ajuster. Équivalent CLI : ', h('code', {}, 'alpharust generate astro <nom> --preset <id>'), '.'));
}

function multiCheck(items, selected, onToggle, keyPrefix, disabled) {
  return h('div', { class: 'options' }, items.map((i) => {
    const id = typeof i === 'string' ? i : i.id;
    const label = typeof i === 'string' ? i : (i.label || i.id);
    const desc = typeof i === 'string' ? null : (i.description || null);
    return checkbox(label, desc, selected.includes(id), (v) => onToggle(id, v), { key: `${keyPrefix}:${id}`, disabled });
  }));
}

function toggleIn(list, id, on) {
  const rest = list.filter((x) => x !== id);
  return on ? [...rest, id] : rest;
}

function platformOptions(G, o, change) {
  const a = G.astro;
  const platform = a.template === 'alpharust';
  const blocked = !platform || !G.install;
  const why = !platform ? 'Les briques plateforme exigent le template « AlphaRust ».' : !G.install ? 'Les briques plateforme exigent l’installation des dépendances.' : null;
  return h('div', { class: 'platform' },
    h('h3', { class: 'subhead' }, 'Plateforme'),
    why ? h('div', { class: 'callout warn' }, why) : null,
    h('fieldset', { disabled: blocked }, h('legend', {}, 'Interface'),
      radioGroup('ui', a.ui, o.ui, (v) => change(() => {
        a.ui = v;
        if (v === 'concordia') a.integrations = a.integrations.filter((x) => x !== 'tailwind');
      }, { rebuild: true }), 'ui'),
      a.ui === 'starwind' ? h('div', {},
        h('p', { class: 'hint' }, 'Installé avec l’outil officiel : ', h('code', {}, 'starwind init'), ' puis ', h('code', {}, 'starwind add'), '.'),
        h('details', {}, h('summary', {}, `Composants Starwind (${a.ui_components.length || o.starwind_default.length})`),
          multiCheck(o.starwind_components, a.ui_components.length ? a.ui_components : o.starwind_default, (id, v) => change(() => {
            const cur = a.ui_components.length ? a.ui_components : [...o.starwind_default];
            a.ui_components = toggleIn(cur, id, v);
          }), 'swc', blocked))) : null,
      a.ui === 'concordia' ? concordiaOptions(a, o, change, blocked) : null),
    h('fieldset', { disabled: blocked }, h('legend', {}, 'Base de données (Drizzle ORM)'),
      radioGroup('db', a.database, o.database, (v) => change(() => {
        a.database = v;
        if (v === 'none') { a.auth = false; a.auth_plugins = []; }
      }, { rebuild: true }), 'dbx'),
      a.database === 'postgres' ? h('div', { class: 'field' },
        h('label', { for: 'f-dburl' }, 'URL PostgreSQL de développement (optionnelle)'),
        h('input', { id: 'f-dburl', type: 'text', value: a.db_url, placeholder: 'postgresql://…:5432/ma_base', autocomplete: 'off', spellcheck: 'false', oninput: (e) => change(() => (a.db_url = e.target.value)) }),
        h('p', { class: 'hint' }, 'Si renseignée, les migrations sont appliquées ; la base `<nom>_test` sert aux tests. Les identifiants sont masqués dans le manifeste.')) : null,
      a.database === 'sqlite' ? h('p', { class: 'hint' }, 'Fichier ', h('code', {}, 'data/<nom>.db'), ' ; migrations générées par drizzle-kit et appliquées.') : null),
    h('fieldset', { disabled: blocked }, h('legend', {}, 'Authentification (Better Auth)'),
      checkbox('Comptes utilisateurs', 'Inscription, connexion, session, journal d’audit — schéma généré par `auth generate`', a.auth, (v) => change(() => {
        a.auth = v;
        if (v && a.database === 'none') a.database = 'sqlite';
        if (!v) a.auth_plugins = [];
      }, { rebuild: true }), { key: 'auth' }),
      a.auth ? multiCheck(o.auth_plugins, a.auth_plugins, (id, v) => change(() => (a.auth_plugins = toggleIn(a.auth_plugins, id, v))), 'plg', blocked) : null),
    h('fieldset', { disabled: blocked }, h('legend', {}, 'Langues (i18n)'),
      multiCheck(o.locales, a.i18n, (id, v) => change(() => (a.i18n = toggleIn(a.i18n, id, v)), { rebuild: true }), 'loc', blocked),
      a.i18n.length === 1 ? h('p', { class: 'hint warn' }, 'Choisissez au moins deux langues (la première est la langue par défaut).') : h('p', { class: 'hint' }, 'Messages fr/en/es/ar, routage Astro, arabe en RTL, sélecteur de langue.')),
    h('fieldset', { disabled: blocked }, h('legend', {}, 'Déploiement'),
      h('div', { class: 'options' },
        checkbox('Dockerfile + docker-compose', 'Adaptateur Node, image node:24, sonde /api/health', a.docker, (v) => change(() => (a.docker = v)), { key: 'docker' }),
        checkbox('Workflow GitHub Actions', 'install, check, tests, build', a.ci, (v) => change(() => (a.ci = v)), { key: 'ci' }))),
  );
}

function concordiaOptions(a, o, change, blocked) {
  // « initial » (base) est toujours chargée ; seules les variantes optionnelles se choisissent.
  const variants = o.css_variants.filter((v) => v !== 'initial');
  return h('div', { class: 'concordia-box' },
    h('p', { class: 'hint' }, 'Reprend fidèlement les jetons, variantes et ', h('strong', {}, 'tous'), ' les composants du dépôt Concordia.'),
    h('div', { class: 'options' },
      variants.map((v) => checkbox(`Variante ${v}`, null, a.css_variants.length === 0 || a.css_variants.includes(v), (on) => change(() => {
        const cur = a.css_variants.length ? a.css_variants : [...variants];
        const next = toggleIn(cur, v, on);
        a.css_variants = next.length === variants.length ? [] : next.length ? next : ['initial'];
      }), { key: `cv:${v}`, disabled: blocked }))),
    checkbox('Générer un thème (contrôle WCAG)', 'Couleurs de marque + textes lisibles calculés comme dans themecsutomizer', a.theme_on, (v) => change(() => (a.theme_on = v), { rebuild: true }), { key: 'theme', disabled: blocked }),
    a.theme_on ? themePanel(a, change) : null);
}

// La CSP interdit les attributs `style` : les couleurs passent par le CSSOM.
function swatch(bg, fg, text) {
  const el = h('span', { class: 'swatch-btn' }, text);
  el.style.background = bg;
  el.style.color = fg;
  return el;
}

function themePanel(a, change) {
  const out = h('div', { class: 'theme-report', 'aria-live': 'polite' });
  let seq = 0;
  let timer = null;
  const refresh = () => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      const mine = ++seq;
      try {
        const r = await api('/api/theme/check', { method: 'POST', body: a.theme });
        if (mine !== seq) return;
        clear(out).append(h('table', { class: 'contrast' },
          h('thead', {}, h('tr', {}, h('th', {}, 'Couleur'), h('th', {}, 'Aperçu'), h('th', {}, 'Contraste'), h('th', {}, 'Mode sombre'))),
          h('tbody', {}, r.colors.map((c) => h('tr', {},
            h('td', {}, c.name),
            h('td', {}, swatch(c.hex, c.text, 'Bouton')),
            h('td', {}, `${c.ratio.toFixed(2)} : 1 `, h('span', { class: `badge ${c.rating === 'FAIL' ? 'error' : c.rating === 'AAA' ? 'ok' : 'warning'}` }, c.rating === 'FAIL' ? 'insuffisant' : c.rating)),
            h('td', {}, swatch('#0f172a', c.dark_hex, c.dark_hex), ` ${c.dark_ratio.toFixed(1)} : 1`))))));
      } catch (e) {
        if (mine === seq) clear(out).append(h('p', { class: 'hint warn' }, e.message));
      }
    }, 150);
  };
  refresh();
  const input = (key, label) => h('label', { class: 'color-field' }, label,
    h('input', { type: 'color', value: a.theme[key], 'data-k': `th:${key}`, oninput: (e) => { change(() => (a.theme[key] = e.target.value)); refresh(); } }),
    h('code', {}, a.theme[key]));
  return h('div', { class: 'theme-panel' }, h('div', { class: 'color-row' }, input('primary', 'Primaire'), input('secondary', 'Secondaire'), input('accent', 'Accent')), out);
}

function frameworkOptions(G, o, change) {
  if (G.framework === 'astro') {
    const a = G.astro;
    const needsSite = a.integrations.includes('sitemap') || a.template === 'blog';
    return h(
      'div',
      {},
      presetBar(G, o, change),
      h('fieldset', {}, h('legend', {}, 'Gestionnaire de paquets'), radioGroup('pm', a.package_manager, o.package_manager, (v) => change(() => (a.package_manager = v)), 'pm')),
      h('fieldset', {}, h('legend', {}, 'Template'), radioGroup('template', a.template, o.template, (v) => change(() => (a.template = v), { rebuild: true }), 'tpl'),
        h('p', { class: 'hint' }, a.template === 'alpharust' ? 'Template officiel « minimal » + layout, SEO, thème clair/sombre, collection blog typée, 404.' : 'Template officiel inchangé ; AlphaRust ajoute seulement la config qualité, la doc et le thème.')),
      h('fieldset', {}, h('legend', {}, 'TypeScript'), radioGroup('ts', a.typescript, o.typescript, (v) => change(() => (a.typescript = v)), 'ts'),
        h('p', { class: 'hint' }, 'Preset officiel astro/tsconfigs/', h('code', {}, a.typescript), '.')),
      h(
        'fieldset',
        {},
        h('legend', {}, 'Intégrations (astro add)'),
        h(
          'div',
          { class: 'options' },
          o.integrations.map((i) => {
            const on = a.integrations.includes(i.id);
            const blocked = !G.install || (UI_FW.includes(i.id) && !on && a.integrations.some((x) => UI_FW.includes(x)));
            return checkbox(i.id, i.description, on, (v) => change(() => {
              a.integrations = a.integrations.filter((x) => x !== i.id && !(v && UI_FW.includes(i.id) && UI_FW.includes(x)));
              if (v) a.integrations.push(i.id);
            }, { rebuild: true }), { key: `int:${i.id}`, disabled: blocked && !on });
          }),
        ),
        !G.install ? h('p', { class: 'hint' }, 'Les intégrations exigent l’installation des dépendances.') : h('p', { class: 'hint' }, 'Un seul framework d’îlots à la fois (react, vue ou svelte).'),
      ),
      platformOptions(G, o, change),
      h(
        'div',
        { class: 'field' },
        h('label', { for: 'f-site' }, needsSite ? 'URL du site (requise)' : 'URL du site (optionnelle)'),
        // alpharust-ignore: no-hardcoded-urls
        h('input', { id: 'f-site', type: 'url', value: a.site, placeholder: 'https://exemple.com', autocomplete: 'off', oninput: (e) => change(() => (a.site = e.target.value)) }),
        h('p', { class: 'hint' }, 'Écrite dans astro.config.mjs (', h('code', {}, 'site'), '). Exigée par l’intégration sitemap.'),
      ),
      checkbox('Vérification des types', '@astrojs/check + typescript, puis astro check', a.typecheck, (v) => change(() => (a.typecheck = v)), { key: 'typecheck' }),
    );
  }
  if (G.framework === 'laravel') {
    const l = G.laravel;
    const hasKit = l.starter !== 'none';
    return h(
      'div',
      {},
      h('fieldset', {}, h('legend', {}, 'Kit de démarrage'), h('div', { class: 'options' }, o.starter.map((s) =>
        h('label', { class: 'radio' }, h('input', { type: 'radio', name: 'starter', value: s.id, checked: l.starter === s.id, 'data-k': `st:${s.id}`, onchange: () => change(() => {
          l.starter = s.id;
          if (s.id === 'none') { l.dark = false; l.pest = false; }
          if (s.id === 'breeze-api') l.dark = false;
        }, { rebuild: true }) }), h('span', {}, s.label))))),
      h('fieldset', {}, h('legend', {}, 'Base de données'), radioGroup('db', l.database, o.database, (v) => change(() => (l.database = v)), 'db'),
        h('p', { class: 'hint' }, l.database === 'sqlite' ? 'SQLite : migrations et données d’exemple exécutées pendant la génération.' : 'La base doit exister : migrations non exécutées.')),
      h('div', { class: 'options' },
        checkbox('Mode sombre (Breeze)', null, l.dark, (v) => change(() => (l.dark = v)), { key: 'dark', disabled: !hasKit || l.starter === 'breeze-api' }),
        checkbox('Pest au lieu de PHPUnit', 'Kit Breeze uniquement', l.pest, (v) => change(() => (l.pest = v)), { key: 'pest', disabled: !hasKit }),
        checkbox('Compiler le front (npm)', 'npm install && npm run build même sans kit', l.frontend, (v) => change(() => (l.frontend = v)), { key: 'frontend' }),
      ),
    );
  }
  if (G.framework === 'wagtail') {
    const w = G.wagtail;
    return h(
      'div',
      {},
      h('div', { class: 'field' },
        h('label', { for: 'f-pkgs' }, 'Paquets pip supplémentaires'),
        h('input', { id: 'f-pkgs', type: 'text', value: w.packages, placeholder: 'django-debug-toolbar, psycopg[binary]', autocomplete: 'off', spellcheck: 'false', oninput: (e) => change(() => (w.packages = e.target.value)) }),
        h('p', { class: 'hint' }, 'Séparés par des espaces ou virgules ; ajoutés à requirements.txt. Un venv Python est toujours créé (.venv).')),
      checkbox('Exécuter les migrations', 'manage.py migrate (SQLite)', w.migrate, (v) => change(() => (w.migrate = v)), { key: 'migrate' }),
      !G.install ? h('div', { class: 'callout warn' }, 'Wagtail exige l’installation : `wagtail start` a besoin de wagtail dans le venv.') : null,
    );
  }
  const r = G.rust;
  return h(
    'div',
    {},
    h('fieldset', {}, h('legend', {}, 'Type de crate'), radioGroup('kind', r.kind, o.kind, (v) => change(() => (r.kind = v)), 'kind')),
    h('fieldset', {}, h('legend', {}, 'Édition'), radioGroup('edition', r.edition || '', [{ id: '', label: 'défaut cargo' }, ...o.edition.map((e) => ({ id: e, label: e }))], (v) => change(() => (r.edition = v)), 'ed')),
    h('fieldset', {}, h('legend', {}, 'Dépendances (cargo add)'), h('div', { class: 'options' }, o.deps.map((d) =>
      checkbox(d.id, d.features.length ? `features : ${d.features.join(', ')}` : null, r.deps.includes(d.id), (v) => change(() => {
        r.deps = r.deps.filter((x) => x !== d.id);
        if (v) r.deps.push(d.id);
      }), { key: `dep:${d.id}` })))),
  );
}

function planView(plan) {
  const wrap = h('div', {});
  if (plan.exists) wrap.append(h('div', { class: 'callout warn' }, h('strong', {}, 'Le dossier existe déjà'), plan.target, ' — cocher « Remplacer si existant » (autorisé uniquement pour un projet généré par AlphaRust).'));
  if (!plan.ready) wrap.append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, 'Outils manquants'), plan.missing || 'Voir l’onglet Outils.'));
  const phases = [];
  for (const s of plan.steps) {
    let p = phases[phases.length - 1];
    if (!p || p.label !== s.phase_label) phases.push((p = { label: s.phase_label, steps: [] }));
    p.steps.push(s);
  }
  let n = 0;
  for (const p of phases) {
    wrap.append(h('div', { class: 'phase' }, h('div', { class: 'phase-title' }, p.label), p.steps.map((s) => {
      n += 1;
      return h('div', { class: 'plan-step' }, h('span', { class: 'n' }, n), h('div', {}, h('span', {}, s.label, s.fatal ? null : h('span', { class: 'badge outline' }, 'non bloquant')), s.command ? h('code', {}, s.command) : null));
    })));
  }
  return wrap;
}

function toolsView(plan) {
  return h('div', {}, plan.tools.map(toolRow));
}

function cliView(G) {
  const cmd = cliFor(G);
  return h('div', { class: 'stack' }, h('p', { class: 'muted' }, 'La même configuration en ligne de commande (mode non interactif, utilisable en CI) :'),
    h('div', { class: 'cli-box' }, h('pre', {}, h('code', {}, cmd)), h('button', { type: 'button', class: 'btn small', onclick: () => copyText(cmd) }, 'Copier')),
    h('p', { class: 'hint' }, 'Ajouter ', h('code', {}, '--dry-run'), ' pour n’afficher que le plan, ou lancer ', h('code', {}, 'alpharust new'), ' pour l’assistant interactif.'));
}

// ---------------------------------------------------------------------------
// Job : progression en direct (SSE)
// ---------------------------------------------------------------------------
async function viewJob(main, route) {
  const id = route.arg;
  const summary = await api(`/api/jobs/${encodeURIComponent(id)}`);
  const st = { steps: [], status: summary.status, active: -1, filter: null, lines: [], report: null, failure: null, target: summary.target };
  const progress = h('div', { class: 'progress', role: 'progressbar', 'aria-valuemin': '0', 'aria-valuemax': '100', 'aria-valuenow': '0', 'aria-label': 'Progression' }, h('span'));
  const stepsEl = h('ol', { class: 'steps', 'aria-label': 'Étapes' });
  const term = h('div', { class: 'terminal', role: 'log', 'aria-live': 'off', tabindex: '0', 'aria-label': 'Journal de génération' });
  const head = h('div', { class: 'card card-pad stack' });
  const result = h('div', {});
  const stickRef = { v: true };
  term.addEventListener('scroll', () => { stickRef.v = term.scrollTop + term.clientHeight >= term.scrollHeight - 24; });
  const live = h('div', { class: 'sr-only', role: 'status', 'aria-live': 'polite' });

  const addLine = (index, cls, text) => {
    st.lines.push({ index, cls, text });
    if (st.lines.length > 6000) st.lines.shift();
    if (st.filter == null || st.filter === index) term.append(lineEl(cls, text));
    if (stickRef.v) term.scrollTop = term.scrollHeight;
  };
  const lineEl = (cls, text) => h('span', { class: cls }, text + '\n');
  const redrawTerm = () => { clear(term); for (const l of st.lines) if (st.filter == null || st.filter === l.index) term.append(lineEl(l.cls, l.text)); term.scrollTop = term.scrollHeight; };

  const renderSteps = () => {
    clear(stepsEl);
    const total = st.steps.length;
    const done = st.steps.filter((s) => s.state === 'ok' || s.state === 'warn').length;
    const pct = st.status === 'succeeded' ? 100 : total ? Math.round((done / total) * 100) : 0;
    progress.firstChild.style.width = `${pct}%`;
    progress.setAttribute('aria-valuenow', String(pct));
    progress.className = `progress ${st.status === 'succeeded' ? 'done' : st.status === 'failed' ? 'failed' : ''}`;
    st.steps.forEach((s, i) => {
      const ic = { run: h('span', { class: 'spinner', role: 'img', 'aria-label': 'en cours' }), ok: '✔', warn: '⚠', fail: '✘', wait: '○' }[s.state];
      stepsEl.append(h('li', { class: `st-${s.state} ${st.filter === i ? 'active' : ''}`, tabindex: '0', role: 'button', 'aria-pressed': st.filter === i ? 'true' : 'false', onclick: () => toggle(i), onkeydown: (e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); toggle(i); } } },
        h('span', { class: 'icon' }, ic), h('span', {}, s.label, h('div', { class: 'muted small' }, s.phase_label || '')), h('span', { class: 'time' }, s.millis != null ? fmtMs(s.millis) : '')));
    });
  };
  const toggle = (i) => { st.filter = st.filter === i ? null : i; renderSteps(); redrawTerm(); };

  const renderHead = () => {
    clear(head).append(
      h('div', { class: 'row between' }, h('div', {}, h('h2', {}, summary.name, ' ', frameworkBadge(summary.framework)), h('div', { class: 'muted small mono' }, summary.target)), h('div', { class: 'row' }, statusBadge(st.status),
        st.status === 'running' ? h('button', { type: 'button', class: 'btn danger small', onclick: cancel }, 'Annuler') : null)),
      progress,
      h('div', { class: 'row small muted' }, h('span', {}, st.status === 'running' ? 'Exécution en cours…' : ''), st.filter != null ? h('button', { type: 'button', class: 'btn small ghost', onclick: () => toggle(st.filter) }, 'Afficher tout le journal') : null),
    );
  };

  const cancel = async () => {
    try { await api(`/api/jobs/${encodeURIComponent(id)}/cancel`, { method: 'POST' }); toast('Annulation demandée'); } catch (e) { toast(e.message, 'err'); }
  };

  const renderResult = () => {
    clear(result);
    const isTask = String(summary.kind || '').startsWith('task:');
    if (st.failure) result.append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, isTask ? 'Échec de la tâche' : 'Échec de la génération'), h('pre', {}, st.failure),
      isTask ? h('div', { class: 'row' }, h('a', { class: 'btn', href: '#/projects' }, 'Retour aux projets'))
        : h('p', { class: 'hint' }, 'Le projet partiel est conservé dans ', h('code', {}, st.target), ' pour analyse ; relancer avec « Remplacer si existant ».'),
      isTask ? null : h('div', { class: 'row' }, h('a', { class: 'btn', href: '#/generate' }, 'Retour au formulaire'))));
    const r = st.report;
    if (r) {
      const failed = r.steps.filter((s) => !s.ok);
      result.append(h('section', { class: 'card', 'aria-labelledby': 'h-res' },
        h('div', { class: 'card-head' }, h('h2', { id: 'h-res' }, isTask ? 'Tâche terminée' : 'Projet prêt'), h('span', { class: 'badge ok' }, isTask ? fmtMs(r.elapsed_ms) : `${r.files.length} fichiers · ${fmtMs(r.elapsed_ms)}`)),
        h('div', { class: 'card-body stack' },
          failed.length && !isTask ? h('div', { class: 'callout warn' }, h('strong', {}, `${plural(failed.length, 'étape')} non bloquante${failed.length > 1 ? 's' : ''} en échec`), failed.map((f) => h('div', {}, '• ', f.label, f.note ? h('span', { class: 'muted' }, ' — ', String(f.note).split('\n')[0]) : null))) : h('div', { class: 'callout ok' }, h('strong', {}, 'Toutes les vérifications sont passées'), isTask ? 'La commande s’est terminée avec le code 0.' : 'Installation, types/build/tests et lint AlphaRust.'),
          r.lint ? h('div', { class: 'row' }, h('span', {}, 'Lint AlphaRust :'), h('span', { class: `badge ${r.lint.errors ? 'error' : 'ok'}` }, plural(r.lint.errors, 'erreur')), h('span', { class: `badge ${r.lint.warnings ? 'warning' : 'ok'}` }, plural(r.lint.warnings, 'avertissement')), h('span', { class: 'muted small' }, `${r.lint.files} fichiers · ${r.lint.rules} règles`)) : null,
          isTask ? null : h('div', {}, h('div', { class: 'label' }, 'Pour continuer'), h('pre', {}, h('code', {}, r.next_steps.join('\n')))),
          h('div', { class: 'row' }, h('a', { class: 'btn primary', href: `#/lint?path=${encodeURIComponent(r.target)}` }, 'Analyser avec AlphaRust'), h('a', { class: 'btn', href: '#/projects' }, 'Voir les projets'), h('a', { class: 'btn', href: isTask ? '#/history' : '#/generate' }, isTask ? 'Historique' : 'Générer un autre projet'),
            isTask ? null : h('button', { type: 'button', class: 'btn ghost', onclick: () => copyText(r.next_steps.join('\n')) }, 'Copier les commandes')),
        )));
    }
  };

  const onEvent = (ev) => {
    switch (ev.type) {
      case 'preflight':
        addLine(-1, 'l-dim', `Outils : ${ev.tools.map((t) => `${t.label} ${t.version || '✘'}`).join(' · ')}`);
        break;
      case 'plan':
        st.steps = ev.steps.map((s) => ({ ...s, state: 'wait', millis: null }));
        st.target = ev.target;
        addLine(-1, 'l-dim', `${ev.steps.length} étapes → ${ev.target}`);
        break;
      case 'step_start': {
        const s = st.steps[ev.index];
        if (s) s.state = 'run';
        st.active = ev.index;
        addLine(ev.index, 'l-step', `▶ [${ev.index + 1}/${ev.total}] ${ev.label}`);
        if (ev.command) addLine(ev.index, 'l-cmd', `$ ${ev.command}`);
        live.textContent = `Étape ${ev.index + 1} sur ${ev.total} : ${ev.label}`;
        break;
      }
      case 'log':
        addLine(ev.index, /\b(ERR!|ERROR|Error:|error:|FAILED|Failed)\b|✘/.test(ev.line) ? 'l-err' : '', ev.line);
        break;
      case 'step_end': {
        const s = st.steps[ev.index];
        if (s) { s.state = ev.ok ? 'ok' : ev.fatal ? 'fail' : 'warn'; s.millis = ev.millis; }
        addLine(ev.index, ev.ok ? 'l-ok' : ev.fatal ? 'l-err' : 'l-warn', `${ev.ok ? '✔' : ev.fatal ? '✘' : '⚠'} ${fmtMs(ev.millis)}${ev.note ? ' — ' + String(ev.note).split('\n')[0] : ''}`);
        break;
      }
      case 'done':
        st.report = ev.report;
        st.target = ev.report.target;
        break;
      case 'failed':
        st.failure = ev.message;
        addLine(-1, 'l-err', `✘ ${ev.message}`);
        break;
      default:
    }
    renderSteps();
  };

  clear(main).append(
    h('div', { class: 'stack' }, head, h('div', { class: 'job-layout' },
      h('section', { class: 'card', 'aria-label': 'Étapes' }, h('div', { class: 'card-head' }, h('h2', {}, 'Étapes'), h('span', { class: 'muted small' }, 'Cliquer pour filtrer le journal')), stepsEl),
      h('section', { class: 'card', 'aria-label': 'Journal' }, h('div', { class: 'card-head' }, h('h2', {}, 'Journal en direct'), h('button', { type: 'button', class: 'btn small', onclick: () => copyText(st.lines.map((l) => l.text).join('\n')) }, 'Copier le journal')), h('div', { class: 'card-body' }, term))),
      result, live),
  );
  renderHead();
  renderSteps();

  const es = new EventSource(`/api/jobs/${encodeURIComponent(id)}/events`);
  es.onmessage = (m) => { try { onEvent(JSON.parse(m.data)); } catch { /* ligne invalide ignorée */ } };
  es.addEventListener('end', (m) => {
    es.close();
    let d = {};
    try { d = JSON.parse(m.data); } catch { /* ignoré */ }
    st.status = d.status || st.status;
    summary.status = st.status;
    renderHead();
    renderSteps();
    renderResult();
    live.textContent = { succeeded: 'Génération terminée', failed: 'Génération en échec', cancelled: 'Génération annulée' }[st.status] || '';
    refreshProjects().then(() => renderNav('generate'));
  });
  es.onerror = () => { if (es.readyState === EventSource.CLOSED) return; /* reconnexion automatique avec Last-Event-ID */ };
  return () => es.close();
}

// ---------------------------------------------------------------------------
// Projets
// ---------------------------------------------------------------------------
async function startTask(project, task, btn) {
  if (btn) btn.disabled = true;
  try {
    const j = await api(`/api/projects/${encodeURIComponent(project)}/tasks/${task}`, { method: 'POST' });
    location.hash = `#/jobs/${j.id}`;
  } catch (e) {
    toast(e.message, 'error');
    if (btn) btn.disabled = false;
  }
}

function gitBadge(g) {
  if (!g) return h('span', { class: 'muted small' }, 'pas de dépôt');
  return h('span', { class: `badge ${g.dirty ? 'warning' : 'ok'}`, title: g.dirty ? 'Modifications non validées' : 'Arbre propre' }, `${g.branch}${g.dirty ? ' · modifié' : ''}`);
}

async function viewProjects(main) {
  await refreshProjects();
  renderNav('projects');
  $('#topbar-actions').append(h('a', { class: 'btn primary', href: '#/generate' }, 'Nouveau projet'));
  if (S.projects.length === 0) {
    clear(main).append(h('div', { class: 'card empty' }, h('p', {}, h('strong', {}, 'Aucun projet dans '), h('code', {}, S.projectsDir)), h('a', { class: 'btn primary', href: '#/generate' }, 'Générer un premier projet')));
    return;
  }
  const cards = S.projects.map((p) => {
    if (!p.generated) {
      return h('article', { class: 'card project' }, h('div', { class: 'card-body' }, h('strong', {}, p.name), ' ', h('span', { class: 'badge outline' }, 'non généré par AlphaRust'), h('div', { class: 'muted small mono' }, p.path)));
    }
    const tasks = (p.tasks || []).filter((t) => t.interactive && t.id !== 'lint');
    return h('article', { class: 'card project', 'aria-label': p.name },
      h('div', { class: 'card-head' },
        h('div', {}, h('h2', {}, p.name), h('div', { class: 'muted small mono' }, p.path)),
        h('div', { class: 'row' }, frameworkBadge(p.framework), p.package_manager ? h('span', { class: 'badge outline' }, p.package_manager) : null, gitBadge(p.git))),
      h('div', { class: 'card-body stack' },
        h('div', { class: 'chips' }, (p.features || []).length ? p.features.map((f) => h('span', { class: 'chip' }, f)) : h('span', { class: 'muted small' }, 'aucune brique optionnelle')),
        h('div', { class: 'row between' },
          h('span', { class: 'muted small' }, p.installed ? '● dépendances installées' : '○ dépendances non installées', ' · créé le ', p.created ? String(p.created).replace('T', ' ').replace('Z', ' UTC') : '—'),
          h('div', { class: 'row' },
            tasks.map((t) => { const b = h('button', { type: 'button', class: 'btn small', title: t.command, onclick: () => startTask(p.name, t.id, b) }, t.label.split(' ')[0] === 'Installer' ? 'Installer' : t.id === 'update' ? 'Mettre à jour' : t.id === 'build' ? 'Construire' : t.id === 'check' ? 'Vérifier' : t.id === 'test' ? 'Tester' : t.label); return b; }),
            h('button', { type: 'button', class: 'btn small primary', title: 'alpharust lint', onclick: (e) => startTask(p.name, 'lint', e.currentTarget) }, 'Lint'),
            h('a', { class: 'btn small', href: `#/lint?path=${encodeURIComponent(p.path)}` }, 'Rapport lint'))),
        h('details', {}, h('summary', {}, 'Commandes exécutées'), h('ul', { class: 'cmds' }, (p.tasks || []).map((t) => h('li', {}, h('strong', {}, t.id), ' ', h('code', {}, t.command)))))));
  });
  clear(main).append(
    h('p', { class: 'page-intro' }, 'Projets de ', h('code', {}, S.projectsDir), '. Chaque action lance la commande native du projet (gestionnaire de paquets, composer, cargo…) et diffuse sa sortie ; le serveur de développement se lance en CLI : ', h('code', {}, 'alpharust run <projet> dev'), '.'),
    h('div', { class: 'stack' }, cards),
  );
}

// ---------------------------------------------------------------------------
// Historique des tâches de la session
// ---------------------------------------------------------------------------
async function viewHistory(main) {
  renderNav('history');
  const jobs = await api('/api/jobs');
  if (jobs.length === 0) {
    clear(main).append(h('div', { class: 'card empty' }, h('p', {}, 'Aucune génération ni tâche lancée pendant cette session.'), h('a', { class: 'btn primary', href: '#/generate' }, 'Générer un projet')));
    return;
  }
  const kindLabel = (k) => (k === 'generate' ? 'Génération' : `Tâche · ${k.replace('task:', '')}`);
  clear(main).append(
    h('p', { class: 'page-intro' }, 'Les 20 dernières exécutions de cette session (génération et tâches de projet). Ouvrez-en une pour rejouer son journal.'),
    h('div', { class: 'table-wrap' }, h('table', {},
      h('thead', {}, h('tr', {}, h('th', {}, 'Projet'), h('th', {}, 'Type'), h('th', {}, 'Statut'), h('th', {}, 'Durée'), h('th', {}, 'Début'), h('th', {}, ''))),
      h('tbody', {}, jobs.map((j) => h('tr', {},
        h('td', {}, h('strong', {}, j.name), ' ', frameworkBadge(j.framework)),
        h('td', {}, kindLabel(j.kind || 'generate')),
        h('td', {}, statusBadge(j.status)),
        h('td', { class: 'nowrap' }, j.elapsed_ms != null ? `${(j.elapsed_ms / 1000).toFixed(1)} s` : '…'),
        h('td', { class: 'nowrap' }, new Date(j.started_at * 1000).toLocaleTimeString('fr-FR')),
        h('td', {}, h('a', { class: 'btn small', href: `#/jobs/${j.id}` }, 'Ouvrir'))))))));
}

// ---------------------------------------------------------------------------
// Lint
// ---------------------------------------------------------------------------
async function viewLint(main, route) {
  await refreshProjects();
  const L = { path: route.query.get('path') || '.', profile: route.query.get('profile') || '', data: null, sev: new Set(['error', 'warning', 'info']), q: '', rule: '', group: true, shown: 150, error: null, loading: false };
  const results = h('div', { class: 'stack', 'aria-live': 'polite' });
  const pathInput = h('input', { id: 'l-path', type: 'text', value: L.path, list: 'l-paths', autocomplete: 'off', spellcheck: 'false' });
  const paths = h('datalist', { id: 'l-paths' }, [h('option', { value: '.' }), S.projects.map((p) => h('option', { value: p.path }))]);
  const profileSel = h('select', { id: 'l-profile' }, h('option', { value: '' }, 'Selon .alpharust.toml / toutes les règles'), S.meta.profiles.map((p) => h('option', { value: p.name, selected: p.name === L.profile }, `${p.name} (${p.rules} règles)`)));
  const run = async () => {
    L.path = pathInput.value.trim() || '.';
    L.profile = profileSel.value;
    L.loading = true; L.error = null; L.shown = 150;
    renderResults();
    try {
      L.data = await api(`/api/lint?path=${encodeURIComponent(L.path)}&profile=${encodeURIComponent(L.profile)}`);
    } catch (e) { L.error = e.message; L.data = null; }
    L.loading = false;
    renderResults();
  };
  const exportJson = () => download('alpharust-lint.json', 'application/json', JSON.stringify(L.data, null, 2));
  const exportSarif = async () => {
    try {
      const res = await fetch(`/api/sarif?path=${encodeURIComponent(L.path)}&profile=${encodeURIComponent(L.profile)}`);
      if (!res.ok) throw new Error((await res.json()).error);
      download('alpharust.sarif', 'application/sarif+json', await res.text());
    } catch (e) { toast(e.message, 'err'); }
  };

  const filtered = () => {
    const q = L.q.trim().toLowerCase();
    return L.data.violations.filter((v) => L.sev.has(v.severity) && (!L.rule || v.rule_id === L.rule) && (!q || `${v.file} ${v.rule} ${v.rule_id} ${v.message}`.toLowerCase().includes(q)));
  };

  function renderResults() {
    clear(results);
    if (L.loading) return results.append(h('div', { class: 'skeleton', role: 'status', 'aria-label': 'Analyse en cours' }));
    if (L.error) return results.append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, 'Lint impossible'), L.error));
    if (!L.data) return results.append(h('div', { class: 'card empty' }, 'Choisir un dossier et lancer l’analyse.'));
    const d = L.data;
    const list = filtered();
    const rules = [...new Set(d.violations.map((v) => v.rule_id))].sort();
    results.append(
      h('div', { class: 'grid cols-4' }, statCard(d.counts.error, d.counts.error > 1 ? 'erreurs' : 'erreur'), statCard(d.counts.warning, d.counts.warning > 1 ? 'avertissements' : 'avertissement'), statCard(d.files_scanned, 'fichiers analysés'), statCard(d.rules_used, `règles (${fmtMs(d.elapsed_ms)})`)),
      h('div', { class: 'toolbar', role: 'group', 'aria-label': 'Filtres' },
        ['error', 'warning', 'info'].map((s) => h('button', { type: 'button', class: 'chip', 'aria-pressed': L.sev.has(s) ? 'true' : 'false', onclick: () => { L.sev.has(s) ? L.sev.delete(s) : L.sev.add(s); L.shown = 150; renderResults(); } }, SEV_LABEL[s], h('span', { class: 'n' }, d.counts[s]))),
        h('input', { class: 'grow', type: 'search', placeholder: 'Filtrer par texte (fichier, règle, message)…', 'aria-label': 'Filtrer par texte', value: L.q, id: 'l-q', oninput: debounce((e) => { L.q = e.target.value; L.shown = 150; renderResults(); const q = $('#l-q'); q.focus(); q.setSelectionRange(q.value.length, q.value.length); }, 200) }),
        h('select', { 'aria-label': 'Filtrer par règle', onchange: (e) => { L.rule = e.target.value; L.shown = 150; renderResults(); } }, h('option', { value: '' }, 'Toutes les règles'), rules.map((r) => h('option', { value: r, selected: r === L.rule }, `${r} (${d.violations.filter((v) => v.rule_id === r).length})`))),
        h('button', { type: 'button', class: 'chip', 'aria-pressed': L.group ? 'true' : 'false', onclick: () => { L.group = !L.group; renderResults(); } }, 'Grouper par fichier'),
        h('button', { type: 'button', class: 'btn small', onclick: exportJson }, 'JSON'),
        h('button', { type: 'button', class: 'btn small', onclick: exportSarif }, 'SARIF'),
      ),
      h('p', { class: 'muted small' }, `${list.length} sur ${d.violations.length} violation(s) — `, h('span', { class: 'mono' }, d.path), d.profile ? ` · profil ${d.profile}` : ''),
    );
    if (d.violations.length === 0) return results.append(h('div', { class: 'callout ok' }, h('strong', {}, '✔ Aucune violation'), 'Ce dossier respecte toutes les règles actives.'));
    if (list.length === 0) return results.append(h('div', { class: 'card empty' }, 'Aucune violation ne correspond aux filtres.'));
    const rows = list.slice(0, L.shown);
    const vEl = (v, showFile) => h('div', { class: 'violation' }, sevBadge(v.severity), h('span', { class: 'loc' }, showFile ? `${v.file}:${v.line}` : `L${v.line}`), h('div', {}, v.message, h('a', { class: 'rule', href: `#/rules/${encodeURIComponent(v.rule_id)}` }, v.rule_id)));
    if (L.group) {
      const byFile = new Map();
      for (const v of rows) { if (!byFile.has(v.file)) byFile.set(v.file, []); byFile.get(v.file).push(v); }
      for (const [file, vs] of byFile) results.append(h('div', { class: 'file-group' }, h('h3', {}, h('span', {}, file), h('span', { class: 'muted' }, plural(vs.length, 'violation'))), vs.sort((a, b) => SEV_ORDER[a.severity] - SEV_ORDER[b.severity] || a.line - b.line).map((v) => vEl(v, false))));
    } else {
      results.append(h('div', { class: 'file-group' }, rows.map((v) => vEl(v, true))));
    }
    if (list.length > rows.length) results.append(h('div', { class: 'pager' }, h('span', { class: 'muted' }, `${rows.length} / ${list.length}`), h('button', { type: 'button', class: 'btn', onclick: () => { L.shown += 300; renderResults(); } }, 'Afficher plus')));
  }

  clear(main).append(h('div', { class: 'stack' },
    h('form', { class: 'card card-pad', onsubmit: (e) => { e.preventDefault(); run(); } },
      h('div', { class: 'grid cols-2' }, h('div', { class: 'field' }, h('label', { for: 'l-path' }, 'Dossier à analyser'), pathInput, paths, h('p', { class: 'hint' }, 'Le dossier courant ou un projet de sites/.')), h('div', { class: 'field' }, h('label', { for: 'l-profile' }, 'Profil'), profileSel)),
      h('div', { class: 'row' }, h('button', { type: 'submit', class: 'btn primary' }, 'Lancer le lint'))),
    results));
  renderResults();
  if (route.query.get('path') || route.query.get('profile')) run();
}

// ---------------------------------------------------------------------------
// Règles
// ---------------------------------------------------------------------------
function inlineCode(text) {
  const out = [];
  text.split(/(`[^`]+`)/).forEach((part) => {
    if (part.length > 2 && part.startsWith('`') && part.endsWith('`')) out.push(h('code', {}, part.slice(1, -1)));
    else if (part) out.push(part);
  });
  return out;
}

/** Markdown minimal et sûr (titres, listes, blocs de code, paragraphes) : uniquement des nœuds DOM. */
function markdown(src) {
  const root = h('div', { class: 'markdown' });
  const lines = String(src || '').replace(/\r/g, '').split('\n');
  let para = [];
  let list = null;
  const flush = () => {
    if (para.length) root.append(h('p', {}, inlineCode(para.join(' '))));
    para = [];
    list = null;
  };
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (l.startsWith('```')) {
      flush();
      const code = [];
      for (i++; i < lines.length && !lines[i].startsWith('```'); i++) code.push(lines[i]);
      root.append(h('pre', {}, h('code', {}, code.join('\n'))));
    } else if (/^#{1,3}\s/.test(l)) {
      flush();
      root.append(h(`h${Math.min(3, l.match(/^#+/)[0].length + 1)}`, {}, l.replace(/^#+\s*/, '')));
    } else if (/^\s*[-*]\s+/.test(l)) {
      if (!list) { para.length && flush(); list = h('ul'); root.append(list); }
      list.append(h('li', {}, inlineCode(l.replace(/^\s*[-*]\s+/, ''))));
    } else if (!l.trim()) flush();
    else { if (list) list = null; para.push(l.trim()); }
  }
  flush();
  return root;
}

async function viewRules(main, route) {
  if (route.arg) return viewRule(main, route.arg);
  const rules = await api('/api/rules');
  const F = { q: route.query.get('q') || '', sev: new Set(), kind: '', cat: '' };
  const cats = [...new Set(rules.map((r) => r.category))].sort();
  const kinds = [...new Set(rules.map((r) => r.kind))].sort();
  const body = h('div', {});
  const count = h('p', { class: 'muted small', 'aria-live': 'polite' });
  const search = h('input', { id: 'r-q', type: 'search', class: 'grow', placeholder: 'Rechercher une règle (id, nom, message)…   ( / )', 'aria-label': 'Rechercher une règle', value: F.q, oninput: debounce((e) => { F.q = e.target.value; draw(); }, 120) });
  const chips = h('div', { class: 'row' });
  const draw = () => {
    clear(chips);
    for (const s of ['error', 'warning', 'info']) chips.append(h('button', { type: 'button', class: 'chip', 'aria-pressed': F.sev.has(s) ? 'true' : 'false', onclick: () => { F.sev.has(s) ? F.sev.delete(s) : F.sev.add(s); draw(); } }, SEV_LABEL[s], h('span', { class: 'n' }, rules.filter((r) => r.severity === s).length)));
    const q = F.q.trim().toLowerCase();
    const list = rules.filter((r) => (!F.sev.size || F.sev.has(r.severity)) && (!F.kind || r.kind === F.kind) && (!F.cat || r.category === F.cat) && (!q || `${r.id} ${r.name} ${r.message} ${r.apply}`.toLowerCase().includes(q)));
    count.textContent = `${list.length} règle(s) sur ${rules.length}`;
    clear(body);
    if (!list.length) return body.append(h('div', { class: 'card empty' }, 'Aucune règle ne correspond.'));
    body.append(h('div', { class: 'table-wrap' }, h('table', {},
      h('thead', {}, h('tr', {}, h('th', {}, 'Règle'), h('th', {}, 'Sévérité'), h('th', {}, 'Type'), h('th', {}, 'Cible'), h('th', {}, 'Message'))),
      h('tbody', {}, list.slice(0, 400).map((r) => h('tr', {}, h('td', { class: 'nowrap' }, h('a', { class: 'mono', href: `#/rules/${encodeURIComponent(r.id)}` }, r.id)), h('td', {}, sevBadge(r.severity)), h('td', {}, h('span', { class: 'badge outline' }, r.kind)), h('td', { class: 'mono small' }, r.apply), h('td', {}, r.message)))))));
    if (list.length > 400) body.append(h('p', { class: 'muted small' }, `Affichage limité à 400 règles : affiner la recherche.`));
  };
  const onKey = (e) => { if (e.key === '/' && !/INPUT|SELECT|TEXTAREA/.test(document.activeElement.tagName)) { e.preventDefault(); search.focus(); } };
  document.addEventListener('keydown', onKey);
  clear(main).append(h('div', {},
    h('div', { class: 'toolbar', role: 'search' }, search,
      h('select', { 'aria-label': 'Catégorie', onchange: (e) => { F.cat = e.target.value; draw(); } }, h('option', { value: '' }, 'Toutes catégories'), cats.map((c) => h('option', { value: c }, c))),
      h('select', { 'aria-label': 'Type de règle', onchange: (e) => { F.kind = e.target.value; draw(); } }, h('option', { value: '' }, 'Tous types'), kinds.map((c) => h('option', { value: c }, c)))),
    chips, count, body));
  draw();
  return () => document.removeEventListener('keydown', onKey);
}

async function viewRule(main, id) {
  $('#page-title').textContent = 'Règle';
  const r = await api(`/api/rule?id=${encodeURIComponent(id)}`);
  document.title = `${r.id} · AlphaRust`;
  const ex = (title, items, cls) => h('div', {}, h('h3', {}, title), items.length ? items.map((c) => h('pre', { class: cls }, h('code', {}, c))) : h('p', { class: 'muted' }, 'Aucun exemple.'));
  clear(main).append(h('div', { class: 'stack' },
    h('div', {}, h('a', { href: '#/rules' }, '← Toutes les règles')),
    h('section', { class: 'card' },
      h('div', { class: 'card-head' }, h('h2', { class: 'mono' }, r.id), sevBadge(r.severity)),
      h('div', { class: 'card-body stack' },
        h('dl', { class: 'kv' },
          h('dt', {}, 'Nom'), h('dd', {}, r.name),
          h('dt', {}, 'Message'), h('dd', {}, r.message),
          h('dt', {}, 'Cible (apply)'), h('dd', {}, h('code', {}, r.apply)),
          h('dt', {}, 'Type'), h('dd', {}, h('span', { class: 'badge outline' }, r.kind)),
          r.pattern ? [h('dt', {}, 'Motif'), h('dd', {}, h('code', {}, r.pattern))] : null,
          r.checker ? [h('dt', {}, 'Checker'), h('dd', {}, h('a', { href: '#/checkers' }, r.checker), Object.keys(r.args || {}).length ? h('pre', {}, h('code', {}, JSON.stringify(r.args, null, 2))) : null)] : null),
        r.description ? markdown(r.description) : null)),
    r.examples.bad.length || r.examples.good.length ? h('div', { class: 'examples' }, ex('❌ À éviter', r.examples.bad), ex('✅ Correct', r.examples.good)) : null,
  ));
}

// ---------------------------------------------------------------------------
// Checkers, Doctor
// ---------------------------------------------------------------------------
async function viewCheckers(main) {
  const list = await api('/api/checkers');
  const grid = h('div', { class: 'grid cols-3' });
  const draw = (q) => {
    clear(grid);
    const f = list.filter((c) => `${c.name} ${c.description}`.toLowerCase().includes(q.toLowerCase()));
    if (!f.length) grid.append(h('div', { class: 'card empty' }, 'Aucun checker.'));
    f.forEach((c) => grid.append(h('div', { class: 'card card-pad' }, h('h3', { class: 'mono' }, c.name), h('p', { class: 'muted' }, c.description))));
  };
  clear(main).append(h('div', { class: 'stack' }, h('p', { class: 'page-intro' }, 'Les checkers sont des analyses sémantiques en Rust que les règles Markdown appellent via `checker:` (au-delà des expressions régulières).'),
    h('div', { class: 'toolbar' }, h('input', { type: 'search', class: 'grow', 'aria-label': 'Filtrer les checkers', placeholder: 'Filtrer…', oninput: (e) => draw(e.target.value) })), grid));
  draw('');
}

async function viewDoctor(main) {
  const out = h('div', { class: 'grid cols-2' });
  const load = async () => {
    clear(out).append(h('div', { class: 'skeleton' }));
    const d = await api('/api/doctor');
    clear(out);
    d.forEach((f) => out.append(h('section', { class: 'card' },
      h('div', { class: 'card-head' }, h('h2', {}, f.label), h('span', { class: `badge ${f.ready ? 'ok' : 'warning'}` }, f.ready ? 'Prêt' : 'Outils manquants')),
      h('div', { class: 'card-body' }, f.tools.map(toolRow)))));
  };
  $('#topbar-actions').append(h('button', { type: 'button', class: 'btn', onclick: () => load().then(() => toast('Outils revérifiés', 'ok')).catch((e) => toast(e.message, 'err')) }, 'Revérifier'));
  clear(main).append(h('div', { class: 'stack' }, h('p', { class: 'page-intro' }, 'Détection des outils nécessaires aux générateurs : présence, version minimale et commande d’installation conseillée.'), out));
  await load();
}

// ---------------------------------------------------------------------------
// Démarrage
// ---------------------------------------------------------------------------
async function boot() {
  initTheme();
  $('#menu-toggle').addEventListener('click', () => {
    const open = $('#sidebar').classList.toggle('open');
    $('#menu-toggle').setAttribute('aria-expanded', String(open));
  });
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      $('#sidebar').classList.remove('open');
      $('#menu-toggle').setAttribute('aria-expanded', 'false');
    }
  });
  try {
    S.meta = await api('/api/meta');
    await refreshProjects();
  } catch (e) {
    clear($('#main')).append(h('div', { class: 'callout err', role: 'alert' }, h('strong', {}, 'Serveur AlphaRust injoignable'), e.message));
    return;
  }
  $('#version').textContent = `v${S.meta.version}`;
  $('#cwd').textContent = S.meta.cwd;
  window.addEventListener('hashchange', route);
  route();
}

boot();
