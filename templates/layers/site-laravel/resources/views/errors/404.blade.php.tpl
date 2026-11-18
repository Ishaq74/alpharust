{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$site['ui']['not_found']">
    <section class="site-section site-404">
        <div class="site-container site-container--narrow">
            <p class="site-404__code">
                404
            </p>
            <h1 class="site-page-header__title">
                {{ $site['ui']['not_found'] ?? '' }}
            </h1>
            <p class="site-lead">
                {{ $site['ui']['not_found_text'] ?? '' }}
            </p>
            <x-ds.button :href="$site['home_url'] ?? ''" variant="primary">
                <x-site.icon name="arrow-left" :size="16" />
                {{ $site['ui']['back_home'] ?? '' }}
            </x-ds.button>
        </div>
    </section>
</x-layouts.site>
