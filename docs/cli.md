# CLI AlphaRust

```text
alpharust [--no-cache] <commande>
```

## Commandes

| Commande | Description |
|----------|-------------|
| `lint [path]` | Lint un chemin (défaut : `.`). Code de sortie 1 si une violation atteint le seuil `fail_on` (défaut : `error`) |
| `lint --profile <nom>` | Restreint aux règles d'un profil (`list --profiles`) |
| `lint --sarif` | Sortie SARIF 2.1.0 pure sur stdout (compatible GitHub code scanning) |
| `lint --json` | Violations au format JSON |
| `lint --fail-on error\|warning\|info` | Sévérité minimale qui fait échouer la commande |
| `list` | Liste les règles : id, sévérité, type (`regex`, `checker`, `info`), `apply` |
| `list --profiles` | Liste les profils et le nombre de règles sélectionnées |
| `list --checkers` | Liste les checkers sémantiques |
| `check-rules` | Valide toutes les règles (YAML, regex, checkers), leurs exemples et les profils |
| `new [name] [-f framework]` | Assistant interactif : questions guidées, aperçu des commandes, confirmation, génération (TTY requis) |
| `generate astro\|laravel\|wagtail\|rust <name>` | Projet créé avec l'outil officiel, dépendances installées, couche de modèles, vérifications (`docs/generators.md`) |
| `generate … --dry-run` / `--json` / `-v` | Aperçu des commandes sans rien écrire / événements JSON / journal complet |
| `generate … --path <dir>` / `--force` | Destination personnalisée / remplace un projet *généré par AlphaRust* (jamais un dossier étranger) |
| `generate … --no-install --no-verify --no-git` | Sans installation / sans vérification / sans dépôt Git |
| `init <name>` | Alias de `generate rust <name>` |
| `doctor [-f framework]` | Outils détectés (node, npm/pnpm/yarn/bun, php, composer, python, cargo, git), versions minimales, conseils |
| `readme <name>` / `docs <name>` | Affiche un README / une documentation de départ |
| `checks [--profile p]` | Checklist Markdown des règles actives |
| `cache status\|clear` | Cache des règles parsées (`.alpharust-cache/`) |
| `tui` | Interface terminal interactive (nécessite un TTY) |
| `gui [--port 3000]` | Interface web sur `http://127.0.0.1:<port>` |

## Codes de sortie

| Code | Signification |
|------|---------------|
| 0 | Succès (ou violations sous le seuil `fail_on`) |
| 1 | Violations au-dessus du seuil, `check-rules` en échec, ou génération en échec (outil manquant, commande en échec) |
| 2 | Erreur d'utilisation (chemin introuvable, profil inconnu, nom invalide, option incompatible ou d'un autre framework…) |

## Résolution de la racine

Les dossiers `rules/`, `profiles/` et `sites/` sont cherchés dans l'ordre :
1. `ALPHARUST_HOME` ;
2. le répertoire courant et ses parents ;
3. les parents de l'exécutable (`target/release/alpharust` → racine du dépôt).

`alpharust` fonctionne donc depuis n'importe quel dossier.

## Suppression ponctuelle

Un commentaire `alpharust-ignore` sur la ligne (ou la ligne précédente) supprime les violations ;
`alpharust-ignore: no-eval, no-hardcoded-urls` cible des règles précises (nom ou id).

## Exemples

```bash
alpharust lint --profile strict
alpharust lint src --sarif > alpharust.sarif
alpharust new
alpharust generate astro mon-site -i tailwind,mdx --pm pnpm --ts strictest
alpharust generate laravel mon-app --starter breeze-blade --pest
alpharust generate astro mon-site --dry-run
alpharust doctor
alpharust check-rules
```
