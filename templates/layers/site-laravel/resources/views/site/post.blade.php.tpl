{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$page['title']" :description="$page['description']">
    <article class="site-article">
        <header class="site-page-header">
            <div class="site-container site-container--narrow">
                <x-ds.breadcrumb>
                    <x-ds.breadcrumb.list>
                        @foreach ($page['crumbs'] ?? [] as $crumb)
                            <x-ds.breadcrumb.item>
                                <x-ds.breadcrumb.link :href="$crumb['url'] ?? ''">
                                    {{ $crumb['label'] ?? '' }}
                                </x-ds.breadcrumb.link>
                            </x-ds.breadcrumb.item>
                            <x-ds.breadcrumb.separator />
                        @endforeach
                        <x-ds.breadcrumb.item>
                            <x-ds.breadcrumb.page>
                                {{ $page['title'] ?? '' }}
                            </x-ds.breadcrumb.page>
                        </x-ds.breadcrumb.item>
                    </x-ds.breadcrumb.list>
                </x-ds.breadcrumb>
                <x-ds.badge variant="outline">
                    {{ $page['category'] ?? '' }}
                </x-ds.badge>
                <h1 class="site-page-header__title">
                    {{ $page['title'] ?? '' }}
                </h1>
                <p class="site-meta">
                    <x-site.icon name="calendar" :size="16" />
                    <time datetime="{{ $page['date_iso'] ?? '' }}">
                        {{ $page['date_label'] ?? '' }}
                    </time>
                    <span aria-hidden="true">
                        ·
                    </span>
                    <span>
                        {{ $page['author'] ?? '' }}
                    </span>
                </p>
            </div>
        </header>
        <div class="site-section">
            <div class="site-container site-container--narrow">
                <x-ds.prose class="site-prose">
                    {!! $page['body'] ?? '' !!}
                </x-ds.prose>
                <p class="site-article__back">
                    <x-ds.button :href="$page['back_url'] ?? ''" variant="outline">
                        <x-site.icon name="arrow-left" :size="16" />
                        {{ $site['ui']['all_posts'] ?? '' }}
                    </x-ds.button>
                </p>
            </div>
        </div>
    </article>
</x-layouts.site>
