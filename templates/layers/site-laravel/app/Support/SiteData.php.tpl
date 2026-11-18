<?php

namespace App\Support;

use App\Models\Post;
use App\Models\Service;
use Illuminate\Support\Facades\App;

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
                'phone_url' => 'tel:'.preg_replace('/\s+/', '', $s['contact']['phone']),
            ],
            'year' => (int) date('Y'),
            'copyright' => $s['copyright'],
            'copyright_line' => '© '.date('Y').' '.$s['name'].' · '.$s['copyright'],
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
