# Configuration `.alpharust.toml`

Lu dans le dossier analysé s'il en contient un, sinon dans le répertoire courant. Les clés inconnues
et les valeurs invalides sont refusées avec un message clair.

```toml
profile  = "strict"                  # profil par défaut de `lint`
exclude  = ["vendor/**", "dist/**"]  # globs relatifs à la racine analysée
disable  = ["core/no-hardcoded-urls"]  # ids ou noms de règles (globs acceptés)
fail_on  = "error"                   # error | warning | info

[severity]                           # surcharge de sévérité par règle
"core/no-todo-in-prod" = "warning"
```

Les dossiers `target`, `.git`, `node_modules`, `.venv`, `__pycache__` et `.alpharust-cache` ne sont jamais analysés ;
les fichiers de plus de 2 Mo ou non UTF-8 sont ignorés.

## Cache des règles

Le parsing YAML de toutes les règles est mis en cache dans `.alpharust-cache/rules.json` (ou `ALPHARUST_CACHE_DIR`).
L'empreinte (chemins, tailles, dates de modification, version) invalide le cache dès qu'une règle change ;
un cache illisible est ignoré. `--no-cache` ou `ALPHARUST_NO_CACHE=1` le désactive ; `alpharust cache clear` le supprime.
Les regex sont compilées une seule fois par exécution (et non plus par fichier).

## Variables d'environnement

| Variable | Effet |
|----------|-------|
| `ALPHARUST_HOME` | Racine contenant `rules/`, `profiles/`, `sites/` |
| `ALPHARUST_CACHE_DIR` | Dossier du cache |
| `ALPHARUST_NO_CACHE` | Désactive le cache |
