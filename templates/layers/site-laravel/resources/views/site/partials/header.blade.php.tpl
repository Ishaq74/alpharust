{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<header class="site-header">
    <div class="site-container site-header__bar">
        <a class="site-brand" href="{{ $site['home_url'] ?? '' }}">
            <span class="site-brand__mark" aria-hidden="true">
                {{ $site['initial'] ?? '' }}
            </span>
            <span class="site-brand__name">
                {{ $site['name'] ?? '' }}
            </span>
        </a>
        <nav class="site-nav" aria-label="Navigation principale">
            <ul class="site-nav__list">
                @foreach ($site['nav'] ?? [] as $item)
                    <li>
                        @if (! empty($item['children']))
                            <x-ds.dropdown :open-on-hover="true" class="site-nav__dropdown">
                                <x-ds.dropdown.trigger class="site-nav__link" :aria-current="$item['current'] ?? null">
                                    {{ $item['label'] ?? '' }}
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
                                </x-ds.dropdown.trigger>
                                <x-ds.dropdown.content class="site-nav__menu">
                                    <x-ds.dropdown.item as="a" :href="$item['url'] ?? ''">
                                        {{ $item['label'] ?? '' }}
                                    </x-ds.dropdown.item>
                                    @foreach ($item['children'] ?? [] as $child)
                                        <x-ds.dropdown.item as="a" :href="$child['url'] ?? ''" :aria-current="$child['current'] ?? null">
                                            {{ $child['label'] ?? '' }}
                                        </x-ds.dropdown.item>
                                    @endforeach
                                </x-ds.dropdown.content>
                            </x-ds.dropdown>
                        @else
                            <a class="site-nav__link" href="{{ $item['url'] ?? '' }}" @if (! empty($item['current']))aria-current="{{ $item['current'] }}"@endif>
                                {{ $item['label'] ?? '' }}
                            </a>
                        @endif
                    </li>
                @endforeach
            </ul>
        </nav>
        <div class="site-header__actions">
            <x-ds.button :href="$site['cta']['url'] ?? ''" variant="primary" size="sm" class="site-header__cta">
                {{ $site['cta']['label'] ?? '' }}
            </x-ds.button>
            <x-ds.theme-toggle size="sm" aria-label="Changer de thème" />
            <x-ds.sheet class="site-header__mobile">
                <x-ds.sheet.trigger :as-child="true">
                    <x-ds.button variant="ghost" size="icon" aria-label="Ouvrir le menu">
                        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 6h16M4 12h16M4 18h16"/></svg>
                    </x-ds.button>
                </x-ds.sheet.trigger>
                <x-ds.sheet.content side="right" class="site-mobile">
                    <x-ds.sheet.header>
                        <x-ds.sheet.title>
                            {{ $site['name'] ?? '' }}
                        </x-ds.sheet.title>
                    </x-ds.sheet.header>
                    <nav class="site-mobile__nav" aria-label="Navigation mobile">
                        <ul class="site-mobile__list">
                            @foreach ($site['nav'] ?? [] as $item)
                                <li>
                                    @if (! empty($item['children']))
                                        <x-ds.collapsible class="site-mobile__group">
                                            <x-ds.collapsible.trigger class="site-mobile__link">
                                                {{ $item['label'] ?? '' }}
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
                                            </x-ds.collapsible.trigger>
                                            <x-ds.collapsible.content>
                                                <ul class="site-mobile__sub">
                                                    <li>
                                                        <a href="{{ $item['url'] ?? '' }}">
                                                            {{ $item['label'] ?? '' }}
                                                        </a>
                                                    </li>
                                                    @foreach ($item['children'] ?? [] as $child)
                                                        <li>
                                                            <a href="{{ $child['url'] ?? '' }}" @if (! empty($child['current']))aria-current="{{ $child['current'] }}"@endif>
                                                                {{ $child['label'] ?? '' }}
                                                            </a>
                                                        </li>
                                                    @endforeach
                                                </ul>
                                            </x-ds.collapsible.content>
                                        </x-ds.collapsible>
                                    @else
                                        <a class="site-mobile__link" href="{{ $item['url'] ?? '' }}" @if (! empty($item['current']))aria-current="{{ $item['current'] }}"@endif>
                                            {{ $item['label'] ?? '' }}
                                        </a>
                                    @endif
                                </li>
                            @endforeach
                        </ul>
                    </nav>
                    <x-ds.sheet.footer>
                        <x-ds.button :href="$site['cta']['url'] ?? ''" variant="primary" class="site-mobile__cta">
                            {{ $site['cta']['label'] ?? '' }}
                        </x-ds.button>
                    </x-ds.sheet.footer>
                </x-ds.sheet.content>
            </x-ds.sheet>
        </div>
    </div>
</header>
