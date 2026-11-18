# ADR 0014 — Design kit partagé : Starwind, CSS pur, Laravel (Blade) et Wagtail (Django)

Statut : accepté (complète l'ADR 0013)

## Contexte

L'ADR 0013 a unifié l'arborescence `src/styles/` d'Astro et repris les 46 composants Starwind d'atomic. Deux demandes
ont suivi : (1) chaque composant d'atomic doit exister aussi **en CSS pur**, avec la même API, le même balisage et la même
accessibilité, pour que Starwind (Tailwind) et le CSS intégral soient interchangeables ; (2) ces composants serviront aussi
dans des projets **Laravel** et **Wagtail** : il faut une source unique, pas trois copies qui divergent.

## Décision

1. **Un contrat de balisage** : chaque composant est un élément portant `data-slot="<nom>"` (noms d'atomic), plus
   `data-variant` et `data-size` quand le composant a des variantes ou des tailles. Le contrat est écrit dans
   `templates/layers/design-kit/catalog.toml` (23 composants « core » ; les 23 autres composants d'atomic sont déclarés
   `starwind_only`).
2. **Une source pour le CSS** : `templates/layers/design-kit/` contient `css/` (jetons, thème, base, un fichier par composant,
   `custom.css` pour les surcharges) et `js/ds.js` (onglets, dialogue, accordéon, interrupteur, info-bulle). Le CSS ne lit que le
   « pont » `tokens/site.css` (`--bg --surface --surface-2 --text --text-muted --brand --brand-contrast --focus --tone-*`)
   et des jetons par composant avec valeurs de repli ; chaque kit relie ce pont à ses propres jetons.
3. **Une écriture par cible, générée** : le kit est copié par `Internal::DesignKit(cible)` (jamais réécrit s'il existe :
   `custom.css`, `theme.css`) :

   | Cible | CSS | JS | Composants |
   |-------|-----|----|------------|
   | Astro + Starwind | jetons Starwind (+ typographie commune) | — (scripts d'atomic) | atomes d'atomic (Tailwind) |
   | Astro + Concordia | `src/styles/components/*.css` | `src/scripts/ds.js` | `src/components/atoms/*` (`ui-css`) |
   | Astro CSS ou Tailwind seul | `src/styles/tokens/*` | — | — |
   | Laravel (`--ui css`) | `resources/css/design/` | `resources/js/design/ds.js` | `<x-ds.*>` (`ui-blade`) |
   | Wagtail (`--ui css`) | `ds/static/ds/css/` | `ds/static/ds/js/ds.js` | `{% ds_* %}` (`ui-django`) |

   Les composants Astro-CSS, Blade et Django sont produits par un générateur de développement à partir des atomes d'atomic
   (mêmes noms de composants, de sous-composants et de props) ; les fichiers générés sont versionnés dans `templates/layers/`.
4. **Pas de doublon avec Concordia** : les 23 composants du catalogue sont l'API canonique ; les composants propres à
   Concordia (`src/components/ui/`) restent verbatim et ont leur page `/concordia`. Les feuilles de Concordia sont importées
   dans la couche CSS `concordia`, ce qui laisse les composants du catalogue l'emporter sans guerre de spécificité (le CSS
   élémentaire de Concordia, p. ex. `input:not([type=…])`, aurait sinon gagné).
5. **Même page de démonstration partout** : `/components` (Astro Starwind et Concordia : un seul gabarit `ui-catalog`),
   `/components` (Laravel), `/components/` (Wagtail), avec les mêmes sections (`data-demo=…`).
6. **Laravel** (squelette sans Breeze) : composants Blade anonymes ; Vite + Tailwind v4 par le plugin `@tailwindcss/vite`
   déjà présent dans le squelette ; le kit est importé dans une couche `ds` (`@layer theme, base, ds, components, utilities;`),
   donc après le preflight de Tailwind et avant ses utilitaires. `@vite` n'est émis que si `public/build/manifest.json` ou
   `public/hot` existe, pour que les tests passent avant le premier build.
7. **Wagtail** : application Django `ds` (balises `simple_tag` / `simple_block_tag`, fichiers statiques d'application, vue
   `TemplateView`). Aucun outil de build : pas de Tailwind côté Wagtail. Les trois fichiers du squelette `wagtail start`
   (`settings/base.py`, `urls.py`, `templates/base.html`) sont modifiés par insertion après une ancre ; si l'ancre manque,
   la génération échoue avec un message plutôt que de patcher à l'aveugle (test d'idempotence inclus).
8. **Parité vérifiée** (`src/scaffold/catalog.rs`) : chaque composant core a son CSS, ses atomes Astro, ses composants Blade et sa
   balise Django ; chaque slot du contrat est présent dans chaque cible ; chaque slot des atomes Starwind est dans le contrat
   ou déclaré `atom_only` ; chaque dossier d'atomes est « core » ou `starwind_only` ; chaque page de catalogue montre chaque
   composant. Un contrôle **runtime** compare les `data-slot` réellement rendus par les quatre pages.

## Sources officielles suivies

- Astro, styles : <https://docs.astro.build/en/guides/styling/> — Starwind UI : <https://starwind.dev/docs/getting-started/>
- Tailwind CSS v4 : `@theme`, `@custom-variant`, `@source`, `@layer` — <https://tailwindcss.com/docs/functions-and-directives> ;
  avec Laravel et Vite : <https://tailwindcss.com/docs/installation/framework-guides/laravel/vite>
- Laravel : composants Blade anonymes, `@props`, `$attributes->merge()` — <https://laravel.com/docs/blade#components> ; Vite — <https://laravel.com/docs/vite>
- Django : balises de gabarit (`simple_tag`, `simple_block_tag`, ajoutée en 5.2) — <https://docs.djangoproject.com/en/stable/howto/custom-template-tags/> ;
  fichiers statiques des applications — <https://docs.djangoproject.com/en/stable/howto/static-files/> ;
  `format_html`, `format_html_join` — <https://docs.djangoproject.com/en/stable/ref/utils/#module-django.utils.html> ; Wagtail — <https://docs.wagtail.org/en/stable/>
- Accessibilité : `<dialog>` — <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog> ; motif ARIA des onglets —
  <https://www.w3.org/WAI/ARIA/apg/patterns/tabs/> ; `@layer` — <https://developer.mozilla.org/en-US/docs/Web/CSS/@layer>

## Limites assumées

- 23 des 46 composants d'atomic existent en CSS ; les autres (carrousel, barre latérale, toast, sélecteurs riches…) restent
  réservés à Starwind (liste `starwind_only`).
- Les atomes d'atomic sont repris tels quels sauf un ajout : `data-variant` (et `data-size`) sur `Button`, `Badge`, `Alert`,
  `Progress`. Cinq slots du contrat n'ont pas d'équivalent dans le DOM d'atomic (`checkbox`, `switch`, `tooltip-trigger`,
  `pagination-previous`, `pagination-next` : atomic écrase le slot par celui de `PaginationLink`) ; ils sont listés dans
  `[known_gaps]` du catalogue, et le contrôle runtime refuse tout autre écart.
- `--ui css` pour Laravel exige `--starter none` : Breeze impose ses propres vues et feuilles de style.
- Le thème WCAG généré (`--theme-*`) reste réservé à Astro ; Laravel et Wagtail personnalisent `themes/theme.css` à la main.
