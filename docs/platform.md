# Briques plateforme (Astro)

Au-delà du squelette, `alpharust generate astro` compose une **plateforme** à partir de briques indépendantes. Chacune
est installée avec l'**outil officiel** quand il existe, puis une couche de modèles pose le code de liaison ; l'ensemble
est ensuite **vérifié pour de vrai** (`astro check`, `astro build`, `vitest`, lint).

| Brique | Option | Outil officiel / source | Ce qui est vérifié |
|--------|--------|-------------------------|--------------------|
| Tailwind + **Starwind UI** | `--ui starwind` | `starwind@latest init --defaults`, puis les **46 composants Starwind d'atomic** (`src/components/atoms`) + effets `wow/` + `lib/starwind` | `astro check` sur ~305 fichiers, build, page `/components` |
| **Concordia** (CSS pur) | `--ui concordia [--css-variant …] [--theme-*]` | copie fidèle du dépôt `Ishaq74/concordia` (jetons, variantes, **tous** les composants) | `astro check` sur les 80+ composants, pages `/components` (catalogue partagé) et `/concordia` (composants propres) |
| Base de données | `--db sqlite\|postgres [--db-url …]` | `drizzle-orm` + `drizzle-kit generate` / `migrate` | migrations appliquées, test d'insertion/lecture |
| Authentification | `--auth [--auth-plugin admin,username,organization]` | **Better Auth** ; schéma Drizzle produit par `auth@latest generate` | inscription, connexion, mauvais mot de passe (tests) + parcours navigateur |
| Langues | `--i18n fr,en,es,ar` | routage i18n natif d'Astro (`prefixDefaultLocale: false`), arabe en RTL | parité des clés de messages (test) |
| Déploiement | `--docker`, `--ci` | adaptateur `@astrojs/node` (`astro add node`) | Dockerfile, compose, workflow GitHub Actions |

Préréglages : `alpharust presets` (`vitrine`, `blog-i18n`, `app-auth`, `saas-pg`, `concordia`) →
`alpharust generate astro mon-site --preset saas-pg`. Les options explicites complètent ou remplacent le préréglage.

Règles de compatibilité (refusées avant toute exécution) : briques plateforme ⇒ template `alpharust` et installation ;
`--auth` ⇒ base de données ; `--i18n` ⇒ au moins deux langues ; Concordia ⇒ pas de Tailwind ; Docker ⇒ npm/pnpm/yarn.

## Ce que contient un projet généré

```text
src/
  database/        env.ts (LOCAL | TEST | PROD), drizzle.ts (connexion paresseuse), schemas/, migrations/
  lib/             auth.ts (Better Auth + plugins + journal d'audit), audit.ts, auth-forms.ts, auth-guards.ts
  i18n/            config.ts, utils.ts, messages/{fr,en,es,ar}.json
  middleware.ts    session → Astro.locals + en-têtes de sécurité
  pages/           auth/{sign-in,sign-up,dashboard}, admin, api/health, api/auth/[...all], [lang]/…
  components/      auth/ (Starwind atoms, Concordia ou HTML simple), LanguageSwitcher
tests/             base de données (TEST), authentification, parité i18n
```

* **Environnements de base** : `DB_ENV=LOCAL|TEST|PROD`, une URL par environnement (`DATABASE_URL_*`). Les tests utilisent
  toujours `TEST` (SQLite en mémoire ou base `<nom>_test`).
* **Authentification** : sessions Better Auth, redirections 303 avec cookies, limites de débit, journal d'audit en base
  (repli fichier), rôle `admin` attribué à `ADMIN_EMAIL`. Le secret `BETTER_AUTH_SECRET` est aléatoire (32 octets) dans `.env`.
* **Aucun identifiant en clair dans les fichiers versionnés** : `.env.example` s'appuie sur `PGUSER`/`PGPASSWORD`, la CI et
  compose lisent leurs mots de passe via variables, et le manifeste `.alpharust-project.json` masque les URL (`://***@`).

## Design system : une arborescence pour tous les kits

Quel que soit le kit (CSS simple, Tailwind, Tailwind + Starwind, Concordia), le projet généré reçoit **la même
arborescence `src/styles/`** — celle des dépôts `concordia` / `astro-css-drizzle-better-auth`
(`global.css`, `base.css`, `tokens/`, `components/`), complétée par `themes/` :

```text
src/styles/                      CSS simple · Tailwind · Starwind          Concordia
├── global.css                   entrée unique (BaseLayout)                idem (importe base.css + variantes)
├── base.css                     remise à zéro, valeurs globales           Concordia (verbatim) : jetons + variante initiale
├── starwind.css                 Starwind seul : `starwind init`           —
├── tokens/
│   ├── colors.css               --background, --primary… (clair/sombre)   Concordia (verbatim)
│   ├── typography.css · spacing.css                                      Concordia (verbatim)
│   ├── components.css           --button-border-radius, --card-padding…   Concordia (verbatim)
│   ├── tailwind.css             @theme (Tailwind / Starwind)              —
│   └── site.css                 pont vers les variables du gabarit        idem, branché sur les jetons Concordia
├── themes/theme.css             palette de marque WCAG (--brand-*)        palette de marque WCAG (--color-*)
└── components/
    ├── layout.css · button.css · custom.css                              layout.css · custom.css + 46 fichiers du catalogue + utilities.css
    ├── card.css                 (CSS simple, Tailwind, Starwind)         retro/modern/futuristic/initial.css (verbatim, couche `concordia`)
    └── input · textarea · select · badge · alert · dialog · tabs · table · tooltip .css   (Starwind seul)
```

Depuis l'ADR 0014, les jetons des kits « CSS simple » et « Tailwind seul » (`tokens/{colors,typography,spacing,components,site}.css`,
`tailwind.css`) et les composants CSS de Concordia ne sont plus des modèles Astro : ils sont **copiés depuis le design kit
partagé** (`templates/layers/design-kit`), le même que celui de Laravel et de Wagtail.

**Comment personnaliser** (détail dans `docs/design.md` du projet) :

1. une valeur de **jeton** (`tokens/components.css`, p. ex. `--button-border-radius: 9999px`) change tous les composants concernés ;
2. la **palette** (`themes/theme.css`) est générée avec contrôle WCAG pour *tous* les kits (`--theme-primary/-secondary/-accent`) ;
   les kits non-Concordia lisent `--brand-*` (aucune collision avec l'espace `--color-*` de Tailwind), Concordia lit ses `--color-*` ;
3. **Starwind** : les composants d'`atoms/` ne sont jamais modifiés. Chacun expose `data-slot="…"` ; `components/<nom>.css`
   cible ce hook dans `@layer overrides`, déclarée *après* les utilitaires Tailwind, et lit les jetons. Un composant sans
   fichier se surcharge dans `components/custom.css` ;
4. **Concordia** : jetons de `tokens/components.css` ou règles dans `components/custom.css`, variantes = fichiers `components/<variante>.css`.

`tokens/site.css` relie les variables du gabarit AlphaRust (`--bg`, `--text`, `--brand`…) aux jetons du kit : le site suit donc
le thème sans réécriture. La page `/components` (Starwind **et** Concordia) montre les 23 composants core avec vos jetons, `/components/advanced`
les 23 autres (dialogues, menus, select, OTP, carrousel, barre latérale, toasts…).

## Design kit partagé : Astro, Laravel, Wagtail (ADR 0014)

Les composants d'atomic existent aussi en **CSS pur**, avec le même balisage (`data-slot`, `data-variant`, `data-size`), les mêmes
noms et props, la même accessibilité (éléments natifs `<details>`, `<dialog>`, `<progress>`, onglets ARIA par `ds.js`). Une
source (`templates/layers/design-kit`) alimente quatre cibles :

| Cible | Option | Où | Utilisation |
|-------|--------|----|-------------|
| Astro + Starwind | `--ui starwind` | `src/components/atoms/` (Tailwind) | `<Button variant="primary">` |
| Astro + CSS | `--ui concordia` | `src/components/atoms/` (CSS pur) + `src/styles/components/*.css` + `src/scripts/ds.js` | `<Button variant="primary">` (mêmes imports `@atoms/…`) |
| Laravel | `generate laravel … --ui css` | `resources/views/components/ds/`, `resources/css/design/`, `resources/js/design/ds.js` | `<x-ds.button variant="primary">`, `<x-ds.select.trigger>` ; pages `/components` et `/components/advanced` |
| Wagtail | `generate wagtail … --ui css` | application `ds/` (`templatetags/ds.py`, `static/ds/`) | `{% ds_button variant="primary" %}…{% endds_button %}`, `{% ds_select_trigger %}` ; pages `/components/` et `/components/advanced/` |

Laravel : Vite + Tailwind v4 (`@tailwindcss/vite`) du squelette officiel, kit dans la couche `ds` ; `npm run build` est exécuté par
la génération, puis `php artisan test` (dont `ComponentsTest`). Wagtail : fichiers statiques Django, sans outil de build ; `INSTALLED_APPS`,
`urls.py` et `base.html` sont branchés par insertion après une ancre, avec échec explicite si le squelette a changé.

Personnaliser (identique partout) : couleurs dans `themes/theme.css`, allure d'un composant par un jeton de `tokens/components.css`,
le reste dans `components/custom.css`. Parité : `src/scaffold/catalog.rs` (voir `docs/tests.md`).

Les **46** composants d'atomic existent en CSS pur pour Astro (23 « core » + 23 « css » dans `catalog.toml`) : mêmes fichiers,
props, `data-slot` et événements Starwind (`starwind-select:change`, `window.__starwind__.toast`…). Ils sont **générés** par
`tools/dsgen/gen_ds.py` (CSS pur Astro, Blade, Django et `catalog.toml`) ; la CI régénère et échoue sur tout écart. Pour changer
un composant : modifier `tools/dsgen/` (ou la feuille `design-kit/css/components/<nom>.css`), lancer `python3 tools/dsgen/gen_ds.py`,
puis `cargo test catalog`. Blade et Django couvrent les **46** composants : les 23 étendus sont décrits une fois dans
`tools/dsgen/ext_server.py` et rendus dans les deux langages (props en camelCase côté Blade — `:as-child="true"`,
`:default-value="[20, 80]"` —, en snake_case côté Django — `as_child=True`, `default_value="20,80"`). Les calculs du curseur et
de l'intégration YouTube vivent dans `app/View/DesignKit.php` (Laravel) et dans `ds.py` (Django). Limites : `--ui css` Laravel exclut Breeze ; le thème WCAG généré reste propre à Astro.

## Starwind : ce qui vient d'atomic

* `src/components/atoms/**` — les composants Starwind d'atomic, **copiés sans modification de code** (46 dossiers : accordion,
  alert, alert-dialog, avatar, badge, button, card, carousel, dialog, dropdown, input, select, sheet, sidebar, table, tabs,
  toast, tooltip…), `src/lib/starwind/positioning.ts`, `src/components/wow/**` (effets).
* `tokens/colors.css` : jeux « Fire Brand » OKLCH d'atomic (`:root` et `.dark`) ; seules les couleurs primaires lisent `--brand-primary`.
* `starwind.config.json` aligné sur atomic (`componentDir: src/components/atoms`, `utilsDir: src/lib/starwind`).
* Non repris : `icon-picker` et `media-picker` (liés à la base média d'atomic).
* Corrections : imports de la documentation (`@components/starwind/…` → `@atoms/…`) ; alias `@atoms/*`, `@starwind/*` ;
  dépendances installées par le gestionnaire de paquets (`tailwind-variants`, `@tabler/icons`, `embla-carousel`, `motion`, `astro-icon`).

## Concordia et themecsutomizer

`--ui concordia` copie sans réécriture `src/styles/{base.css,tokens,components/*}` et `src/components/ui/**`.
Corrections minimales apportées (liste complète dans l'ADR 0012) : `SearchBar` (TypeScript strict), shim
`@lib/i18n/route-helpers`, `concordia.css` (sélection des variantes), `astro-icon` + `@iconify-json/mdi` installés.

`--theme-primary/-secondary/-accent` (tous les kits) génère `src/styles/themes/theme.css` avec l'algorithme WCAG du dépôt
`themecsutomizer` (luminance relative, contraste, AA ≥ 4,5 / AAA ≥ 7, texte lisible blanc ou `#111827`, variante sombre
recalculée sur `#0f172a`). La GUI affiche ce contrôle **en direct** ; `npm run theme:check` le rejoue dans le projet.

## Orchestration

```bash
alpharust projects                       # état : briques, dépendances installées, git, tâches possibles
alpharust run mon-site build             # install | dev | build | check | test | lint | update
alpharust run mon-site update --json     # événements JSON
```

Chaque tâche lance la commande native d'après le manifeste (`pnpm run build`, `php artisan test`, `cargo test`…). La GUI
propose les mêmes actions (page *Projets*, historique des exécutions) ; le serveur de développement reste réservé à la CLI.
