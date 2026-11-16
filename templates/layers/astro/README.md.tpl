@@meta mode=replace@@
# @@title@@

Site [Astro](https://astro.build) créé avec [AlphaRust](https://github.com/Ishaq74/alpharust) : projet généré par
`create-astro` (template `@@template@@`), gestionnaire de paquets **@@pm@@**, TypeScript **@@typescript@@**.

## Installation

Prérequis : Node.js ≥ 22.

```bash
@@install_cmd@@
```

## Usage

| Commande | Action |
|----------|--------|
| `@@run@@ dev` | Serveur de développement sur http://localhost:4321 |
| `@@run@@ build` | Build de production dans `dist/` |
| `@@run@@ preview` | Prévisualise le build |
@@if typecheck@@
| `@@run@@ check` | Vérification des types (`astro check`) |
@@end@@

@@if !official@@
## Structure

```text
src/
├── components/   # Header, Footer, SEO, ThemeToggle…
├── content/      # Articles Markdown/MDX (collection « blog », typée)
├── layouts/      # BaseLayout (SEO, accessibilité, thème clair/sombre)
├── pages/        # Routes : accueil, à propos, blog, 404
└── styles/       # design system, voir « Personnaliser le design »
```

## Personnaliser le design

Tout le style vit dans `src/styles/`, avec la **même arborescence quel que soit le kit** (kit : @@kit@@) :

```text
src/styles/
├── global.css            # ENTRÉE UNIQUE, importée par BaseLayout : l'ordre des imports se règle ici
├── base.css              # @@if concordia@@Concordia : jetons + variante initiale@@else@@remise à zéro et valeurs globales@@end@@
├── tokens/               # variables : couleurs, typographie, espacements, composants (+ site.css : pont du gabarit)
@@if starwind@@
│   └── tailwind.css      # compléments @theme (dégradés, couleurs de marque)
@@end@@
@@if !starwind&tailwind@@
│   └── tailwind.css      # @theme : bg-primary, text-foreground… lisent les mêmes jetons
@@end@@
├── themes/theme.css      # palette de marque (générée avec contrôle WCAG, ou à remplir)
@@if starwind@@
├── starwind.css          # généré par `starwind init` (Tailwind 4, jetons officiels)
@@end@@
└── components/           # un fichier par composant : layout, button, card@@if starwind@@, input, select, dialog, tabs…@@end@@
    └── custom.css        # vos surcharges, chargé en dernier
```

1. **Changer une couleur** : `themes/theme.css` (ou `npm run theme:check` après modification, @@if concordia@@variables `--color-*`@@else@@variables `--brand-*`@@end@@).
2. **Changer l'allure d'un composant** : son jeton dans `tokens/components.css`@@if concordia@@ (`--button-border-radius`, `--card-padding`…, noms de Concordia)@@else@@ (`--button-border-radius: 9999px;` → boutons en pilule)@@end@@.
@@if starwind@@
3. **Restyler un composant Starwind sans toucher à `src/components/atoms/`** : chaque composant expose `data-slot="…"` ; les fichiers `components/*.css` ciblent ce hook dans `@layer overrides`, déclarée après les utilitaires Tailwind (elle l'emporte donc sur les classes du composant).
@@else@@
@@if concordia@@
3. **Restyler un composant Concordia sans modifier `src/components/ui/`** : surcharger ses jetons (`tokens/components.css`) ou ajouter des règles dans `components/custom.css`. Les variantes (`retro`, `modern`, `futuristic`) sont les fichiers `components/<variante>.css` importés par `global.css`.
@@else@@
3. **Ajouter un composant stylé** : créez `components/<nom>.css`, importez-le dans `global.css`.
@@end@@
@@end@@
@@if theme@@
4. **Thème généré** : `themes/theme.css` a été écrit avec les couleurs passées à la génération ; relancez `npm run theme:check` après toute modification.
@@end@@

Détails : [`docs/design.md`](docs/design.md).
@@end@@

## Qualité

```bash
alpharust lint     # règles AlphaRust (profil « web », voir .alpharust.toml)
```

Voir aussi [`docs/index.md`](docs/index.md).

## License

MIT.
