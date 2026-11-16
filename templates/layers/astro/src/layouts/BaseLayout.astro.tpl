@@meta when=!official@@
---
import '../styles/global.css';
import Footer from '../components/Footer.astro';
import Header from '../components/Header.astro';
import SEO from '../components/SEO.astro';

interface Props {
  title: string;
  description?: string;
}

const { title, description } = Astro.props;
---

<!doctype html>
<html lang="fr">
  <head>
    <SEO title={title} {...(description ? { description } : {})} />
    <script is:inline>
      // Applique le thème choisi avant le premier rendu (évite le flash).
      try {
        const saved = localStorage.getItem('theme');
        if (saved === 'light' || saved === 'dark') {
          document.documentElement.dataset.theme = saved;
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
