# AlphaRust

Fondation intégrale pour les primitives, les règles, la génération et l’orchestration de plateformes.

Prêt-à-porter sur mesure. Source de vérité unique. Binaire Rust autonome.

## Installation

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
```

## Usage

```bash
# Lister toutes les règles
./target/release/alpharust list

# Linter un projet
./target/release/alpharust lint .

# Initialiser un projet
./target/release/alpharust init mon-projet

# Générer un README
./target/release/alpharust readme mon-projet
```

## Structure

Voir `docs/arborescence.md` pour l’arborescence définitive intégrale.

## Principes

- `rules/` = source de vérité
- `profiles/` = orchestration
- `generators/` = prêt-à-porter
- Domaines complets (database, smtp, payments, styles, design, auth, docs, readme, checks, tests) avec third-parties
- Aucune dette technique

## Licence

MIT
