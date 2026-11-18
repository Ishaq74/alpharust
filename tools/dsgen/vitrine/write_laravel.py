"""Rendu Laravel : layout (composant anonyme, comme manecms) + partials + routes/contrôleurs + modèles/migrations/seeder."""
import os
from . import data as D, templates as TPL
from .ir import Blade
from .icons import ICONS

GEN = '{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}\n'
PHPGEN = '// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.\n'

def php(v, ind=0):
    p, q = '    ' * ind, '    ' * (ind + 1)
    if v is None: return 'null'
    if v is True: return 'true'
    if v is False: return 'false'
    if isinstance(v, (int, float)): return str(v)
    if isinstance(v, str): return "'" + v.replace('\\', '\\\\').replace("'", "\\'") + "'"
    if isinstance(v, list):
        if not v: return '[]'
        return '[\n' + ''.join(f'{q}{php(x, ind + 1)},\n' for x in v) + p + ']'
    if isinstance(v, dict):
        ign = lambda x: ' // alpharust-ignore: no-hardcoded-urls' if isinstance(x, str) and '://' in x else ''
        return '[\n' + ''.join(f"{q}'{k}' => {php(x, ind + 1)},{ign(x)}\n" for k, x in v.items()) + p + ']'
    raise TypeError(v)

def config():
    s = {k: D.SITE[k] for k in ('name', 'tagline', 'description', 'cta', 'contact', 'social', 'nav', 'footer', 'copyright')}
    s['locale'] = 'fr'
    s['pages'] = D.PAGES
    s['contact_fields'] = D.FIELDS
    return ("<?php\n\n" + PHPGEN + "// Contenu du site (navigation, pied de page, pages composées de sections) : lu par App\\Support\\SiteData.\n"
            "// Les services et les articles sont en base (modèles Service et Post, database/seeders/SiteSeeder.php).\n\n"
            "return " + php(s) + ";\n")

ICON = (GEN + "@props(['name', 'size' => 20])\n@php\n    // Tracés Lucide (ISC) : mêmes icônes qu’Astro (SiteIcon.astro) et Django ({% site_icon %}).\n    $paths = [\n"
        + ''.join(f"        '{k}' => '{v}',\n" for k, v in ICONS.items())
        + "    ];\n@endphp\n<svg width=\"{{ $size }}\" height=\"{{ $size }}\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" "
          "stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\" class=\"site-icon\">{!! $paths[$name] ?? '' !!}</svg>\n")

LAYOUT = GEN + """{{-- Gabarit commun du site (modèle manecms : composant « shell » + partials head/header/footer). --}}
@props(['title' => null, 'description' => null])
<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    @include('site.partials.head', ['title' => $title, 'description' => $description])
</head>
<body class="site-body">
    <a class="site-skip" href="#contenu">Aller au contenu</a>
    @include('site.partials.header')
    <main id="contenu" class="site-main" tabindex="-1">
        {{ $slot }}
    </main>
    @include('site.partials.footer')
</body>
</html>
"""

HEAD = GEN + """<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{{ $title ? $title.' · '.$site['name'] : $site['name'] }}</title>
<meta name="description" content="{{ $description ?: $site['description'] }}">
<link rel="canonical" href="{{ url()->current() }}">
<link rel="icon" type="image/svg+xml" href="/favicon.svg">
<meta property="og:type" content="website">
<meta property="og:title" content="{{ $title ? $title.' · '.$site['name'] : $site['name'] }}">
<meta property="og:description" content="{{ $description ?: $site['description'] }}">
<script>
    // Applique le thème choisi avant le premier rendu (évite le flash).
    try {
        const saved = localStorage.getItem('theme');
        const theme = saved === 'light' || saved === 'dark' ? saved : (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
        document.documentElement.dataset.theme = theme;
        document.documentElement.classList.toggle('dark', theme === 'dark');
    } catch (e) {}
</script>
@if (file_exists(public_path('build/manifest.json')) || file_exists(public_path('hot')))
    @vite(['resources/css/app.css', 'resources/js/app.js'])
@endif
"""

def page_view():
    return GEN + """<x-layouts.site :title="$page['title'] ?? null" :description="$page['description'] ?? null">
    @foreach ($page['sections'] as $s)
        @include('site.sections.'.$s['type'], ['s' => $s])
    @endforeach
</x-layouts.site>
"""

def detail_view(cat, nodes):
    return GEN + "<x-layouts.site :title=\"$page['title']\" :description=\"$page['description']\">\n" + Blade(cat).nodes(nodes, 1) + "</x-layouts.site>\n"

def not_found(cat):
    return GEN + '<x-layouts.site title="Page introuvable">\n' + Blade(cat).nodes(TPL.NOT_FOUND, 1) + '</x-layouts.site>\n'

SITE_DATA = """<?php

namespace App\\Support;

use App\\Models\\Post;
use App\\Models\\Service;

/**
 * Données normalisées du site : mêmes clés que src/lib/site.ts (Astro) et layout/context_processors.py (Wagtail),
 * si bien que les vues Blade produisent le même balisage.
 */
final class SiteData
{
    /** Adresse interne au format Laravel (sans barre finale). */
    public static function href(string $url): string
    {
        return $url === '/' || preg_match('/^[a-z]+:/', $url) ? $url : '/'.trim($url, '/');
    }

    /** 'page' (page exacte), 'true' (rubrique) ou null — même règle que les autres frameworks. */
    public static function current(string $url, string $path): ?string
    {
        $u = self::href($url);
        $p = '/'.trim($path, '/');
        if ($p === $u) {
            return 'page';
        }

        return $u !== '/' && str_starts_with($p.'/', $u.'/') ? 'true' : null;
    }

    /** @return list<array<string, mixed>> */
    public function services(): array
    {
        return Service::query()->orderBy('position')->get()->map(fn (Service $s) => $s->toCard())->all();
    }

    /** @return list<array<string, mixed>> */
    public function posts(): array
    {
        return Post::query()->orderByDesc('published_at')->get()->map(fn (Post $p) => $p->toCard())->all();
    }

    /** @return array<string, mixed> */
    public function site(string $path): array
    {
        $s = config('site');
        $services = array_map(fn (array $x) => [
            'label' => $x['title'], 'url' => $x['url'], 'current' => self::current($x['url'], $path),
        ], $this->services());
        $links = fn ($v) => $v === 'services' ? $services : array_map(
            fn (array $l) => ['label' => $l['label'], 'url' => self::href($l['url'])], $v
        );

        return [
            'name' => $s['name'],
            'initial' => mb_strtoupper(mb_substr($s['name'], 0, 1)),
            'tagline' => $s['tagline'],
            'description' => $s['description'],
            'home_url' => '/',
            'cta' => ['label' => $s['cta']['label'], 'url' => self::href($s['cta']['url'])],
            'nav' => array_map(fn (array $i) => [
                'label' => $i['label'],
                'url' => self::href($i['url']),
                'current' => self::current($i['url'], $path),
                'children' => isset($i['children']) ? $links($i['children']) : null,
            ], $s['nav']),
            'footer' => array_map(fn (array $c) => ['title' => $c['title'], 'links' => $links($c['links'])], $s['footer']),
            'social' => $s['social'],
            'contact' => $s['contact'] + [
                'email_url' => 'mailto:'.$s['contact']['email'],
                'phone_url' => 'tel:'.preg_replace('/\\s+/', '', $s['contact']['phone']),
            ],
            'year' => (int) date('Y'),
            'copyright' => $s['copyright'],
        ];
    }

    /**
     * Sections d'une page avec listes et formulaire résolus.
     *
     * @param  array{sent?: bool, values?: array<string, mixed>, errors?: array<string, string>}  $state
     * @return array<string, mixed>
     */
    public function page(string $key, array $state = []): array
    {
        $page = config("site.pages.$key");
        $fix = fn (?array $a) => $a ? ['label' => $a['label'], 'url' => self::href($a['url'])] : null;
        $page['sections'] = array_map(function (array $s) use ($page, $state, $fix) {
            $s['link'] = $fix($s['link'] ?? null);

            return match ($s['type']) {
                'services' => $s + ['items' => array_slice($this->services(), 0, $s['limit'] ?: null)],
                'posts' => $s + ['items' => array_slice($this->posts(), 0, $s['limit'] ?: null)],
                'hero' => array_merge($s, ['primary' => $fix($s['primary']), 'secondary' => $fix($s['secondary'] ?? null)]),
                'cta' => array_merge($s, ['action' => $fix($s['action'])]),
                'contact' => $s + [
                    'action' => self::href($page['url']),
                    'sent' => (bool) ($state['sent'] ?? false),
                    'fields' => array_map(function (array $f) use ($state) {
                        $error = $state['errors'][$f['name']] ?? null;
                        $id = 'contact-'.$f['name'];

                        return $f + [
                            'id' => $id,
                            'textarea' => $f['type'] === 'textarea',
                            'value' => (string) ($state['values'][$f['name']] ?? ''),
                            'error' => $error,
                            'invalid' => $error ? 'true' : null,
                            'error_id' => $error ? $id.'-error' : null,
                        ];
                    }, config('site.contact_fields')),
                ],
                default => $s,
            };
        }, $page['sections']);

        return $page;
    }

    /** @return list<array{label: string, url: string}> */
    public static function crumbs(string $label, string $url): array
    {
        return [['label' => 'Accueil', 'url' => '/'], ['label' => $label, 'url' => self::href($url)]];
    }
}
"""

COMPOSER = """<?php

namespace App\\View\\Composers;

use App\\Support\\SiteData;
use Illuminate\\View\\View;

/** Partage `$site` (navigation, pied de page, coordonnées) avec le gabarit et les vues du site. */
class SiteComposer
{
    public function __construct(private SiteData $data) {}

    public function compose(View $view): void
    {
        $view->with('site', $this->data->site(request()->path()));
    }
}
"""

PROVIDER = """<?php

namespace App\\Providers;

use App\\View\\Composers\\SiteComposer;
use Illuminate\\Support\\Facades\\App;
use Illuminate\\Support\\Facades\\View;
use Illuminate\\Support\\ServiceProvider;

class SiteServiceProvider extends ServiceProvider
{
    public function boot(): void
    {
        App::setLocale(config('site.locale', config('app.locale')));
        View::composer(['components.layouts.site', 'site.*', 'errors::404', 'errors.404'], SiteComposer::class);
    }
}
"""

PROVIDERS = """@@meta mode=replace@@
<?php

use App\\Providers\\AppServiceProvider;
use App\\Providers\\SiteServiceProvider;

return [
    AppServiceProvider::class,
    SiteServiceProvider::class,
];
"""

CONTROLLERS = {
'SiteController': """<?php

namespace App\\Http\\Controllers;

use App\\Support\\SiteData;
use Illuminate\\View\\View;

class SiteController extends Controller
{
    public function __construct(private SiteData $data) {}

    public function home(): View
    {
        return view('site.page', ['page' => ['title' => null] + $this->data->page('home')]);
    }

    /** Pages composées de sections : à propos, mentions légales, confidentialité, index services/blog. */
    public function page(string $key): View
    {
        return view('site.page', ['page' => $this->data->page($key)]);
    }
}
""",
'ServiceController': """<?php

namespace App\\Http\\Controllers;

use App\\Models\\Service;
use App\\Support\\SiteData;
use Illuminate\\View\\View;

class ServiceController extends Controller
{
    public function show(Service $service): View
    {
        return view('site.service', ['page' => $service->toCard() + [
            'description' => $service->summary,
            'body' => $service->body,
            'crumbs' => SiteData::crumbs('Services', '/services'),
        ]]);
    }
}
""",
'BlogController': """<?php

namespace App\\Http\\Controllers;

use App\\Models\\Post;
use App\\Support\\SiteData;
use Illuminate\\View\\View;

class BlogController extends Controller
{
    public function show(Post $post): View
    {
        return view('site.post', ['page' => $post->toCard() + [
            'description' => $post->excerpt,
            'body' => $post->body,
            'crumbs' => SiteData::crumbs('Blog', '/blog'),
            'back_url' => SiteData::href('/blog'),
        ]]);
    }
}
""",
'ContactController': """<?php

namespace App\\Http\\Controllers;

use App\\Models\\ContactMessage;
use App\\Support\\SiteData;
use Illuminate\\Http\\RedirectResponse;
use Illuminate\\Http\\Request;
use Illuminate\\View\\View;

class ContactController extends Controller
{
    public function __construct(private SiteData $data) {}

    public function create(Request $request): View
    {
        $bag = $request->session()->get('errors');
        $errors = $bag ? array_map(fn (array $m) => $m[0], $bag->getBag('default')->messages()) : [];

        return view('site.page', ['page' => $this->data->page('contact', [
            'sent' => $request->session()->get('status') === 'sent',
            'values' => $request->old(),
            'errors' => $errors,
        ])]);
    }

    public function store(Request $request): RedirectResponse
    {
        $rules = [];
        foreach (config('site.contact_fields') as $f) {
            $rules[$f['name']] = array_filter([
                $f['required'] ? 'required' : 'nullable', 'string', 'max:5000', $f['type'] === 'email' ? 'email' : null,
            ]);
        }
        $data = $request->validate($rules, [
            'required' => 'Ce champ est obligatoire.',
            'email' => 'Adresse e-mail invalide.',
        ]);
        ContactMessage::create(['data' => $data]);

        return redirect(SiteData::href('/contact'))->with('status', 'sent');
    }
}
""",
}

MODELS = {
'Service': """<?php

namespace App\\Models;

use App\\Support\\SiteData;
use Illuminate\\Database\\Eloquent\\Model;

class Service extends Model
{
    protected $fillable = ['slug', 'title', 'summary', 'icon', 'price', 'features', 'body', 'position'];

    protected function casts(): array
    {
        return ['features' => 'array', 'position' => 'integer'];
    }

    /** @return array<string, mixed> */
    public function toCard(): array
    {
        $url = SiteData::href('/services/'.$this->slug);

        return [
            'title' => $this->title,
            'url' => $url,
            'summary' => $this->summary,
            'icon' => $this->icon,
            'price' => $this->price,
            'features' => $this->features ?? [],
            'more_label' => 'En savoir plus : '.$this->title,
        ];
    }
}
""",
'Post': """<?php

namespace App\\Models;

use App\\Support\\SiteData;
use Illuminate\\Database\\Eloquent\\Model;

class Post extends Model
{
    protected $fillable = ['slug', 'title', 'excerpt', 'category', 'author', 'published_at', 'body'];

    protected function casts(): array
    {
        return ['published_at' => 'date'];
    }

    /** @return array<string, mixed> */
    public function toCard(): array
    {
        return [
            'title' => $this->title,
            'url' => SiteData::href('/blog/'.$this->slug),
            'excerpt' => $this->excerpt,
            'category' => $this->category,
            'author' => $this->author,
            'date_iso' => $this->published_at->toDateString(),
            'date_label' => $this->published_at->locale('fr')->translatedFormat('j F Y'),
        ];
    }
}
""",
'ContactMessage': """<?php

namespace App\\Models;

use Illuminate\\Database\\Eloquent\\Model;

/** Message envoyé depuis /contact (équivalent des FormSubmission de Wagtail). */
class ContactMessage extends Model
{
    protected $fillable = ['data'];

    protected function casts(): array
    {
        return ['data' => 'array'];
    }
}
""",
}

MIGRATIONS = {
'2026_01_02_000000_create_services_table.php': """<?php

use Illuminate\\Database\\Migrations\\Migration;
use Illuminate\\Database\\Schema\\Blueprint;
use Illuminate\\Support\\Facades\\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('services', function (Blueprint $table) {
            $table->id();
            $table->string('slug')->unique();
            $table->string('title');
            $table->text('summary');
            $table->string('icon');
            $table->string('price');
            $table->json('features');
            $table->longText('body');
            $table->unsignedInteger('position')->default(0);
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('services');
    }
};
""",
'2026_01_02_000001_create_posts_table.php': """<?php

use Illuminate\\Database\\Migrations\\Migration;
use Illuminate\\Database\\Schema\\Blueprint;
use Illuminate\\Support\\Facades\\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('posts', function (Blueprint $table) {
            $table->id();
            $table->string('slug')->unique();
            $table->string('title');
            $table->text('excerpt');
            $table->string('category');
            $table->string('author');
            $table->date('published_at');
            $table->longText('body');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('posts');
    }
};
""",
'2026_01_02_000002_create_contact_messages_table.php': """<?php

use Illuminate\\Database\\Migrations\\Migration;
use Illuminate\\Database\\Schema\\Blueprint;
use Illuminate\\Support\\Facades\\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('contact_messages', function (Blueprint $table) {
            $table->id();
            $table->json('data');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('contact_messages');
    }
};
""",
}

def seeder():
    services = [{'slug': s['slug'], 'title': s['title'], 'summary': s['summary'], 'icon': s['icon'], 'price': s['price'],
                 'features': s['features'], 'body': s['body_html'], 'position': s['order']} for s in D.SERVICES]
    posts = [{'slug': p['slug'], 'title': p['title'], 'excerpt': p['excerpt'], 'category': p['category'],
              'author': p['author'], 'published_at': p['date'], 'body': p['body_html']} for p in D.POSTS]
    return ("<?php\n\nnamespace Database\\Seeders;\n\nuse App\\Models\\Post;\nuse App\\Models\\Service;\nuse Illuminate\\Database\\Seeder;\n\n"
            + PHPGEN + "class SiteSeeder extends Seeder\n{\n    public function run(): void\n    {\n"
            "        foreach (" + php(services, 2) + " as $s) {\n            Service::updateOrCreate(['slug' => $s['slug']], $s);\n        }\n"
            "        foreach (" + php(posts, 2) + " as $p) {\n            Post::updateOrCreate(['slug' => $p['slug']], $p);\n        }\n    }\n}\n")

ROUTES = """@@meta mode=replace@@
<?php

use App\\Http\\Controllers\\BlogController;
use App\\Http\\Controllers\\ContactController;
use App\\Http\\Controllers\\ServiceController;
use App\\Http\\Controllers\\SiteController;
use Illuminate\\Support\\Facades\\Route;

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
"""

TEST = """<?php

namespace Tests\\Feature;

use App\\Models\\ContactMessage;
use Database\\Seeders\\SiteSeeder;
use Illuminate\\Foundation\\Testing\\RefreshDatabase;
use Tests\\TestCase;

class SiteTest extends TestCase
{
    use RefreshDatabase;

    protected function setUp(): void
    {
        parent::setUp();
        $this->seed(SiteSeeder::class);
    }

    public static function pages(): array
    {
        return [
            'accueil' => ['/', 'Des sites rapides, accessibles'],
            'à propos' => ['/a-propos', 'Une petite équipe'],
            'services' => ['/services', 'Ce que nous faisons'],
            'service' => ['/services/accessibilite', 'Ce qui est inclus'],
            'blog' => ['/blog', 'Articles et conseils'],
            'article' => ['/blog/choisir-son-framework', 'Tous les articles'],
            'contact' => ['/contact', 'Parlons de votre projet'],
            'mentions légales' => ['/mentions-legales', 'Éditeur'],
            'confidentialité' => ['/confidentialite', 'Vos droits'],
        ];
    }

    /** @dataProvider pages */
    #[\\PHPUnit\\Framework\\Attributes\\DataProvider('pages')]
    public function test_every_page_renders_inside_the_shared_layout(string $url, string $text): void
    {
        $this->get($url)
            ->assertOk()
            ->assertSee($text)
            ->assertSee('href="#contenu"', false)
            ->assertSee('aria-label="Navigation principale"', false)
            ->assertSee('data-slot="theme-toggle"', false)
            ->assertSee('data-slot="sheet-content"', false)
            ->assertSee('<footer class="site-footer">', false);
    }

    public function test_the_current_page_is_marked_in_the_menu(): void
    {
        $this->get('/blog')->assertSee('href="/blog" aria-current="page"', false);
        $this->get('/services/maintenance')->assertSee('href="/services/maintenance" aria-current="page"', false);
    }

    public function test_unknown_pages_use_the_site_404(): void
    {
        $this->get('/nexiste-pas')->assertNotFound()->assertSee('Page introuvable')->assertSee('site-header', false);
        $this->get('/services/inconnu')->assertNotFound();
    }

    public function test_contact_form_validates_and_stores_messages(): void
    {
        $this->post('/contact', ['nom' => '', 'email' => 'x'])->assertRedirect()->assertSessionHasErrors(['nom', 'email', 'message']);
        $this->post('/contact', ['nom' => 'Ada', 'email' => 'ada@example.com', 'message' => 'Bonjour'])
            ->assertRedirect('/contact');
        $this->assertSame(1, ContactMessage::count());
        $this->withSession(['status' => 'sent'])->get('/contact')->assertSee('Votre message a bien été envoyé');
    }
}
"""

ROOT_LAYERS = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'templates', 'layers'))

EXAMPLE_TEST = """@@meta mode=replace@@
<?php

namespace Tests\\Feature;

use Illuminate\\Foundation\\Testing\\RefreshDatabase;
use Tests\\TestCase;

class ExampleTest extends TestCase
{
    // La page d'accueil du site lit les services et les articles en base.
    use RefreshDatabase;

    public function test_the_application_returns_a_successful_response(): void
    {
        $response = $this->get('/');

        $response->assertStatus(200);
    }
}
"""

def files(cat, css):
    gen = '@@meta when=ui@@\n'
    out = {
        'config/site.php.tpl': config(),
        'app/Support/SiteData.php.tpl': SITE_DATA,
        'app/View/Composers/SiteComposer.php.tpl': COMPOSER,
        'app/Providers/SiteServiceProvider.php.tpl': PROVIDER,
        'bootstrap/providers.php.tpl': PROVIDERS,
        'routes/web.php.tpl': ROUTES,
        'database/seeders/SiteSeeder.php.tpl': seeder(),
        'tests/Feature/SiteTest.php.tpl': TEST,
        'tests/Feature/ExampleTest.php.tpl': EXAMPLE_TEST,
        'resources/views/components/layouts/site.blade.php.tpl': LAYOUT,
        'resources/views/components/site/icon.blade.php.tpl': ICON,
        'resources/views/site/partials/head.blade.php.tpl': HEAD,
        'resources/views/site/partials/header.blade.php.tpl': GEN + Blade(cat).nodes(TPL.HEADER),
        'resources/views/site/partials/footer.blade.php.tpl': GEN + Blade(cat).nodes(TPL.FOOTER),
        'resources/views/site/page.blade.php.tpl': page_view(),
        'resources/views/site/service.blade.php.tpl': detail_view(cat, TPL.SERVICE_DETAIL),
        'resources/views/site/post.blade.php.tpl': detail_view(cat, TPL.POST_DETAIL),
        'resources/views/errors/404.blade.php.tpl': not_found(cat),
        'resources/css/site.css.tpl': css,
        # même icône que Wagtail (ds/static/ds/favicon.svg) ; Laravel ne fournit qu'un favicon.ico vide
        'public/favicon.svg.tpl': open(os.path.join(ROOT_LAYERS, 'wagtail-ds/ds/static/ds/favicon.svg')).read(),
    }
    for t, nodes in TPL.SECTIONS.items():
        out[f'resources/views/site/sections/{t}.blade.php.tpl'] = GEN + Blade(cat).nodes(nodes)
    for n, body in CONTROLLERS.items(): out[f'app/Http/Controllers/{n}.php.tpl'] = body
    for n, body in MODELS.items(): out[f'app/Models/{n}.php.tpl'] = body
    for n, body in MIGRATIONS.items(): out[f'database/migrations/{n}.tpl'] = body
    # La couche n'est appliquée qu'avec --ui (voir src/scaffold/laravel.rs) : pas de condition par fichier.
    del gen
    return out
