# ADR 0015 — Une spec de site partagée : même site vitrine en Astro, Laravel et Wagtail

Statut : accepté (complète les ADR 0013 et 0014)

## Contexte

L'ADR 0014 a rendu les **composants** identiques dans les quatre cibles (Starwind, CSS pur, Blade, Django). Les projets
générés restaient pourtant inégaux : Astro avait un gabarit et des pages, Laravel et Wagtail seulement une page de
catalogue. La demande : générer un **site complet** (gabarit, en-tête, menu mobile, thème, pied de page, pages, contenus,
formulaire) et obtenir **le même résultat** quel que soit le framework (Astro, Laravel, Wagtail) et le kit
(Tailwind + Starwind ou CSS pur), en suivant les dépôts de référence et les docs officielles.

## Décision

1. **Une source unique** : `tools/dsgen/vitrine/` (paquet Python, lancé par `gen_ds.py`).
   * `spec.py` : identité du site, menu (avec sous-menu des services), pied de page, réseaux, coordonnées, 4 services,
     3 articles, pages (accueil, à propos, services, blog, contact, mentions légales, confidentialité) décrites par
     **sections** typées, page 404, champs du formulaire de contact ;
   * `templates.py` : en-tête, pied de page, 12 sections (`hero`, `page_header`, `stats`, `features`, `services`,
     `testimonials`, `posts`, `faq`, `cta`, `text`, `team`, `contact`), fiches service et article, 404 — écrits **une fois**
     dans une petite représentation intermédiaire (`ir.py`) faite des composants du contrat de l'ADR 0014 ;
   * trois moteurs de rendu idiomatiques : Astro (`@atoms/*`), Blade (`<x-ds.*>`), Django (`{% ds_* %}`). Le catalogue
     vérifie que chaque composant utilisé existe dans les trois kits ;
   * `site.css` : le gabarit (`site-*`), seul fichier de mise en page, identique partout ; il ne lit que le contrat de
     jetons (`--bg`, `--text`, `--brand`…) et fixe ce qui différait d'un kit à l'autre (police, neutres dérivés du fond et du
     texte, retraits des cartes, typographie des contenus riches `site-prose`).
2. **Trois couches générées** (`templates/layers/site-astro`, `site-laravel`, `site-wagtail`), réécrites à chaque
   `gen_ds.py` et vérifiées par la CI (`git diff --exit-code templates/layers`).
3. **Le modèle de chaque framework est respecté** :

   | | Astro | Laravel (modèle **manecms**) | Wagtail (modèle **wagtail-cms**) |
   |---|---|---|---|
   | Gabarit | `BaseLayout.astro` + `SiteHeader`/`SiteFooter` | composant anonyme `layouts.site` + partials `head`/`header`/`footer` | `base.html` + `includes/header.html`/`footer.html` |
   | Données du site | `src/data/site.json` + `src/lib/site.ts` | `config/site.php`, `SiteData`, composer de vue (`SiteServiceProvider`) | app `layout` (menus `NavigationMenu`/`NavigationItem` en snippet, `HeaderSettings`/`FooterSettings`), app `site_settings` (organisation, contact, réseaux), processeur de contexte |
   | Pages | `src/pages/*` + `Sections.astro` | contrôleurs + `site/page.blade.php` (sections) | `HomePage`, `FlexPage` (StreamField), `ServiceIndexPage`/`ServicePage`, `BlogIndexPage`/`BlogPage` |
   | Sections | composants `components/site/sections/*` | vues `site/sections/*` | blocs `streams/blocks.py` (StructBlock + gabarit) |
   | Contenus | collections `services` et `blog` | modèles `Service`, `Post` + migrations | pages Wagtail |
   | Contact | validation serveur (`ssr`) | `ContactController` + `ContactMessage` | `ContactPage` (`AbstractEmailForm`, `FormField`) |
   | Données de départ | fichiers générés | `SiteSeeder` (`db:seed`) | commande `seed_site` (idempotente) |
   | 404 | `404.astro` | `errors/404.blade.php` | `404.html` |

4. **Même palette partout** : `--theme-primary/secondary/accent` est accepté pour `generate laravel|wagtail --ui css`
   (écrit `themes/theme.css` du kit), comme pour Astro.
5. **Parité vérifiée** : `docs/screenshots/scripts/site_parity.py` parcourt 10 pages × 4 combinaisons (Astro Starwind,
   Astro CSS, Laravel, Wagtail), bureau 1280 px et mobile 390 px, compare une signature de structure (repères, titres, menu,
   sous-menu, menu mobile, pied de page, champs, boutons, cartes, lien d'évitement, page courante), relève les erreurs de
   console et produit des planches côte à côte.

## Ce qui a été repris

* **wagtail-cms** : découpage en apps (`layout`, `site_settings`, `streams`, apps de pages), menus en snippet
  `ClusterableModel` + `Orderable` à `ParentalKey` (enfants, page ou lien libre), réglages `BaseGenericSetting` +
  `register_setting`, blocs StreamField avec gabarit, `ContactPage` sur `AbstractEmailForm`, processeur de contexte.
* **manecms** : gabarit en composant Blade anonyme + partials, composer de vue enregistré par un provider, `config/site.php`,
  contrôleurs fins, modèles + migrations + seeder.
* **atomic / polyweb** : les composants Starwind (ADR 0013), les jetons, et les types de section de
  `src/lib/section-schemas.ts` (`hero`, `text`, `cta`, `features`, `testimonials`, `faq`, `contact`).

## Conséquences

* Un changement de page, de section ou de menu se fait dans `spec.py`/`templates.py` puis `gen_ds.py` : les trois
  frameworks suivent.
* Différences restantes connues, propres aux kits : le déclencheur d'accordéon de Starwind est un `<button>` (CSS pur :
  `<summary>`), les neutres de Starwind (atomic) sont chauds, les ombres et anneaux de cartes diffèrent légèrement.
* En Astro, le formulaire de contact ne traite le POST qu’en rendu serveur (`ssr`, adaptateur Node) ; en site statique il reste affiché sans traitement.
