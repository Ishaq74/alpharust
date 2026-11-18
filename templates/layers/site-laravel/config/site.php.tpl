<?php

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
// Langues du site (localisation officielle de Laravel, doc « localization ») : celles choisies à la génération
// (--langs). dir : sens d'écriture (rtl pour l'arabe) ; date : format Carbon translatedFormat des articles.
// Le contenu (navigation, pied de page, pages, libellés) est dans lang/<langue>/site.php ; les services et
// les articles sont en base, une ligne par langue (modèles Service et Post), semés depuis database/data/site/<langue>.php.

return [
    'locale' => 'fr',
    'locales' => [
        'fr' => [
            'name' => 'Français',
            'label' => 'FR',
            'dir' => 'ltr',
            'date' => 'j F Y',
        ],
@@if lang_en@@
        'en' => [
            'name' => 'English',
            'label' => 'EN',
            'dir' => 'ltr',
            'date' => 'F j, Y',
        ],
@@end@@
@@if lang_es@@
        'es' => [
            'name' => 'Español',
            'label' => 'ES',
            'dir' => 'ltr',
            'date' => 'j \\d\\e F \\d\\e Y',
        ],
@@end@@
@@if lang_ar@@
        'ar' => [
            'name' => 'العربية',
            'label' => 'AR',
            'dir' => 'rtl',
            'date' => 'j F Y',
        ],
@@end@@
    ],
    'auth' => [
        'routes' => [
            'sign_in' => '/auth/sign-in',
            'sign_up' => '/auth/sign-up',
            'account' => '/auth/dashboard',
        ],
        'fields' => [
            'sign_in' => [
                [
                    'name' => 'email',
                    'type' => 'email',
                    'autocomplete' => 'email',
                ],
                [
                    'name' => 'password',
                    'type' => 'password',
                    'autocomplete' => 'current-password',
                ],
            ],
            'sign_up' => [
                [
                    'name' => 'name',
                    'type' => 'text',
                    'autocomplete' => 'name',
                ],
                [
                    'name' => 'email',
                    'type' => 'email',
                    'autocomplete' => 'email',
                ],
                [
                    'name' => 'password',
                    'type' => 'password',
                    'autocomplete' => 'new-password',
                ],
            ],
        ],
        'password_min' => 8,
    ],
];
