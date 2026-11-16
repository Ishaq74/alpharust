@@meta when=!official@@
---
import '../styles/global.css';
@@if starwind@@
// Starwind UI (Tailwind 4) : jetons et composants ; initialisé par `starwind init`.
@@end@@
@@if concordia@@
import '../styles/concordia.css';
@@if theme@@
import '../styles/theme.css';
@@end@@
@@end@@
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
        if (saved === 'light' || saved === 'dark') {
          document.documentElement.dataset.theme = saved;
@@if starwind@@
          document.documentElement.classList.toggle('dark', saved === 'dark');
@@end@@
        }
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
  </body>
</html>
