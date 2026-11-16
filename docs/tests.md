# Tests

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
alpharust check-rules
```

| Suite | Fichier | Contenu |
|-------|---------|---------|
| Unitaires | `src/**` | glob, parser de règles, checkers, runner, config, profils, composer, SARIF, TUI (rendu `TestBackend`, flux d'événements), pipeline de génération (étapes, couche de modèles, validations) |
| Moteur | `tests/integration.rs` | règles réelles contre `tests/fixtures/`, profils, secrets, config, SARIF, auto-lint du dépôt |
| Cache | `tests/cache.rs` | écriture, hit, invalidation, cache corrompu |
| CLI | `tests/cli.rs` | binaire réel : codes de sortie, SARIF/JSON purs, `generate --dry-run/--json`, options incompatibles, `doctor`, `cargo new` réel, noms dangereux, tube fermé, TUI/`new` sans TTY |
| GUI | `tests/gui.rs` | SPA embarquée (CSP, aucun `innerHTML`), API méta/plan/règles/lint, chemins refusés, CSRF (`Origin`/`Host`), **job de génération avec flux SSE** (rejeu, `Last-Event-ID`) |
| Générateurs | `tests/generators.rs` | pipeline hermétique (faux `npm`/`node`), outil absent, échec d'outil, annulation, `--force` sûr, `cargo new` réel ; **`e2e_*` réels** (create-astro npm/pnpm, composer + Breeze + Pest, wagtail) avec `ALPHARUST_E2E=1` |

Les fixtures : `tests/fixtures/violations/` (doit déclencher des règles précises), `clean/` (zéro violation),
`projects/{good,bad}-project/` (checkers de projet).
Les jetons réalistes sont assemblés **à l'exécution** dans les tests pour ne pas ressembler à de vrais secrets.

## Générations réelles

```bash
ALPHARUST_E2E=1 cargo test --test generators e2e -- --test-threads=1 --nocapture
```

Réseau requis (npm, composer, pip). Chaque test détecte les outils et s'ignore avec un message s'ils manquent ;
`PATH` est global au processus : ces tests sont sérialisés (`--test-threads=1` conseillé, verrou interne sinon).

### Plateforme Astro

```bash
ALPHARUST_E2E=1 cargo test --test generators e2e_astro_platform -- --test-threads=1
# PostgreSQL réel : une base vide `<base>` et `<base>_test` doivent exister
ALPHARUST_E2E=1 ALPHARUST_E2E_PG_URL=postgresql://…/ma_base cargo test --test generators e2e_astro_platform_postgres
```

Les trois tests génèrent de vrais projets (SQLite + Better Auth + Starwind + i18n ; Concordia + thème ; PostgreSQL) et
vérifient chaque étape (`starwind-add`, `auth-schema`, `db-migrate`, `check`, `build`, `test`) puis le lint.
Les projets générés exécutent eux-mêmes leurs tests `vitest` (base, authentification, parité i18n).
