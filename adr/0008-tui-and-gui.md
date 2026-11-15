# ADR 0008 — TUI et GUI web

## Statut
Accepté

## Contexte
AlphaRust doit offrir une expérience complète : CLI pour l’automatisation, TUI pour l’usage interactif terminal, et GUI web style forge pour une utilisation visuelle.

## Décision
- Mode `tui` avec ratatui
- Mode `gui` avec axum servant une interface web sur le port 3000
- Les trois modes partagent le même moteur (règles, générateurs, profils)

## Conséquences
Une seule base de code, trois interfaces. La GUI peut évoluer vers une forge complète.
