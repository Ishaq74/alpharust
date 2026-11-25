@@meta mode=replace when=vitrine@@
/*
 * Gabarit du site (en-tête, pied de page, sections) — généré par tools/dsgen/vitrine, IDENTIQUE dans Astro
 * (Tailwind + Starwind ou CSS pur), Laravel et Wagtail. Ne lit que le contrat de jetons (--bg, --text, --brand…) :
 * pour changer d'identité, modifiez le thème, pas ce fichier. Les composants (boutons, cartes…) gardent leur CSS.
 */
/* Palette neutre commune : mêmes gris dans tous les kits (Starwind/atomic a des neutres chauds, Concordia du slate).
   Les deux familles de noms sont posées : jetons Starwind (--background, --card…) et contrat du kit CSS (--bg, --surface…). */
.site-body {
  --background: #ffffff; --foreground: #151821; --card: #f5f6fa; --card-foreground: #151821;
  --popover: #ffffff; --popover-foreground: #151821; --muted: #eceef5; --muted-foreground: #4a5163;
  --accent: #eceef5; --accent-foreground: #151821; --border: #dde0ea; --input: #dde0ea;
  --bg: #ffffff; --surface: #f5f6fa; --surface-2: #eceef5; --text: #151821; --text-muted: #4a5163; --site-error: #b91c1c;
}
:root[data-theme='dark'] .site-body, .dark .site-body {
  --background: #0f1117; --foreground: #eceef5; --card: #181b25; --card-foreground: #eceef5;
  --popover: #181b25; --popover-foreground: #eceef5; --muted: #222635; --muted-foreground: #a9b0c4;
  --accent: #222635; --accent-foreground: #eceef5; --border: #2a2f3f; --input: #2a2f3f;
  --bg: #0f1117; --surface: #181b25; --surface-2: #222635; --text: #eceef5; --text-muted: #a9b0c4; --site-error: #f87171;
}
.site-body {
  --site-width: 72rem;
  --site-gutter: clamp(1rem, 4vw, 2rem);
  /* Neutres dérivés du fond et du texte (pas des neutres propres au kit) : même rendu partout. */
  --site-muted: color-mix(in srgb, var(--text) 66%, var(--bg));
  --site-line: color-mix(in srgb, var(--text) 13%, var(--bg));
  --site-soft: color-mix(in srgb, var(--text) 4%, var(--bg));
  /* Police commune à tous les kits (Concordia, Starwind, kit CSS) et à tous les frameworks. */
  --site-font: ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --site-radius: 1rem;
  margin: 0;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg);
  color: var(--text);
  font-family: var(--site-font);
  font-size: 1rem;
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
}
.site-body h1, .site-body h2, .site-body h3 { margin: 0; font-family: inherit; text-wrap: balance; line-height: 1.2; font-weight: 700; color: var(--text); letter-spacing: -0.01em; }
.site-body p, .site-body li { margin: 0; max-width: none; }
.site-body a { text-underline-offset: 0.2em; }
/* Liens : taille héritée (Concordia fixe une taille propre aux liens) ; les règles .site-* plus bas priment. */
.site-body :where(a:not([data-slot])) { font-size: inherit; font-weight: inherit; line-height: inherit; }
.site-body ul.site-nav__list, .site-body ul.site-mobile__list, .site-body ul.site-mobile__sub, .site-body ul.site-footer__links,
.site-body ul.site-social, .site-body ul.site-checklist, .site-body ul.site-contact__list, .site-body ul.site-team {
  list-style: none; margin: 0; padding: 0;
}
.site-body address { font-style: normal; }
/* Icônes en bloc partout (comme le preflight de Tailwind) : même hauteur de ligne dans tous les kits. */
/* Icônes du site en bloc (comme le preflight de Tailwind) : même hauteur de ligne dans tous les kits. */
.site-icon { display: block; flex-shrink: 0; }
.site-body .site-main { flex: 1; width: auto; margin: 0; padding: 0; outline: none; }
/* Pages hors gabarit (catalogue, compte…) : contenu centré dans la même largeur. */
.site-body .site-main--page { width: min(100% - 2 * var(--site-gutter), var(--site-width)); margin-inline: auto; padding-block: 2.5rem 4rem; }
.site-body .site-main > section { margin-block: 0; }
.site-container { width: min(100% - 2 * var(--site-gutter), var(--site-width)); margin-inline: auto; }
.site-container--narrow { width: min(100% - 2 * var(--site-gutter), 48rem); }
.site-skip {
  position: absolute; inset-inline-start: 1rem; top: -4rem; z-index: 100; padding: 0.5rem 1rem; border-radius: 0.5rem;
  background: var(--brand); color: var(--brand-contrast); font-weight: 600; text-decoration: none;
}
.site-skip:focus { top: 1rem; }

/* ---------- en-tête ---------- */
.site-header {
  position: sticky; top: 0; z-index: 40; border-bottom: 1px solid var(--site-line);
  background: color-mix(in srgb, var(--bg) 88%, transparent); backdrop-filter: blur(10px);
}
.site-header__bar { display: flex; align-items: center; gap: 1.5rem; min-height: 4rem; }
.site-brand { display: inline-flex; align-items: center; gap: 0.6rem; color: var(--text); text-decoration: none; font-weight: 700; font-size: 1.1rem; }
.site-brand__mark {
  display: inline-grid; place-items: center; width: 2rem; height: 2rem; border-radius: 0.6rem;
  background: var(--brand); color: var(--brand-contrast); font-size: 1rem; line-height: 1;
}
.site-nav { margin-inline: auto; }
.site-nav__list { display: flex; align-items: center; gap: 0.25rem; }
.site-body .site-nav__link {
  display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.45rem 0.75rem; border: 0; border-radius: 0.5rem;
  background: transparent; color: var(--site-muted); font: inherit; font-size: 0.95rem; font-weight: 500;
  text-decoration: none; cursor: pointer;
}
.site-body .site-nav__link:hover, .site-body .site-nav__link[aria-current] { color: var(--text); background: var(--site-soft); }
.site-nav__menu { min-width: 15rem; }
.site-nav__menu a { text-decoration: none; color: inherit; }
.site-header__actions { display: flex; align-items: center; gap: 0.5rem; margin-inline-start: auto; }
/* Sélecteur de langue (FR / EN) : liens vers la même page dans l'autre langue. */
.site-lang__list { display: flex; align-items: center; gap: 0.125rem; margin: 0; padding: 0; list-style: none; }
.site-lang__link {
  display: inline-flex; align-items: center; gap: 0.35rem; height: 2.25rem; padding: 0 0.5rem; border-radius: 0.5rem;
  font-size: 0.875rem; font-weight: 500; line-height: 1; color: var(--site-muted); text-decoration: none;
}
.site-lang__link:hover { color: var(--text); background: var(--site-soft); }
.site-lang__link[aria-current] { color: var(--text); font-weight: 600; text-decoration: underline; text-underline-offset: 0.3em; }
.site-header__mobile { display: none; }
.site-mobile__nav { padding: 0.5rem 0; }
.site-mobile__list { display: grid; gap: 0.15rem; }
.site-mobile__link {
  display: flex; width: 100%; align-items: center; justify-content: space-between; padding: 0.7rem 0.75rem; border: 0;
  border-radius: 0.5rem; background: transparent; color: var(--text); font: inherit; font-weight: 500; text-align: start;
  text-decoration: none; cursor: pointer;
}
.site-mobile__link:hover, .site-mobile__link[aria-current] { background: var(--site-soft); }
.site-mobile__sub { display: grid; gap: 0.1rem; padding: 0.25rem 0 0.5rem 1rem; }
.site-mobile__sub a { display: block; padding: 0.45rem 0.75rem; border-radius: 0.5rem; color: var(--site-muted); text-decoration: none; }
.site-mobile__sub a:hover, .site-mobile__sub a[aria-current] { color: var(--text); background: var(--site-soft); }
.site-mobile__cta { width: 100%; }
.site-mobile__cta + .site-mobile__cta { margin-top: 0.5rem; }
@media (max-width: 60rem) {
  .site-nav, .site-header__cta, .site-header__account { display: none; }
  .site-header__mobile { display: block; }
}
/* Téléphone : drapeaux seuls (le code reste le nom accessible du lien), sinon 4 langues débordent de l'écran. */
@media (max-width: 30rem) {
  .site-lang__link { padding: 0 0.375rem; }
  .site-lang__link span {
    position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap;
  }
  .site-lang__link[aria-current] { background: var(--site-soft); }
}

/* ---------- sections ---------- */
.site-section { padding-block: clamp(3rem, 7vw, 5rem); }
.site-section + .site-section { padding-top: 0; }
.site-section__head { display: flex; flex-wrap: wrap; align-items: end; justify-content: space-between; gap: 1rem; margin-bottom: 2rem; }
.site-section__title { font-size: clamp(1.6rem, 3vw, 2.1rem); }
.site-lead { margin-top: 0.75rem; max-width: 42rem; color: var(--site-muted); font-size: 1.1rem; }
.site-eyebrow { margin-bottom: 0.75rem; color: var(--brand-text); font-size: 0.85rem; font-weight: 700; letter-spacing: 0.08em; text-transform: uppercase; }
.site-muted { color: var(--site-muted); font-size: 0.9rem; }

.site-hero { padding-block: clamp(4rem, 10vw, 7rem); background: radial-gradient(60rem 30rem at 50% -10%, color-mix(in srgb, var(--brand) 16%, transparent), transparent); }
.site-hero__inner { display: grid; justify-items: center; gap: 1.25rem; text-align: center; }
.site-hero__title { max-width: 52rem; font-size: clamp(2.3rem, 6vw, 3.8rem); letter-spacing: -0.03em; }
.site-hero__lead { max-width: 40rem; color: var(--site-muted); font-size: 1.2rem; }
.site-hero__actions { display: flex; flex-wrap: wrap; justify-content: center; gap: 0.75rem; margin-top: 0.75rem; }

.site-page-header { padding-block: clamp(3rem, 7vw, 4.5rem) clamp(2rem, 5vw, 3rem); border-bottom: 1px solid var(--site-line); background: var(--site-soft); }
.site-page-header .site-container { display: grid; justify-items: start; gap: 0.25rem; }
.site-page-header__title { font-size: clamp(2rem, 5vw, 3rem); letter-spacing: -0.02em; }
.site-page-header [data-slot="breadcrumb"] { margin-bottom: 1rem; }
.site-page-header [data-slot="badge"] { margin-top: 1rem; }

.site-section--stats { padding-block: 2.5rem; }
.site-stats__grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr)); gap: 1rem; margin: 0; }
.site-stat { display: flex; flex-direction: column-reverse; gap: 0.25rem; padding: 1.25rem; border: 1px solid var(--site-line); border-radius: var(--site-radius); text-align: center; }
.site-stat__value { margin: 0; color: var(--brand-text); font-size: 2rem; font-weight: 800; line-height: 1.1; }
.site-stat__label { color: var(--site-muted); font-size: 0.9rem; }

.site-grid { display: grid; gap: 1.25rem; }
.site-grid--3 { grid-template-columns: repeat(auto-fill, minmax(min(100%, 18rem), 1fr)); }
.site-grid--4 { grid-template-columns: repeat(auto-fill, minmax(min(100%, 14rem), 1fr)); }
.site-card { height: 100%; }
.site-card__title { font-size: 1.15rem; }
.site-card__title a { color: inherit; text-decoration: none; }
.site-card__title a:hover { text-decoration: underline; }
.site-card__text { color: var(--site-muted); }
.site-card__footer { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 0.5rem; margin-top: auto; }
.site-icon-box {
  display: inline-grid; place-items: center; width: 2.75rem; height: 2.75rem; margin-bottom: 0.75rem; border-radius: 0.75rem;
  background: color-mix(in srgb, var(--brand) 14%, transparent); color: var(--brand-text);
}
.site-icon-box--lg { width: 3.5rem; height: 3.5rem; margin-bottom: 1rem; }
.site-meta { display: flex; flex-wrap: wrap; align-items: center; gap: 0.5rem; color: var(--site-muted); font-size: 0.9rem; }
.site-page-header .site-meta { margin-top: 0.75rem; }

.site-quote svg { color: var(--brand-text); opacity: 0.6; }
.site-quote__text { margin: 0.75rem 0 0; font-size: 1.05rem; }
.site-person { display: flex; align-items: center; gap: 0.75rem; }
.site-person__name { font-weight: 600; }
.site-person__role { color: var(--site-muted); font-size: 0.9rem; }

.site-faq__list { border-top: 1px solid var(--site-line); }
.site-faq__item { border-bottom: 1px solid var(--site-line); }
.site-faq__question {
  display: flex; align-items: center; justify-content: space-between; gap: 1rem; padding: 1rem 0;
  font-weight: 500; cursor: pointer; list-style: none; border-radius: 0.375rem;
}
.site-faq__question::-webkit-details-marker { display: none; }
.site-faq__question:hover span { text-decoration: underline; text-underline-offset: 0.2em; }
.site-faq__question .site-icon { color: var(--site-muted); transition: transform 0.2s; }
.site-faq__item[open] .site-faq__question .site-icon { transform: rotate(180deg); }
.site-faq__answer { padding-bottom: 1rem; color: var(--site-muted); }

.site-cta {
  display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 1.5rem; padding: clamp(2rem, 5vw, 3rem);
  border-radius: calc(var(--site-radius) * 1.5); background: var(--brand); color: var(--brand-contrast);
}
.site-body .site-cta__title { color: inherit; font-size: clamp(1.5rem, 3vw, 2rem); }
/* pas d'opacité : le couple --brand / --brand-contrast est calculé pour 4.5:1, une opacité le ferait tomber dessous */
.site-cta__text { margin-top: 0.5rem; }

.site-team { text-align: center; }
.site-team__member { display: grid; justify-items: center; gap: 0.35rem; padding: 1.5rem 1rem; border: 1px solid var(--site-line); border-radius: var(--site-radius); }
.site-team__member [data-slot="avatar"] { margin-bottom: 0.5rem; }

.site-contact__grid { display: grid; grid-template-columns: minmax(0, 1.6fr) minmax(0, 1fr); gap: 1.5rem; align-items: start; }
.site-form { display: grid; gap: 1.1rem; margin-top: 0.5rem; }
.site-form__notice { margin-bottom: 1rem; }
.site-field { display: grid; gap: 0.4rem; }
.site-field__error { color: var(--tone-error, #b91c1c); font-size: 0.875rem; }
.site-required { margin-inline-start: 0.2rem; color: var(--tone-error, #b91c1c); }
.site-form [type="submit"] { justify-self: start; }
.site-contact__list { display: grid; gap: 1rem; }
.site-contact__list li { display: flex; align-items: flex-start; gap: 0.75rem; }
.site-contact__list svg { flex: none; margin-top: 0.2rem; color: var(--brand-text); }
.site-contact__list a { color: inherit; }
@media (max-width: 48rem) { .site-contact__grid { grid-template-columns: 1fr; } }

.site-detail { display: grid; grid-template-columns: minmax(0, 1fr) 20rem; gap: 2.5rem; align-items: start; }
.site-detail__aside { position: sticky; top: 5.5rem; }
.site-checklist { display: grid; gap: 0.6rem; }
.site-checklist li { display: flex; gap: 0.5rem; align-items: flex-start; }
.site-checklist svg { flex: none; margin-top: 0.2rem; color: var(--brand-text); }
.site-block { width: 100%; }
@media (max-width: 56rem) { .site-detail { grid-template-columns: 1fr; } .site-detail__aside { position: static; } }
.site-article__back { margin-top: 2.5rem; }

.site-404 { text-align: center; }
.site-404 .site-container { display: grid; justify-items: center; gap: 0.75rem; }
.site-404__code { color: var(--brand-text); font-size: clamp(4rem, 14vw, 7rem); font-weight: 800; line-height: 1; }

/* ---------- pied de page ---------- */
.site-footer { margin-top: clamp(3rem, 7vw, 5rem); border-top: 1px solid var(--site-line); background: var(--site-soft); font-size: 0.95rem; }
.site-footer__grid { display: grid; grid-template-columns: minmax(0, 1.6fr) repeat(4, minmax(0, 1fr)); gap: 2rem; padding-block: 3rem; }
.site-footer__tagline { margin-top: 0.75rem; max-width: 22rem; color: var(--site-muted); }
.site-social { display: flex; gap: 0.5rem; margin-top: 1rem; }
.site-social a {
  display: inline-grid; place-items: center; width: 2.25rem; height: 2.25rem; border: 1px solid var(--site-line); border-radius: 0.6rem;
  color: var(--site-muted);
}
.site-social a:hover { color: var(--text); border-color: var(--text); }
.site-body .site-footer__title { margin-bottom: 0.9rem; font-size: 0.8rem; letter-spacing: 0.08em; text-transform: uppercase; color: var(--site-muted); }
.site-footer__links { display: grid; gap: 0.5rem; }
.site-footer__links a, .site-footer__contact a { color: var(--text); text-decoration: none; }
.site-footer__links a:hover, .site-footer__contact a:hover { color: var(--brand-text); text-decoration: underline; }
.site-footer__contact { display: grid; gap: 0.5rem; }
.site-footer__contact span { color: var(--site-muted); }
.site-footer__bottom { padding-block: 1.25rem; border-top: 1px solid var(--site-line); color: var(--site-muted); font-size: 0.875rem; }
@media (max-width: 60rem) { .site-footer__grid { grid-template-columns: repeat(2, minmax(0, 1fr)); } .site-footer__about { grid-column: 1 / -1; } }
@media (max-width: 30rem) { .site-footer__grid { grid-template-columns: 1fr; } }

/* Pied de carte : même retrait dans tous les kits (Starwind ajoute fond, filet et marge propres). */
.site-body .site-card > [data-slot='card-footer'] { padding: 0 1.5rem; background: transparent; border-top: 0; border-radius: 0; }
.site-body .site-card:has(> [data-slot='card-footer']) { padding-bottom: 1.5rem; }

/* Contenu riche (Markdown, RichText) : même typographie quel que soit le kit (Prose CSS ou sw-prose de Starwind). */
.site-body .site-prose { max-width: 65ch; font-size: 1rem; line-height: 1.75; color: var(--text); }
.site-body .site-prose :is(p, ul, ol, blockquote, pre, table, figure) { margin: 1.25em 0; }
.site-body .site-prose h2 { margin: 2em 0 0.8em; font-size: 1.5em; line-height: 1.3; }
.site-body .site-prose h3 { margin: 1.6em 0 0.6em; font-size: 1.25em; line-height: 1.4; }
.site-body .site-prose :is(ul, ol) { padding-inline-start: 1.5em; }
.site-body .site-prose ul { list-style: disc; }
.site-body .site-prose ol { list-style: decimal; }
.site-body .site-prose li { margin: 0.4em 0; }
.site-body .site-prose a { color: var(--brand-text, var(--brand)); text-decoration: underline; }
.site-body .site-prose strong { font-weight: 600; }
.site-body .site-prose blockquote { padding-inline-start: 1em; border-inline-start: 3px solid var(--site-line); color: var(--site-muted); }
.site-body .site-prose code { padding: 0.1em 0.35em; border-radius: 0.35rem; background: var(--site-soft); font-size: 0.9em; }
.site-body .site-prose > :first-child { margin-top: 0; }
.site-body .site-prose > :last-child { margin-bottom: 0; }

/* Cartes : même filet, ombre et arrondi (Starwind dessine un anneau en box-shadow, le kit CSS une bordure). */
.site-body .site-card { border: 1px solid var(--border); border-radius: 0.75rem; box-shadow: 0 1px 2px rgb(0 0 0 / 0.08); background: var(--card); color: var(--card-foreground); }

/* Formes communes des composants dans le site (Starwind et kit CSS ont des tailles par défaut différentes) :
   seules les dimensions sont fixées ici, les couleurs restent celles des variantes du kit. */
body.site-body [data-slot='button'] { height: 2.75rem; padding: 0 1.25rem; gap: 0.375rem; font-size: 1rem; font-weight: 500; line-height: 1; border-radius: 0.5rem; }
body.site-body [data-slot='button'][data-size='sm'] { height: 2.25rem; padding: 0 1rem; font-size: 0.875rem; }
body.site-body [data-slot='button'][data-size='lg'] { height: 3rem; padding: 0 2rem; font-size: 1.125rem; }
body.site-body [data-slot='button']:is([data-size='icon'], [data-size='icon-sm']) { width: 2.25rem; height: 2.25rem; padding: 0; }
body.site-body [data-slot='button'] svg { width: 1.125rem; height: 1.125rem; }
body.site-body [data-slot='badge'] { padding: 0.125rem 0.75rem; gap: 0.375rem; font-size: 0.875rem; font-weight: 500; line-height: 1.4; border-radius: 9999px; }
body.site-body [data-slot='label'] { margin: 0; display: flex; align-items: center; gap: 0.25rem; font-size: 1rem; font-weight: 500; line-height: 1.2; }
body.site-body :is([data-slot='input'], [data-slot='textarea']) {
  width: 100%; padding: 0 0.75rem; font: inherit; font-size: 1rem; color: var(--text); background: var(--bg);
  border: 1px solid var(--border); border-radius: 0.5rem; box-shadow: 0 1px 2px rgb(0 0 0 / 0.05);
}
body.site-body [data-slot='input'] { height: 2.75rem; }
body.site-body [data-slot='textarea'] { min-height: 5rem; padding-block: 0.5rem; }
body.site-body [data-slot='breadcrumb-list'] { display: flex; flex-wrap: wrap; align-items: center; gap: 0.375rem; margin: 0; padding: 0; font-size: 0.875rem; color: var(--text-muted); }
body.site-body [data-slot='breadcrumb-item'] { display: inline-flex; align-items: center; gap: 0.375rem; font-size: inherit; }
body.site-body [data-slot='breadcrumb-link'] { color: inherit; font-size: inherit; text-decoration: none; }
body.site-body [data-slot='breadcrumb-link']:hover { color: var(--text); text-decoration: underline; }
body.site-body [data-slot='breadcrumb-page'] { color: var(--text); font-weight: 500; font-size: inherit; }
body.site-body [data-slot='breadcrumb-separator'] svg { width: 0.875rem; height: 0.875rem; }
[dir='rtl'] [data-slot='breadcrumb-separator'] svg { transform: scaleX(-1); }
/* Croix du menu mobile : même place sur les 6 combinaisons (Starwind serveur n'a pas de règle pour elle), côté fin de
   ligne, donc à gauche en RTL. */
body.site-body [data-slot='sheet-close'] { position: absolute; inset-block-start: 1rem; inset-inline-end: 1rem; inset-inline-start: auto; display: inline-flex; padding: 0.25rem; color: var(--text-muted); background: transparent; border: 0; border-radius: 0.375rem; cursor: pointer; }
body.site-body [data-slot='sheet-close']:hover { color: var(--text); }
body.site-body [data-slot='sheet-close'] svg { width: 1.25rem; height: 1.25rem; }
body.site-body [data-slot='theme-toggle'] { width: 2.25rem; height: 2.25rem; min-width: 0; padding: 0; border-radius: 0.5rem; }
body.site-body .site-required { margin: 0; color: var(--site-error); }
body.site-body blockquote.site-quote__text { padding: 0; border: 0; color: var(--text); font-style: normal; }
body.site-body [data-slot='badge'] { border: 1px solid transparent; }
body.site-body [data-slot='badge'][data-variant='outline'] { border-color: var(--border); color: var(--text); background: transparent; }
body.site-body [data-slot='button'][data-variant='outline'] { color: var(--text); background: var(--bg); border: 1px solid var(--border); }
body.site-body [data-slot='button'][data-variant='ghost'] { color: var(--text); background: transparent; border: 1px solid transparent; }
body.site-body [data-slot='button'][data-variant='primary'] { border: 1px solid transparent; }
body.site-body [data-slot='avatar'] { position: relative; display: inline-block; flex-shrink: 0; width: 2.5rem; height: 2.5rem; margin: 0; overflow: hidden; font-size: 0.875rem; line-height: 1.6; color: var(--text); background: var(--surface-2); border: 2px solid var(--border); border-radius: 9999px; }
body.site-body [data-slot='avatar-fallback'] { position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: inherit; font-weight: 500; line-height: inherit; background: transparent; border-radius: 9999px; }

/* ---------- comptes (--auth) : connexion, inscription, compte ---------- */
.site-auth .site-container--narrow { max-width: 28rem; }
.site-auth__title { margin: 0; font-size: 1.6rem; font-weight: 700; line-height: 1.2; }
.site-auth .site-lead { margin: 0.35rem 0 0; font-size: 1rem; }
.site-auth .site-icon-box { margin-bottom: 0.75rem; }
.site-auth__card .site-form [type="submit"] { justify-self: stretch; }
.site-auth__alt { display: flex; flex-wrap: wrap; gap: 0.35rem; justify-content: center; font-size: 0.9rem; color: var(--site-muted); }
.site-auth__alt a { color: var(--text); font-weight: 600; }
.site-auth__details { display: grid; grid-template-columns: auto 1fr; gap: 0.5rem 1.25rem; margin: 0; }
.site-auth__details dt { color: var(--site-muted); }
.site-auth__details dd { margin: 0; font-weight: 500; overflow-wrap: anywhere; }

/* Écriture de droite à gauche (dir="rtl", ex. arabe) : la mise en page suit les propriétés logiques ci-dessus ; seules
   les flèches de direction sont retournées. */
[dir='rtl'] .site-icon[data-icon^='arrow-'] { transform: scaleX(-1); }
