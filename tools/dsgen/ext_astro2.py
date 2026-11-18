G = 'dropzone/'
F[G+'Dropzone.astro'] = f'''---
{HA}
import DropzoneFilesList from "./DropzoneFilesList.astro";
import DropzoneLoadingIndicator from "./DropzoneLoadingIndicator.astro";
import DropzoneUploadIndicator from "./DropzoneUploadIndicator.astro";

type Props = HTMLAttributes<"input"> & {{
  /** Affiche l'indicateur de chargement. */
  isUploading?: boolean;
}};

const {{ isUploading = false, id, class: className, ...rest }} = Astro.props;
---

<label
  id={{id}}
  tabindex="0"
  class={{className}}
  data-slot="dropzone"
  data-drag-active="false"
  data-has-files="false"
  data-is-uploading={{String(isUploading)}}
>
  <slot>
    <DropzoneUploadIndicator />
    <DropzoneLoadingIndicator />
    <DropzoneFilesList />
  </slot>
  <input tabindex="-1" type="file" class="sr-only" {{...rest}} />
</label>
'''
F[G+'DropzoneFilesList.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div">;

const {{ class: className, ...rest }} = Astro.props;
---

<div class={{className}} aria-live="polite" aria-label="Uploaded files" {{...rest}} data-slot="dropzone-files-list"></div>
'''
F[G+'DropzoneLoadingIndicator.astro'] = f'''---
---

<div hidden data-slot="dropzone-loading-indicator">
  <slot>
    {ICON['loader']}
    <p>Uploading file(s)...</p>
  </slot>
</div>
'''
F[G+'DropzoneUploadIndicator.astro'] = f'''---
---

<div data-slot="dropzone-upload-indicator">
  <slot>
    {ICON['upload']}
    <p>Click to upload or drag and drop</p>
  </slot>
</div>
'''

F['image/Image.astro'] = '''---
import type { ComponentProps } from "astro/types";
import { Image as AstroImage } from "astro:assets";

type Props = Partial<ComponentProps<typeof AstroImage>> & { inferSize?: boolean };

const { class: className, src, alt = "", inferSize = true, ...rest } = Astro.props;
---

{
  src && (
    <AstroImage
      class={className}
      src={src}
      alt={alt}
      inferSize={inferSize}
      {...(rest as any)}
      data-slot="image"
    />
  )
}
'''

H = 'input-group/'
F[H+'InputGroup.astro'] = simple('div', 'input-group', ' role="group"')
F[H+'InputGroupAddon.astro'] = simple('div', 'input-group-addon', ' role="group" data-align={align}',
    '  align?: "inline-start" | "inline-end" | "block-start" | "block-end";\n', 'align = "inline-start"')
F[H+'InputGroupButton.astro'] = f'''---
import type {{ ComponentProps }} from "astro/types";
import Button from "../button/Button.astro";

type Props = Omit<ComponentProps<typeof Button>, "size"> & {{
  size?: "sm" | "icon-sm";
}};

const {{ type = "button", variant = "ghost", size = "sm", class: className, ...rest }} = Astro.props;
---

<Button type={{type}} data-size={{size}} size={{size === "sm" ? "sm" : "icon-sm"}} variant={{variant}} class={{className}} data-input-group-button {{...rest}}>
  <slot />
</Button>
'''
F[H+'InputGroupInput.astro'] = '''---
import type { ComponentProps } from "astro/types";
import Input from "../input/Input.astro";

type Props = ComponentProps<typeof Input>;

const { class: className, ...rest } = Astro.props;
---

<Input class={className} {...rest} data-slot="input-group-control" />
'''
F[H+'InputGroupTextarea.astro'] = '''---
import type { ComponentProps } from "astro/types";
import Textarea from "../textarea/Textarea.astro";

type Props = ComponentProps<typeof Textarea>;

const { class: className, ...rest } = Astro.props;
---

<Textarea class={className} {...rest} data-slot="input-group-control" />
'''
F[H+'InputGroupText.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"span">;

const {{ class: className, ...rest }} = Astro.props;
---

<span class={{className}} data-input-group-text {{...rest}}>
  <slot />
</span>
'''

I = 'input-otp/'
F[I+'InputOtp.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  maxLength?: number;
  value?: string;
  defaultValue?: string;
  disabled?: boolean;
  /** Caractères admis (défaut : chiffres). */
  pattern?: RegExp | string;
  name?: string;
  id?: string;
  required?: boolean;
}};

const {{
  maxLength = 6,
  value,
  defaultValue,
  disabled = false,
  pattern,
  name,
  id,
  required,
  class: className,
  ...rest
}} = Astro.props;

const patternStr = pattern instanceof RegExp ? pattern.source : (pattern ?? "^\\\\d+$");
const inputMode = /\\\\d|0-9/.test(patternStr) && !/[a-zA-Z]-|\\\\w/.test(patternStr) ? "numeric" : "text";
---

<div
  class={{className}}
  data-max-length={{maxLength}}
  data-value={{value || defaultValue || ""}}
  data-disabled={{disabled}}
  data-pattern={{patternStr}}
  role="group"
  aria-label={{`OTP input, ${{maxLength}} digits`}}
  {{...rest}}
  data-slot="input-otp"
>
  <input
    autocomplete="one-time-code"
    inputmode={{inputMode}}
    name={{name}}
    id={{id}}
    class="sr-only"
    required={{required}}
    maxlength={{maxLength.toString()}}
    value={{value || defaultValue || ""}}
    disabled={{disabled}}
    aria-label={{`OTP, ${{maxLength}}`}}
    data-otp-hidden-input
  />
  <slot />
</div>
'''
F[I+'InputOtpGroup.astro'] = simple('div', 'input-otp-group')
F[I+'InputOtpSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div">;

const {{ class: className, ...rest }} = Astro.props;
---

<div aria-hidden="true" class={{className}} role="separator" {{...rest}} data-slot="input-otp-separator">
  <slot name="icon">{ICON['minus']}</slot>
</div>
'''
F[I+'InputOtpSlot.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  size?: "sm" | "md" | "lg";
  index?: number;
}};

const {{ size = "md", index, class: className, ...rest }} = Astro.props;
---

<div class={{className}} data-otp-slot data-index={{index}} data-size={{size}} aria-hidden="true" {{...rest}} data-slot="input-otp-slot">
  <span data-otp-char></span>
  <div data-otp-caret hidden>
    <slot name="caret"><div data-otp-caret-line></div></slot>
  </div>
</div>
'''

J = 'item/'
F[J+'Item.astro'] = f'''---
import type {{ HTMLAttributes, HTMLTag }} from "astro/types";

type Props = HTMLAttributes<"div"> & {{
  as?: HTMLTag;
  href?: string;
  variant?: "default" | "outline" | "muted";
  size?: "default" | "sm";
}};

const {{ variant = "default", size = "default", as: Tag = "div", class: className, ...rest }} = Astro.props;
---

<Tag class={{className}} data-variant={{variant}} data-size={{size}} data-slot="item" {{...rest}}>
  <slot />
</Tag>
'''
for n, s, tag, ex in (('ItemActions','item-actions','div',''),('ItemContent','item-content','div',''),('ItemDescription','item-description','p',''),
                      ('ItemFooter','item-footer','div',''),('ItemGroup','item-group','div',' role="list"'),('ItemHeader','item-header','div',''),('ItemTitle','item-title','div','')):
    F[J+n+'.astro'] = simple(tag, s, ex)
F[J+'ItemMedia.astro'] = simple('div', 'item-media', ' data-variant={variant}', '  variant?: "default" | "icon" | "image";\n', 'variant = "default"')
F[J+'ItemSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  orientation?: "horizontal" | "vertical";
}};

const {{ orientation = "horizontal", class: className, ...rest }} = Astro.props;
---

<div class={{className}} role="none" data-orientation={{orientation}} {{...rest}} data-slot="item-separator"></div>
'''

K = 'popover/'
F[K+'Popover.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  openOnHover?: boolean;
  closeDelay?: number;
  defaultOpen?: boolean;
}};

const {{ openOnHover = false, closeDelay = 200, defaultOpen = false, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-open-on-hover={{openOnHover ? "true" : undefined}}
  data-close-delay={{closeDelay}}
  data-state={{defaultOpen ? "open" : "closed"}}
  data-default-open={{defaultOpen ? "true" : undefined}}
  {{...rest}}
  data-slot="popover"
>
  <slot />
</div>
'''
F[K+'PopoverTrigger.astro'] = f'''---
{HA}
type Props = Omit<HTMLAttributes<"button">, "role" | "type"> & {{
  asChild?: boolean;
}};

const {{ asChild = false, class: className, ...rest }} = Astro.props;
---

{{
  asChild ? (
    <div class={{className}} data-slot="popover-trigger" data-as-child>
      <slot />
    </div>
  ) : (
    <button
      class={{className}}
      type="button"
      aria-haspopup="dialog"
      aria-expanded="false"
      data-state="closed"
      {{...rest}}
      data-slot="popover-trigger"
    >
      <slot />
    </button>
  )
}}
'''
F[K+'PopoverContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  side?: "top" | "bottom" | "left" | "right";
  align?: "start" | "center" | "end";
  sideOffset?: number;
  animationDuration?: number;
}};

const {{
  side = "bottom",
  align = "center",
  sideOffset = 4,
  animationDuration = 150,
  "aria-label": ariaLabel,
  "aria-labelledby": ariaLabelledBy,
  class: className,
  ...rest
}} = Astro.props;
---

<div
  class={{className}}
  data-side={{side}}
  data-align={{align}}
  data-side-offset={{sideOffset}}
  data-state="closed"
  role="dialog"
  aria-label={{ariaLabel}}
  aria-labelledby={{ariaLabelledBy}}
  tabindex="-1"
  hidden
  style={{{{ "--ds-offset": `${{sideOffset}}px`, "--ds-duration": `${{animationDuration}}ms` }}}}
  {{...rest}}
  data-slot="popover-content"
>
  <slot />
</div>
'''
F[K+'PopoverHeader.astro'] = simple('div', 'popover-header')
F[K+'PopoverTitle.astro'] = simple('h3', 'popover-title')
F[K+'PopoverDescription.astro'] = simple('p', 'popover-description')

F['prose/Prose.astro'] = simple('div', 'prose', ' data-size={size}', '  size?: "sm" | "default" | "lg";\n', 'size = "default"')

L = 'radio-group/'
F[L+'RadioGroup.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"fieldset"> & {{
  value?: string;
  defaultValue?: string;
  legend?: string;
  disabled?: boolean;
  orientation?: "horizontal" | "vertical";
}};

const {{ value, defaultValue, legend, disabled = false, orientation = "vertical", class: className, ...rest }} = Astro.props;
---

<fieldset
  class={{className}}
  role="radiogroup"
  data-value={{value || defaultValue}}
  data-orientation={{orientation}}
  disabled={{disabled}}
  {{...rest}}
  data-slot="radio-group"
>
  {{legend && <legend class="sr-only">{{legend}}</legend>}}
  <slot />
</fieldset>
'''
F[L+'RadioGroupItem.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"input"> & {{
  value: string;
  disabled?: boolean;
  id?: string;
  name?: string;
  checked?: boolean;
  size?: "sm" | "md" | "lg";
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
}};

const {{ value, name, disabled = false, checked = false, id, size = "md", variant = "default", class: className, ...rest }} = Astro.props;
---

<div data-size={{size}} data-slot="radio-group-item-wrapper">
  <input type="radio" name={{name}} id={{id}} value={{value}} checked={{checked}} disabled={{disabled}} {{...rest}} data-slot="radio-group-item" />
  <span class={{className}} data-variant={{variant}} aria-hidden="true" data-slot="radio-group-item-control">
    <slot name="icon">{ICON['dot']}</slot>
  </span>
</div>
'''

M = 'select/'
F[M+'Select.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  name?: string;
  defaultValue?: string;
  required?: boolean;
}};

const {{ name, defaultValue, required, class: className, ...rest }} = Astro.props;
---

<div class={{className}} data-name={{name}} data-value={{defaultValue}} data-required={{required}} {{...rest}} data-slot="select">
  <slot />
  {{name && <input type="hidden" name={{name}} value={{defaultValue ?? ""}} required={{required}} data-select-input />}}
</div>
'''
F[M+'SelectTrigger.astro'] = f'''---
{HA}
type Props = Omit<HTMLAttributes<"button">, "role" | "type"> & {{
  size?: "sm" | "md" | "lg";
}};

const {{ size = "md", class: className, "aria-label": ariaLabel = "Select field", ...rest }} = Astro.props;
---

<button
  class={{className}}
  type="button"
  role="combobox"
  aria-haspopup="listbox"
  aria-label={{ariaLabel}}
  aria-expanded="false"
  data-state="closed"
  data-size={{size}}
  {{...rest}}
  data-slot="select-trigger"
>
  <slot />
  <slot name="icon">{ICON['chev-down']}</slot>
</button>
'''
F[M+'SelectValue.astro'] = simple('span', 'select-value', '', '  placeholder?: string;\n', 'placeholder = "select"').replace('<slot />', '{placeholder}').replace('class={className} {...rest}', 'class={className} data-placeholder={placeholder} {...rest}')
F[M+'SelectContent.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  side?: "top" | "bottom";
  align?: "start" | "center" | "end";
  sideOffset?: number;
  animationDuration?: number;
  size?: "sm" | "md" | "lg";
}};

const {{
  side = "bottom",
  align = "start",
  size = "md",
  sideOffset = 4,
  animationDuration = 150,
  class: className,
  ...rest
}} = Astro.props;
---

<div
  class={{className}}
  role="listbox"
  data-side={{side}}
  data-align={{align}}
  data-size={{size}}
  data-state="closed"
  tabindex="-1"
  hidden
  style={{{{ "--ds-offset": `${{sideOffset}}px`, "--ds-duration": `${{animationDuration}}ms` }}}}
  {{...rest}}
  data-slot="select-content"
>
  <div data-select-inner>
    <slot />
  </div>
</div>
'''
F[M+'SelectGroup.astro'] = '''---
type Props = { children?: any };
---

<slot />
'''
F[M+'SelectLabel.astro'] = simple('div', 'select-label')
F[M+'SelectItem.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div"> & {{
  value: string;
  disabled?: boolean;
}};

const {{ value, disabled, class: className, ...rest }} = Astro.props;
---

<div
  class={{className}}
  data-value={{value}}
  data-disabled={{disabled}}
  aria-disabled={{disabled ? "true" : undefined}}
  aria-selected="false"
  role="option"
  tabindex="-1"
  {{...rest}}
  data-slot="select-item"
>
  <span><slot /></span>
  <span data-select-check><slot name="icon">{ICON['check']}</slot></span>
</div>
'''
F[M+'SelectSearch.astro'] = f'''---
{HA}
type Props = Omit<HTMLAttributes<"input">, "type" | "autocomplete"> & {{
  emptyText?: string;
}};

const {{ placeholder = "Search...", emptyText = "No results found.", class: className, ...rest }} = Astro.props;
---

<div data-slot="select-search-wrapper">
  <slot name="icon">{ICON['search']}</slot>
  <input type="text" class={{className}} placeholder={{placeholder}} autocomplete="off" aria-label={{placeholder}} {{...rest}} data-slot="select-search" />
</div>
<div hidden data-slot="select-empty">
  <slot name="empty">{{emptyText}}</slot>
</div>
'''
F[M+'SelectSeparator.astro'] = f'''---
{HA}
type Props = HTMLAttributes<"div">;

const {{ class: className, ...rest }} = Astro.props;
---

<div class={{className}} role="separator" {{...rest}} data-slot="select-separator"></div>
'''
