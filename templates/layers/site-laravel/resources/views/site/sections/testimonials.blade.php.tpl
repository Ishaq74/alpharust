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
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                    </x-ds.button>
                @endif
            </div>
        @endif
        <div class="site-grid site-grid--3">
            @foreach ($s['items'] ?? [] as $item)
                <x-ds.card class="site-card site-quote">
                    <x-ds.card.content>
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M16 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/><path d="M5 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/></svg>
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
