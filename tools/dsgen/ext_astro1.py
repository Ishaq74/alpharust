# Composants CSS pur (Astro) — partie 1 : alert-dialog, aspect-ratio, button-group, carousel, collapsible, container, dropdown
F = {}
HA = 'import type { HTMLAttributes } from "astro/types";\n'
ICON = {
 'x': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>',
 'left': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-left"><path d="M5 12h14M5 12l6 6M5 12l6-6"/></svg>',
 'right': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-right"><path d="M5 12h14M13 18l6-6M13 6l6 6"/></svg>',
 'chev-right': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="chevron-right"><path d="m9 6 6 6-6 6"/></svg>',
 'chev-down': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="chevron-down"><path d="m6 9 6 6 6-6"/></svg>',
 'check': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="check"><path d="m5 12 5 5L20 7"/></svg>',
 'search': '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="search"><circle cx="10" cy="10" r="7"/><path d="m21 21-6-6"/></svg>',
 'upload': '<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="cloud-upload"><path d="M7 18a4.6 4.4 0 0 1 0-9 5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1"/><path d="m9 15 3-3 3 3M12 12v9"/></svg>',
 'loader': '<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg>',
 'minus': '<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true" data-icon="minus"><path d="M5 12h14"/></svg>',
 'dot': '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" data-icon="circle"><circle cx="12" cy="12" r="6"/></svg>',
 'panel': '<svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="layout-sidebar"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 4v16"/></svg>',
 'sun': '<svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="light"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg>',
 'moon': '<svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="dark"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9z"/></svg>',
 'success': '<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m9 12 2 2 4-4"/></svg>',
 'error': '<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m10 10 4 4m0-4-4 4"/></svg>',
 'warning': '<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 9v4M12 17h.01M10.3 3.9 1.8 18a2 2 0 0 0 1.7 3h17a2 2 0 0 0 1.7-3L13.7 3.9a2 2 0 0 0-3.4 0z"/></svg>',
 'info': '<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M12 8h.01M11 12h1v4h1"/></svg>',
}

def simple(tag, slot, extra_attrs='', props='', destr='', ptype=None):
    t = ptype or f'HTMLAttributes<"{tag}">'
    pt = f'type Props = {t}' + (f' & {{\n{props}}};' if props else ';')
    d = (destr + ', ') if destr else ''
    return f'---\n{HA}\n{pt}\n\nconst {{ {d}class: className, ...rest }} = Astro.props;\n---\n\n<{tag} class={{className}}{extra_attrs} {{...rest}} data-slot="{slot}">\n  <slot />\n</{tag}>\n'

def trigger(slot, for_attr=None, popup=None, extra=''):
    forp = '  /** `id` du conteneur à ouvrir ; inutile à l\'intérieur de celui-ci. */\n  for?: string;\n' if for_attr else ''
    dfor = f' {for_attr}={{target}}' if for_attr else ''
    destr = 'for: target, ' if for_attr else ''
    hp = f' aria-haspopup="{popup}"' if popup else ''
    return f'''---
{HA}
type Props = Omit<HTMLAttributes<"button">, "type"> & {{
{forp}  /** Habille un élément enfant (ex. <Button>) au lieu de rendre son propre <button>. */
  asChild?: boolean;
}};

const {{ {destr}asChild = false, class: className, ...rest }} = Astro.props;
---

{{
  asChild ? (
    <div class={{className}} data-slot="{slot}"{dfor} data-as-child{extra}>
      <slot />
    </div>
  ) : (
    <button type="button" class={{className}}{hp}{dfor}{extra} {{...rest}} data-slot="{slot}">
      <slot />
    </button>
  )
}}
'''

A = 'alert-dialog/'
F[A+'AlertDialog.astro'] = simple('div', 'alert-dialog')
F[A+'AlertDialogTrigger.astro'] = trigger('alert-dialog-trigger', 'data-dialog-for', 'dialog')
F[A+'AlertDialogContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"dialog"> & {{
  /** Durée des animations d'ouverture et de fermeture (ms). */
  animationDuration?: number;
}};

const {{ animationDuration = 200, class: className, ...rest }} = Astro.props;
---

<!-- Fond : le ::backdrop natif du <dialog> fait le travail ; l'élément reste pour la parité du balisage. -->
<slot name="backdrop">
  <div data-state="closed" data-slot="alert-dialog-backdrop" aria-hidden="true"></div>
</slot>
<dialog
  class={{className}}
  data-state="closed"
  role="alertdialog"
  style={{{{ "--ds-duration": `${{animationDuration}}ms` }}}}
  {{...rest}}
  data-slot="alert-dialog-content"
>
  <slot />
</dialog>
'''
F[A+'AlertDialogHeader.astro'] = simple('div', 'alert-dialog-header')
F[A+'AlertDialogFooter.astro'] = simple('div', 'alert-dialog-footer')
F[A+'AlertDialogTitle.astro'] = simple('h2', 'alert-dialog-title')
F[A+'AlertDialogDescription.astro'] = simple('p', 'alert-dialog-description')
for name, slot, var in (('AlertDialogAction', 'alert-dialog-action', 'primary'), ('AlertDialogCancel', 'alert-dialog-cancel', 'outline')):
    F[A+name+'.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button"> & {{
  /** Habille un élément enfant au lieu de rendre son propre <button>. */
  asChild?: boolean;
}};

const {{ asChild = false, class: className, ...rest }} = Astro.props;
---

{{
  asChild ? (
    <div class={{className}} data-slot="{slot}" data-as-child>
      <slot />
    </div>
  ) : (
    <button type="button" class={{className}} data-variant="{var}" {{...rest}} data-slot="{slot}">
      <slot />
    </button>
  )
}}
'''

F['aspect-ratio/AspectRatio.astro'] = '''---
import type { HTMLAttributes, HTMLTag } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  /** Élément rendu (défaut : div). */
  as?: HTMLTag;
  /** Largeur / hauteur (ex. 16 / 9). */
  ratio?: number;
};

const { ratio = 1, as: Tag = "div", class: className, ...rest } = Astro.props;
---

{/* alpharust-ignore: no-inline-styles — padding calculé par instance, comme l'atome d'atomic */}
<div style={{ "--ds-ratio": String(ratio), paddingBottom: `${100 / ratio}%` }} data-slot="aspect-ratio-wrapper">
  <Tag class={className} data-slot="aspect-ratio" {...rest}>
    <slot />
  </Tag>
</div>
'''

B = 'button-group/'
F[B+'ButtonGroup.astro'] = simple('div', 'button-group', ' role="group" data-orientation={orientation}',
    '  orientation?: "horizontal" | "vertical";\n', 'orientation = "horizontal"')
F[B+'ButtonGroupSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  orientation?: "horizontal" | "vertical";
}};

const {{ orientation = "vertical", class: className, ...rest }} = Astro.props;
---

<div class={{className}} role="separator" aria-orientation={{orientation}} data-orientation={{orientation}} {{...rest}} data-slot="button-group-separator"></div>
'''
F[B+'ButtonGroupText.astro'] = simple('div', 'button-group-text')

C = 'carousel/'
F[C+'Carousel.astro'] = f'''---
{HA}
import type {{ CarouselOptions }} from "./carousel-script";

export interface Props extends HTMLAttributes<"div"> {{
  orientation?: "horizontal" | "vertical";
  /** Options (sous-ensemble compatible avec Embla : loop, align, startIndex). */
  opts?: CarouselOptions;
  /** Initialisé automatiquement par ds.js (défaut : true). */
  autoInit?: boolean;
  "aria-label"?: string;
}}

const {{
  orientation = "horizontal",
  opts = {{}},
  autoInit = true,
  "aria-label": ariaLabel,
  class: className,
  ...rest
}} = Astro.props;
---

<div
  class={{className}}
  role="region"
  aria-roledescription="carousel"
  aria-label={{ariaLabel || "Carousel"}}
  data-axis={{orientation === "horizontal" ? "x" : "y"}}
  data-opts={{JSON.stringify(opts)}}
  data-auto-init={{autoInit}}
  {{...rest}}
  data-slot="carousel"
>
  <slot />
</div>
'''
F[C+'carousel-script.ts'] = '''/**
 * Carrousel CSS pur : défilement natif (scroll-snap) piloté par ds.js. Même API que la version Starwind (Embla) :
 * `initCarousel(élément)` renvoie un petit contrôleur (précédent, suivant, aller à).
 */
export interface CarouselOptions {
  loop?: boolean;
  align?: "start" | "center" | "end";
  startIndex?: number;
}

export interface CarouselApi {
  scrollPrev(): void;
  scrollNext(): void;
  scrollTo(index: number): void;
  selectedScrollSnap(): number;
}

/** Nom de l'API dans la version Starwind (Embla). */
export type CarouselManager = CarouselApi;

type DesignKit = { carousel?: (el: HTMLElement) => CarouselApi };

export function initCarousel(el: HTMLElement): CarouselApi | undefined {
  const kit = (window as unknown as { DesignKit?: DesignKit }).DesignKit;
  return kit?.carousel?.(el);
}
'''
F[C+'CarouselContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div">;

const {{ class: className, ...rest }} = Astro.props;
---

<div {{...rest}} data-slot="carousel-content">
  <div class={{className}} data-slot="carousel-container">
    <slot />
  </div>
</div>
'''
F[C+'CarouselItem.astro'] = simple('div', 'carousel-item', ' role="group" aria-roledescription="slide"')
for name, slot, label, icon in (('CarouselNext', 'carousel-next', 'Next slide', 'right'), ('CarouselPrevious', 'carousel-previous', 'Previous slide', 'left')):
    F[C+name+'.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button"> & {{
  variant?: "default" | "primary" | "secondary" | "outline" | "ghost";
  size?: "sm" | "md" | "lg" | "icon-sm" | "icon" | "icon-lg";
}};

const {{ variant = "outline", size = "icon", class: className, ...rest }} = Astro.props;
---

<button type="button" class={{className}} data-variant={{variant}} data-size={{size}} aria-label="{label}" {{...rest}} data-slot="{slot}">
  <slot name="icon">{ICON[icon]}</slot>
  <slot><span class="sr-only">{label}</span></slot>
</button>
'''

D = 'collapsible/'
F[D+'Collapsible.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  defaultOpen?: boolean;
  disabled?: boolean;
}};

const {{ defaultOpen = false, disabled = false, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-state={{defaultOpen ? "open" : "closed"}}
  data-disabled={{disabled ? "" : undefined}}
  {{...rest}}
  data-slot="collapsible"
>
  <slot />
</div>
'''
F[D+'CollapsibleTrigger.astro'] = trigger('collapsible-trigger', extra=' aria-expanded="false"')
F[D+'CollapsibleContent.astro'] = simple('div', 'collapsible-content', ' data-state="closed" hidden')

F['container/Container.astro'] = '''---
import type { HTMLAttributes, HTMLTag } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  as?: HTMLTag;
  size?: "xs" | "sm" | "md" | "lg" | "xl" | "prose" | "full";
  gutter?: "none" | "sm" | "md" | "lg" | "xl";
  paddingY?: "none" | "sm" | "md" | "lg" | "xl";
  background?: "none" | "background" | "card" | "muted" | "accent" | "primary" | "secondary" | "dark";
  border?: "none" | "all" | "top" | "bottom" | "x" | "y";
  rounded?: "none" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl";
  minHeight?: "none" | "screen" | "half";
  layout?: "stack" | "centered" | "split";
  /** Fond pleine largeur, contenu centré. */
  bleed?: boolean;
  centered?: boolean;
};

const {
  as: Tag = "div",
  bleed = false,
  size = "lg",
  gutter = "md",
  paddingY = "none",
  background = "none",
  border = "none",
  rounded = "none",
  minHeight = "none",
  layout: layoutProp,
  centered,
  class: className,
  ...rest
} = Astro.props;

const layout = centered ? "centered" : (layoutProp ?? "stack");
const useWrapper = bleed || layout === "split";
---

{
  useWrapper ? (
    <Tag
      class={className}
      data-background={background}
      data-padding-y={paddingY}
      data-min-height={minHeight}
      data-border={border}
      data-rounded={rounded}
      data-bleed
      data-slot="container"
      {...rest}
    >
      <div data-container-inner data-size={size} data-gutter={gutter} data-layout={layout}>
        <div data-container-main>
          <slot />
        </div>
        <div data-container-media>
          <slot name="media" />
        </div>
      </div>
    </Tag>
  ) : (
    <Tag
      class={className}
      data-size={size}
      data-gutter={gutter}
      data-padding-y={paddingY}
      data-background={background}
      data-border={border}
      data-rounded={rounded}
      data-min-height={minHeight}
      data-layout={layout}
      data-slot="container"
      {...rest}
    >
      <slot />
    </Tag>
  )
}
'''

E = 'dropdown/'
F[E+'Dropdown.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  /** Ouvre le menu au survol (en plus du clic). */
  openOnHover?: boolean;
  /** Délai de fermeture après le survol (ms). */
  closeDelay?: number;
}};

const {{ openOnHover = false, closeDelay = 200, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-open-on-hover={{openOnHover ? "true" : undefined}}
  data-close-delay={{closeDelay}}
  {{...rest}}
  data-slot="dropdown"
>
  <slot />
</div>
'''
F[E+'DropdownTrigger.astro'] = f'''---
{HA}
type Props = Omit<HTMLAttributes<"button">, "role" | "type"> & {{
  /** Habille un élément enfant (ex. <Button>) au lieu de rendre son propre <button>. */
  asChild?: boolean;
}};

const {{ asChild = false, class: className, ...rest }} = Astro.props;
---

{{
  asChild ? (
    <div class={{className}} data-slot="dropdown-trigger" data-as-child aria-haspopup="true" aria-expanded="false">
      <slot />
    </div>
  ) : (
    <button
      class={{className}}
      type="button"
      aria-haspopup="true"
      aria-expanded="false"
      data-state="closed"
      {{...rest}}
      data-slot="dropdown-trigger"
    >
      <slot />
    </button>
  )
}}
'''
F[E+'DropdownContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  side?: "top" | "bottom" | "left" | "right";
  align?: "start" | "center" | "end";
  sideOffset?: number;
  animationDuration?: number;
}};

const {{
  side = "bottom",
  align = "start",
  sideOffset = 4,
  animationDuration = 150,
  class: className,
  ...rest
}} = Astro.props;
---

<div
  class={{className}}
  role="menu"
  data-side={{side}}
  data-align={{align}}
  data-state="closed"
  tabindex="-1"
  aria-orientation="vertical"
  hidden
  style={{{{ "--ds-offset": `${{sideOffset}}px`, "--ds-duration": `${{animationDuration}}ms` }}}}
  {{...rest}}
  data-slot="dropdown-content"
>
  <slot />
</div>
'''
F[E+'DropdownItem.astro'] = '''---
import type { HTMLAttributes, HTMLTag } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  /** Élément rendu (ex. "a" avec href). */
  as?: HTMLTag;
  href?: string;
  inset?: boolean;
  disabled?: boolean;
};

const { inset = false, disabled = false, as: Tag = "div", class: className, ...rest } = Astro.props;
---

<Tag
  class={className}
  role="menuitem"
  tabindex={disabled ? "-1" : "0"}
  aria-disabled={disabled ? "true" : undefined}
  data-disabled={disabled ? "true" : undefined}
  data-inset={inset ? "" : undefined}
  data-slot="dropdown-item"
  {...rest}
>
  <slot />
</Tag>
'''
F[E+'DropdownLabel.astro'] = simple('div', 'dropdown-label', ' data-inset={inset ? "" : undefined}', '  inset?: boolean;\n', 'inset = false')
F[E+'DropdownSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div">;

const {{ class: className, ...rest }} = Astro.props;
---

<div class={{className}} role="separator" aria-orientation="horizontal" {{...rest}} data-slot="dropdown-separator"></div>
'''
F[E+'DropdownShortcut.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"span">;

const {{ class: className, ...rest }} = Astro.props;
---

<span class={{className}} data-dropdown-shortcut {{...rest}}>
  <slot />
</span>
'''
F[E+'DropdownSub.astro'] = simple('div', 'dropdown-sub')
F[E+'DropdownSubTrigger.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  inset?: boolean;
  disabled?: boolean;
}};

const {{ inset = false, disabled = false, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  role="menuitem"
  aria-haspopup="true"
  aria-expanded="false"
  tabindex={{disabled ? "-1" : "0"}}
  data-disabled={{disabled ? "true" : undefined}}
  data-inset={{inset ? "" : undefined}}
  data-sub-trigger
  {{...rest}}
  data-slot="dropdown-item"
>
  <slot />
  {ICON['chev-right']}
</div>
'''
F[E+'DropdownSubContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  animationDuration?: number;
}};

const {{ animationDuration = 150, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-sub-content
  role="menu"
  data-side="right"
  data-align="start"
  data-state="closed"
  tabindex="-1"
  aria-orientation="vertical"
  hidden
  style={{{{ "--ds-duration": `${{animationDuration}}ms` }}}}
  {{...rest}}
  data-slot="dropdown-content"
>
  <slot />
</div>
'''
