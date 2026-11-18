<?php

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
// Langues du site (localisation officielle de Laravel, doc « localization »)
// Le contenu (navigation, pied de page, pages, libellés) est dans lang/<langue>/site.php ; les services et
// les articles sont en base, une ligne par langue (modèles Service et Post, database/seeders/SiteSeeder.php).

return [
    'locale' => 'fr',
    'locales' => [
        'fr' => [
            'name' => 'Français',
            'label' => 'FR',
        ],
        'en' => [
            'name' => 'English',
            'label' => 'EN',
        ],
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
