<?php

namespace Database\Seeders;

use App\Models\Post;
use App\Models\Service;
use Illuminate\Database\Seeder;

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
class SiteSeeder extends Seeder
{
    /** Une ligne par langue de config/site.php : database/data/site/<langue>.php. */
    public function run(): void
    {
        foreach (array_keys(config('site.locales')) as $locale) {
            /** @var array{services: list<array<string, mixed>>, posts: list<array<string, mixed>>} $data */
            $data = require database_path("data/site/{$locale}.php");
            foreach ($data['services'] as $s) {
                Service::updateOrCreate(['locale' => $locale, 'slug' => $s['slug']], ['locale' => $locale] + $s);
            }
            foreach ($data['posts'] as $p) {
                Post::updateOrCreate(['locale' => $locale, 'slug' => $p['slug']], ['locale' => $locale] + $p);
            }
        }
    }
}
