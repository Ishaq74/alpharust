{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--text">
    <div class="site-container site-container--narrow">
        @if (! empty($s['title']))
            <h2 class="site-section__title">
                {{ $s['title'] ?? '' }}
            </h2>
        @endif
        <x-ds.prose class="site-prose">
            {!! $s['body'] ?? '' !!}
        </x-ds.prose>
    </div>
</section>
