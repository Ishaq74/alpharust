@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import BaseLayout from '@layouts/BaseLayout.astro';
import { getSite } from '@lib/site';

const site = await getSite(Astro.url.pathname);
---

<BaseLayout title="Page introuvable" bleed>
<section class="site-section site-404">
  <div class="site-container site-container--narrow">
    <p class="site-404__code">
      404
    </p>
    <h1 class="site-page-header__title">
      Page introuvable
    </h1>
    <p class="site-lead">
      La page demandée n’existe pas ou a été déplacée.
    </p>
    <Button href={site.home_url} variant="primary">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
      Retour à l’accueil
    </Button>
  </div>
</section>
</BaseLayout>
