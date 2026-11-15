# AlphaRust

Fondation intégrale : CLI + TUI + GUI web.

## Installation

```bash
git clone https://github.com/Ishaq74/alpharust.git
cd alpharust
cargo build --release
```

## Modes

### CLI
```bash
./target/release/alpharust lint
./target/release/alpharust generate wagtail mon-site
./target/release/alpharust init mon-projet
```

### TUI
```bash
./target/release/alpharust tui
```

### GUI Web (style forge)
```bash
./target/release/alpharust gui
# → http://127.0.0.1:3000
```

Les projets générés vont dans `sites/`.

## Licence

MIT
