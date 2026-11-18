{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-page-header">
    <div class="site-container">
        @if (! empty($s['eyebrow']))
            <p class="site-eyebrow">
                {{ $s['eyebrow'] ?? '' }}
            </p>
        @endif
        <h1 class="site-page-header__title">
            {{ $s['title'] ?? '' }}
        </h1>
        @if (! empty($s['lead']))
            <p class="site-lead">
                {{ $s['lead'] ?? '' }}
            </p>
        @endif
    </div>
</section>
