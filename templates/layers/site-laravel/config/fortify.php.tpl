@@meta when=auth@@
<?php

use Laravel\Fortify\Features;

// Laravel Fortify (authentification sans interface) : le site fournit ses pages (AccountController, vues site.auth
// et site.account) ; Fortify traite les POST aux mêmes adresses qu'Astro et Wagtail.
return [
    'guard' => 'web',
    'middleware' => ['web'],
    'auth_middleware' => 'auth',
    'passwords' => 'users',
    'username' => 'email',
    'email' => 'email',
    'lowercase_usernames' => true,
    'views' => false,
    'home' => '/auth/dashboard',
    'prefix' => '',
    'domain' => null,
    'limiters' => [
        'login' => null,
    ],
    'paths' => [
        'login' => '/auth/sign-in',
        'logout' => '/auth/sign-out',
        'register' => '/auth/sign-up',
    ],
    'features' => [
        Features::registration(),
    ],
];
