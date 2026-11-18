# ADR 0013 — Design system unifié et composants Starwind d'atomic

Statut : accepté (complète l'ADR 0012)

## Contexte

Chaque kit posait ses styles différemment : Concordia avait `tokens/` + `components/`, Starwind seulement `starwind.css`
+ `tokens.css`, le CSS simple un `global.css`. Impossible de savoir où personnaliser un composant, et le thème WCAG
n'existait que pour Concordia. Les composants Starwind installés par `starwind add` (v2, primitives `@starwind-ui/astro`)
différaient de ceux d'atomic, que l'auteur considère comme la référence testée.

## Décision

1. **Une arborescence `src/styles/` unique** (convention des dépôts `concordia` et `astro-css-drizzle-better-auth` :
   `global.css`, `base.css`, `tokens/`, `components/`) complétée par `themes/`. `global.css` est l'unique entrée ;
   les tests vérifient que chaque import existe.
2. **Jetons nommés comme Concordia** (`--button-border-radius`, `--card-padding`…) pour tous les kits ; `tokens/site.css`
   relie les variables du gabarit au kit. `--accent` (collision avec le `--accent` neutre de Starwind) devient `--brand`.
3. **Palette de marque pour tous les kits** : `themes/theme.css`, préfixe `--brand-*` (Tailwind/Starwind/CSS simple, pour ne pas
   entrer dans l'espace `--color-*` de Tailwind) ou `--color-*` (jetons Concordia). Même algorithme WCAG partout.
4. **Starwind = composants d'atomic** : `starwind init` (outil officiel : Tailwind 4, jetons, config) puis calque `ui-atomic`
   (46 composants `atoms/`, `wow/`, `lib/starwind`), au lieu de `starwind add`. `--ui-component` disparaît (tous les
   composants sont posés, comme tous ceux de Concordia). `starwind.config.json` est aligné sur atomic.
5. **Surcharge sans toucher aux composants** : chaque composant Starwind expose `data-slot` ; `components/<nom>.css` le cible
   dans `@layer overrides` (déclarée après `utilities`) et lit les jetons. Valeurs par défaut = valeurs des composants
   (aucun changement visuel tant que le jeton n'est pas modifié).
6. **Thème sombre sans flash pour tous** : `BaseLayout` pose `data-theme` (préférence système si rien n'est enregistré) ; `.dark`
   est ajouté pour Starwind.

## Corrections apportées

| Source | Correction |
|--------|-----------|
| atomic `atoms/*.md` | imports `@components/starwind/…` → `@atoms/…` |
| atomic `global.css` | découpé : jetons → `tokens/colors.css` (valeurs OKLCH inchangées, primaires reliées à `--brand-primary`), `@layer base` → `base.css`, `@theme` → `tokens/tailwind.css` |
| atomic | `icon-picker`, `media-picker` non repris (dépendent de la base média) |
| AlphaRust | `--accent`/`--accent-contrast` renommés `--brand`/`--brand-contrast` (collision Starwind) ; `starwind.css` + dossier vide `components/starwind` nettoyés |

## Conséquences

* Limites : le ciblage `[data-slot]` couvre 11 composants (button, card, input, textarea, select, badge, alert, dialog,
  tabs, table, tooltip) ; les autres se surchargent dans `custom.css`. Les composants atomic produisent 3 *hints* TypeScript
  (propriété dépréciée, variables inutilisées) conservés tels quels.
* Les projets générés avant cette ADR gardent `src/styles/tokens.css` ; la régénération (`--force`) migre vers la nouvelle structure.
