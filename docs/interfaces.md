# TUI et GUI

Les trois interfaces (CLI, TUI, GUI) appellent le même moteur : `engine::lint` pour l'analyse et
`scaffold::{plan, preflight, execute}` pour la génération (mêmes options, mêmes événements).

## TUI (`alpharust tui`)

Menu à gauche, résultat à droite, barre d'aide en bas. Nécessite un terminal interactif.

| Touche | Action |
|--------|--------|
| ↑ ↓ / j k | Naviguer |
| Entrée | Lancer : lint, générer Astro / Laravel / Wagtail / Rust, vérifier les outils (doctor), lister règles / profils / checkers |
| p | Changer de profil (cycle, puis « toutes les règles ») |
| x | Annuler la génération en cours |
| PgUp / PgDn | Défiler (désactive le suivi automatique du journal) |
| q / Échap / Ctrl-C | Quitter (le terminal est toujours restauré) |

Une génération demande le nom du projet puis s'exécute **en arrière-plan** : le journal défile en direct
(étapes, commandes officielles, sortie des outils, durées). Pour choisir toutes les options, utiliser
`alpharust new` (assistant) ou `alpharust generate … --flags`, ou la GUI.

## GUI (`alpharust gui [--port 3000]`)

Application web d'une page, servie par le binaire (HTML/CSS/JS embarqués, **aucune ressource externe**) sur
`127.0.0.1` uniquement. Thème sombre/clair (préférence système, mémorisé), responsive (menu mobile), navigation clavier,
lien d'évitement, régions ARIA, `prefers-reduced-motion`.

| Page | Contenu |
|------|---------|
| Tableau de bord | Compteurs (règles, profils, checkers, projets), état des chaînes d'outils, générations récentes, profils |
| Générer un projet | Choix du framework, formulaire complet (toutes les options du générateur, incompatibilités appliquées en direct), **aperçu à droite** : commandes officielles exactes, outils détectés, équivalent en ligne de commande ; état mémorisé |
| Génération (`#/jobs/<id>`) | **Progression en direct** (SSE) : barre, liste d'étapes avec durées, journal filtrable par étape, annulation, rapport final (fichiers, lint, commandes suivantes) |
| Projets | Projets de `sites/` (framework, outil officiel, date) → lien vers le lint |
| Lint | Dossier + profil ; résultats avec filtres par sévérité, texte et règle, regroupement par fichier, export JSON / SARIF |
| Règles | Recherche instantanée (`/` focalise), filtres sévérité/catégorie/type, détail d'une règle (description, motif, exemples) |
| Checkers / Outils | Catalogue des checkers ; détection des outils par framework avec conseils d'installation |

### API (JSON)

| Route | Rôle |
|-------|------|
| `GET /api/meta` | Version, compteurs, profils, schéma des options de chaque framework |
| `GET /api/doctor[?framework=]` | Outils détectés |
| `POST /api/plan` | Valide une demande et renvoie les étapes/commandes, les outils et la destination (422 si invalide) |
| `POST /api/jobs` · `GET /api/jobs[/:id]` · `POST /api/jobs/:id/cancel` | Lance / liste / annule une génération (un seul job à la fois, 409 sinon) |
| `GET /api/jobs/:id/events[?from=N]` | Flux **SSE** rejouable (`Last-Event-ID`), se termine par `event: end` |
| `GET /api/projects` | Projets générés |
| `GET /api/lint?path=&profile=` · `/api/sarif` | Résultats du lint (JSON / SARIF 2.1.0) |
| `GET /api/rules` · `/api/rule?id=` · `/api/checkers` | Règles et checkers |

### Sécurité

* écoute sur `127.0.0.1` ; les `POST` exigent `Origin` **et** `Host` locaux (CSRF, DNS rebinding) → 403 sinon ;
* la destination d'un projet est toujours `sites/<nom>` (le client ne choisit jamais un chemin) ; le lint n'accepte que le
  dossier courant ou `sites/` (chemins canonisés) ;
* le client n'utilise jamais `innerHTML` (un test le vérifie) : tout texte passe par `textContent` ;
  en-têtes `Content-Security-Policy` (`default-src 'self'`), `X-Frame-Options: DENY`, `Referrer-Policy: no-referrer` ;
* noms et options validés côté serveur avant toute exécution ; commandes construites sans shell (arguments séparés).
