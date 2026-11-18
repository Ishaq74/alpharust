# Tests

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
alpharust check-rules
```

| Suite | Fichier | Contenu |
|-------|---------|---------|
| Unitaires | `src/**` | glob, parser de règles, checkers, runner, config, profils, composer, SARIF, TUI (rendu `TestBackend`, flux d'événements), pipeline de génération (étapes, couche de modèles, validations) |
| Moteur | `tests/integration.rs` | règles réelles contre `tests/fixtures/`, profils, secrets, config, SARIF, auto-lint du dépôt |
| Cache | `tests/cache.rs` | écriture, hit, invalidation, cache corrompu |
| CLI | `tests/cli.rs` | binaire réel : codes de sortie, SARIF/JSON purs, `generate --dry-run/--json`, options incompatibles, `doctor`, `cargo new` réel, noms dangereux, tube fermé, TUI/`new` sans TTY |
| GUI | `tests/gui.rs` | SPA embarquée (CSP, aucun `innerHTML`), API méta/plan/règles/lint, chemins refusés, CSRF (`Origin`/`Host`), **job de génération avec flux SSE** (rejeu, `Last-Event-ID`) |
| Générateurs | `tests/generators.rs` | pipeline hermétique (faux `npm`/`node`), outil absent, échec d'outil, annulation, `--force` sûr, `cargo new` réel ; **`e2e_*` réels** (create-astro npm/pnpm, composer + Breeze + Pest, wagtail) avec `ALPHARUST_E2E=1` |

Les fixtures : `tests/fixtures/violations/` (doit déclencher des règles précises), `clean/` (zéro violation),
`projects/{good,bad}-project/` (checkers de projet).
Les jetons réalistes sont assemblés **à l'exécution** dans les tests pour ne pas ressembler à de vrais secrets.

## Générations réelles

```bash
ALPHARUST_E2E=1 cargo test --test generators e2e -- --test-threads=1 --nocapture
```

Réseau requis (npm, composer, pip). Chaque test détecte les outils et s'ignore avec un message s'ils manquent ;
`PATH` est global au processus : ces tests sont sérialisés (`--test-threads=1` conseillé, verrou interne sinon).

### Plateforme Astro

```bash
ALPHARUST_E2E=1 cargo test --test generators e2e_astro_platform -- --test-threads=1
# PostgreSQL réel : une base vide `<base>` et `<base>_test` doivent exister
ALPHARUST_E2E=1 ALPHARUST_E2E_PG_URL=postgresql://…/ma_base cargo test --test generators e2e_astro_platform_postgres
```

Les trois tests génèrent de vrais projets (SQLite + Better Auth + Starwind + i18n ; Concordia + thème ; PostgreSQL) et
vérifient chaque étape (`starwind-config`, `atoms-deps`, `auth-schema`, `db-migrate`, `check`, `build`, `test`) puis le lint.
Les projets générés exécutent eux-mêmes leurs tests `vitest` (base, authentification, parité i18n).

### Arborescence de design (src/styles)

* **Unitaires** (`src/scaffold/internal.rs`) : `design_system_layout_is_identical_for_every_ui_option` rend les couches de
  chaque kit (CSS simple, Tailwind, Starwind, Concordia) et vérifie l'arborescence, que chaque `@import` de `global.css`
  pointe vers un fichier existant, l'absence de directive non résolue, le thème (`--brand-*` / `--color-*`) et le README ;
  `starwind_components_have_a_file_each_and_target_real_slots` vérifie que chaque `[data-slot='…']` ciblé existe dans les composants ;
  `concordia_keeps_its_files_verbatim…` compare les fichiers Concordia à l'original.
* **e2e réels** : `e2e_astro_design_system_plain_and_tailwind` (palette + `astro check` + build) et les e2e plateforme
  (Starwind + thème, Concordia + thème) vérifient `src/styles/` après installation.

### Design kit partagé et parité du catalogue (ADR 0014)

* **Unitaires** (`src/scaffold/catalog.rs`, `cargo test catalog`) — le contrat `templates/layers/design-kit/catalog.toml`
  contre les fichiers réels :
  * 23 composants `core` et 23 `css`, slots uniques ; chaque composant a sa feuille `css/components/<nom>.css`, importée par
    `index.css`, qui ne cible que des slots du contrat ;
  * les 46 composants : chaque slot est présent dans les atomes CSS pur, les composants Blade (un dossier par composant) et
    les balises Django ;
  * css : mêmes fichiers, mêmes `data-slot` et mêmes exports (hors `*Variants`) que l'atome Starwind d'atomic ;
  * chaque dossier d'atomes d'atomic est déclaré ; les pages `/components` (4 cibles) et `/components/advanced` (Starwind,
    CSS, Laravel, Wagtail) utilisent chaque composant, routes et liens compris ; le kit CSS d'Astro installe les 46 feuilles, `utilities.css` et `ds.js` ;
  * branchement Laravel (refus de Breeze) et Wagtail (insertion idempotente, échec si l'ancre manque).
* **Générateur à jour** (CI) : `python3 tools/dsgen/gen_ds.py && git diff --exit-code templates/layers`.
* **e2e réels** : `ALPHARUST_E2E=1 cargo test --release --test generators design_kit -- --test-threads=1` (Laravel : Vite compile
  le kit, slots étendus compris, `php artisan test` — dont `/components/advanced` : 23 composants et contrat a11y —, puis
  `php artisan view:cache` compile les 184 parties Blade ; Wagtail : `manage.py check` et tests, dont `/components/advanced/`,
  puis contrôle des 184 balises enregistrées et du rendu de la page avancée).
* **Runtime** (preuves) : `parity.py starwind=URL concordia=URL laravel=URL wagtail=URL` compare les `data-slot` rendus par
  les pages `/components` et `/components/advanced` des quatre cibles (écarts tolérés : `[known_gaps]`, Starwind seulement) ;
  `attrdiff.py` compare en plus, élément par élément, tous les attributs de contrat (`data-*`, `aria-*`, `role`, `type`…) du
  CSS pur, de Blade et de Django ; `interact.py` ouvre et pilote les composants de `/components/advanced` dans les quatre
  cibles (dialogue, panneau, menu, select, OTP, curseur, radio, carrousel, barre latérale,
  toast, thème) et relève les erreurs de console. Scripts : `docs/screenshots/scripts/`.

### Site vitrine identique (ADR 0015)

* **Unitaires** : couches et étapes (`site-astro`, `site-laravel`, `site-wagtail`, `seed-site`, `theme`, `makemigrations`) ;
  branchement Wagtail idempotent (apps du site, processeurs de contexte, `LANGUAGE_CODE = "fr"`) dans `catalog.rs`.
* **Générateur à jour** (CI) : la même commande `gen_ds.py && git diff --exit-code templates/layers` couvre les trois couches.
* **Tests des projets générés** (lancés par la génération) : Laravel `tests/Feature/SiteTest.php` (chaque page dans le gabarit,
  page courante marquée, 404 du site, validation et enregistrement du contact) ; Wagtail `home/test_site.py` (mêmes cas, plus
  `seed_site` deux fois sans effet) ; Astro `astro check` + build.
* **Runtime** (preuves) : `site_parity.py SORTIE [SEUIL]` (10 pages × combinaisons, bureau et mobile : signature de structure,
  erreurs de console, écart en pixels avec la référence — code de sortie 1 au-delà du seuil —, planches `site-<page>.png`) et `site_interact.py SORTIE` (sous-menu au survol et au clavier, thème, menu mobile,
  envoi du contact). Serveurs attendus : 4401 Astro Starwind, 4402 Astro CSS, 4403 Laravel, 4404 Wagtail.
