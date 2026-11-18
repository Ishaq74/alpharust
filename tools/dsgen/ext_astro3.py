N = 'sheet/'
F[N+'Sheet.astro'] = '''---
import type { ComponentProps } from "astro/types";
import Dialog from "../dialog/Dialog.astro";

type Props = ComponentProps<typeof Dialog>;

const { class: className, ...rest } = Astro.props;
---

<Dialog class={className} {...rest} data-slot="sheet">
  <slot />
</Dialog>
'''
F[N+'SheetTrigger.astro'] = '''---
import type { ComponentProps } from "astro/types";
import DialogTrigger from "../dialog/DialogTrigger.astro";

type Props = ComponentProps<typeof DialogTrigger>;

const { class: className, ...rest } = Astro.props;
---

<DialogTrigger class={className} {...rest}>
  <slot />
</DialogTrigger>
'''
F[N+'SheetClose.astro'] = '''---
import type { ComponentProps } from "astro/types";
import DialogClose from "../dialog/DialogClose.astro";

type Props = ComponentProps<typeof DialogClose>;

const { class: className, ...rest } = Astro.props;
---

<DialogClose class={className} {...rest}>
  <slot />
</DialogClose>
'''
F[N+'SheetContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"dialog"> & {{
  side?: "top" | "right" | "bottom" | "left";
}};

const {{ side = "right", class: className, ...rest }} = Astro.props;
---

<!-- Fond : ::backdrop natif ; l'élément reste pour la parité du balisage. -->
<slot name="backdrop">
  <div data-state="closed" data-slot="dialog-backdrop" aria-hidden="true"></div>
</slot>
<dialog class={{className}} data-side={{side}} data-state="closed" data-close-duration="300" {{...rest}} data-slot="sheet-content">
  <slot />
  <button type="button" data-slot="sheet-close" aria-label="Close dialog">
    <slot name="icon">{ICON['x']}</slot>
  </button>
</dialog>
'''
F[N+'SheetHeader.astro'] = simple('div', 'sheet-header')
F[N+'SheetFooter.astro'] = simple('div', 'sheet-footer')
F[N+'SheetTitle.astro'] = simple('h2', 'sheet-title')
F[N+'SheetDescription.astro'] = simple('p', 'sheet-description')

S = 'sidebar/'
F[S+'SidebarProvider.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  defaultOpen?: boolean;
  /** Raccourci clavier (Ctrl/Cmd + touche) pour replier la barre. */
  keyboardShortcut?: string;
}};

const {{ defaultOpen = true, keyboardShortcut = "b", class: className, style, ...rest }} = Astro.props;

const styleObj = typeof style === "object" && style ? (style as Record<string, string>) : {{}};
const mergedStyle = {{
  "--sidebar-width": styleObj["--sidebar-width"] ?? "18rem",
  "--sidebar-width-icon": styleObj["--sidebar-width-icon"] ?? "3.5rem",
  ...styleObj,
}};
---

<div
  class={{className}}
  data-state={{defaultOpen ? "expanded" : "collapsed"}}
  data-mobile-open="false"
  data-keyboard-shortcut={{keyboardShortcut}}
  style={{mergedStyle}}
  {{...rest}}
  data-slot="sidebar-provider"
>
  <slot />
</div>
'''
F[S+'Sidebar.astro'] = f'''---
{HA}
import Sheet from "../sheet/Sheet.astro";
import SheetContent from "../sheet/SheetContent.astro";
import SheetDescription from "../sheet/SheetDescription.astro";
import SheetHeader from "../sheet/SheetHeader.astro";
import SheetTitle from "../sheet/SheetTitle.astro";

type Props = HTMLAttributes<"div"> & {{
  side?: "left" | "right";
  variant?: "sidebar" | "floating" | "inset";
  collapsible?: "offcanvas" | "icon" | "none";
}};

const {{ side = "left", variant = "sidebar", collapsible = "offcanvas", class: className, ...rest }} = Astro.props;
---

{{
  collapsible === "none" ? (
    <div class={{className}} data-collapsible-mode="none" {{...rest}} data-slot="sidebar">
      <slot />
    </div>
  ) : (
    <>
      <div
        class={{className}}
        data-state="expanded"
        data-collapsible=""
        data-collapsible-mode={{collapsible}}
        data-variant={{variant}}
        data-side={{side}}
        data-slot="sidebar"
      >
        <div data-slot="sidebar-gap"></div>
        <div {{...rest}} data-slot="sidebar-container">
          <div data-slot="sidebar-inner">
            <slot />
          </div>
        </div>
      </div>
      <Sheet data-slot="sidebar-mobile" data-sidebar-mobile>
        <SheetContent side={{side}} data-sidebar="mobile">
          <SheetHeader class="sr-only">
            <SheetTitle>Sidebar</SheetTitle>
            <SheetDescription>Mobile navigation sidebar</SheetDescription>
          </SheetHeader>
          <div data-sidebar-mobile-inner>
            <slot />
          </div>
        </SheetContent>
      </Sheet>
    </>
  )
}}
'''
for n, s, tag, sb in (('SidebarContent','sidebar-content','div','content'),('SidebarFooter','sidebar-footer','div','footer'),
                      ('SidebarGroup','sidebar-group','div','group'),('SidebarGroupContent','sidebar-group-content','div','group-content'),
                      ('SidebarHeader','sidebar-header','div','header'),('SidebarMenu','sidebar-menu','ul','menu'),
                      ('SidebarMenuBadge','sidebar-menu-badge','div','menu-badge'),('SidebarMenuItem','sidebar-menu-item','li','menu-item'),
                      ('SidebarMenuSub','sidebar-menu-sub','ul','menu-sub'),('SidebarMenuSubItem','sidebar-menu-sub-item','li','menu-sub-item')):
    F[S+n+'.astro'] = simple(tag, s, f' data-sidebar="{sb}"')
F[S+'SidebarGroupLabel.astro'] = simple('div', 'sidebar-group-label', ' data-sidebar="group-label" data-as-child={asChild ? "" : undefined}', '  asChild?: boolean;\n', 'asChild = false')
F[S+'SidebarInset.astro'] = simple('main', 'sidebar-inset', ' id="main"')
F[S+'SidebarInput.astro'] = '''---
import type { ComponentProps } from "astro/types";
import Input from "../input/Input.astro";

type Props = ComponentProps<typeof Input>;

const { size = "md", class: className, ...rest } = Astro.props;
---

<Input size={size} class={className} data-sidebar="input" {...rest} data-slot="sidebar-input" />
'''
F[S+'SidebarSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  orientation?: "horizontal" | "vertical";
}};

const {{ orientation = "horizontal", class: className, ...rest }} = Astro.props;
---

<div class={{className}} role="none" data-orientation={{orientation}} data-sidebar="separator" {{...rest}} data-slot="sidebar-separator"></div>
'''
F[S+'SidebarMenuAction.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button"> & {{
  showOnHover?: boolean;
  variant?: "default" | "primary" | "secondary" | "outline" | "ghost";
  size?: "sm" | "md" | "lg" | "icon-sm" | "icon" | "icon-lg";
}};

const {{ showOnHover = false, variant = "ghost", size = "icon-sm", class: className, ...rest }} = Astro.props;
---

<button type="button" class={{className}} data-variant={{variant}} data-size={{size}} data-show-on-hover={{showOnHover ? "" : undefined}} data-sidebar="menu-action" {{...rest}} data-slot="sidebar-menu-action">
  <slot />
</button>
'''
F[S+'SidebarMenuButton.astro'] = f'''---
{HA}
import Tooltip from "../tooltip/Tooltip.astro";
import TooltipContent from "../tooltip/TooltipContent.astro";
import TooltipTrigger from "../tooltip/TooltipTrigger.astro";

type Props = HTMLAttributes<"button"> &
  Omit<HTMLAttributes<"a">, "type"> & {{
    asChild?: boolean;
    isActive?: boolean;
    /** Libellé affiché en info-bulle quand la barre est repliée en icônes. */
    tooltip?: string;
    variant?: "default" | "outline";
    size?: "default" | "sm" | "lg";
  }};

const {{ asChild = false, isActive = false, tooltip, variant = "default", size, class: className, ...rest }} = Astro.props;

const Tag = rest.href ? "a" : "button";
const buttonDataAttrs = {{
  "data-slot": "sidebar-menu-button",
  "data-sidebar": "menu-button",
  "data-size": size || "default",
  "data-variant": variant,
  "data-active": isActive,
  "aria-current": isActive && rest.href ? ("page" as const) : undefined,
}};
---

{{
  tooltip ? (
    <Tooltip data-sidebar-tooltip>
      <TooltipTrigger>
        {{asChild ? (
          <div class={{className}} {{...buttonDataAttrs}} data-as-child>
            <slot />
          </div>
        ) : (
          <Tag class={{className}} type={{Tag === "button" ? "button" : undefined}} {{...buttonDataAttrs}} {{...rest}}>
            <slot />
          </Tag>
        )}}
      </TooltipTrigger>
      <TooltipContent side="right">{{tooltip}}</TooltipContent>
    </Tooltip>
  ) : asChild ? (
    <div class={{className}} {{...buttonDataAttrs}} data-as-child>
      <slot />
    </div>
  ) : (
    <Tag class={{className}} type={{Tag === "button" ? "button" : undefined}} {{...buttonDataAttrs}} {{...rest}}>
      <slot />
    </Tag>
  )
}}
'''
F[S+'SidebarMenuSkeleton.astro'] = f'''---
{HA}
import Skeleton from "../skeleton/Skeleton.astro";

type Props = HTMLAttributes<"div"> & {{
  showIcon?: boolean;
  /** Largeur du texte simulé (défaut : aléatoire entre 50 et 90 %). */
  width?: string;
}};

const {{ showIcon = false, width, class: className, ...rest }} = Astro.props;
const skeletonWidth = width ?? `${{Math.floor(Math.random() * 40) + 50}}%`;
---

<div class={{className}} data-sidebar="menu-skeleton" {{...rest}} data-slot="sidebar-menu-skeleton">
  {{showIcon && <Skeleton data-sidebar="menu-skeleton-icon" />}}
  {{/* alpharust-ignore: no-inline-styles — largeur aléatoire par instance, comme l'atome d'atomic */}}
  <Skeleton data-sidebar="menu-skeleton-text" style={{`--skeleton-width: ${{skeletonWidth}}`}} />
</div>
'''
F[S+'SidebarMenuSubButton.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"a"> & {{
  size?: "sm" | "md";
  isActive?: boolean;
}};

const {{ size, isActive = false, class: className, ...rest }} = Astro.props;
---

<a
  class={{className}}
  data-sidebar="menu-sub-button"
  data-size={{size || "md"}}
  data-active={{isActive}}
  aria-current={{isActive ? "page" : undefined}}
  {{...rest}}
  data-slot="sidebar-menu-sub-button"
>
  <slot />
</a>
'''
F[S+'SidebarRail.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button">;

const {{ class: className, ...rest }} = Astro.props;
---

<button type="button" class={{className}} data-sidebar="rail" aria-label="Toggle Sidebar" tabindex="-1" {{...rest}} data-slot="sidebar-rail"></button>
'''
F[S+'SidebarTrigger.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button"> & {{
  variant?: "default" | "primary" | "secondary" | "outline" | "ghost";
  size?: "sm" | "md" | "lg" | "icon-sm" | "icon" | "icon-lg";
}};

const {{ size = "icon-sm", variant = "ghost", class: className, ...rest }} = Astro.props;
---

<button type="button" class={{className}} data-variant={{variant}} data-size={{size}} data-sidebar="trigger" aria-label="Toggle Sidebar" aria-expanded="true" {{...rest}} data-slot="sidebar-trigger">
  <slot name="icon">{ICON['panel']}</slot>
  <span class="sr-only">Toggle Sidebar</span>
</button>
'''

F['slider/Slider.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  defaultValue?: number | number[];
  value?: number | number[];
  min?: number;
  max?: number;
  step?: number;
  /** Pas avec Page précédente / Page suivante et Maj + flèches. */
  largeStep?: number;
  orientation?: "horizontal" | "vertical";
  disabled?: boolean;
  name?: string;
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
  "aria-label"?: string;
  "aria-labelledby"?: string;
}};

const {{
  defaultValue = 0,
  value,
  min = 0,
  max = 100,
  step = 1,
  largeStep = 10,
  orientation = "horizontal",
  disabled = false,
  name,
  variant = "default",
  "aria-label": ariaLabel,
  "aria-labelledby": ariaLabelledby,
  class: className,
  ...rest
}} = Astro.props;

const initial = value ?? defaultValue;
const values = (Array.isArray(initial) ? initial : [initial]).map((v) => Math.min(max, Math.max(min, v)));
const isRange = values.length > 1;
const pct = (v: number) => ((v - min) / (max - min || 1)) * 100;
const rangeStart = isRange ? pct(Math.min(...values)) : 0;
const rangeEnd = pct(Math.max(...values));
---

<div
  class={{className}}
  data-orientation={{orientation}}
  data-disabled={{disabled ? "" : undefined}}
  data-variant={{variant}}
  role="group"
  aria-labelledby={{ariaLabelledby}}
  data-min={{min}}
  data-max={{max}}
  data-step={{step}}
  data-large-step={{largeStep}}
  {{...rest}}
  data-slot="slider"
>
  <div data-slot="slider-control" data-orientation={{orientation}}>
    <div data-slot="slider-track" data-orientation={{orientation}}>
      <div
        data-slot="slider-range"
        data-orientation={{orientation}}
        style={{orientation === "horizontal" ? `left: ${{rangeStart}}%; width: ${{rangeEnd - rangeStart}}%` : `bottom: ${{rangeStart}}%; height: ${{rangeEnd - rangeStart}}%`}}
      ></div>
    </div>
    {{
      values.map((val, index) => (
        <div
          data-slot="slider-thumb"
          data-index={{index}}
          data-orientation={{orientation}}
          style={{orientation === "horizontal" ? `left: ${{pct(val)}}%` : `bottom: ${{pct(val)}}%`}}
          tabindex={{disabled ? -1 : 0}}
          role="slider"
          aria-label={{ariaLabel}}
          aria-labelledby={{ariaLabelledby}}
          aria-valuemin={{min}}
          aria-valuemax={{max}}
          aria-valuenow={{val}}
          aria-orientation={{orientation}}
          aria-disabled={{disabled}}
        >
          <input
            type="range"
            min={{min}}
            max={{max}}
            step={{step}}
            value={{val}}
            name={{name ? (isRange ? `${{name}}[${{index}}]` : name) : undefined}}
            disabled={{disabled}}
            tabindex={{-1}}
            aria-hidden="true"
            class="sr-only"
          />
        </div>
      ))
    }}
  </div>
</div>
'''

F['toggle/Toggle.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"button"> & {{
  variant?: "default" | "outline";
  size?: "sm" | "md" | "lg";
  defaultPressed?: boolean;
  /** Les bascules d'un même groupe restent synchronisées. */
  syncGroup?: string;
}};

const {{ defaultPressed = false, syncGroup, variant = "default", size = "md", class: className, ...rest }} = Astro.props;
---

<button
  type="button"
  class={{className}}
  data-state={{defaultPressed ? "on" : "off"}}
  data-sync-group={{syncGroup}}
  data-variant={{variant}}
  data-size={{size}}
  aria-pressed={{defaultPressed ? "true" : "false"}}
  {{...rest}}
  data-slot="toggle"
>
  <slot />
</button>
'''
F['toggle/ToggleTypes.ts'] = None  # copié tel quel d'atomic

F['theme-toggle/ThemeToggle.astro'] = f'''---
import Toggle from "../toggle/Toggle.astro";

type Props = {{
  variant?: "default" | "outline";
  size?: "sm" | "md" | "lg";
  ariaLabel?: string;
  class?: string;
}};

const {{ variant = "outline", size = "md", ariaLabel = "Toggle theme", class: className, ...rest }} = Astro.props;
---

<Toggle class={{className}} variant={{variant}} size={{size}} aria-label={{ariaLabel}} {{...rest}} data-slot="theme-toggle">
  <slot>
    <span data-theme-icon-wrapper>
      <slot name="light-icon">{ICON['sun']}</slot>
      <slot name="dark-icon">{ICON['moon']}</slot>
    </span>
  </slot>
</Toggle>
'''

T = 'toast/'
F[T+'toast-manager.ts'] = None  # copié tel quel d'atomic (API window.__starwind__.toast, fournie ici par ds.js)
F[T+'Toaster.astro'] = f'''---
{HA}
import ToastTemplate from "./ToastTemplate.astro";

type Position = "top-left" | "top-center" | "top-right" | "bottom-left" | "bottom-center" | "bottom-right";

type Props = HTMLAttributes<"div"> & {{
  position?: Position;
  /** Nombre de notifications visibles. */
  limit?: number;
  gap?: string;
  peek?: string;
  /** Durée d'affichage par défaut (ms). */
  duration?: number;
}};

const {{ position = "bottom-right", gap = "0.5rem", peek = "1rem", limit = 3, duration = 5000, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-position={{position}}
  data-limit={{limit}}
  data-duration={{duration}}
  role="region"
  aria-live="polite"
  aria-atomic="false"
  aria-relevant="additions text"
  aria-label="Notifications"
  tabindex={{-1}}
  style={{`--gap: ${{gap}}; --peek: ${{peek}};`}}
  {{...rest}}
  data-slot="toast-viewport"
>
  <slot>
    <ToastTemplate variant="default" />
    <ToastTemplate variant="success" />
    <ToastTemplate variant="error" />
    <ToastTemplate variant="warning" />
    <ToastTemplate variant="info" />
    <ToastTemplate variant="loading" />
  </slot>
</div>
'''
F[T+'ToastTemplate.astro'] = '''---
import type { Variant } from "./toast-manager";
import ToastDescription from "./ToastDescription.astro";
import ToastItem from "./ToastItem.astro";
import ToastTitle from "./ToastTitle.astro";

interface Props {
  variant: Variant;
}

const { variant } = Astro.props;
const itemVariant = variant === "loading" ? "default" : variant;
---

<template data-toast-template={variant}>
  <slot>
    <ToastItem variant={itemVariant}>
      <ToastTitle variant={variant}>Title</ToastTitle>
      <ToastDescription>Description</ToastDescription>
    </ToastItem>
  </slot>
</template>
'''
F[T+'ToastItem.astro'] = f'''---
{HA}
import type {{ Variant }} from "./toast-manager";

type Props = HTMLAttributes<"div"> & {{
  variant?: Exclude<Variant, "loading">;
}};

const {{ variant = "default", class: className, ...rest }} = Astro.props;
const urgent = variant === "error" || variant === "warning";
---

<div
  class={{className}}
  data-variant={{variant}}
  data-state="open"
  role={{urgent ? "alert" : "status"}}
  aria-live={{urgent ? "assertive" : "polite"}}
  {{...rest}}
  data-slot="toast"
>
  <div data-slot="toast-content">
    <slot />
  </div>
  <button type="button" data-slot="toast-close" aria-label="Close notification">{ICON['x']}</button>
</div>
'''
F[T+'ToastTitle.astro'] = f'''---
{HA}
import type {{ Variant }} from "./toast-manager";

type Props = HTMLAttributes<"div"> & {{
  variant?: Variant;
}};

const {{ variant = "default", class: className, ...rest }} = Astro.props;
---

<div class={{className}} data-variant={{variant}} data-slot="toast-title" data-toast-title {{...rest}}>
  <slot name="icon">
    {{variant === "success" && <Fragment set:html={{`{ICON['success']}`}} />}}
    {{variant === "error" && <Fragment set:html={{`{ICON['error']}`}} />}}
    {{variant === "warning" && <Fragment set:html={{`{ICON['warning']}`}} />}}
    {{variant === "info" && <Fragment set:html={{`{ICON['info']}`}} />}}
    {{variant === "loading" && <Fragment set:html={{`{ICON['loader'].replace('2.5rem', '1.1em').replace('2.5rem', '1.1em')}`}} />}}
  </slot>
  <span data-toast-title-text><slot /></span>
</div>
'''
F[T+'ToastDescription.astro'] = simple('div', 'toast-description', ' data-toast-description')

F['video/Video.astro'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"video"> &
  HTMLAttributes<"iframe"> & {
    src: string;
    title?: string;
    autoplay?: boolean;
    muted?: boolean;
    loop?: boolean;
    controls?: boolean;
    poster?: string;
  };

const {
  class: className,
  src,
  title = "Video",
  autoplay = false,
  muted = false,
  loop = false,
  controls = true,
  poster,
  ...rest
} = Astro.props;

// Même détection que le composant Starwind d'atomic : YouTube (y compris Shorts) en iframe sans cookies, sinon <video>.
function getVideoType(url: string): "youtube" | "youtube-shorts" | "native" {
  if (url.includes("youtube.com/shorts/") || url.includes("youtu.be/shorts/")) return "youtube-shorts";
  if (url.includes("youtube.com") || url.includes("youtu.be") || url.includes("youtube-nocookie.com")) return "youtube";
  return "native";
}

function getYouTubeId(url: string): string | null {
  const patterns = [
    /youtube\\.com\\/shorts\\/([^?&]+)/,
    /youtube\\.com\\/watch\\?v=([^&]+)/,
    /youtube\\.com\\/embed\\/([^?&]+)/,
    /youtu\\.be\\/([^?&]+)/,
    /youtube-nocookie\\.com\\/embed\\/([^?&]+)/,
  ];
  for (const pattern of patterns) {
    const match = url.match(pattern);
    if (match) return match[1];
  }
  return null;
}

function buildYouTubeEmbedUrl(videoId: string): string {
  const params = new URLSearchParams();
  if (autoplay) params.set("autoplay", "1");
  if (muted) params.set("mute", "1");
  if (loop) {
    params.set("loop", "1");
    params.set("playlist", videoId);
  }
  if (!controls) params.set("controls", "0");
  const query = params.toString();
  return `https://www.youtube-nocookie.com/embed/${videoId}${query ? `?${query}` : ""}`;
}

const videoType = getVideoType(src);
const youtubeId = videoType !== "native" ? getYouTubeId(src) : null;
const isShort = videoType === "youtube-shorts";
const embedUrl = youtubeId ? buildYouTubeEmbedUrl(youtubeId) : null;
---

{
  videoType === "native" || !embedUrl ? (
    <video class={className} src={src} autoplay={autoplay} muted={muted} loop={loop} controls={controls} poster={poster} {...rest} data-slot="video">
      <track kind="captions" />
    </video>
  ) : (
    <iframe
      class={className}
      src={embedUrl}
      title={title}
      allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      referrerpolicy="strict-origin-when-cross-origin"
      allowfullscreen
      data-video-type={isShort ? "youtube-shorts" : "youtube"}
      {...rest}
      data-slot="video"
    />
  )
}
'''
for t in ('select/SelectTypes.ts', 'radio-group/RadioGroupTypes.ts', 'input-otp/InputOtpTypes.ts'):
    F[t] = None
