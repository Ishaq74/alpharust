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
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                    </x-ds.button>
                @endif
            </div>
        @endif
        <x-ds.accordion type="single" class="site-faq__list">
            @foreach ($s['items'] ?? [] as $item)
                <x-ds.accordion.item :value="$item['question'] ?? ''">
                    <x-ds.accordion.trigger>
                        {{ $item['question'] ?? '' }}
                    </x-ds.accordion.trigger>
                    <x-ds.accordion.content>
                        <p>
                            {{ $item['answer'] ?? '' }}
                        </p>
                    </x-ds.accordion.content>
                </x-ds.accordion.item>
            @endforeach
        </x-ds.accordion>
    </div>
</section>
