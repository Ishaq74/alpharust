{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--faq">
    <div class="site-container site-container--narrow">
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
        <div class="site-faq__list">
            @foreach ($s['items'] ?? [] as $item)
                <details class="site-faq__item" name="faq">
                    <summary class="site-faq__question">
                        <span>
                            {{ $item['question'] ?? '' }}
                        </span>
                        <x-site.icon name="chevron-down" :size="18" />
                    </summary>
                    <div class="site-faq__answer">
                        <p>
                            {{ $item['answer'] ?? '' }}
                        </p>
                    </div>
                </details>
            @endforeach
        </div>
    </div>
</section>
