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
   `templates/layers/design-kit/catalog.toml` : 23 composants `kind = "core"` (quatre cibles) et 23 composants
   `kind = "css"` (atomes Starwind + équivalents CSS pur Astro). Les 46 dossiers d'atomic y sont tous déclarés.
2. **Une source pour le CSS** : `templates/layers/design-kit/` contient `css/` (jetons, thème, base, un fichier par composant,
   `custom.css` pour les surcharges, `utilities.css` pour `.sr-only` et les animations) et `js/ds.js` (onglets, dialogue,
   accordéon, interrupteur, info-bulle, et pour les composants étendus : menus et sous-menus, popover, select avec recherche
   et saisie au clavier, OTP, curseur, carrousel, barre latérale, toasts…). Le CSS ne lit que le
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

   Les composants Astro-CSS, Blade et Django sont produits par le générateur versionné **`tools/dsgen/gen_ds.py`** à partir
   des atomes d'atomic (mêmes noms de composants, de sous-composants et de props) : `spec.py` décrit le catalogue core,
   `ext_astro*.py` les 23 composants étendus côté Astro et `ext_server.py` leurs gabarits Blade et Django : chaque partie
   est décrite **une fois** (représentation intermédiaire : éléments, attributs, conditions, boucles, `<slot>`), puis rendue
   en composant Blade anonyme (`@props`, `$attributes->merge`) et en balise Django (`simple_block_tag` / `simple_tag`) ;
   les parties simples sont dérivées automatiquement de l'atome CSS pur. Les fichiers générés sont versionnés dans `templates/layers/` et la CI
   régénère puis échoue sur tout écart (`git diff --exit-code templates/layers`) : aucune copie ne peut diverger.
   Comme dans atomic, `{...rest}` précède `data-slot` sur les éléments HTML (Astro ne fusionne pas les attributs en double
   et le navigateur garde le premier) : un parent peut surcharger le slot (`InputGroupInput` → `input-group-control`).
4. **Pas de doublon avec Concordia** : les 23 composants du catalogue sont l'API canonique ; les composants propres à
   Concordia (`src/components/ui/`) restent verbatim et ont leur page `/concordia`. Les feuilles de Concordia sont importées
   dans la couche CSS `concordia`, ce qui laisse les composants du catalogue l'emporter sans guerre de spécificité (le CSS
   élémentaire de Concordia, p. ex. `input:not([type=…])`, aurait sinon gagné).
5. **Même page de démonstration partout** : `/components` (Astro Starwind et Concordia : un seul gabarit `ui-catalog`),
   `/components` (Laravel), `/components/` (Wagtail), avec les mêmes sections (`data-demo=…`). Les composants étendus ont
   leur page `/components/advanced` (Astro, Laravel) et `/components/advanced/` (Wagtail), mêmes sections, liée depuis
   `/components`.
5 bis. **Composants étendus en CSS pur** : mêmes fichiers que l'atome Starwind (y compris `index.ts`, `*Types.ts`,
   `toast-manager.ts`, `carousel-script.ts`), mêmes props, mêmes `data-slot`, mêmes événements
   (`starwind-select:change`, `starwind-toggle:change`, `starwind:value-change`, `starwind-input-otp:change`,
   `slider-change`/`slider-commit`, `theme:change`, `sidebar:change`, gestionnaire `window.__starwind__.toast`). Seuls les
   objets `*Variants` (fonctions tailwind-variants) ne sont pas exportés. Leurs feuilles vivent dans le design kit, donc
   Laravel et Wagtail les reçoivent aussi, avec les **46** composants en Blade (`<x-ds.select.trigger>`) et en balises Django
   (`{% ds_select_trigger %}`), même balisage que l'atome (vérifié attribut par attribut à l'exécution).
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
   ou déclaré `atom_only` ; chaque dossier d'atomes est déclaré (core ou css) ; chaque composant css a les mêmes fichiers,
   slots et exports que son atome ; chaque page de catalogue montre chaque composant. Un contrôle **runtime** compare les
   `data-slot` réellement rendus par les quatre pages `/components` et par les deux pages `/components/advanced`.

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

- Les 46 composants d'atomic existent en CSS pur pour Astro ; en Blade et en Django, seuls les 23 composants core existent
  (les feuilles des 23 autres sont livrées, leurs gabarits serveur restent à écrire).
- Les atomes d'atomic sont repris tels quels sauf un ajout : `data-variant` (et `data-size`, valeurs par défaut comprises) sur
  `Button`, `Badge`, `Alert`, `Switch`, `Progress`. Trois slots du contrat n'ont pas d'équivalent dans le DOM d'atomic
  (`checkbox`, `switch`, `tooltip-trigger`) et atomic ajoute `dialog-backdrop` ; ils sont listés dans `[known_gaps]` du
  catalogue, et le contrôle runtime refuse tout autre écart. Le contrôle lit les attributs comme le navigateur (en cas de
  doublon, le premier l'emporte) : l'écart `pagination-previous`/`pagination-next` relevé auparavant venait du script de
  contrôle, pas d'atomic.
- Sur `/components/advanced`, les composants étendus CSS posent `data-variant` comme crochet de style là où l'atome d'atomic
  (verbatim) n'en pose pas : différence informative, les `data-slot` sont identiques.
- `--ui css` pour Laravel exige `--starter none` : Breeze impose ses propres vues et feuilles de style.
- Le thème WCAG généré (`--theme-*`) reste réservé à Astro ; Laravel et Wagtail personnalisent `themes/theme.css` à la main.
