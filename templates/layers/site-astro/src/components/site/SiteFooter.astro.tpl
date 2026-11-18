@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Site } from '@lib/site';

interface Props {
  site: Site;
}

const { site } = Astro.props;
---

<footer class="site-footer">
  <div class="site-container site-footer__grid">
    <div class="site-footer__about">
      <a class="site-brand" href={site.home_url}>
        <span class="site-brand__mark" aria-hidden="true">
          {site.initial}
        </span>
        <span class="site-brand__name">
          {site.name}
        </span>
      </a>
      <p class="site-footer__tagline">
        {site.tagline}
      </p>
      <ul class="site-social" aria-label={site.ui.social}>
        {(site.social ?? []).map((link: any) => (
          <Fragment>
            <li>
              <a href={link.url} aria-label={link.label} rel="me noopener">
                <SiteIcon name={link.icon} size={20} />
              </a>
            </li>
          </Fragment>
        ))}
      </ul>
    </div>
    {(site.footer ?? []).map((col: any) => (
      <Fragment>
        <nav class="site-footer__col" aria-label={col.title}>
          <h2 class="site-footer__title">
            {col.title}
          </h2>
          <ul class="site-footer__links">
            {(col.links ?? []).map((link: any) => (
              <Fragment>
                <li>
                  <a href={link.url}>
                    {link.label}
                  </a>
                </li>
              </Fragment>
            ))}
          </ul>
        </nav>
      </Fragment>
    ))}
    <div class="site-footer__col">
      <h2 class="site-footer__title">
        {site.ui.contact}
      </h2>
      <address class="site-footer__contact">
        <a href={site.contact.email_url}>
          {site.contact.email}
        </a>
        <a href={site.contact.phone_url}>
          {site.contact.phone}
        </a>
        <span>
          {site.contact.address}
        </span>
      </address>
    </div>
  </div>
  <div class="site-container site-footer__bottom">
    <p>
      {site.copyright_line}
    </p>
  </div>
</footer>
