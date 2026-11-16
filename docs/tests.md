# Tests

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
alpharust check-rules
```

| Suite | Fichier | Contenu |
|-------|---------|---------|
| Unitaires | `src/**` | glob, parser de règles, checkers, runner, config, profils, composer, SARIF, TUI (rendu `TestBackend`), générateurs |
| Moteur | `tests/integration.rs` | règles réelles contre `tests/fixtures/`, profils, secrets, config, SARIF, auto-lint du dépôt |
| Cache | `tests/cache.rs` | écriture, hit, invalidation, cache corrompu |
| CLI | `tests/cli.rs` | binaire réel : codes de sortie, SARIF/JSON purs, génération, noms dangereux, tube fermé, TUI sans TTY |
| GUI | `tests/gui.rs` | pages et API via `tower::ServiceExt` : échappement HTML, CSRF (`Origin`), validation des noms |
| Générateurs | `tests/generators.rs` | projets complets, aucun marqueur résiduel, lint propre, `cargo test` du projet Rust généré, syntaxe Python/PHP |

Les fixtures : `tests/fixtures/violations/` (doit déclencher des règles précises), `clean/` (zéro violation),
`projects/{good,bad}-project/` (checkers de projet).
Les jetons réalistes sont assemblés **à l'exécution** dans les tests pour ne pas ressembler à de vrais secrets.
