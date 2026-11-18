"""Gabarits du site, décrits une fois (voir ir.py) : en-tête, pied de page, sections, pages de détail, 404.

Données reçues (mêmes clés dans les trois frameworks) :
  site : name, initial, tagline, home_url, cta{label,url}, nav[{label,url,current,children[{label,url,current}]}],
         footer[{title,links[{label,url}]}], social[{label,url,icon}], contact{email,email_url,phone,phone_url,address,hours},
         year, copyright
  s    : une section (type + champs, `items` déjà résolus pour services et posts)
  page : page de détail (crumbs[{label,url}], title, summary, …)
"""
from .ir import E, C, T, O, Html, For, If, Icon, Csrf, Opt, V

def cls(c): return [('class', c)]

# ------------------------------------------------------------------ en-tête (atomic : Header + Sheet mobile)
def nav_children_desktop():
    return [C('Dropdown', [('openOnHover', True), ('class', 'site-nav__dropdown')], [
        C('DropdownTrigger', [('class', 'site-nav__link'), ('aria-current', Opt(V('item.current')))], [
            O(V('item.label')), Icon('chevron-down', 16)]),
        C('DropdownContent', [('class', 'site-nav__menu')], [
            C('DropdownItem', [('as', 'a'), ('href', V('item.url'))], [O(V('item.label'))]),
            For('child', V('item.children'), [
                C('DropdownItem', [('as', 'a'), ('href', V('child.url')), ('aria-current', Opt(V('child.current')))], [
                    O(V('child.label'))])]),
        ]),
    ])]

def nav_children_mobile():
    return [C('Collapsible', [('class', 'site-mobile__group')], [
        C('CollapsibleTrigger', [('class', 'site-mobile__link')], [O(V('item.label')), Icon('chevron-down', 16)]),
        C('CollapsibleContent', [], [
            E('ul', cls('site-mobile__sub'), [
                E('li', [], [E('a', [('href', V('item.url'))], [O(V('item.label'))])]),
                For('child', V('item.children'), [
                    E('li', [], [E('a', [('href', V('child.url')), ('aria-current', Opt(V('child.current')))], [O(V('child.label'))])])]),
            ])]),
    ])]

def brand():
    return E('a', [('class', 'site-brand'), ('href', V('site.home_url'))], [
        E('span', [('class', 'site-brand__mark'), ('aria-hidden', 'true')], [O(V('site.initial'))]),
        E('span', cls('site-brand__name'), [O(V('site.name'))]),
    ])

HEADER = [E('header', cls('site-header'), [
    E('div', cls('site-container site-header__bar'), [
        brand(),
        E('nav', [('class', 'site-nav'), ('aria-label', 'Navigation principale')], [
            E('ul', cls('site-nav__list'), [
                For('item', V('site.nav'), [
                    E('li', [], [
                        If(V('item.children'), nav_children_desktop(), [
                            E('a', [('class', 'site-nav__link'), ('href', V('item.url')), ('aria-current', Opt(V('item.current')))],
                              [O(V('item.label'))])]),
                    ])]),
            ])]),
        E('div', cls('site-header__actions'), [
            C('Button', [('href', V('site.cta.url')), ('variant', 'primary'), ('size', 'sm'), ('class', 'site-header__cta')],
              [O(V('site.cta.label'))]),
            C('ThemeToggle', [('size', 'sm'), ('ariaLabel', 'Changer de thème')]),
            C('Sheet', [('class', 'site-header__mobile')], [
                C('SheetTrigger', [('asChild', True)], [
                    C('Button', [('variant', 'ghost'), ('size', 'icon'), ('aria-label', 'Ouvrir le menu')], [Icon('menu', 22)])]),
                C('SheetContent', [('side', 'right'), ('class', 'site-mobile')], [
                    C('SheetHeader', [], [C('SheetTitle', [], [O(V('site.name'))])]),
                    E('nav', [('class', 'site-mobile__nav'), ('aria-label', 'Navigation mobile')], [
                        E('ul', cls('site-mobile__list'), [
                            For('item', V('site.nav'), [
                                E('li', [], [
                                    If(V('item.children'), nav_children_mobile(), [
                                        E('a', [('class', 'site-mobile__link'), ('href', V('item.url')),
                                                ('aria-current', Opt(V('item.current')))], [O(V('item.label'))])]),
                                ])]),
                        ])]),
                    C('SheetFooter', [], [
                        C('Button', [('href', V('site.cta.url')), ('variant', 'primary'), ('class', 'site-mobile__cta')],
                          [O(V('site.cta.label'))])]),
                ]),
            ]),
        ]),
    ]),
])]

# ------------------------------------------------------------------ pied de page (atomic : 3 menus + réseaux + copyright)
FOOTER = [E('footer', cls('site-footer'), [
    E('div', cls('site-container site-footer__grid'), [
        E('div', cls('site-footer__about'), [
            brand(),
            E('p', cls('site-footer__tagline'), [O(V('site.tagline'))]),
            E('ul', [('class', 'site-social'), ('aria-label', 'Réseaux sociaux')], [
                For('link', V('site.social'), [
                    E('li', [], [E('a', [('href', V('link.url')), ('aria-label', V('link.label')), ('rel', 'me noopener')],
                                   [Icon(V('link.icon'), 20)])])]),
            ]),
        ]),
        For('col', V('site.footer'), [
            E('nav', [('class', 'site-footer__col'), ('aria-label', V('col.title'))], [
                E('h2', cls('site-footer__title'), [O(V('col.title'))]),
                E('ul', cls('site-footer__links'), [
                    For('link', V('col.links'), [E('li', [], [E('a', [('href', V('link.url'))], [O(V('link.label'))])])]),
                ]),
            ])]),
        E('div', cls('site-footer__col'), [
            E('h2', cls('site-footer__title'), [T('Contact')]),
            E('address', cls('site-footer__contact'), [
                E('a', [('href', V('site.contact.email_url'))], [O(V('site.contact.email'))]),
                E('a', [('href', V('site.contact.phone_url'))], [O(V('site.contact.phone'))]),
                E('span', [], [O(V('site.contact.address'))]),
            ]),
        ]),
    ]),
    E('div', cls('site-container site-footer__bottom'), [
        E('p', [], [O(V('site.copyright_line'))]),
    ]),
])]

# ------------------------------------------------------------------ sections
def head(level='h2'):
    """Titre + chapeau + lien « voir tout » d'une section (tous optionnels)."""
    return If(V('s.title'), [E('div', cls('site-section__head'), [
        E('div', [], [
            E(level, cls('site-section__title'), [O(V('s.title'))]),
            If(V('s.lead'), [E('p', cls('site-lead'), [O(V('s.lead'))])]),
        ]),
        If(V('s.link'), [C('Button', [('href', V('s.link.url')), ('variant', 'ghost')], [O(V('s.link.label')), Icon('arrow-right', 16)])]),
    ])])

def section(mod, inner, narrow=False):
    return [E('section', cls(f'site-section site-section--{mod}'), [
        E('div', cls('site-container' + (' site-container--narrow' if narrow else '')), inner)])]

def service_card(level='h3'):
    return C('Card', [('class', 'site-card')], [
        C('CardHeader', [], [
            E('span', cls('site-icon-box'), [Icon(V('item.icon'), 22)]),
            E(level, cls('site-card__title'), [E('a', [('href', V('item.url'))], [O(V('item.title'))])]),
        ]),
        C('CardContent', [], [E('p', cls('site-card__text'), [O(V('item.summary'))])]),
        C('CardFooter', [('class', 'site-card__footer')], [
            C('Badge', [('variant', 'outline')], [O(V('item.price'))]),
            C('Button', [('href', V('item.url')), ('variant', 'outline'), ('size', 'sm'), ('aria-label', V('item.more_label'))],
              [T('En savoir plus')]),
        ]),
    ])

def post_card():
    return C('Card', [('class', 'site-card')], [
        C('CardHeader', [], [
            E('div', cls('site-meta'), [
                C('Badge', [('variant', 'outline')], [O(V('item.category'))]),
                E('time', [('datetime', V('item.date_iso'))], [O(V('item.date_label'))]),
            ]),
            E('h3', cls('site-card__title'), [E('a', [('href', V('item.url'))], [O(V('item.title'))])]),
        ]),
        C('CardContent', [], [E('p', cls('site-card__text'), [O(V('item.excerpt'))])]),
        C('CardFooter', [], [E('span', cls('site-muted'), [O(V('item.author'))])]),
    ])

SECTIONS = {
    'hero': [E('section', cls('site-hero'), [E('div', cls('site-container site-hero__inner'), [
        If(V('s.badge'), [C('Badge', [('variant', 'primary')], [O(V('s.badge'))])]),
        E('h1', cls('site-hero__title'), [O(V('s.title'))]),
        E('p', cls('site-hero__lead'), [O(V('s.lead'))]),
        E('div', cls('site-hero__actions'), [
            C('Button', [('href', V('s.primary.url')), ('variant', 'primary'), ('size', 'lg')], [O(V('s.primary.label')), Icon('arrow-right', 18)]),
            If(V('s.secondary'), [C('Button', [('href', V('s.secondary.url')), ('variant', 'outline'), ('size', 'lg')], [O(V('s.secondary.label'))])]),
        ]),
    ])])],
    'page_header': [E('section', cls('site-page-header'), [E('div', cls('site-container'), [
        If(V('s.eyebrow'), [E('p', cls('site-eyebrow'), [O(V('s.eyebrow'))])]),
        E('h1', cls('site-page-header__title'), [O(V('s.title'))]),
        If(V('s.lead'), [E('p', cls('site-lead'), [O(V('s.lead'))])]),
    ])])],
    'stats': section('stats', [E('dl', cls('site-stats__grid'), [
        For('item', V('s.items'), [E('div', cls('site-stat'), [
            E('dt', cls('site-stat__label'), [O(V('item.label'))]),
            E('dd', cls('site-stat__value'), [O(V('item.value'))]),
        ])])])]),
    'features': section('features', [head(), E('div', cls('site-grid site-grid--4'), [
        For('item', V('s.items'), [C('Card', [('class', 'site-card')], [
            C('CardHeader', [], [
                E('span', cls('site-icon-box'), [Icon(V('item.icon'), 22)]),
                E('h3', cls('site-card__title'), [O(V('item.title'))]),
            ]),
            C('CardContent', [], [E('p', cls('site-card__text'), [O(V('item.text'))])]),
        ])])])]),
    'services': section('services', [head(), E('div', cls('site-grid site-grid--3'), [
        For('item', V('s.items'), [service_card()])])]),
    'testimonials': section('testimonials', [head(), E('div', cls('site-grid site-grid--3'), [
        For('item', V('s.items'), [C('Card', [('class', 'site-card site-quote')], [
            C('CardContent', [], [
                Icon('quote', 24),
                E('blockquote', cls('site-quote__text'), [E('p', [], [O(V('item.quote'))])]),
            ]),
            C('CardFooter', [('class', 'site-person')], [
                C('Avatar', [('size', 'md')], [C('AvatarFallback', [], [O(V('item.initials'))])]),
                E('div', [], [
                    E('p', cls('site-person__name'), [O(V('item.name'))]),
                    E('p', cls('site-person__role'), [O(V('item.role'))]),
                ]),
            ]),
        ])])])]),
    'posts': section('posts', [head(), E('div', cls('site-grid site-grid--3'), [
        For('item', V('s.items'), [post_card()])])]),
    # FAQ : <details name> natifs (une seule réponse ouverte, clavier et lecteurs d'écran sans script) — même élément
    # dans tous les kits (l'accordéon Starwind est un <button> scripté, celui du CSS pur un <details>).
    'faq': section('faq', [head(), E('div', cls('site-faq__list'), [
        For('item', V('s.items'), [E('details', [('class', 'site-faq__item'), ('name', 'faq')], [
            E('summary', cls('site-faq__question'), [E('span', [], [O(V('item.question'))]), Icon('chevron-down', 18)]),
            E('div', cls('site-faq__answer'), [E('p', [], [O(V('item.answer'))])]),
        ])])])], narrow=True),
    'cta': section('cta', [E('div', cls('site-cta'), [
        E('div', [], [
            E('h2', cls('site-cta__title'), [O(V('s.title'))]),
            E('p', cls('site-cta__text'), [O(V('s.text'))]),
        ]),
        C('Button', [('href', V('s.action.url')), ('variant', 'default'), ('size', 'lg')], [O(V('s.action.label')), Icon('arrow-right', 18)]),
    ])]),
    'text': section('text', [
        If(V('s.title'), [E('h2', cls('site-section__title'), [O(V('s.title'))])]),
        C('Prose', [('class', 'site-prose')], [Html(V('s.body'))]),
    ], narrow=True),
    'team': section('team', [head(), E('ul', cls('site-grid site-grid--4 site-team'), [
        For('item', V('s.items'), [E('li', cls('site-team__member'), [
            C('Avatar', [('size', 'lg')], [C('AvatarFallback', [], [O(V('item.initials'))])]),
            E('p', cls('site-person__name'), [O(V('item.name'))]),
            E('p', cls('site-person__role'), [O(V('item.role'))]),
        ])])])]),
    'contact': section('contact', [E('div', cls('site-contact__grid'), [
        C('Card', [('class', 'site-card')], [
            C('CardHeader', [], [E('h2', cls('site-card__title'), [O(V('s.title'))])]),
            C('CardContent', [], [
                If(V('s.sent'), [C('Alert', [('variant', 'success'), ('class', 'site-form__notice')], [
                    C('AlertTitle', [], [O(V('s.success'))])])]),
                E('form', [('class', 'site-form'), ('method', 'post'), ('action', V('s.action'))], [
                    Csrf(),
                    For('field', V('s.fields'), [E('div', cls('site-field'), [
                        C('Label', [('for', V('field.id'))], [O(V('field.label')),
                            If(V('field.required'), [E('span', [('class', 'site-required'), ('aria-hidden', 'true')], [T('*')])])]),
                        If(V('field.textarea'), [
                            C('Textarea', [('id', V('field.id')), ('name', V('field.name')), ('rows', '6'),
                                           ('required', Opt(V('field.required'))), ('aria-invalid', Opt(V('field.invalid'))),
                                           ('aria-describedby', Opt(V('field.error_id'))), ('defaultValue', V('field.value'))]),
                        ], [
                            C('Input', [('id', V('field.id')), ('name', V('field.name')), ('type', V('field.type')),
                                        ('autocomplete', V('field.autocomplete')), ('value', V('field.value')),
                                        ('required', Opt(V('field.required'))), ('aria-invalid', Opt(V('field.invalid'))),
                                        ('aria-describedby', Opt(V('field.error_id')))]),
                        ]),
                        If(V('field.error'), [E('p', [('class', 'site-field__error'), ('id', V('field.error_id'))], [O(V('field.error'))])]),
                    ])]),
                    C('Button', [('type', 'submit'), ('variant', 'primary')], [O(V('s.submit')), Icon('send', 16)]),
                ]),
            ]),
        ]),
        E('aside', cls('site-contact__info'), [
            C('Card', [('class', 'site-card')], [
                C('CardHeader', [], [E('h2', cls('site-card__title'), [T('Coordonnées')])]),
                C('CardContent', [], [E('ul', cls('site-contact__list'), [
                    E('li', [], [Icon('mail', 18), E('a', [('href', V('site.contact.email_url'))], [O(V('site.contact.email'))])]),
                    E('li', [], [Icon('phone', 18), E('a', [('href', V('site.contact.phone_url'))], [O(V('site.contact.phone'))])]),
                    E('li', [], [Icon('map-pin', 18), E('span', [], [O(V('site.contact.address'))])]),
                    E('li', [], [Icon('clock', 18), E('span', [], [O(V('site.contact.hours'))])]),
                ])]),
            ]),
        ]),
    ])]),
}

# ------------------------------------------------------------------ pages de détail
def crumbs():
    return C('Breadcrumb', [], [C('BreadcrumbList', [], [
        For('crumb', V('page.crumbs'), [
            C('BreadcrumbItem', [], [C('BreadcrumbLink', [('href', V('crumb.url'))], [O(V('crumb.label'))])]),
            C('BreadcrumbSeparator', []),
        ]),
        C('BreadcrumbItem', [], [C('BreadcrumbPage', [], [O(V('page.title'))])]),
    ])])

SERVICE_DETAIL = [
    E('section', cls('site-page-header'), [E('div', cls('site-container'), [
        crumbs(),
        E('span', cls('site-icon-box site-icon-box--lg'), [Icon(V('page.icon'), 28)]),
        E('h1', cls('site-page-header__title'), [O(V('page.title'))]),
        E('p', cls('site-lead'), [O(V('page.summary'))]),
        C('Badge', [('variant', 'primary'), ('size', 'lg')], [O(V('page.price'))]),
    ])]),
    E('section', cls('site-section'), [E('div', cls('site-container site-detail'), [
        C('Prose', [('class', 'site-prose site-detail__body')], [Html(V('page.body'), astro='<Content />', django='{{ page.body|richtext }}')]),
        E('aside', cls('site-detail__aside'), [C('Card', [('class', 'site-card')], [
            C('CardHeader', [], [E('h2', cls('site-card__title'), [T('Ce qui est inclus')])]),
            C('CardContent', [], [E('ul', cls('site-checklist'), [
                For('feature', V('page.features'), [E('li', [], [Icon('check', 18), E('span', [], [O(V('feature'))])])])])]),
            C('CardFooter', [], [C('Button', [('href', V('site.cta.url')), ('variant', 'primary'), ('class', 'site-block')], [O(V('site.cta.label'))])]),
        ])]),
    ])]),
]

POST_DETAIL = [
    E('article', cls('site-article'), [
        E('header', cls('site-page-header'), [E('div', cls('site-container site-container--narrow'), [
            crumbs(),
            C('Badge', [('variant', 'outline')], [O(V('page.category'))]),
            E('h1', cls('site-page-header__title'), [O(V('page.title'))]),
            E('p', cls('site-meta'), [
                Icon('calendar', 16), E('time', [('datetime', V('page.date_iso'))], [O(V('page.date_label'))]),
                E('span', [('aria-hidden', 'true')], [T('·')]), E('span', [], [O(V('page.author'))]),
            ]),
        ])]),
        E('div', cls('site-section'), [E('div', cls('site-container site-container--narrow'), [
            C('Prose', [('class', 'site-prose')], [Html(V('page.body'), astro='<Content />', django='{{ page.body|richtext }}')]),
            E('p', cls('site-article__back'), [C('Button', [('href', V('page.back_url')), ('variant', 'outline')], [
                Icon('arrow-left', 16), T('Tous les articles')])]),
        ])]),
    ]),
]

NOT_FOUND = [E('section', cls('site-section site-404'), [E('div', cls('site-container site-container--narrow'), [
    E('p', cls('site-404__code'), [T('404')]),
    E('h1', cls('site-page-header__title'), [T('Page introuvable')]),
    E('p', cls('site-lead'), [T("La page demandée n’existe pas ou a été déplacée.")]),
    C('Button', [('href', V('site.home_url')), ('variant', 'primary')], [Icon('arrow-left', 16), T("Retour à l’accueil")]),
])])]
