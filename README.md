# AlphaRust

Fondation intégrale pour les primitives, les règles, la génération et l’orchestration de plateformes.

Prêt-à-porter sur mesure. Zéro dépendance runtime hors le binaire. Source de vérité unique.

## Objectif

Encoder ce qui ne change jamais (sécurité, structure, conventions, domaines) dans des règles Markdown. Le binaire Rust les applique, génère, et orchestre. Tout le reste en découle.

## Structure

Voir l’arborescence définitive validée dans `docs/arborescence.md`.

## Démarrage

```bash
cargo build --release
./target/release/alpharust --help
```

## Principes

- Les règles sont la source de vérité.
- Les profils orchestrent les combinaisons.
- Les générateurs produisent le prêt-à-porter.
- Aucune dette technique dès le départ.
