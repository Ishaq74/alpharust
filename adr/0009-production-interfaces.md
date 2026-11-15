# ADR 0009 — Interfaces production-ready

## Statut
Accepté

## Contexte
Les modes TUI et GUI doivent être fonctionnels et utilisables, pas de simples squelettes.

## Décision
- TUI interactive avec navigation et actions réelles (lint, generate)
- GUI web avec pages règles, formulaire de génération et lint
- Les trois interfaces (CLI, TUI, GUI) appellent le même moteur

## Conséquences
AlphaRust est utilisable en production via n’importe laquelle des trois interfaces.
