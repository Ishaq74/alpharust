# TODO AlphaRust

Suivi strict. Aucune régression. Aucune facilité. Aucun oubli.

## Immédiat
- [ ] Cargo.toml minimal fonctionnel
- [ ] src/main.rs + cli de base (help, lint)
- [ ] Parser de règles Markdown (frontmatter + pattern)
- [ ] Première règle core : no-eval.md + checker générique
- [ ] Tests fixtures clean/violations pour no-eval

## Court terme
- [ ] Engine + composer (priorités)
- [ ] Profils de base (core.toml, strict.toml)
- [ ] Générateur bootstrap minimal
- [ ] Générateur README
- [ ] Domaines database / smtp / payments (règles générales + 1 third-party chacun)

## Moyen terme
- [ ] Tous les domaines complets
- [ ] Générateurs de tests unitaires
- [ ] Frameworks et languages complets
- [ ] CI GitHub Actions
- [ ] Documentation complète

## Règles de travail
- Chaque changement passe par une règle + fixture.
- Aucun code sans test.
- ADR pour toute décision structurelle.
