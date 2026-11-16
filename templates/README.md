# Couches de modèles

Ces fichiers sont posés **après** l'outil officiel (create-astro, composer, `wagtail start`, `cargo new`) ;
ils sont embarqués dans le binaire (`include_dir!`, recompilation automatique via `build.rs`).

```text
layers/
├── common/     # appliqué à tous : .alpharust.toml, docs/…
├── astro/      # structure Astro (layout, composants, pages, collection blog, thème)
├── laravel/    # pages, modèle, migration, seeder, tests (sans conflit avec Breeze)
├── wagtail/    # README, docs, .gitignore, .env.example, test d'accueil, .alpharust.toml
└── rust/       # tests, CI, LICENSE, README, docs
```

Syntaxe (détails : `docs/generators.md`) :

* chemins : `dot-xxx` → `.xxx`, suffixe `.tpl` retiré, `@@var@@` substitué ;
* en-tête facultatif `@@meta mode=create|replace when=a|b&!c@@` (`create` n'écrase jamais un fichier officiel) ;
* blocs `@@if expr@@ … @@else@@ … @@end@@` (propres lignes) ou en ligne ;
* variables : `name slug pkg title upper version folder pm run install_cmd typescript template site …` selon le framework.

Les tests vérifient l'équilibre des blocs, l'absence de marqueur résiduel et que le projet généré passe son propre lint.
