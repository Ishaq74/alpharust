# Composants (design kit)

Les composants de ce projet viennent du **design kit AlphaRust**, partagé avec les projets Astro (Starwind / CSS) et Wagtail :
une seule source pour les jetons et le CSS, un même balisage (`data-slot`, `data-variant`, `data-size`) partout.

## Où est quoi

```text
resources/
├── css/
│   ├── app.css                 # entrée Vite : Tailwind v4 + le kit dans la couche `ds`
│   └── design/
│       ├── index.css           # importe jetons, thème, base et composants (ordre fixe)
│       ├── tokens/             # colors, typography, spacing, components, site, tailwind
│       ├── themes/theme.css    # palette de marque (à remplir ou générer)
│       ├── base.css
│       └── components/*.css    # un fichier par composant ; custom.css = vos surcharges (chargé en dernier)
├── js/
│   ├── app.js                  # importe design/ds.js
│   └── design/ds.js            # onglets, dialogue, accordéon, interrupteur, info-bulle (HTML natif + ARIA)
└── views/
    ├── components/ds/<composant>/…   # composants Blade anonymes : <x-ds.button>, <x-ds.card.header>…
    └── pages/components.blade.php    # la page /components
```

## Utiliser un composant

```blade
<x-ds.button variant="primary" size="lg">Enregistrer</x-ds.button>
<x-ds.button href="/pages" variant="outline">Lien</x-ds.button>
<x-ds.card>
    <x-ds.card.header><x-ds.card.title>Titre</x-ds.card.title></x-ds.card.header>
    <x-ds.card.content>…</x-ds.card.content>
</x-ds.card>
<x-ds.dialog.trigger :as-child="true"><x-ds.button>Ouvrir</x-ds.button></x-ds.dialog.trigger>
```

Les attributs supplémentaires (`class`, `id`, `aria-*`, `wire:*`…) sont fusionnés par `$attributes->merge()`.

## Personnaliser

1. **Couleurs** : `resources/css/design/themes/theme.css` (ou `tokens/colors.css`).
2. **Allure d'un composant** : son jeton dans `tokens/components.css` (`--button-border-radius: 9999px;`).
3. **Autre chose** : `components/custom.css`, jamais les fichiers des composants.

Le kit est dans la couche CSS `ds`, déclarée entre `base` (preflight Tailwind) et `utilities` : `class="p-4"` sur un composant l'emporte,
et vos règles hors couche l'emportent sur tout. Après modification : `npm run build` (ou `npm run dev`).

## Documentation officielle

- Composants Blade anonymes, `@props`, `$attributes->merge()` : <https://laravel.com/docs/blade#components>
- Vite : <https://laravel.com/docs/vite>
- Tailwind CSS v4 avec Laravel et Vite : <https://tailwindcss.com/docs/installation/framework-guides/laravel/vite>
- Couches CSS (`@layer`) : <https://developer.mozilla.org/en-US/docs/Web/CSS/@layer>
