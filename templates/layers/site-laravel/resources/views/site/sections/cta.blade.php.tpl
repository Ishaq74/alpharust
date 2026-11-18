{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--cta">
    <div class="site-container">
        <div class="site-cta">
            <div>
                <h2 class="site-cta__title">
                    {{ $s['title'] ?? '' }}
                </h2>
                <p class="site-cta__text">
                    {{ $s['text'] ?? '' }}
                </p>
            </div>
            <x-ds.button :href="$s['action']['url'] ?? ''" variant="default" size="lg">
                {{ $s['action']['label'] ?? '' }}
                <x-site.icon name="arrow-right" :size="18" />
            </x-ds.button>
        </div>
    </div>
</section>
