# Checkers sémantiques

Les checkers vont au-delà de la regex ligne à ligne : ils analysent la structure du fichier
ou du projet. Une règle les active avec `checker:` et `args:` (voir `docs/rule-format.md`).
`alpharust list --checkers` donne le catalogue ; la page `/checkers` de la GUI aussi.

## Checkers de fichier

| Checker | Analyse | Règles qui l'utilisent |
|---------|---------|------------------------|
| `html-img-alt` | Scanner de balises (multi-lignes, commentaires, `<script>`, spreads JSX) : `<img>` sans `alt` | `alt-text-required` |
| `html-lang` | `<html>` sans `lang` | `html-lang-required` |
| `html-heading-order` | un seul `<h1>`, pas de niveau sauté | `heading-order` |
| `html-semantic` | `<body>` sans `main/header/nav/footer/article/section/aside` | `semantic-html` |
| `markdown-headings` | niveaux de titres (hors blocs de code, frontmatter pris en compte) | `consistent-headings` |
| `markdown-title` | premier titre = H1 unique | `consistent-structure` |
| `markdown-frontmatter` | frontmatter fermé et YAML valide | `valid-frontmatter` |
| `readme-sections` | un titre contient l'un des mots de `args.any` | `has-installation`, `has-usage`, `has-license` |
| `css-contrast` | contraste WCAG 4.5:1 entre `color` et `background` d'un même bloc | `accessible-contrast` |
| `rust-unsafe` | lexer Rust : `unsafe` hors commentaires, chaînes, chaînes brutes | `rust-no-unsafe` |
| `indent-consistency` | tabulations et espaces mélangés | `consistent-spacing` |
| `deps-consistency` | `package.json`, `composer.json`, `Cargo.toml`, `requirements.txt` : versions joker, doublons, divergences | `libs-consistency` |
| `requires-companion` | si `args.trigger` (regex) apparaît, `args.require` doit aussi apparaître | `csrf-protection`, `webhook-signature`, `stripe-webhook-signature`, `idempotency-required`, `connection-pooling` |

## Checkers de projet

Exécutés une fois sur la racine analysée, en voyant **tous** les fichiers (même ceux exclus par `exclude`).

| Checker | Arguments | Exemple |
|---------|-----------|---------|
| `require-glob` | `any` (globs), `when_any` (marqueurs de projet), `when_contains` (`[glob, regex]`), `message` | README, tests, migrations, docs |
| `file-contains` | `file` (glob), `any` (regex), mêmes conditions | `.gitignore` ignore `.venv/` |

`when_any` évite les faux positifs : `required-readme` ne s'applique que si le dossier ressemble à un projet
(`Cargo.toml`, `package.json`, `composer.json`, `manage.py`…).

## Écrire un checker

1. Ajouter une fonction `fn(&FileCtx) -> Vec<Finding>` (ou `ProjectCtx`) dans `src/checkers/`.
2. L'enregistrer dans `REGISTRY` (`src/checkers/mod.rs`) avec une description.
3. Écrire ses tests unitaires dans le même fichier.
4. L'utiliser dans une règle avec des `examples` (`alpharust check-rules`).
