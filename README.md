# AlphaRust

Fondation intégrale production-ready : **CLI + TUI + GUI web**.

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
./target/release/alpharust lint --profile strict --sarif
./target/release/alpharust generate wagtail mon-site
./target/release/alpharust init mon-projet
./target/release/alpharust list
```

### TUI
```bash
./target/release/alpharust tui
```

### GUI Web
```bash
./target/release/alpharust gui
```
Ouvre http://127.0.0.1:3000  
Interface complète : consultation des règles, génération de projets, lint.

Tous les projets générés vont dans `sites/`.

## Licence

MIT
