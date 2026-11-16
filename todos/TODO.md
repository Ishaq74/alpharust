# TODO AlphaRust

## Terminé
- [x] Structure intégrale
- [x] Parser + Engine + Composer + CLI
- [x] TUI fonctionnelle avec actions réelles (testée via `TestBackend`)
- [x] GUI web fonctionnelle (échappement HTML, contrôle `Origin`)
- [x] Profils + config `.alpharust.toml`
- [x] Sortie SARIF 2.1.0
- [x] Générateurs (bootstrap, wagtail, astro, laravel) vérifiés avec les vrais outils
- [x] Dossier `sites/`
- [x] Tests unitaires, d'intégration, CLI, GUI, générateurs
- [x] CI : `fmt --check`, `clippy -D warnings`, `cargo test --all-targets`, projets générés
- [x] ADR + docs

## Améliorations (terminées dans l'ADR 0010)
- [x] Plus de third-parties concrets (23 règles : AWS, GCP, GitHub, Slack, OpenAI, Twilio, Azure, SendGrid, Postmark, Brevo, Adyen, Square, Braintree, Paddle, Auth0, Clerk, Supabase, Firebase, MongoDB, Redis, PlanetScale)
- [x] Checkers sémantiques (au-delà du regex) : HTML, Markdown, CSS/WCAG, lexer Rust, dépendances, projet
- [x] Générateurs de projets plus complets (Wagtail / Astro / Laravel exécutables, testés en CI)
- [x] Cache des règles (`.alpharust-cache/`) et regex compilées une seule fois

## Idées pour la suite
- [ ] Générateur Next.js (les règles `frameworks/nextjs` existent déjà)
- [ ] Checkers JS/TS basés sur un vrai parseur (tree-sitter)
- [ ] Mode `--fix` pour les règles à correction triviale
- [ ] Publication du binaire (releases GitHub) et `cargo install alpharust`
