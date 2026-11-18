@@meta when=!official@@
---
// Entrée unique du design system : jetons, thème, composants (voir README → « Personnaliser le design »).
import '../styles/global.css';
import Footer from '../components/Footer.astro';
import Header from '../components/Header.astro';
import SEO from '../components/SEO.astro';

@@if messages@@
import { DEFAULT_LOCALE, dirOf } from '@i18n/config';
@@end@@

interface Props {
  title: string;
  description?: string;
}

const { title, description } = Astro.props;
@@if messages@@
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
@@end@@
---

<!doctype html>
@@if messages@@
<html lang={locale} dir={dirOf(locale)}>
@@else@@
<html lang="fr">
@@end@@
  <head>
    <SEO title={title} {...(description ? { description } : {})} />
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
@@if starwind@@
        document.documentElement.classList.toggle('dark', theme === 'dark');
@@end@@
      } catch {}
    </script>
  </head>
  <body>
    <a class="skip-link" href="#main">Aller au contenu</a>
    <Header />
    <main id="main">
      <slot />
    </main>
    <Footer />
@@if concordia@@
    <script>
      // Comportements du design kit (onglets, dialogue, accordéon…) : même script que Laravel et Wagtail.
      import '../scripts/ds.js';
    </script>
@@end@@
  </body>
</html>
