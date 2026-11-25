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
- [x] Laravel (`<x-ds.*>`, Vite + Tailwind v4) et Wagtail (app `ds`) : les 46 composants et leurs feuilles
- [x] Gabarits Blade et Django des 23 composants étendus, générés par `tools/dsgen/ext_server.py`, pages `/components/advanced`
- [x] Générateur versionné `tools/dsgen`, contrôle CI régénération + diff, parité statique et runtime

## Site vitrine identique (ADR 0015)
- [x] Spec partagée `tools/dsgen/vitrine` : pages, menu, sections, contenus, formulaire ; rendu Astro, Blade, Django
- [x] Astro (Starwind et CSS), Wagtail (modèle wagtail-cms : snippets, réglages, StreamField, `seed_site`), Laravel (modèle manecms)
- [x] `--theme-*` pour Laravel et Wagtail, contrôle de parité (`site_parity.py`, `site_interact.py`) et planches

## Langues, comptes, 6 combinaisons (ADR 0016)
- [x] Kit Tailwind + Starwind côté Laravel et Wagtail (`--ui starwind`) : 6 combinaisons au rendu identique
- [x] Anglais et sélecteur de langue dans les trois frameworks (i18n officielle de chacun), depuis la spec partagée
- [x] Comptes (`--auth`) : Better Auth, Laravel Fortify, auth Django ; liens « Connexion » / « Mon compte » dans l'en-tête
- [x] Icônes MDI par défaut, drapeaux Circle Flags, `--icons <jeu Iconify>`
- [x] Preuves : `site_parity.py` (19 pages × 6), `site_auth.py` (78/78), planches `docs/screenshots/phase10-*`
- [x] README complet (tous profils), docs et ADR à jour

## Langues au choix (ADR 0017)
- [x] Espagnol et arabe (RTL : `dir`, propriétés logiques, flèches et menu mobile retournés) sur les 6 combinaisons
- [x] `--langs` (CLI, assistant, GUI), défaut `fr,en` ; une langue = un fichier `vitrine/locales/<code>.py`
- [x] Dates selon la langue, drapeau Circle Flags par langue
- [x] Preuves : `site_parity.py` (35 pages × 6), `site_auth.py` (120/120), planches `docs/screenshots/phase11-*`

## v0.3.0 « qualité prouvée et publiée » (ADR 0018, audit de novembre 2026)
- [x] 19 constats de l'audit corrigés (C1–C3, I1–I8, M1–M9), détail dans `CHANGELOG.md`
- [x] Palette par défaut unique et contraste WCAG du texte de marque ; contact Astro enregistré ; base SQLite hors Git
- [x] SEO commun (canonical, hreflang, Open Graph, JSON-LD, robots, sitemap multilingue) vérifié par `site_parity.py`
- [x] CSP de base identique en production ; `settings/production.py` Wagtail durci ; `ds.bundle.css`
- [x] CI Windows, macOS, MSRV, cargo-deny ; nightly 6 combinaisons × 4 langues (parité, comptes, axe-core)
- [x] Releases cargo-dist (binaires 3 OS, installateurs shell et PowerShell), `cargo install --git … --tag v0.3.0`
- [x] Preuves : planches et relevés `docs/screenshots/phase12-*`

## À faire
- [ ] Publication sur crates.io (`cargo publish`) : demande le jeton du propriétaire du crate
- [ ] Sous-commandes `generate astro|laravel|wagtail` avec leurs options propres ; retrait de `--i18n` (version majeure)
- [ ] CSP sans `'unsafe-inline'` (nonces) quand l'admin Wagtail le permettra

## Pistes hors périmètre (idées non engagées, aucune tâche ouverte)
- Générateur WordPress (wp-cli) — reporté, voir ADR 0012
- Envoi d'e-mails (vérification, invitations) dans la plateforme Astro
- Vérification `docker build` en CI (le Dockerfile généré n'est pas construit aujourd'hui)
- Export zip d'un projet généré depuis la GUI
- Générateur Next.js (les règles `frameworks/nextjs` existent déjà)
- Checkers JS/TS basés sur un vrai parseur (tree-sitter)
- Mode `--fix` pour les règles à correction triviale
