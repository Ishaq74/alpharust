# TODO AlphaRust

## Terminé
- [x] Structure intégrale
- [x] Parser + Engine + Composer + CLI
- [x] TUI fonctionnelle avec actions réelles (testée via `TestBackend`)
- [x] GUI web fonctionnelle (échappement HTML, contrôle `Origin`)
- [x] Profils + config `.alpharust.toml`
- [x] Sortie SARIF 2.1.0
- [x] Générateurs (astro, laravel, wagtail, rust) fondés sur les outils officiels, avec installation et vérification réelles (ADR 0011)
- [x] Dossier `sites/`
- [x] Tests unitaires, d'intégration, CLI, GUI, générateurs
- [x] CI : `fmt --check`, `clippy -D warnings`, `cargo test --all-targets`, projets générés
- [x] ADR + docs

## Améliorations (terminées dans l'ADR 0010)
- [x] Plus de third-parties concrets (23 règles : AWS, GCP, GitHub, Slack, OpenAI, Twilio, Azure, SendGrid, Postmark, Brevo, Adyen, Square, Braintree, Paddle, Auth0, Clerk, Supabase, Firebase, MongoDB, Redis, PlanetScale)
- [x] Checkers sémantiques (au-delà du regex) : HTML, Markdown, CSS/WCAG, lexer Rust, dépendances, projet
- [x] Générateurs de projets plus complets (Wagtail / Astro / Laravel exécutables, testés en CI)
- [x] Cache des règles (`.alpharust-cache/`) et regex compilées une seule fois

## Refonte ADR 0011
- [x] Pipeline `scaffold` : preflight, plan, exécution en événements, couche de modèles, vérification, rapport
- [x] Astro : npm/pnpm/yarn/bun, templates officiels, TypeScript base/strict/strictest, Tailwind/MDX/sitemap/React/Vue/Svelte
- [x] Laravel : Breeze (blade/livewire/react/vue/api), Pest, mode sombre, base de données
- [x] CLI `new` (assistant), `generate` (options, `--dry-run`, `--json`), `doctor`
- [x] TUI : génération en arrière-plan avec journal en direct
- [x] GUI SPA : assistant + aperçu, SSE, lint filtrable, recherche de règles, thème clair/sombre, responsive, accessible
- [x] Tests hermétiques (faux outils) + tests réels `ALPHARUST_E2E=1`

## Plateforme Astro (ADR 0012)
- [x] Starwind (outil officiel), Concordia (tous les composants), thème WCAG (themecsutomizer)
- [x] Drizzle PostgreSQL/SQLite, Better Auth + plugins + audit, i18n fr/en/es/ar (RTL), Docker, CI
- [x] Préréglages, `projects`, `run`, GUI (projets, historique, thème en direct), TUI
- [x] Tests e2e réels (SQLite, Concordia, PostgreSQL)

## Design kit partagé (ADR 0013, 0014)
- [x] `src/styles` unifié, 46 atomes Starwind d'atomic, Concordia verbatim (couche `concordia`)
- [x] Les 46 composants en CSS pur (même API, `data-slot`, événements), page `/components/advanced` pour les deux kits
- [x] Laravel (`<x-ds.*>`, Vite + Tailwind v4) et Wagtail (app `ds`) : catalogue core, feuilles des 46 composants
- [x] Générateur versionné `tools/dsgen`, contrôle CI régénération + diff, parité statique et runtime

## Idées pour la suite
- [ ] Gabarits Blade et Django des 23 composants étendus (les feuilles CSS et `ds.js` sont déjà livrées)
- [ ] Générateur WordPress (wp-cli) — reporté, voir ADR 0012
- [ ] Envoi d'e-mails (vérification, invitations) dans la plateforme Astro
- [ ] Vérification `docker build` en CI (le Dockerfile généré n'est pas construit aujourd'hui)
- [ ] Export zip d'un projet généré depuis la GUI
- [ ] Générateur Next.js (les règles `frameworks/nextjs` existent déjà)
- [ ] Checkers JS/TS basés sur un vrai parseur (tree-sitter)
- [ ] Mode `--fix` pour les règles à correction triviale
- [ ] Publication du binaire (releases GitHub) et `cargo install alpharust`
