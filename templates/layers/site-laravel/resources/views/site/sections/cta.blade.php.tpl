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
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
            </x-ds.button>
        </div>
    </div>
</section>
