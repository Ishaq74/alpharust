@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@atoms/collapsible';
import { Dropdown, DropdownContent, DropdownItem, DropdownTrigger } from '@atoms/dropdown';
import { Sheet, SheetContent, SheetFooter, SheetHeader, SheetTitle, SheetTrigger } from '@atoms/sheet';
import { ThemeToggle } from '@atoms/theme-toggle';
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
    <nav class="site-nav" aria-label="Navigation principale">
      <ul class="site-nav__list">
        {(site.nav ?? []).map((item: any) => (
          <Fragment>
            <li>
              {item.children ? (
                <Fragment>
                  <Dropdown openOnHover class="site-nav__dropdown">
                    <DropdownTrigger class="site-nav__link" aria-current={item.current || undefined}>
                      {item.label}
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
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
      <Button href={site.cta.url} variant="primary" size="sm" class="site-header__cta">
        {site.cta.label}
      </Button>
      <ThemeToggle size="sm" ariaLabel="Changer de thème" />
      <Sheet class="site-header__mobile">
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" aria-label="Ouvrir le menu">
            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 6h16M4 12h16M4 18h16"/></svg>
          </Button>
        </SheetTrigger>
        <SheetContent side="right" class="site-mobile">
          <SheetHeader>
            <SheetTitle>
              {site.name}
            </SheetTitle>
          </SheetHeader>
          <nav class="site-mobile__nav" aria-label="Navigation mobile">
            <ul class="site-mobile__list">
              {(site.nav ?? []).map((item: any) => (
                <Fragment>
                  <li>
                    {item.children ? (
                      <Fragment>
                        <Collapsible class="site-mobile__group">
                          <CollapsibleTrigger class="site-mobile__link">
                            {item.label}
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
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
            <Button href={site.cta.url} variant="primary" class="site-mobile__cta">
              {site.cta.label}
            </Button>
          </SheetFooter>
        </SheetContent>
      </Sheet>
    </div>
  </div>
</header>
