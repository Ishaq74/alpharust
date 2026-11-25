{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--posts">
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
                        <x-site.icon name="arrow-right" :size="16" />
                    </x-ds.button>
                @endif
            </div>
        @endif
        <div class="site-grid site-grid--3">
            @foreach ($s['items'] ?? [] as $item)
                <x-ds.card class="site-card">
                    <x-ds.card.header>
                        <div class="site-meta">
                            <x-ds.badge variant="outline">
                                {{ $item['category'] ?? '' }}
                            </x-ds.badge>
                            <time datetime="{{ $item['date_iso'] ?? '' }}">
                                {{ $item['date_label'] ?? '' }}
                            </time>
                        </div>
                        <h3 class="site-card__title">
                            <a href="{{ $item['url'] ?? '' }}">
                                {{ $item['title'] ?? '' }}
                            </a>
                        </h3>
                    </x-ds.card.header>
                    <x-ds.card.content>
                        <p class="site-card__text">
                            {{ $item['excerpt'] ?? '' }}
                        </p>
                    </x-ds.card.content>
                    <x-ds.card.footer>
                        <span class="site-muted">
                            {{ $item['author'] ?? '' }}
                        </span>
                    </x-ds.card.footer>
                </x-ds.card>
            @endforeach
        </div>
    </div>
</section>
