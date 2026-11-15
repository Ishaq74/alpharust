# TODO AlphaRust

Audit du 15 novembre 2026. État réel du repo.

## Fait et stable
- [x] Repo créé, CI qui passe
- [x] Parser de règles Markdown + frontmatter YAML
- [x] Engine + runner (regex) + reporter
- [x] CLI : `lint`, `list`, `list --profiles`, `init`, `readme`
- [x] Profils de base (core, strict, web, ecommerce, cms)
- [x] Générateurs bootstrap et readme fonctionnels
- [x] Structure de dossiers conforme à l’arborescence validée
- [x] ADR 0001 à 0004
- [x] Tests unitaires du parseur

## À faire (priorité haute)
- [ ] Corriger toutes les règles restantes pour qu’elles aient un YAML valide (`apply: "*"`)
- [ ] Implémenter la priorité réelle dans le composer (framework > domain > language > core)
- [ ] Ajouter des fixtures de tests d’intégration (clean + violations) pour les règles core
- [ ] Rendre le chargement des règles plus strict (loguer les règles qui échouent au parse)
- [ ] Documenter le format de règle de façon exhaustive dans `docs/rule-format.md`

## À faire (priorité moyenne)
- [ ] Générateurs de tests unitaires réellement utiles
- [ ] Support des profils dans le lint par défaut via un fichier `.alpharust.toml`
- [ ] Self-validation des règles (dossier `rules/markdown/`)
- [ ] Plus de third-parties concrets et testés
- [ ] Sortie JSON / SARIF pour intégration CI externe

## À faire (plus tard)
- [ ] Générateurs de frameworks complets (Wagtail, Astro, Laravel)
- [ ] Checkers sémantiques au-delà du regex
- [ ] Versioning des règles

## Règles de travail
- La CI doit toujours passer
- Toute nouvelle règle = frontmatter valide + test ou fixture
- Pas de dépendance inutilisée
- ADR pour toute décision structurelle
