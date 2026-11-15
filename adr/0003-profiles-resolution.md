# ADR 0003 — Résolution des profils

## Statut
Accepté

## Contexte
Les profils (fichiers `.toml` dans `profiles/`) sélectionnent des ensembles de règles. La résolution actuelle est basée sur des préfixes et des noms.

## Décision
- Un profil déclare une liste de patterns (`core/*`, `domains/database/*`, ou un nom de règle exact)
- La résolution charge toutes les règles puis filtre
- Priorité future : framework > domain > language > core (non encore appliquée finement)

## Conséquences
Les profils sont la façon officielle d’orchestrer. La logique de priorité fine reste à implémenter dans `composer`.
