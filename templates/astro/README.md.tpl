# @@title@@

Projet Astro généré par [AlphaRust](https://github.com/Ishaq74/alpharust) (profil `web`).

## Installation

Prérequis : Node.js ≥ 22.

```bash
npm install
```

## Usage

```bash
npm run dev      # http://localhost:4321
npm run build    # sortie statique dans dist/
alpharust lint --profile web .
```

## Structure

- `src/content.config.ts` + `src/content/blog/` : content collections typées
- `src/layouts/Layout.astro` : layout accessible (`lang`, navigation, `<main>`)
- `src/pages/` : routes (accueil, à propos, blog)

## License

MIT
