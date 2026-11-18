# Spécification unique du catalogue : (composant, partie) -> balise, slot, attributs.
# Les trois cibles (Astro CSS, Blade, Django) sont générées à partir de ce fichier (outil de développement).
def P(comp, name, slot, tag, data=(), attrs=None, linkable=False, void=False, empty=False, text=None, props_note=""):
    return dict(comp=comp, name=name, slot=slot, tag=tag, data=list(data), attrs=attrs or {}, linkable=linkable,
                void=void, empty=empty, text=text)

V9 = ("variant", "default")
SZ = ("size", "md")

SIMPLE = [
    P("button", "Button", "button", "button", [V9, SZ], {"type": "button"}, linkable=True),
    P("badge", "Badge", "badge", "div", [V9, SZ], linkable=True),
    P("alert", "AlertTitle", "alert-title", "h5"),
    P("alert", "AlertDescription", "alert-description", "p"),
    P("card", "Card", "card", "div", [("size", "default")]),
    P("card", "CardHeader", "card-header", "div"),
    P("card", "CardTitle", "card-title", "div"),
    P("card", "CardDescription", "card-description", "div"),
    P("card", "CardAction", "card-action", "div"),
    P("card", "CardContent", "card-content", "div"),
    P("card", "CardFooter", "card-footer", "div"),
    P("input", "Input", "input", "input", [SZ], void=True),
    P("label", "Label", "label", "label", [SZ]),
    P("skeleton", "Skeleton", "skeleton", "div", empty=True),
    P("spinner", "Spinner", "spinner", "span", attrs={"role": "status", "aria-label": "Loading"}, empty=True),
    P("kbd", "Kbd", "kbd", "kbd"),
    P("kbd", "KbdGroup", "kbd-group", "kbd"),
    P("table", "TableHeader", "table-header", "thead"),
    P("table", "TableBody", "table-body", "tbody"),
    P("table", "TableFoot", "table-foot", "tfoot"),
    P("table", "TableRow", "table-row", "tr"),
    P("table", "TableHead", "table-head", "th", attrs={"scope": "col"}),
    P("table", "TableCell", "table-cell", "td"),
    P("table", "TableCaption", "table-caption", "caption"),
    P("breadcrumb", "Breadcrumb", "breadcrumb", "nav", attrs={"aria-label": "breadcrumb"}),
    P("breadcrumb", "BreadcrumbList", "breadcrumb-list", "ol"),
    P("breadcrumb", "BreadcrumbItem", "breadcrumb-item", "li"),
    P("breadcrumb", "BreadcrumbLink", "breadcrumb-link", "a"),
    P("breadcrumb", "BreadcrumbPage", "breadcrumb-page", "span", attrs={"aria-current": "page"}),
    P("breadcrumb", "BreadcrumbSeparator", "breadcrumb-separator", "li", attrs={"role": "presentation", "aria-hidden": "true"}, text="›"),
    P("breadcrumb", "BreadcrumbEllipsis", "breadcrumb-ellipsis", "span", attrs={"role": "presentation", "aria-hidden": "true"}, text="…"),
    P("pagination", "Pagination", "pagination", "nav", attrs={"aria-label": "pagination"}),
    P("pagination", "PaginationContent", "pagination-content", "ul"),
    P("pagination", "PaginationItem", "pagination-item", "li"),
    P("pagination", "PaginationEllipsis", "pagination-ellipsis", "span", attrs={"aria-hidden": "true"}, text="…"),
    P("pagination", "PaginationPrevious", "pagination-previous", "a", attrs={"aria-label": "Go to previous page"}, text="‹ Previous"),
    P("pagination", "PaginationNext", "pagination-next", "a", attrs={"aria-label": "Go to next page"}, text="Next ›"),
    P("avatar", "Avatar", "avatar", "figure", [SZ]),
    P("avatar", "AvatarFallback", "avatar-fallback", "div"),
    P("accordion", "AccordionTrigger", "accordion-trigger", "summary"),
    P("accordion", "AccordionContent", "accordion-content", "div"),
    P("dialog", "DialogHeader", "dialog-header", "div"),
    P("dialog", "DialogTitle", "dialog-title", "h2"),
    P("dialog", "DialogDescription", "dialog-description", "p"),
    P("dialog", "DialogFooter", "dialog-footer", "div"),
    P("tooltip", "TooltipTrigger", "tooltip-trigger", "span"),
    P("tooltip", "Tooltip", "tooltip", "span"),
    P("tooltip", "TooltipContent", "tooltip-content", "span", [("side", "top")], {"role": "tooltip"}),
    P("tabs", "TabsList", "tabs-list", "div", attrs={"role": "tablist"}),
    P("native-select", "NativeSelectOption", "native-select-option", "option"),
    P("native-select", "NativeSelectOptGroup", "native-select-optgroup", "optgroup"),
]

# Parties à écriture spécifique (voir gen_ds.py) : slot par partie
SPECIAL = {
    "alert": ["Alert"],
    "separator": ["Separator"],
    "textarea": ["Textarea"],
    "checkbox": ["Checkbox"],
    "switch": ["Switch"],
    "progress": ["Progress"],
    "native-select": ["NativeSelect"],
    "avatar": ["AvatarImage"],
    "tabs": ["Tabs", "TabsTrigger", "TabsContent"],
    "accordion": ["Accordion", "AccordionItem"],
    "dialog": ["Dialog", "DialogTrigger", "DialogContent", "DialogClose"],
    "pagination": ["PaginationLink"],
    "table": ["Table"],
}

# Ordre et noms exposés par les index.ts (même clés que celles des atoms Starwind)
