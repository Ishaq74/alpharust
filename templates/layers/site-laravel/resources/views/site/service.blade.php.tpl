{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$page['title']" :description="$page['description']">
    <section class="site-page-header">
        <div class="site-container">
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
            <span class="site-icon-box site-icon-box--lg">
                <x-site.icon :name="$page['icon'] ?? ''" :size="28" />
            </span>
            <h1 class="site-page-header__title">
                {{ $page['title'] ?? '' }}
            </h1>
            <p class="site-lead">
                {{ $page['summary'] ?? '' }}
            </p>
            <x-ds.badge variant="primary" size="lg">
                {{ $page['price'] ?? '' }}
            </x-ds.badge>
        </div>
    </section>
    <section class="site-section">
        <div class="site-container site-detail">
            <x-ds.prose class="site-prose site-detail__body">
                {!! $page['body'] ?? '' !!}
            </x-ds.prose>
            <aside class="site-detail__aside">
                <x-ds.card class="site-card">
                    <x-ds.card.header>
                        <h2 class="site-card__title">
                            {{ $site['ui']['included'] ?? '' }}
                        </h2>
                    </x-ds.card.header>
                    <x-ds.card.content>
                        <ul class="site-checklist">
                            @foreach ($page['features'] ?? [] as $feature)
                                <li>
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M20 6 9 17l-5-5"/></svg>
                                    <span>
                                        {{ $feature }}
                                    </span>
                                </li>
                            @endforeach
                        </ul>
                    </x-ds.card.content>
                    <x-ds.card.footer>
                        <x-ds.button :href="$site['cta']['url'] ?? ''" variant="primary" class="site-block">
                            {{ $site['cta']['label'] ?? '' }}
                        </x-ds.button>
                    </x-ds.card.footer>
                </x-ds.card>
            </aside>
        </div>
    </section>
</x-layouts.site>
