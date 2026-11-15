# AlphaRust

Fondation intégrale pour les primitives, les règles, la génération et l’orchestration de plateformes.

## Installation

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
```

## Usage

```bash
# Lint
./target/release/alpharust lint
./target/release/alpharust lint --profile strict
./target/release/alpharust lint --sarif

# Lister
./target/release/alpharust list
./target/release/alpharust list --profiles

# Initialiser
./target/release/alpharust init mon-projet

# Générer un projet framework
./target/release/alpharust generate wagtail mon-site
./target/release/alpharust generate astro mon-site
./target/release/alpharust generate laravel mon-site

# Générer README / Docs
./target/release/alpharust readme mon-projet
./target/release/alpharust docs mon-projet
```

## Configuration

Créer un `.alpharust.toml` :

```toml
profile = "strict"
```

## Licence

MIT
