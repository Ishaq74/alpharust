# ADR 0017 — Site vitrine : langues au choix (`--langs`), espagnol et arabe (RTL)

## Statut

Accepté (complète l'ADR 0016)

## Contexte

L'ADR 0016 rendait le site vitrine bilingue (français, anglais) dans les trois frameworks, avec les traductions dans
un seul fichier `vitrine/i18n.py`. Il fallait aussi l'espagnol et l'arabe, une langue qui s'écrit de droite à gauche.
L'utilisateur devait pouvoir choisir les langues à la génération et en ajouter d'autres sans toucher aux écrivains de
couches.

## Décision

1. **Une langue = un fichier** `tools/dsgen/vitrine/locales/<code>.py` avec `META` (code, nom du sélecteur, sens
   `ltr`/`rtl`, drapeau Circle Flags, ordre, formats de date `Intl` / PHP / Django), `UI` (libellés d'interface) et
   `TEXT` (texte français de la spec → traduction ; vide pour le français, langue source). `i18n.py` charge les
   fichiers et fait échouer `gen_ds.py` s'il manque un libellé ou une traduction.
2. **Fichiers par langue dans les couches**, conditionnés par un drapeau `lang_<code>`
   (`@@meta when=…&lang_es@@`) : Astro `src/data/locales/<code>.json`, pages `src/pages/<code>/`, collections ;
   Laravel `lang/<code>/site.php`, `database/data/site/<code>.php`, entrées de `config/site.php` ; Wagtail
   `layout/locale/<code>/LC_MESSAGES/django.po`, blocs de traduction de `seed_site`. Les drapeaux `multilingual`
   (au moins deux langues : sélecteur, tests de traduction) et `rtl` complètent.
3. **`--langs fr,en,es,ar`** pour Astro, Laravel et Wagtail (défaut `fr,en`), cases à cocher dans l'assistant et la GUI.
   Le français est obligatoire. AlphaRust lit la liste des langues connues dans `vitrine/locales.json` (générée), et
   en tire `LANGUAGES` (Django) et les langues de routage d'Astro. Pour Astro, `--i18n` sert de valeur par défaut
   (français ajouté) ; si les deux options sont données, elles doivent concorder.
4. **RTL** : `<html dir>` vient de la langue ; les styles du site utilisent les propriétés logiques ; le panneau mobile
   s'ouvre du côté de début de ligne (`sheet_side`) ; les flèches (`data-icon="arrow-…"`) et les séparateurs du fil
   d'Ariane sont retournés par `[dir='rtl']`. Dates en chiffres latins (`ar-u-nu-latn`).

## Conséquences

* Ajouter une langue : un fichier `locales/<code>.py`, éventuellement `fetch_icons.py` pour un nouveau drapeau, puis
  `gen_ds.py`. Aucun écrivain de couche ni code Rust à modifier.
* Les contrôles de parité couvrent 35 pages (dont l'espagnol et l'arabe) et le parcours de compte en anglais, arabe et
  espagnol (`site_auth.py`, 20 contrôles par combinaison).
* Les traductions espagnoles (vouvoiement, « usted ») et arabes (arabe standard moderne) sont écrites à la main et
  relues ; leur qualité dépend de ces fichiers, que le générateur ne peut pas vérifier.
