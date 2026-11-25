{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--team">
    <div class="site-container">
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
        <ul class="site-grid site-grid--4 site-team">
            @foreach ($s['items'] ?? [] as $item)
                <li class="site-team__member">
                    <x-ds.avatar size="lg">
                        <x-ds.avatar.fallback>
                            {{ $item['initials'] ?? '' }}
                        </x-ds.avatar.fallback>
                    </x-ds.avatar>
                    <p class="site-person__name">
                        {{ $item['name'] ?? '' }}
                    </p>
                    <p class="site-person__role">
                        {{ $item['role'] ?? '' }}
                    </p>
                </li>
            @endforeach
        </ul>
    </div>
</section>
