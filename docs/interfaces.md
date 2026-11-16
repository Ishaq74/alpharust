# TUI et GUI

Les trois interfaces (CLI, TUI, GUI) appellent le même moteur (`engine::lint`, `generators::generate`).

## TUI (`alpharust tui`)

Menu à gauche, résultat à droite (défilable), barre d'aide en bas. Nécessite un terminal interactif.

| Touche | Action |
|--------|--------|
| ↑ ↓ / j k | Naviguer |
| Entrée | Lancer l'action (lint, générer Wagtail / Astro / Laravel / Rust, lister règles / profils / checkers) |
| p | Changer de profil (cycle, puis « toutes les règles ») |
| PgUp / PgDn | Défiler le résultat |
| q / Échap / Ctrl-C | Quitter (le terminal est toujours restauré) |

Une génération demande d'abord le nom du projet (validé comme en CLI).

## GUI (`alpharust gui [--port 3000]`)

Serveur local `127.0.0.1` uniquement.

| Page | Contenu |
|------|---------|
| `/` | Tableau de bord : nombre de règles par sévérité, profils, checkers, projets générés |
| `/rules?q=&severity=` | Table filtrable des règles ; `/rule?id=core/no-eval` détaille une règle |
| `/generate` | Formulaire (Wagtail, Astro, Laravel, Rust) + liste des projets ; affiche les fichiers créés |
| `/lint?path=&profile=` | Formulaire + résultats détaillés (sévérité, fichier:ligne, règle, message) |
| `/checkers` | Catalogue des checkers et règles associées |
| `/api/rules`, `/api/lint`, `/api/sarif` | JSON / SARIF (mêmes paramètres que `/lint`) |

Sécurité : tout texte dynamique est échappé ; les noms de projet sont validés ; les `POST` dont l'en-tête `Origin`
n'est pas l'hôte local sont refusés (403).
