# AlphaRust

Fondation intégrale pour les primitives, les règles, la génération et l’orchestration de plateformes.

Prêt-à-porter sur mesure. Source de vérité unique. Binaire Rust.

## Installation

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
```

## Usage

```bash
# Lister les règles
./target/release/alpharust list

# Lister les profils
./target/release/alpharust list --profiles

# Linter
./target/release/alpharust lint
./target/release/alpharust lint --profile strict

# Initialiser un projet
./target/release/alpharust init mon-projet

# Générer un README
./target/release/alpharust readme mon-projet

# Générer de la documentation
./target/release/alpharust docs mon-projet
```

## Structure

Voir [docs/arborescence.md](docs/arborescence.md).

## Principes

- `rules/` = source de vérité
- Profils pour l’orchestration
- Générateurs intégrés
- Domaines complets avec third-parties
- CI obligatoire

## Licence

MIT
