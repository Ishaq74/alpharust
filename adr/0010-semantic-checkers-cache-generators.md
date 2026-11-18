# ADR 0010 — Checkers sémantiques, cache des règles et générateurs complets

## Statut
Accepté

## Contexte
La fondation (ADR 0001–0009) fournissait des règles regex ligne par ligne, des profils qui ne
sélectionnaient aucune règle (motifs comparés au nom court), un `Cargo.lock` ignoré par git (le build
cassait avec un cargo plus ancien), des générateurs limités et une CI qui ne lançait que `cargo test --lib`.
Les regex étaient recompilées pour chaque fichier. Plusieurs règles étaient inertes : un échappement YAML
invalide les faisait disparaître en silence, d'autres reposaient sur du look-around que le crate `regex`
ne supporte pas.

## Décisions
1. **Checkers sémantiques** (`src/checkers/`) : une règle peut déclarer `checker:` + `args:` au lieu de `pattern:`.
   Deux familles : *fichier* (HTML, Markdown, CSS/WCAG, lexer Rust, indentation, dépendances, « compagnon requis »)
   et *projet* (`require-glob`, `file-contains`). Le registre est explicite et auto-documenté
   (`list --checkers`, page GUI `/checkers`).
2. **Règles auto-testées** : chaque règle porte des `examples` (bad/good) vérifiés par `check-rules` et `cargo test`.
   Les erreurs de frontmatter ne sont plus ignorées : `deny_unknown_fields`, validation de la sévérité, des regex
   et des checkers, doublons de noms, motifs de profil sans correspondance.
3. **Profils** : les motifs portent sur l'id de la règle (`core/no-eval`) ou son nom, avec un glob où `*` traverse
   `/`. Le composer applique réellement la priorité framework > domaine > langage > core (cf. ADR 0003).
4. **Cache des règles** : le parsing est mis en cache (`.alpharust-cache/rules.json`, empreinte FNV des fichiers + version),
   désactivable (`--no-cache`, `ALPHARUST_NO_CACHE`) et gérable (`cache status|clear`). Les regex sont compilées une
   fois par exécution (`CompiledRule`). Un cache corrompu ou obsolète est ignoré et reconstruit.
5. **Tiers** : 23 règles de secrets/configuration par service (AWS, GCP, GitHub, Slack, OpenAI, Twilio, Azure, SendGrid,
   Postmark, Brevo, Adyen, Square, Braintree, Paddle, Auth0, Clerk, Supabase, Firebase, MongoDB, Redis, PlanetScale…),
   regroupées dans les profils `secrets` et `third-party`.
6. **Générateurs** : templates `.tpl` embarqués par `include_str!`, substitution de marqueurs dans contenus et chemins,
   refus d'un dossier existant sans `--force`, validation du nom. Wagtail, Astro et Laravel produisent des projets
   exécutables (migrations, tests, build) et conformes aux profils AlphaRust ; ils sont compilés/testés en CI.
7. **Interfaces** : TUI testable (état + `TestBackend`) et GUI échappée, avec contrôle `Origin` sur les `POST`.
   La TUI refuse de démarrer sans TTY. Les trois interfaces partagent `engine::lint`.
8. **Reproductibilité et CI** : `Cargo.lock` versionné, `rust-version = 1.85`, resolver 3 ; la CI exécute
   `fmt --check`, `clippy -D warnings`, `build --release`, `cargo test --all-targets`, `check-rules`, un auto-lint strict
   et la génération/test des trois projets framework.

## Conséquences
- Les règles sont plus fiables (exemples exécutés) et moins bruyantes (`apply` contextuel, checkers).
- Ajouter une règle ou un checker exige des exemples/tests : la barre de contribution monte légèrement.
- Les templates de générateurs suivent les versions majeures de Wagtail, Astro et Laravel : un job CI dédié détecte les dérives.

## Alternatives écartées
- Parser tree-sitter pour chaque langage : trop lourd ; des scanners ciblés suffisent pour les règles actuelles.
- Cache binaire (bincode) : JSON est plus lisible et suffisamment rapide.
