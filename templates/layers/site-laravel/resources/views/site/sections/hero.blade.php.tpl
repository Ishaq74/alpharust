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
                <x-site.icon name="arrow-right" :size="18" />
            </x-ds.button>
            @if (! empty($s['secondary']))
                <x-ds.button :href="$s['secondary']['url'] ?? ''" variant="outline" size="lg">
                    {{ $s['secondary']['label'] ?? '' }}
                </x-ds.button>
            @endif
        </div>
    </div>
</section>
