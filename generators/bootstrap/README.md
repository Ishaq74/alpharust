# Rust Generator

`alpharust generate rust <nom>` (alias `init`) : `cargo new` officiel (bin ou lib, édition au choix), dépendances via
`cargo add` (clap, anyhow, thiserror, serde, serde_json, tokio, tracing), puis tests, CI GitHub, LICENSE, README, docs et
`.alpharust.toml` ; vérifié par `cargo test`, `cargo fmt --check` et `cargo clippy -D warnings`.
Couche : `templates/layers/rust/`. Code : `src/scaffold/rust.rs`.
