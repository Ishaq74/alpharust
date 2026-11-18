<?php

namespace App\Providers;

use App\Http\Middleware\SetSiteLocale;
use App\View\Composers\SiteComposer;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\View;
use Illuminate\Support\ServiceProvider;

class SiteServiceProvider extends ServiceProvider
{
    public function boot(): void
    {
        App::setLocale(config('site.locale', config('app.locale')));
        // Langue des routes du site (routes/web.php) : /en/… -> en (https://laravel.com/docs/localization). alpharust-ignore: no-hardcoded-urls
        $this->app['router']->aliasMiddleware('site.locale', SetSiteLocale::class);
        View::composer(['components.layouts.site', 'site.*', 'errors::404', 'errors.404'], SiteComposer::class);
    }
}
