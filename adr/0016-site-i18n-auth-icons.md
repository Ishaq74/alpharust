# ADR 0016 — Site vitrine : anglais, comptes, kit Starwind serveur et icônes MDI

## Statut

Accepté

## Contexte

L'ADR 0015 donnait le même site vitrine en Astro, Laravel et Wagtail, en français seulement, sans comptes, et en
CSS pur seulement côté Laravel et Wagtail. Il restait donc trois écarts entre les frameworks : la langue, les comptes et
le kit Tailwind + Starwind. Les icônes, des tracés Lucide écrits en dur, ne correspondaient pas au jeu par défaut de la
plateforme (MDI, `@iconify-json/mdi`).

## Décision

1. **Six combinaisons** : `--ui starwind` existe aussi pour Laravel (Tailwind v4 via `@tailwindcss/vite`) et Wagtail
   (CLI autonome Tailwind v4.3.5 téléchargée dans `.tailwind/`). `tools/dsgen/starwind_server.py` traduit les `tv()`
   des atomes Starwind d'atomic en règles `[data-slot][data-variant] { @apply … }`. Un atome composé (la bascule de
   thème rend une toggle) reçoit aussi les règles de l'atome qu'il compose.
2. **Langues** : chaque framework utilise **son i18n officielle**. Astro range les pages dans des dossiers par langue.
   Laravel passe par `lang/<langue>/site.php`, `trans()` et un middleware de langue. Wagtail active `WAGTAIL_I18N_ENABLED`,
   `wagtail.locales`, `LocaleMiddleware` et `i18n_patterns`, avec gettext et un arbre de pages EN. Le français est la
   langue par défaut, sans préfixe ; l'anglais est servi sous `/en/…` ; les slugs sont communs. Les traductions sont dans
   `vitrine/i18n.py` : une chaîne sans traduction fait échouer `gen_ds.py`. AlphaRust compile les `.po` en `.mo`
   (`Internal::CompileMessages`) pour ne pas exiger GNU gettext.
3. **Comptes** (`--auth`) : les pages `/auth/sign-in`, `/auth/sign-up` et `/auth/dashboard` sont décrites une fois
   (`templates.AUTH_FORM`, `ACCOUNT`), avec le lien « Connexion » / « Mon compte » dans l'en-tête. Le moteur dépend du
   framework : Better Auth (API serveur) en Astro, **Laravel Fortify** en Laravel (sans interface, ce qui laisse le
   site rendre ses propres pages ; Breeze imposerait ses vues), **django.contrib.auth** en Wagtail (routes de
   `HomePage` via `RoutablePageMixin`, donc présentes dans chaque langue sans toucher `urls.py`). Les contrôles et les
   messages sont communs et passent dans le même ordre.
4. **Icônes** : MDI par défaut pour les pictogrammes, Circle Flags pour les drapeaux du sélecteur de langue. Les tracés
   d'un projet sont dans un seul `icons.json`, lu par les trois frameworks. `--icons <jeu>` le réécrit à partir d'un jeu
   embarqué (`mdi`, `lucide` dans `vitrine/iconsets.json`, rafraîchi par `fetch_icons.py`) ou de n'importe quel jeu
   Iconify (API publique). L'icône MDI est gardée quand le jeu n'en a aucune.

## Conséquences

* La parité est mesurée sur 19 pages (dont 6 en anglais et 3 de comptes) et 6 combinaisons : 0 différence de
  structure, liens compris, 0 erreur de console, écart maximal de 0,84 % des pixels. Le parcours de compte
  (`site_auth.py`) passe 78 contrôles sur 78.
* Les projets générés testent eux-mêmes les langues et les comptes : `SiteTest`, `AuthTest` (Laravel), `test_site`,
  `test_auth` (Wagtail).
* Une langue ou une page de plus se déclare dans la spec ; les trois frameworks la reçoivent à la régénération.
