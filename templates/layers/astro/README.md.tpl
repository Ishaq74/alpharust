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
└── styles/       # tokens.css (variables) et global.css
```

Le thème (clair / sombre / système) est défini par des variables CSS dans `src/styles/tokens.css`.
@@end@@

## Qualité

```bash
alpharust lint     # règles AlphaRust (profil « web », voir .alpharust.toml)
```

Voir aussi [`docs/index.md`](docs/index.md).

## License

MIT.
