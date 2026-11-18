<?php

namespace App\Providers;

@@if auth@@
use App\Actions\Fortify\CreateNewUser;
@@end@@
use App\Http\Middleware\SetSiteLocale;
@@if auth@@
use App\Http\Responses\SiteAuthResponse;
@@end@@
use App\View\Composers\SiteComposer;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\View;
use Illuminate\Support\ServiceProvider;
@@if auth@@
use Laravel\Fortify\Contracts\LoginResponse;
use Laravel\Fortify\Contracts\LogoutResponse;
use Laravel\Fortify\Contracts\RegisterResponse;
use Laravel\Fortify\Fortify;
@@end@@

class SiteServiceProvider extends ServiceProvider
{
    public function boot(): void
    {
        App::setLocale(config('site.locale', config('app.locale')));
        // Langue des routes du site (routes/web.php) : /en/… -> en (https://laravel.com/docs/localization). alpharust-ignore: no-hardcoded-urls
        $this->app['router']->aliasMiddleware('site.locale', SetSiteLocale::class);
        View::composer(['components.layouts.site', 'site.*', 'errors::404', 'errors.404'], SiteComposer::class);
@@if auth@@
        // Comptes : Fortify (config/fortify.php) traite les POST, le site affiche ses propres pages.
        Fortify::createUsersUsing(CreateNewUser::class);
@@end@@
    }
@@if auth@@

    public function register(): void
    {
        // Après connexion, inscription ou déconnexion : retour dans la langue du formulaire.
        $this->app->singleton(LoginResponse::class, SiteAuthResponse::class);
        $this->app->singleton(RegisterResponse::class, SiteAuthResponse::class);
        $this->app->singleton(LogoutResponse::class, SiteAuthResponse::class);
    }
@@end@@
}
