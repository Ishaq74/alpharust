@@meta mode=replace@@
# @@title@@

Crate Rust créé avec [AlphaRust](https://github.com/Ishaq74/alpharust) (`cargo new`, puis tests, CI et configuration qualité).

## Installation

```bash
git clone <url-du-depot> @@folder@@
cd @@folder@@
cargo build --release
```

## Usage

@@if bin@@
```bash
cargo run
```
@@else@@
Ajouter la dépendance puis utiliser l'API publique de `src/lib.rs`.
@@end@@

```bash
cargo test                                   # tests
cargo fmt --check                            # format
cargo clippy --all-targets -- -D warnings    # lints
alpharust lint                               # règles AlphaRust (profil « strict »)
```

Voir [`docs/index.md`](docs/index.md).

## License

MIT — voir [`LICENSE`](LICENSE).
