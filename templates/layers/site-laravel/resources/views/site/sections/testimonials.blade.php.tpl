{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--testimonials">
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
                <x-ds.card class="site-card site-quote">
                    <x-ds.card.content>
                        <x-site.icon name="quote" :size="24" />
                        <blockquote class="site-quote__text">
                            <p>
                                {{ $item['quote'] ?? '' }}
                            </p>
                        </blockquote>
                    </x-ds.card.content>
                    <x-ds.card.footer class="site-person">
                        <x-ds.avatar size="md">
                            <x-ds.avatar.fallback>
                                {{ $item['initials'] ?? '' }}
                            </x-ds.avatar.fallback>
                        </x-ds.avatar>
                        <div>
                            <p class="site-person__name">
                                {{ $item['name'] ?? '' }}
                            </p>
                            <p class="site-person__role">
                                {{ $item['role'] ?? '' }}
                            </p>
                        </div>
                    </x-ds.card.footer>
                </x-ds.card>
            @endforeach
        </div>
    </div>
</section>
