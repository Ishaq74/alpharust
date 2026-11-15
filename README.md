# AlphaRust

**Fondation intégrale production-ready** pour les primitives, les règles, la génération et l’orchestration de plateformes.

Prêt-à-porter sur mesure. Source de vérité unique. Binaire Rust autonome et robuste.

## Installation

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
```

Le binaire se trouve dans `target/release/alpharust`.

## Usage

```bash
# Lister toutes les règles
alpharust list

# Lister les profils
alpharust list --profiles

# Linter le répertoire courant
alpharust lint

# Linter avec un profil
alpharust lint --profile strict

# Initialiser un projet
alpharust init mon-projet

# Générer un README
alpharust readme mon-projet
```

## Structure

L’arborescence définitive intégrale est documentée dans [`docs/arborescence.md`](docs/arborescence.md).

## Principes

- `rules/` est la seule source de vérité
- Profils pour l’orchestration
- Générateurs intégrés
- Domaines complets avec third-parties
- Gestion d’erreurs robuste
- Tests unitaires

## Licence

MIT
