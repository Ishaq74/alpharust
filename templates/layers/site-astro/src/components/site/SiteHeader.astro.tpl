@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@atoms/collapsible';
import { Dropdown, DropdownContent, DropdownItem, DropdownTrigger } from '@atoms/dropdown';
import { Sheet, SheetContent, SheetFooter, SheetHeader, SheetTitle, SheetTrigger } from '@atoms/sheet';
import { ThemeToggle } from '@atoms/theme-toggle';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Site } from '@lib/site';

interface Props {
  site: Site;
}

const { site } = Astro.props;
---

<header class="site-header">
  <div class="site-container site-header__bar">
    <a class="site-brand" href={site.home_url}>
      <span class="site-brand__mark" aria-hidden="true">
        {site.initial}
      </span>
      <span class="site-brand__name">
        {site.name}
      </span>
    </a>
    <nav class="site-nav" aria-label={site.ui.main_nav}>
      <ul class="site-nav__list">
        {(site.nav ?? []).map((item: any) => (
          <Fragment>
            <li>
              {item.children ? (
                <Fragment>
                  <Dropdown openOnHover class="site-nav__dropdown">
                    <DropdownTrigger class="site-nav__link" aria-current={item.current || undefined}>
                      {item.label}
                      <SiteIcon name="chevron-down" size={16} />
                    </DropdownTrigger>
                    <DropdownContent class="site-nav__menu">
                      <DropdownItem as="a" href={item.url}>
                        {item.label}
                      </DropdownItem>
                      {(item.children ?? []).map((child: any) => (
                        <Fragment>
                          <DropdownItem as="a" href={child.url} aria-current={child.current || undefined}>
                            {child.label}
                          </DropdownItem>
                        </Fragment>
                      ))}
                    </DropdownContent>
                  </Dropdown>
                </Fragment>
              ) : (
                <Fragment>
                  <a class="site-nav__link" href={item.url} aria-current={item.current || undefined}>
                    {item.label}
                  </a>
                </Fragment>
              )}
            </li>
          </Fragment>
        ))}
      </ul>
    </nav>
    <div class="site-header__actions">
      {site.account && (
        <Fragment>
          <Button href={site.account.url} variant="ghost" size="sm" class="site-header__account">
            <SiteIcon name="user" size={18} />
            <span>
              {site.account.label}
            </span>
          </Button>
        </Fragment>
      )}
      <Button href={site.cta.url} variant="primary" size="sm" class="site-header__cta">
        {site.cta.label}
      </Button>
      <nav class="site-lang" aria-label={site.ui.language}>
        <ul class="site-lang__list">
          {(site.languages ?? []).map((lang: any) => (
            <Fragment>
              <li>
                <a class="site-lang__link" href={lang.url} hreflang={lang.code} lang={lang.code} aria-current={lang.current || undefined} title={lang.name}>
                  <SiteIcon name={lang.flag} size={16} />
                  <span>
                    {lang.label}
                  </span>
                </a>
              </li>
            </Fragment>
          ))}
        </ul>
      </nav>
      <ThemeToggle size="sm" ariaLabel={site.ui.theme} />
      <Sheet class="site-header__mobile">
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" aria-label={site.ui.open_menu}>
            <SiteIcon name="menu" size={22} />
          </Button>
        </SheetTrigger>
        <SheetContent side="right" class="site-mobile">
          <SheetHeader>
            <SheetTitle>
              {site.name}
            </SheetTitle>
          </SheetHeader>
          <nav class="site-mobile__nav" aria-label={site.ui.mobile_nav}>
            <ul class="site-mobile__list">
              {(site.nav ?? []).map((item: any) => (
                <Fragment>
                  <li>
                    {item.children ? (
                      <Fragment>
                        <Collapsible class="site-mobile__group">
                          <CollapsibleTrigger class="site-mobile__link">
                            {item.label}
                            <SiteIcon name="chevron-down" size={16} />
                          </CollapsibleTrigger>
                          <CollapsibleContent>
                            <ul class="site-mobile__sub">
                              <li>
                                <a href={item.url}>
                                  {item.label}
                                </a>
                              </li>
                              {(item.children ?? []).map((child: any) => (
                                <Fragment>
                                  <li>
                                    <a href={child.url} aria-current={child.current || undefined}>
                                      {child.label}
                                    </a>
                                  </li>
                                </Fragment>
                              ))}
                            </ul>
                          </CollapsibleContent>
                        </Collapsible>
                      </Fragment>
                    ) : (
                      <Fragment>
                        <a class="site-mobile__link" href={item.url} aria-current={item.current || undefined}>
                          {item.label}
                        </a>
                      </Fragment>
                    )}
                  </li>
                </Fragment>
              ))}
            </ul>
          </nav>
          <SheetFooter>
            {site.account && (
              <Fragment>
                <Button href={site.account.url} variant="outline" class="site-mobile__cta">
                  <SiteIcon name="user" size={18} />
                  {site.account.label}
                </Button>
              </Fragment>
            )}
            <Button href={site.cta.url} variant="primary" class="site-mobile__cta">
              {site.cta.label}
            </Button>
          </SheetFooter>
        </SheetContent>
      </Sheet>
    </div>
  </div>
</header>
