"""Rendu Laravel : layout (composant anonyme, comme manecms) + partials + routes/contrôleurs + modèles/migrations/seeder."""
import os
from . import data as D, templates as TPL, i18n as I18N
from .ir import Blade
from .icons import icons_json

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
    s = {'locale': D.DEFAULT, 'locales': {c: {'name': n, 'label': l} for c, n, l in I18N.LOCALES},
         # Comptes (--auth, Laravel Fortify) : mêmes adresses et champs qu'Astro et Wagtail.
         'auth': {'routes': D.AUTH_ROUTES, 'fields': {k: [{'name': n, 'type': t, 'autocomplete': a} for n, t, a in v]
                                                     for k, v in D.AUTH_FIELDS.items()}, 'password_min': D.PASSWORD_MIN}}
    return ("<?php\n\n" + PHPGEN + "// Langues du site (localisation officielle de Laravel, doc « localization »)\n"
            "// Le contenu (navigation, pied de page, pages, libellés) est dans lang/<langue>/site.php ; les services et\n"
            "// les articles sont en base, une ligne par langue (modèles Service et Post, database/seeders/SiteSeeder.php).\n\n"
            "return " + php(s) + ";\n")

def lang_file(loc):
    L = D.L[loc]
    s = {k: L.SITE[k] for k in ('name', 'tagline', 'description', 'cta', 'contact', 'social', 'nav', 'footer', 'copyright')}
    s['pages'] = L.PAGES
    s['contact_fields'] = L.FIELDS
    s['ui'] = L.UI
    return ("<?php\n\n" + PHPGEN + f"// Textes du site en « {loc} » : lus par App\\Support\\SiteData via trans('site').\n\n"
            "return " + php(s) + ";\n")

ICON = (GEN + "{{-- Icônes du site : resources/data/icons.json (MDI par défaut, drapeaux Circle Flags), comme Astro et Django. --}}\n"
        "@props(['name', 'size' => 20])\n@php($icon = \\App\\Support\\SiteData::icon($name))\n"
        "@if ($icon)\n<svg width=\"{{ $size }}\" height=\"{{ $size }}\" viewBox=\"0 0 {{ $icon['width'] }} {{ $icon['height'] }}\" "
        "fill=\"currentColor\" aria-hidden=\"true\" class=\"site-icon\">{!! $icon['body'] !!}</svg>\n@endif\n")

LAYOUT = GEN + """{{-- Gabarit commun du site (modèle manecms : composant « shell » + partials head/header/footer). --}}
@props(['title' => null, 'description' => null])
<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    @include('site.partials.head', ['title' => $title, 'description' => $description])
</head>
<body class="site-body">
    <a class="site-skip" href="#contenu">{{ $site['ui']['skip'] }}</a>
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
    return GEN + '<x-layouts.site :title="$site[\'ui\'][\'not_found\']">\n' + Blade(cat).nodes(TPL.NOT_FOUND, 1) + '</x-layouts.site>\n'

SITE_DATA = """<?php

namespace App\\Support;

use App\\Models\\Post;
use App\\Models\\Service;
use Illuminate\\Support\\Facades\\App;

/**
 * Données normalisées du site : mêmes clés que src/lib/site.ts (Astro) et layout/context_processors.py (Wagtail),
 * si bien que les vues Blade produisent le même balisage.
 */
final class SiteData
{
    /** Langue d'une adresse : `/en/…` -> en, sinon la langue par défaut (config/site.php). */
    public static function localeOf(string $path): string
    {
        $first = explode('/', trim($path, '/'))[0];

        return $first !== config('site.locale') && array_key_exists($first, config('site.locales')) ? $first : config('site.locale');
    }

    /** Adresse interne au format Laravel (sans barre finale), préfixée par la langue hors langue par défaut. */
    public static function href(string $url, ?string $locale = null): string
    {
        if (preg_match('/^[a-z]+:/', $url)) {
            return $url;
        }
        $locale ??= App::getLocale();
        $prefix = $locale === config('site.locale') ? '' : '/'.$locale;
        $path = $url === '/' ? '' : '/'.trim($url, '/');

        return ($prefix.$path) ?: '/';
    }

    /** Adresse sans préfixe de langue (`/en/blog` -> `/blog`). */
    public static function unprefixed(string $path): string
    {
        $path = '/'.trim($path, '/');
        $locale = self::localeOf($path);

        return $locale === config('site.locale') ? $path : (substr($path, strlen($locale) + 1) ?: '/');
    }

    /** 'page' (page exacte), 'true' (rubrique) ou null — même règle que les autres frameworks. */
    public static function current(string $url, string $path): ?string
    {
        $u = $url === '/' ? '/' : '/'.trim($url, '/');
        $p = '/'.trim($path, '/');
        if ($p === $u) {
            return 'page';
        }

        return $u !== self::href('/') && str_starts_with($p.'/', $u.'/') ? 'true' : null;
    }

    /** @var array<string, array{body: string, width: int, height: int}>|null */
    private static ?array $icons = null;

    /**
     * Icône du site par son nom logique (resources/data/icons.json, partagé avec Astro et Wagtail).
     *
     * @return array{body: string, width: int, height: int}|null
     */
    public static function icon(string $name): ?array
    {
        self::$icons ??= json_decode((string) file_get_contents(resource_path('data/icons.json')), true)['icons'];

        return self::$icons[$name] ?? null;
    }

    /** @return list<array<string, mixed>> */
    public function services(): array
    {
        return Service::query()->where('locale', App::getLocale())->orderBy('position')->get()->map(fn (Service $s) => $s->toCard())->all();
    }

    /** @return list<array<string, mixed>> */
    public function posts(): array
    {
        return Post::query()->where('locale', App::getLocale())->orderByDesc('published_at')->get()->map(fn (Post $p) => $p->toCard())->all();
    }

    /** @return array<string, mixed> */
    public function site(string $path): array
    {
        $s = trans('site');
        $locale = App::getLocale();
        $rest = self::unprefixed($path);
        $services = array_map(fn (array $x) => [
            'label' => $x['title'], 'url' => $x['url'], 'current' => self::current($x['url'], $path),
        ], $this->services());
        $links = fn ($v) => $v === 'services' ? $services : array_map(
            fn (array $l) => ['label' => $l['label'], 'url' => self::href($l['url'])], $v
        );

        return [
            'locale' => $locale,
            'languages' => array_map(fn (string $code, array $l) => [
                'code' => $code,
                'label' => $l['label'],
                'name' => $l['name'],
                'flag' => 'flag-'.$code,
                'url' => self::href($rest, $code),
                'current' => $code === $locale ? 'page' : null,
            ], array_keys(config('site.locales')), config('site.locales')),
            'ui' => $s['ui'],
            'name' => $s['name'],
            'initial' => mb_strtoupper(mb_substr($s['name'], 0, 1)),
            'tagline' => $s['tagline'],
            'description' => $s['description'],
            'home_url' => self::href('/'),
            'cta' => ['label' => $s['cta']['label'], 'url' => self::href($s['cta']['url'])],
            'nav' => array_map(fn (array $i) => [
                'label' => $i['label'],
                'url' => self::href($i['url']),
                'current' => self::current(self::href($i['url']), $path),
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
            'copyright_line' => '© '.date('Y').' '.$s['name'].' · '.$s['copyright'],
@@if auth@@
            'account' => auth()->check()
                ? ['url' => self::href(config('site.auth.routes.account')), 'label' => $s['ui']['account']]
                : ['url' => self::href(config('site.auth.routes.sign_in')), 'label' => $s['ui']['sign_in']],
@@else@@
            'account' => null,
@@end@@
        ];
    }
@@if auth@@

    /**
     * Pages connexion / inscription (mêmes données qu'Astro et Wagtail) ; le mot de passe n'est jamais ré-affiché.
     *
     * @param  array<string, mixed>  $values
     * @return array<string, mixed>
     */
    public static function authPage(string $kind, ?string $error = null, array $values = []): array
    {
        $ui = trans('site.ui');
        $other = $kind === 'sign_in' ? 'sign_up' : 'sign_in';

        return [
            'title' => $ui[$kind],
            'lead' => $ui[$kind.'_lead'],
            'submit' => $ui[$kind.'_submit'],
            'error' => $error,
            // Formulaire envoyé aux routes POST de Fortify (config/fortify.php : paths).
            'action' => url(config('site.auth.routes.'.$kind)),
            'fields' => array_map(fn (array $f) => $f + [
                'id' => 'auth-'.$f['name'],
                'label' => $ui['field_'.$f['name']],
                'value' => $f['type'] === 'password' ? '' : (string) ($values[$f['name']] ?? ''),
            ], config('site.auth.fields.'.$kind)),
            'alt_text' => $ui[$kind === 'sign_in' ? 'no_account' : 'have_account'],
            'alt_label' => $ui[$other],
            'alt_url' => self::href(config('site.auth.routes.'.$other)),
        ];
    }

    /** @return array<string, mixed> */
    public static function accountPage(\\App\\Models\\User $user): array
    {
        return [
            'title' => trans('site.ui.account'),
            'lead' => trans('site.ui.account_lead'),
            'name' => $user->name,
            'email' => $user->email,
            'logout_action' => url('/auth/sign-out'),
        ];
    }
@@end@@

    /**
     * Sections d'une page avec listes et formulaire résolus.
     *
     * @param  array{sent?: bool, values?: array<string, mixed>, errors?: array<string, string>}  $state
     * @return array<string, mixed>
     */
    public function page(string $key, array $state = []): array
    {
        $page = trans("site.pages.$key");
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
                    }, trans('site.contact_fields')),
                ],
                default => $s,
            };
        }, $page['sections']);

        return $page;
    }

    /** @return list<array{label: string, url: string}> */
    public static function crumbs(string $label, string $url): array
    {
        return [['label' => trans('site.ui.home'), 'url' => self::href('/')], ['label' => $label, 'url' => self::href($url)]];
    }
}
"""

COMPOSER = """<?php

namespace App\\View\\Composers;

use App\\Support\\SiteData;
use Illuminate\\Support\\Facades\\App;
use Illuminate\\View\\View;

/** Partage `$site` (navigation, pied de page, coordonnées) avec le gabarit et les vues du site. */
class SiteComposer
{
    public function __construct(private SiteData $data) {}

    public function compose(View $view): void
    {
        // Les pages d'erreur (404) ne passent pas par le middleware de langue : la langue suit l'adresse.
        App::setLocale(SiteData::localeOf(request()->path()));
        $view->with('site', $this->data->site(request()->path()));
    }
}
"""

PROVIDER = """<?php

namespace App\\Providers;

@@if auth@@
use App\\Actions\\Fortify\\CreateNewUser;
@@end@@
use App\\Http\\Middleware\\SetSiteLocale;
@@if auth@@
use App\\Http\\Responses\\SiteAuthResponse;
@@end@@
use App\\View\\Composers\\SiteComposer;
use Illuminate\\Support\\Facades\\App;
use Illuminate\\Support\\Facades\\View;
use Illuminate\\Support\\ServiceProvider;
@@if auth@@
use Laravel\\Fortify\\Contracts\\LoginResponse;
use Laravel\\Fortify\\Contracts\\LogoutResponse;
use Laravel\\Fortify\\Contracts\\RegisterResponse;
use Laravel\\Fortify\\Fortify;
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
use Illuminate\\Support\\Facades\\App;
use Illuminate\\View\\View;

class ServiceController extends Controller
{
    public function show(string $slug): View
    {
        $service = Service::query()->where('locale', App::getLocale())->where('slug', $slug)->firstOrFail();

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
use Illuminate\\Support\\Facades\\App;
use Illuminate\\View\\View;

class BlogController extends Controller
{
    public function show(string $slug): View
    {
        $post = Post::query()->where('locale', App::getLocale())->where('slug', $slug)->firstOrFail();

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
        foreach (trans('site.contact_fields') as $f) {
            $rules[$f['name']] = array_filter([
                $f['required'] ? 'required' : 'nullable', 'string', 'max:5000', $f['type'] === 'email' ? 'email' : null,
            ]);
        }
        $data = $request->validate($rules, [
            'required' => trans('site.ui.required'),
            'email' => trans('site.ui.invalid_email'),
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
    protected $fillable = ['locale', 'slug', 'title', 'summary', 'icon', 'price', 'features', 'body', 'position'];

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
            'more_label' => trans('site.ui.more_label').$this->title,
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
    protected $fillable = ['locale', 'slug', 'title', 'excerpt', 'category', 'author', 'published_at', 'body'];

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
            'date_label' => $this->published_at->locale($this->locale)->translatedFormat($this->locale === 'en' ? 'F j, Y' : 'j F Y'),
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
            $table->string('locale', 8)->default('fr');
            $table->string('slug');
            $table->string('title');
            $table->text('summary');
            $table->string('icon');
            $table->string('price');
            $table->json('features');
            $table->longText('body');
            $table->unsignedInteger('position')->default(0);
            $table->timestamps();
            $table->unique(['locale', 'slug']);
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
            $table->string('locale', 8)->default('fr');
            $table->string('slug');
            $table->string('title');
            $table->text('excerpt');
            $table->string('category');
            $table->string('author');
            $table->date('published_at');
            $table->longText('body');
            $table->timestamps();
            $table->unique(['locale', 'slug']);
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
    services = [{'locale': loc, 'slug': s['slug'], 'title': s['title'], 'summary': s['summary'], 'icon': s['icon'], 'price': s['price'],
                 'features': s['features'], 'body': s['body_html'], 'position': s['order']} for loc, L in D.L.items() for s in L.SERVICES]
    posts = [{'locale': loc, 'slug': p['slug'], 'title': p['title'], 'excerpt': p['excerpt'], 'category': p['category'],
              'author': p['author'], 'published_at': p['date'], 'body': p['body_html']} for loc, L in D.L.items() for p in L.POSTS]
    return ("<?php\n\nnamespace Database\\Seeders;\n\nuse App\\Models\\Post;\nuse App\\Models\\Service;\nuse Illuminate\\Database\\Seeder;\n\n"
            + PHPGEN + "class SiteSeeder extends Seeder\n{\n    public function run(): void\n    {\n"
            "        foreach (" + php(services, 2) + " as $s) {\n            Service::updateOrCreate(['locale' => $s['locale'], 'slug' => $s['slug']], $s);\n        }\n"
            "        foreach (" + php(posts, 2) + " as $p) {\n            Post::updateOrCreate(['locale' => $p['locale'], 'slug' => $p['slug']], $p);\n        }\n    }\n}\n")

ROUTES = """@@meta mode=replace@@
<?php

@@if auth@@
use App\\Http\\Controllers\\AccountController;
@@end@@
use App\\Http\\Controllers\\BlogController;
use App\\Http\\Controllers\\ContactController;
use App\\Http\\Controllers\\ServiceController;
use App\\Http\\Controllers\\SiteController;
use Illuminate\\Support\\Facades\\Route;

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

    public static function englishPages(): array
    {
        return [
            'home' => ['/en', 'Fast, accessible websites'],
            'about' => ['/en/a-propos', 'A small team'],
            'service' => ['/en/services/accessibilite', 'What’s included'],
            'article' => ['/en/blog/choisir-son-framework', 'All articles'],
            'contact' => ['/en/contact', 'Let’s talk about your project'],
        ];
    }

    /** @dataProvider englishPages */
    #[\\PHPUnit\\Framework\\Attributes\\DataProvider('englishPages')]
    public function test_english_pages_are_translated(string $url, string $text): void
    {
        $this->get($url)
            ->assertOk()
            ->assertSee('<html lang="en">', false)
            ->assertSee($text)
            ->assertSee('aria-label="Main navigation"', false)
            ->assertSee('hreflang="fr"', false);
    }

    public function test_the_language_switcher_links_to_the_same_page(): void
    {
        $this->get('/blog')->assertSee('href="/en/blog"', false);
        $this->get('/en/blog')->assertSee('href="/blog"', false)->assertSee('href="/en/blog" aria-current="page"', false);
        $this->get('/en/nope')->assertNotFound()->assertSee('Page not found');
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

MIDDLEWARE = """<?php

namespace App\\Http\\Middleware;

use Closure;
use Illuminate\\Http\\Request;
use Illuminate\\Support\\Facades\\App;
use Symfony\\Component\\HttpFoundation\\Response;

/** Langue d'un groupe de routes du site (`site.locale:en`) : App::setLocale, comme le recommande la doc Laravel. */
class SetSiteLocale
{
    public function handle(Request $request, Closure $next, string $locale): Response
    {
        App::setLocale($locale);

        return $next($request);
    }
}
"""


AUTH_META = '@@meta when=auth@@\n'

FORTIFY_CONFIG = AUTH_META + """<?php

use Laravel\\Fortify\\Features;

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
"""

CREATE_USER = AUTH_META + """<?php

namespace App\\Actions\\Fortify;

use App\\Models\\User;
use Illuminate\\Support\\Facades\\Hash;
use Illuminate\\Validation\\ValidationException;
use Laravel\\Fortify\\Contracts\\CreatesNewUsers;

/** Inscription (Fortify) : mêmes contrôles et messages qu'Astro (Better Auth) et Wagtail, dans le même ordre. */
class CreateNewUser implements CreatesNewUsers
{
    /** @param  array<string, mixed>  $input */
    public function create(array $input): User
    {
        $value = fn (string $k) => trim((string) ($input[$k] ?? ''));
        $fail = fn (string $key) => throw ValidationException::withMessages(['form' => trans('site.ui.'.$key)]);
        foreach (config('site.auth.fields.sign_up') as $f) {
            if ($value($f['name']) === '') {
                $fail('err_required');
            }
        }
        if (! filter_var($value('email'), FILTER_VALIDATE_EMAIL)) {
            $fail('invalid_email');
        }
        if (mb_strlen((string) $input['password']) < config('site.auth.password_min')) {
            $fail('err_password');
        }
        $email = mb_strtolower($value('email'));
        if (User::query()->where('email', $email)->exists()) {
            $fail('err_exists');
        }

        return User::create([
            'name' => $value('name'),
            'email' => $email,
            'password' => Hash::make((string) $input['password']),
        ]);
    }
}
"""

AUTH_RESPONSE = AUTH_META + """<?php

namespace App\\Http\\Responses;

use App\\Support\\SiteData;
use Illuminate\\Http\\RedirectResponse;
use Laravel\\Fortify\\Contracts\\LoginResponse;
use Laravel\\Fortify\\Contracts\\LogoutResponse;
use Laravel\\Fortify\\Contracts\\RegisterResponse;

/** Réponses de Fortify : « Mon compte » après connexion ou inscription, l'accueil après déconnexion, dans la langue du formulaire. */
class SiteAuthResponse implements LoginResponse, LogoutResponse, RegisterResponse
{
    public function toResponse($request): RedirectResponse
    {
        $locale = (string) $request->input('locale');
        $locale = array_key_exists($locale, config('site.locales')) ? $locale : config('site.locale');
        $to = $request->routeIs('logout') ? '/' : config('site.auth.routes.account');

        return redirect(SiteData::href($to, $locale));
    }
}
"""

ACCOUNT_CONTROLLER = AUTH_META + """<?php

namespace App\\Http\\Controllers;

use App\\Models\\User;
use App\\Support\\SiteData;
use Illuminate\\Http\\RedirectResponse;
use Illuminate\\Http\\Request;
use Illuminate\\View\\View;

/** Pages de compte du site ; les formulaires sont traités par Fortify (config/fortify.php). */
class AccountController extends Controller
{
    public function signIn(Request $request): View|RedirectResponse
    {
        if ($request->user()) {
            return redirect(SiteData::href(config('site.auth.routes.account')));
        }
        // Erreur de Fortify (champ manquant, identifiants refusés) -> libellé du site, comme Astro et Wagtail.
        $bag = $request->session()->get('errors')?->getBag('default');
        $error = null;
        if ($bag && $bag->any()) {
            $email = trim((string) $request->old('email'));
            $error = trans(match (true) {
                $email === '' || $bag->has('password') => 'site.ui.err_required',
                ! filter_var($email, FILTER_VALIDATE_EMAIL) => 'site.ui.invalid_email',
                default => 'site.ui.err_credentials',
            });
        }

        return view('site.auth', ['page' => SiteData::authPage('sign_in', $error, $request->old())]);
    }

    public function signUp(Request $request): View|RedirectResponse
    {
        if ($request->user()) {
            return redirect(SiteData::href(config('site.auth.routes.account')));
        }
        $bag = $request->session()->get('errors')?->getBag('default');

        return view('site.auth', ['page' => SiteData::authPage('sign_up', $bag?->first() ?: null, $request->old())]);
    }

    public function show(Request $request): View|RedirectResponse
    {
        $user = $request->user();
        if (! $user instanceof User) {
            return redirect(SiteData::href(config('site.auth.routes.sign_in')));
        }

        return view('site.account', ['page' => SiteData::accountPage($user)]);
    }
}
"""

AUTH_TEST = AUTH_META + """<?php

namespace Tests\\Feature;

use App\\Models\\User;
use Illuminate\\Foundation\\Testing\\RefreshDatabase;
use Tests\\TestCase;

/** Comptes du site (Fortify) : mêmes pages, adresses et messages qu'Astro et Wagtail. */
class AuthTest extends TestCase
{
    use RefreshDatabase;

    public function test_the_header_links_to_the_sign_in_page(): void
    {
        $this->get('/')->assertOk()->assertSee('href="/auth/sign-in"', false)->assertSee('Connexion');
        $this->get('/en')->assertOk()->assertSee('href="/en/auth/sign-in"', false)->assertSee('Sign in');
    }

    public function test_auth_pages_render_in_the_site_layout(): void
    {
        $this->get('/auth/sign-in')->assertOk()->assertSee('Accédez à votre espace client')->assertSee('site-header', false);
        $this->get('/auth/sign-up')->assertOk()->assertSee('Créer mon compte')->assertSee('autocomplete="new-password"', false);
        $this->get('/en/auth/sign-up')->assertOk()->assertSee('Create my account');
        $this->get('/auth/dashboard')->assertRedirect('/auth/sign-in');
    }

    public function test_sign_up_sign_out_and_sign_in(): void
    {
        $this->post('/auth/sign-up', ['name' => 'Ada', 'email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'fr'])
            ->assertRedirect('/auth/dashboard');
        $this->assertAuthenticated();
        $this->get('/auth/dashboard')->assertOk()->assertSee('ada@example.com')->assertSee('href="/auth/dashboard"', false);
        $this->post('/auth/sign-out', ['locale' => 'en'])->assertRedirect('/en');
        $this->assertGuest();
        $this->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'en'])
            ->assertRedirect('/en/auth/dashboard');
        $this->assertAuthenticated();
    }

    public function test_errors_use_the_site_messages(): void
    {
        User::factory()->create(['email' => 'ada@example.com']);
        $this->from('/auth/sign-in')->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'mauvais'])
            ->assertRedirect('/auth/sign-in');
        $this->get('/auth/sign-in')->assertSee('E-mail ou mot de passe incorrect.');
        $this->from('/auth/sign-up')->post('/auth/sign-up', ['name' => 'Ada', 'email' => 'ada@example.com', 'password' => 'motdepasse']);
        $this->get('/auth/sign-up')->assertSee('Un compte existe déjà avec cette adresse.');
        $this->from('/auth/sign-up')->post('/auth/sign-up', ['name' => 'Bob', 'email' => 'bob@example.com', 'password' => 'court']);
        $this->get('/auth/sign-up')->assertSee('Le mot de passe doit contenir au moins 8 caractères.');
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
        'app/Http/Middleware/SetSiteLocale.php.tpl': MIDDLEWARE,
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
        'resources/data/icons.json.tpl': icons_json(),
        # même icône que Wagtail (ds/static/ds/favicon.svg) ; Laravel ne fournit qu'un favicon.ico vide
        'public/favicon.svg.tpl': open(os.path.join(ROOT_LAYERS, 'wagtail-ds/ds/static/ds/favicon.svg')).read(),
    }
    for loc in D.LOCALES:
        out[f'lang/{loc}/site.php.tpl'] = lang_file(loc)
    for t, nodes in TPL.SECTIONS.items():
        out[f'resources/views/site/sections/{t}.blade.php.tpl'] = GEN + Blade(cat).nodes(nodes)
    for n, body in CONTROLLERS.items(): out[f'app/Http/Controllers/{n}.php.tpl'] = body
    # Comptes (--auth) : Laravel Fortify.
    out.update({
        'config/fortify.php.tpl': FORTIFY_CONFIG,
        'app/Actions/Fortify/CreateNewUser.php.tpl': CREATE_USER,
        'app/Http/Responses/SiteAuthResponse.php.tpl': AUTH_RESPONSE,
        'app/Http/Controllers/AccountController.php.tpl': ACCOUNT_CONTROLLER,
        'tests/Feature/AuthTest.php.tpl': AUTH_TEST,
        'resources/views/site/auth.blade.php.tpl': AUTH_META + "{{-- Connexion / inscription (comptes : --auth). --}}\n" + GEN
            + "<x-layouts.site :title=\"$page['title']\">\n" + Blade(cat).nodes(TPL.AUTH_FORM, 1) + "</x-layouts.site>\n",
        'resources/views/site/account.blade.php.tpl': AUTH_META + GEN
            + "<x-layouts.site :title=\"$page['title']\">\n" + Blade(cat).nodes(TPL.ACCOUNT, 1) + "</x-layouts.site>\n",
    })
    for n, body in MODELS.items(): out[f'app/Models/{n}.php.tpl'] = body
    for n, body in MIGRATIONS.items(): out[f'database/migrations/{n}.tpl'] = body
    # La couche n'est appliquée qu'avec --ui (voir src/scaffold/laravel.rs) : pas de condition par fichier.
    del gen
    return out
