# ADR 0007 — Dossier sites/

## Statut
Accepté

## Contexte
Les projets générés ne doivent pas polluer la racine d’AlphaRust. Il faut un emplacement clair et prévisible.

## Décision
- Création du dossier `sites/`
- Les commandes `init` et `generate` y installent les projets par défaut
- Un `--path` explicite permet de surcharger

## Conséquences
Séparation claire entre le moteur AlphaRust et les projets générés.
