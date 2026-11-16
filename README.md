# AlphaRust

Fondation de qualité pour vos projets : **97 règles** (regex et analyses sémantiques) auto-testées, **8 profils**,
**générateurs** qui s'appuient sur les **outils officiels** (create-astro, composer, `wagtail start`, `cargo new`),
installent réellement les dépendances et vérifient le résultat, et trois interfaces — **CLI, TUI et GUI web** —
sur un même moteur. Sortie **SARIF 2.1.0** pour le code scanning GitHub.

![Générer un projet dans la GUI](docs/screenshots/gui-generate.png)

## Installation

Prérequis : Rust ≥ 1.85.

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
./target/release/alpharust list
```

`alpharust` retrouve `rules/`, `profiles/` et `sites/` depuis n'importe quel dossier (voir `ALPHARUST_HOME`, `docs/cli.md`).

## Usage

### CLI

```bash
alpharust lint                         # lint du dossier courant (config .alpharust.toml)
alpharust lint src --profile strict    # un profil (alpharust list --profiles)
alpharust lint --sarif > out.sarif     # SARIF 2.1.0
alpharust lint --json --fail-on warning
alpharust list                         # règles ; --profiles ; --checkers
alpharust check-rules                  # valide règles, exemples et profils
alpharust doctor                       # outils détectés (node, php, composer, python, cargo…)
alpharust new                          # assistant interactif de génération
alpharust generate astro vitrine -i tailwind,mdx,sitemap --site https://exemple.com --pm pnpm --ts strictest
alpharust generate laravel boutique --starter breeze-livewire --pest
alpharust generate astro demo --dry-run   # affiche les commandes officielles exactes, n'écrit rien
alpharust cache status                 # cache des règles ; cache clear
```

Codes de sortie : `0` OK · `1` violations au-dessus du seuil · `2` erreur. Détails : [`docs/cli.md`](docs/cli.md).

![Lint en CLI](docs/screenshots/cli-lint.png)

![Génération Astro en CLI (étapes, commandes officielles, vérifications)](docs/screenshots/cli-generate-astro.png)

### TUI

```bash
alpharust tui
```

Menu interactif (lint, génération avec journal en direct, doctor, listes), profil avec `p`, annulation avec `x`.
Voir [`docs/interfaces.md`](docs/interfaces.md).

![TUI : lint](docs/screenshots/tui-lint.png)

![TUI : génération Astro avec journal en direct](docs/screenshots/tui-generate.png)

### GUI web

```bash
alpharust gui            # http://127.0.0.1:3000
```

Application web d'une page (thème clair/sombre, responsive, accessible, sans ressource externe) : tableau de bord,
**assistant de génération** (toutes les options, aperçu des commandes officielles, **journal en direct** en SSE),
projets, lint filtrable (sévérité, texte, règle ; export JSON/SARIF), recherche dans les règles. Écoute uniquement sur `127.0.0.1`.

![Progression en direct](docs/screenshots/gui-job-live.png)
![Lint filtrable](docs/screenshots/gui-lint.png)
![Tableau de bord](docs/screenshots/gui-dashboard.png)
![Site Astro généré, dans un navigateur](docs/screenshots/generated-astro-site.png)

## Fonctionnalités

- **Règles** Markdown + frontmatter YAML (`rules/`), validées et auto-testées par leurs exemples — [`docs/rule-format.md`](docs/rule-format.md), [`docs/writing-rules.md`](docs/writing-rules.md).
- **Checkers sémantiques** : HTML (alt, lang, titres, sémantique), Markdown, contraste CSS WCAG, lexer Rust (`unsafe`), indentation, cohérence des dépendances, compagnons requis, présence de fichiers — [`docs/checks.md`](docs/checks.md).
- **Tiers** : secrets et mauvaises configurations pour AWS, GCP, GitHub, Slack, OpenAI, Twilio, Azure, SendGrid, Postmark, Brevo, Stripe, Adyen, Square, Braintree, Paddle, Auth0, Clerk, Supabase, Firebase, MongoDB, Redis, PlanetScale…
- **Profils** (`core`, `starter`, `strict`, `web`, `ecommerce`, `cms`, `secrets`, `third-party`) — [`docs/profiles.md`](docs/profiles.md).
- **Configuration** `.alpharust.toml` (profil, exclusions, désactivation, sévérités, `fail_on`), suppression `alpharust-ignore`, **cache** des règles — [`docs/configuration.md`](docs/configuration.md).
- **Générateurs** : outil officiel → installation → couche de modèles → vérification réelle (types, build, tests, lint) ; Astro (npm/pnpm/yarn/bun, Tailwind, MDX, sitemap, React/Vue/Svelte, TypeScript), Laravel (Breeze, Pest), Wagtail, Rust — [`docs/generators.md`](docs/generators.md).

![Tests](docs/screenshots/cli-tests.png)

## Développement

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
./target/release/alpharust check-rules
./target/release/alpharust lint --profile strict     # le dépôt passe son propre lint
```

La CI (`.github/workflows/ci.yml`) exécute ces étapes, puis génère **pour de vrai** des projets Astro, Laravel, Wagtail et Rust
(`ALPHARUST_E2E=1 cargo test --test generators e2e`).
Voir [`docs/tests.md`](docs/tests.md), [`docs/arborescence.md`](docs/arborescence.md), les décisions dans [`adr/`](adr/README.md)
(dernier en date : ADR 0011) et la feuille de route dans [`todos/`](todos/).

## License

MIT — voir [`LICENSE`](LICENSE).
