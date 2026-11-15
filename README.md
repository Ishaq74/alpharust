# AlphaRust

Fondation intégrale : **CLI + TUI + GUI web**.

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
./target/release/alpharust lint --profile strict
./target/release/alpharust lint --sarif
./target/release/alpharust generate wagtail mon-site
./target/release/alpharust init mon-projet
./target/release/alpharust list
```

### TUI
```bash
./target/release/alpharust tui
```
Menu interactif : lint, génération Wagtail/Astro/Laravel, liste des règles.
Les projets sont créés dans `sites/`.

### GUI Web
```bash
./target/release/alpharust gui
```
Ouvre http://127.0.0.1:3000  
Pages : règles, génération de projets, lint avec résultats détaillés.

## Structure

- `rules/` — source de vérité des règles
- `profiles/` — ensembles de règles
- `sites/` — projets générés
- `src/` — moteur, CLI, TUI, GUI

## Licence

MIT
