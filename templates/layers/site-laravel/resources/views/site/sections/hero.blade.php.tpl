{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-hero">
    <div class="site-container site-hero__inner">
        @if (! empty($s['badge']))
            <x-ds.badge variant="primary">
                {{ $s['badge'] ?? '' }}
            </x-ds.badge>
        @endif
        <h1 class="site-hero__title">
            {{ $s['title'] ?? '' }}
        </h1>
        <p class="site-hero__lead">
            {{ $s['lead'] ?? '' }}
        </p>
        <div class="site-hero__actions">
            <x-ds.button :href="$s['primary']['url'] ?? ''" variant="primary" size="lg">
                {{ $s['primary']['label'] ?? '' }}
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
            </x-ds.button>
            @if (! empty($s['secondary']))
                <x-ds.button :href="$s['secondary']['url'] ?? ''" variant="outline" size="lg">
                    {{ $s['secondary']['label'] ?? '' }}
                </x-ds.button>
            @endif
        </div>
    </div>
</section>
