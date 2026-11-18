@@meta mode=replace@@
<?php

use App\Http\Controllers\BlogController;
use App\Http\Controllers\ContactController;
use App\Http\Controllers\ServiceController;
use App\Http\Controllers\SiteController;
use Illuminate\Support\Facades\Route;

// Site (même arborescence qu'Astro et Wagtail) — contenu : config/site.php, modèles Service et Post.
Route::get('/', [SiteController::class, 'home'])->name('home');
Route::get('/a-propos', [SiteController::class, 'page'])->defaults('key', 'about')->name('about');
Route::get('/services', [SiteController::class, 'page'])->defaults('key', 'services')->name('services.index');
Route::get('/services/{service:slug}', [ServiceController::class, 'show'])->name('services.show');
Route::get('/blog', [SiteController::class, 'page'])->defaults('key', 'blog')->name('blog.index');
Route::get('/blog/{post:slug}', [BlogController::class, 'show'])->name('blog.show');
Route::get('/contact', [ContactController::class, 'create'])->name('contact');
Route::post('/contact', [ContactController::class, 'store'])->name('contact.store');
Route::get('/mentions-legales', [SiteController::class, 'page'])->defaults('key', 'legal')->name('legal');
Route::get('/confidentialite', [SiteController::class, 'page'])->defaults('key', 'privacy')->name('privacy');

require __DIR__.'/pages.php';
