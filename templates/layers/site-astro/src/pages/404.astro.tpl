@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import SiteIcon from '@/components/site/SiteIcon.astro';
import BaseLayout from '@layouts/BaseLayout.astro';
import { getSite } from '@lib/site';

const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={site.ui.not_found} bleed>
<section class="site-section site-404">
  <div class="site-container site-container--narrow">
    <p class="site-404__code">
      404
    </p>
    <h1 class="site-page-header__title">
      {site.ui.not_found}
    </h1>
    <p class="site-lead">
      {site.ui.not_found_text}
    </p>
    <Button href={site.home_url} variant="primary">
      <SiteIcon name="arrow-left" size={16} />
      {site.ui.back_home}
    </Button>
  </div>
</section>
</BaseLayout>
