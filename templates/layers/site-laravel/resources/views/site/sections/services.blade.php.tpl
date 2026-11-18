{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--services">
    <div class="site-container">
        @if (! empty($s['title']))
            <div class="site-section__head">
                <div>
                    <h2 class="site-section__title">
                        {{ $s['title'] ?? '' }}
                    </h2>
                    @if (! empty($s['lead']))
                        <p class="site-lead">
                            {{ $s['lead'] ?? '' }}
                        </p>
                    @endif
                </div>
                @if (! empty($s['link']))
                    <x-ds.button :href="$s['link']['url'] ?? ''" variant="ghost">
                        {{ $s['link']['label'] ?? '' }}
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                    </x-ds.button>
                @endif
            </div>
        @endif
        <div class="site-grid site-grid--3">
            @foreach ($s['items'] ?? [] as $item)
                <x-ds.card class="site-card">
                    <x-ds.card.header>
                        <span class="site-icon-box">
                            <x-site.icon :name="$item['icon'] ?? ''" :size="22" />
                        </span>
                        <h3 class="site-card__title">
                            <a href="{{ $item['url'] ?? '' }}">
                                {{ $item['title'] ?? '' }}
                            </a>
                        </h3>
                    </x-ds.card.header>
                    <x-ds.card.content>
                        <p class="site-card__text">
                            {{ $item['summary'] ?? '' }}
                        </p>
                    </x-ds.card.content>
                    <x-ds.card.footer class="site-card__footer">
                        <x-ds.badge variant="outline">
                            {{ $item['price'] ?? '' }}
                        </x-ds.badge>
                        <x-ds.button :href="$item['url'] ?? ''" variant="outline" size="sm" :aria-label="$item['more_label'] ?? ''">
                            {{ $site['ui']['more'] ?? '' }}
                        </x-ds.button>
                    </x-ds.card.footer>
                </x-ds.card>
            @endforeach
        </div>
    </div>
</section>
