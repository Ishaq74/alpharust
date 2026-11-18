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
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>
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
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                        Tous les articles
                    </x-ds.button>
                </p>
            </div>
        </div>
    </article>
</x-layouts.site>
