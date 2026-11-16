# ADR 0011 — Génération par outils officiels et GUI web en application d'une page

## Statut
Accepté (remplace la partie « générateurs » de l'ADR 0010 et la GUI de l'ADR 0009)

## Contexte
Les générateurs de l'ADR 0010 **écrivaient à la main** des projets complets à partir de chaînes de caractères :
les projets divergeaient des outils officiels (versions figées, fichiers manquants, configuration inventée), rien
n'était réellement installé et la vérification se limitait à des contrôles en CI. La GUI était un ensemble de pages
HTML rendues côté serveur, sans retour en direct. Les dépôts de référence de l'auteur (`ai-web-forge`, `nkboot*`)
montrent le niveau attendu : assistant guidé, **vraies commandes officielles**, installation effective, `git init`,
aperçu des commandes avant exécution, formulaire à gauche / aperçu à droite, options à impact visible.

## Décisions
1. **Pipeline unique** `src/scaffold/` : *preflight* (détection des outils + versions minimales + conseil d'installation)
   → *plan* (liste exacte des commandes, prévisualisable) → *exécution* (événements sérialisables : `preflight`, `plan`,
   `step_start`, `log`, `step_end`, `done`, `failed`) → *couche de modèles* → *vérification* → *rapport*.
   La CLI, la TUI et la GUI consomment les mêmes événements ; une seule structure `Request` (+ `Options` par framework,
   désérialisable depuis la GUI) les alimente.
2. **Outils officiels d'abord** : `create-astro` (+ `astro add`), `composer create-project` (+ Breeze), `wagtail start`
   dans un venv, `cargo new`. Les options (gestionnaire de paquets, template, TypeScript, intégrations, kit, base de
   données, dépendances…) se traduisent en flags/commandes réels. Quand l'outil n'a pas d'option (niveau TypeScript d'Astro),
   on **corrige le fichier produit** (`extends` du tsconfig) au lieu de le réécrire.
3. **Couche de modèles** (`templates/layers/`) : fichiers `create` (jamais d'écrasement d'un fichier officiel) ou `replace`,
   conditions `@@if@@`, variables, chemins `dot-`. Embarquée via `include_dir!` (+ `build.rs` pour la recompilation).
4. **Vérification réelle** : `astro check` + `astro build`, `php artisan test`, `manage.py check/test`, `cargo test/fmt/clippy`,
   puis lint AlphaRust du projet généré ; les échecs non bloquants sont signalés, les bloquants arrêtent le pipeline avec la
   fin du journal de l'outil.
5. **Garde-fous** : `--force` limité aux projets portant `.alpharust-project.json` ; compatibilités validées avant exécution ;
   processus enfants sans stdin, `CI=1`, télémétrie coupée, délai et annulation ; code de sortie `1` pour un échec d'exécution.
6. **CLI** : `generate <fw>` avec toutes les options + `--dry-run`/`--json`, `new` (assistant `dialoguer`), `doctor`.
   **TUI** : génération en arrière-plan avec journal en direct. 
7. **GUI** : application d'une page (HTML/CSS/JS embarqués, sans dépendance ni `innerHTML`), API JSON, **SSE** pour les jobs
   (rejouable, `Last-Event-ID`), aperçu du plan à droite du formulaire, journal filtrable, lint filtrable, recherche de
   règles, thème clair/sombre, responsive, accessible (WCAG : contraste, focus, ARIA, mouvement réduit).
   Sécurité : `127.0.0.1`, `Origin`/`Host` vérifiés, destination imposée (`sites/<nom>`), lint limité au dossier courant et
   à `sites/`, CSP stricte.
8. **Tests** : pipeline hermétique avec faux outils (PATH jetable) ; tests réels `ALPHARUST_E2E=1` (create-astro npm/pnpm,
   composer + Breeze + Pest, wagtail) qui s'ignorent proprement si un outil manque ; tests HTTP/SSE de la GUI.

## Conséquences
* Les projets générés suivent les versions courantes des outils officiels (au prix d'un accès réseau et de quelques minutes).
* Le binaire ne contient plus de projets complets figés, seulement des couches maintenables.
* Ajouter un framework = une fonction `steps` + une couche de modèles (cf. `docs/generators.md`).
