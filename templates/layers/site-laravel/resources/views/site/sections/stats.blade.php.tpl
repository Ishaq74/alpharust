{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--stats">
    <div class="site-container">
        <dl class="site-stats__grid">
            @foreach ($s['items'] ?? [] as $item)
                <div class="site-stat">
                    <dt class="site-stat__label">
                        {{ $item['label'] ?? '' }}
                    </dt>
                    <dd class="site-stat__value">
                        {{ $item['value'] ?? '' }}
                    </dd>
                </div>
            @endforeach
        </dl>
    </div>
</section>
