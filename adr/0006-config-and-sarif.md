# ADR 0006 — Configuration projet et sortie SARIF

## Statut
Accepté

## Contexte
Pour être utilisable en production, AlphaRust doit pouvoir être configuré par projet et produire une sortie standard pour les outils CI.

## Décision
- Support de `.alpharust.toml` à la racine (profil par défaut)
- Option `--sarif` pour une sortie SARIF 2.1.0
- Les templates et générateurs de frameworks sont documentés

## Conséquences
AlphaRust peut être intégré dans des pipelines existants et configuré par projet.
