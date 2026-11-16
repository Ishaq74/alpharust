# ADR 0003 — Résolution des profils

## Statut
Accepté

## Contexte
Les profils (fichiers `.toml` dans `profiles/`) sélectionnent des ensembles de règles. La résolution actuelle est basée sur des préfixes et des noms.

## Décision
- Un profil déclare une liste de patterns (`core/*`, `domains/database/*`, ou un nom de règle exact)
- La résolution charge toutes les règles puis filtre
- Priorité : framework > domain > language > core, appliquée par le composer (cf. ADR 0010)

## Conséquences
Les profils sont la façon officielle d’orchestrer. La priorité est implémentée dans `composer` ; les motifs comparent l'id ou le nom de la règle (ADR 0010).
