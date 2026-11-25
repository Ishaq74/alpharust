@@meta mode=replace@@
<?php

@@if auth@@
use App\Http\Controllers\AccountController;
@@end@@
use App\Http\Controllers\BlogController;
use App\Http\Controllers\ContactController;
use App\Http\Controllers\SeoController;
use App\Http\Controllers\ServiceController;
use App\Http\Controllers\SiteController;
use Illuminate\Support\Facades\Route;

// Site (même arborescence qu'Astro et Wagtail), une fois par langue : /… (fr, langue par défaut) et /en/…
// Contenu : lang/<langue>/site.php, modèles Service et Post (colonne locale).
foreach (array_keys(config('site.locales')) as $locale) {
    $default = $locale === config('site.locale');
    Route::prefix($default ? '' : $locale)
        ->name($default ? '' : $locale.'.')
        ->middleware('site.locale:'.$locale)
        ->group(function () {
            Route::get('/', [SiteController::class, 'home'])->name('home');
            Route::get('/a-propos', [SiteController::class, 'page'])->defaults('key', 'about')->name('about');
            Route::get('/services', [SiteController::class, 'page'])->defaults('key', 'services')->name('services.index');
            Route::get('/services/{slug}', [ServiceController::class, 'show'])->name('services.show');
            Route::get('/blog', [SiteController::class, 'page'])->defaults('key', 'blog')->name('blog.index');
            Route::get('/blog/{slug}', [BlogController::class, 'show'])->name('blog.show');
            Route::get('/contact', [ContactController::class, 'create'])->name('contact');
            Route::post('/contact', [ContactController::class, 'store'])->name('contact.store');
            Route::get('/mentions-legales', [SiteController::class, 'page'])->defaults('key', 'legal')->name('legal');
            Route::get('/confidentialite', [SiteController::class, 'page'])->defaults('key', 'privacy')->name('privacy');
@@if auth@@
            // Comptes : pages du site ; les POST (connexion, inscription, déconnexion) sont les routes de Fortify.
            Route::get('/auth/sign-in', [AccountController::class, 'signIn'])->name('auth.sign-in');
            Route::get('/auth/sign-up', [AccountController::class, 'signUp'])->name('auth.sign-up');
            Route::get('/auth/dashboard', [AccountController::class, 'show'])->name('auth.account');
@@end@@
        });
}

// Référencement : robots.txt et plan du site XML (mêmes adresses qu'Astro et Wagtail).
Route::get('/robots.txt', [SeoController::class, 'robots'])->name('robots');
Route::get('/sitemap.xml', [SeoController::class, 'sitemap'])->name('sitemap');

require __DIR__.'/pages.php';
