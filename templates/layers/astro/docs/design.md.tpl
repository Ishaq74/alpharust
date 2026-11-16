@@meta when=!official@@
# Design system

Kit : **@@kit@@**. Arborescence unique pour tous les kits (CSS simple, Tailwind, Tailwind + Starwind, Concordia) — celle des dépôts
`concordia` et `astro-css-drizzle-better-auth` (`src/styles/{global.css,base.css,tokens/,components/}`), complétée par `themes/`.

| Dossier / fichier | Rôle |
|-------------------|------|
| `global.css` | entrée unique ; ordre : moteur (@@engine@@) → jetons → thème → base → composants → `custom.css` |
| `tokens/colors.css` | @@if concordia@@jetons de couleurs Concordia (clair + sombre `:root[data-theme="dark"]`)@@else@@couleurs nommées comme Starwind/shadcn (`--background`, `--primary`…), clair + sombre@@end@@ |
| `tokens/typography.css`, `tokens/spacing.css` | polices et échelles ; espacements, rayons, ombres |
| `tokens/components.css` | un jeton par réglage de composant (`--button-border-radius`, `--card-padding`…) |
| `tokens/site.css` | variables du gabarit (`--bg`, `--text`, `--brand`…) reliées aux jetons du kit |
| `themes/theme.css` | palette de marque (WCAG) — @@if concordia@@`--color-primary|secondary|accent`@@else@@`--brand-primary|secondary|accent` et `--on-brand-*` (texte lisible)@@end@@ |
| `components/*.css` | un fichier par composant ; `custom.css` pour le reste |

## Flux des valeurs

```text
themes/theme.css  ──►  tokens/colors.css  ──►  tokens/site.css  ──►  components/*.css  ──►  votre page
  (marque, WCAG)         (jetons du kit)        (gabarit)            (jetons de composants)
```

## Exemple : boutons en pilule

@@if concordia@@
```css
/* src/styles/components/custom.css */
:root {
  --button-border-radius: 9999px;
}
```
@@else@@
```css
/* src/styles/tokens/components.css */
:root {
  --button-border-radius: 9999px;
}
```
@@end@@
@@if starwind@@

Les composants Starwind (`src/components/atoms/**`, repris du dépôt atomic) ne sont pas modifiés : `components/button.css`
cible `[data-slot="button"]` dans `@layer overrides` (déclarée après les utilitaires Tailwind) et lit ce jeton.
Pour un composant sans fichier dédié, ajoutez `@layer overrides { [data-slot='avatar'] { … } }` dans `components/custom.css`.
@@end@@

## Contrôle d'accessibilité

@@if theme@@
`npm run theme:check` relit `themes/theme.css` et calcule le contraste de chaque couleur de marque avec son texte
(AA ≥ 4,5 : 1 ; AAA ≥ 7 : 1), en clair et en sombre.
@@else@@
Générez une palette contrôlée avec `--theme-primary #… --theme-secondary #… --theme-accent #…` : `themes/theme.css` est alors
écrit avec les couleurs et leurs textes lisibles (méthode WCAG du dépôt `themecsutomizer`).
@@end@@
