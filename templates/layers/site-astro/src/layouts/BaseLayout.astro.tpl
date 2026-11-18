@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// Gabarit commun : en-tête, pied de page, SEO, thème — le même que resources/views/components/layouts/site.blade.php
// (Laravel) et templates/base.html (Wagtail).
import '../styles/global.css';
import '../styles/site.css';
import SEO from '../components/SEO.astro';
import SiteFooter from '../components/site/SiteFooter.astro';
import SiteHeader from '../components/site/SiteHeader.astro';
import { getSite } from '@lib/site';

interface Props {
  title: string;
  description?: string;
  /** Pages du site (sections pleine largeur) ; sinon le contenu est centré (catalogue, compte…). */
  bleed?: boolean;
}

const { title, description, bleed = false } = Astro.props;
const site = await getSite(Astro.url.pathname);
---

<!doctype html>
<html lang={site.locale}>
  <head>
    <SEO title={title} description={description ?? site.description} />
    <script is:inline>
      // Applique le thème choisi avant le premier rendu (évite le flash).
      try {
        const saved = localStorage.getItem('theme');
        const theme =
          saved === 'light' || saved === 'dark'
            ? saved
            : window.matchMedia('(prefers-color-scheme: dark)').matches
              ? 'dark'
              : 'light';
        document.documentElement.dataset.theme = theme;
        document.documentElement.classList.toggle('dark', theme === 'dark');
      } catch {}
    </script>
  </head>
  <body class="site-body">
    <a class="site-skip" href="#contenu">{site.ui.skip}</a>
    <SiteHeader site={site} />
    <main id="contenu" class:list={['site-main', { 'site-main--page': !bleed }]} tabindex="-1">
      <slot />
    </main>
    <SiteFooter site={site} />
@@if concordia@@
    <script>
      // Comportements du design kit (onglets, dialogue, accordéon…) : même script que Laravel et Wagtail.
      import '../scripts/ds.js';
    </script>
@@end@@
  </body>
</html>
