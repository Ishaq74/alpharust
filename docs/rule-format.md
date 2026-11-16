# Format des règles

Chaque règle est un fichier Markdown avec frontmatter YAML sous `rules/`.
Son **id** est son chemin sans extension (`core/no-eval`, `domains/cloud/github-token`).

```yaml
---
name: nom-de-la-regle        # unique par couche
apply: "*.html,*.astro"      # voir « Sélecteur apply »
severity: error              # error | warning | info
pattern: "regex"             # règle regex (une ligne à la fois)
# — ou —
checker: html-img-alt        # règle sémantique (voir docs/checks.md)
args: { any: ["README*"] }   # arguments du checker
message: "Message affiché"   # optionnel (défaut : premier titre du corps)
examples:                    # optionnel mais recommandé : exemples auto-testés
  file: "example.html"       # nom de fichier fictif pour les checkers
  bad:  ["<img src=a.png>"]
  good: ['<img src=a.png alt="logo">']
---

# Titre (utilisé comme message)

Description de la règle.
```

Les clés inconnues sont refusées (une faute de frappe n'est jamais ignorée en silence).

## Types de règles

| Type | Condition | Exécution |
|------|-----------|-----------|
| `regex` | `pattern` non vide | Regex Rust ligne par ligne (compilée une seule fois par exécution) |
| `checker` | `checker:` | Analyse sémantique d'un fichier ou du projet |
| `info` | ni l'un ni l'autre | Règle documentaire (nécessite un outil externe, ex. PSScriptAnalyzer) |

`pattern` et `checker` sont exclusifs. La syntaxe regex est celle du crate `regex` (pas de look-around).

## Sélecteur `apply`

Liste séparée par des virgules ; il suffit qu'un élément corresponde :

| Forme | Exemple | Signification |
|-------|---------|---------------|
| `*` | `*` | Tous les fichiers |
| `*.ext` | `*.html` | Extension |
| glob | `src/**/*.rs`, `.env*` | Glob sur le chemin relatif (ou le nom de fichier si sans `/`) |
| `/glob` | `/readme*` | Glob ancré à la racine analysée |
| langage | `rust`, `php`, `python`, `javascript`, `powershell`, `markdown`, `html`, `css`, `sql`, `json`, `yaml`, `shell` | Extensions du langage |
| `readme` | `readme` | Fichiers nommés `README*` (tous dossiers) |
| `docs` | `docs` | Fichiers sous un dossier `docs/` |
| nom exact | `package.json` | Nom de fichier |
| mot-clé | `stripe`, `mysql`, `nodemailer` | Le chemin **ou le contenu** du fichier mentionne le mot |

Les checkers « projet » (`require-glob`, `file-contains`) ignorent `apply` : ils s'exécutent une fois sur la racine.

## Exemples auto-testés

`alpharust check-rules` (et `cargo test`) vérifie que chaque exemple `bad` est détecté et que
chaque `good` ne l'est pas. Le caractère `¦` est supprimé avant le test : il permet d'écrire
`sk_¦live_…` dans un exemple sans déclencher les scanners de secrets (GitHub push protection).

## Validation

- `severity` ∈ `error | warning | info`
- regex compilable, checker connu, pas de clé inconnue
- noms uniques au sein d'une couche ; les doublons entre couches sont arbitrés par le composer
- le YAML entre guillemets doubles interprète les antislashs : écrire `"\\s"` (ou utiliser des guillemets simples)
