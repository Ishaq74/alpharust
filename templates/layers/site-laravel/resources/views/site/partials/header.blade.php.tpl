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
        <nav class="site-nav" aria-label="{{ $site['ui']['main_nav'] ?? '' }}">
            <ul class="site-nav__list">
                @foreach ($site['nav'] ?? [] as $item)
                    <li>
                        @if (! empty($item['children']))
                            <x-ds.dropdown :open-on-hover="true" class="site-nav__dropdown">
                                <x-ds.dropdown.trigger class="site-nav__link" :aria-current="$item['current'] ?? null">
                                    {{ $item['label'] ?? '' }}
                                    <x-site.icon name="chevron-down" :size="16" />
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
            @if (! empty($site['account']))
                <x-ds.button :href="$site['account']['url'] ?? ''" variant="ghost" size="sm" class="site-header__account">
                    <x-site.icon name="user" :size="18" />
                    <span>
                        {{ $site['account']['label'] ?? '' }}
                    </span>
                </x-ds.button>
            @endif
            <x-ds.button :href="$site['cta']['url'] ?? ''" variant="primary" size="sm" class="site-header__cta">
                {{ $site['cta']['label'] ?? '' }}
            </x-ds.button>
            <nav class="site-lang" aria-label="{{ $site['ui']['language'] ?? '' }}">
                <ul class="site-lang__list">
                    @foreach ($site['languages'] ?? [] as $lang)
                        <li>
                            <a class="site-lang__link" href="{{ $lang['url'] ?? '' }}" hreflang="{{ $lang['code'] ?? '' }}" lang="{{ $lang['code'] ?? '' }}" @if (! empty($lang['current']))aria-current="{{ $lang['current'] }}"@endif title="{{ $lang['name'] ?? '' }}">
                                <x-site.icon :name="$lang['flag'] ?? ''" :size="16" />
                                <span>
                                    {{ $lang['label'] ?? '' }}
                                </span>
                            </a>
                        </li>
                    @endforeach
                </ul>
            </nav>
            <x-ds.theme-toggle size="sm" :aria-label="$site['ui']['theme'] ?? ''" />
            <x-ds.sheet class="site-header__mobile">
                <x-ds.sheet.trigger :as-child="true">
                    <x-ds.button variant="ghost" size="icon" :aria-label="$site['ui']['open_menu'] ?? ''">
                        <x-site.icon name="menu" :size="22" />
                    </x-ds.button>
                </x-ds.sheet.trigger>
                <x-ds.sheet.content side="right" class="site-mobile">
                    <x-ds.sheet.header>
                        <x-ds.sheet.title>
                            {{ $site['name'] ?? '' }}
                        </x-ds.sheet.title>
                    </x-ds.sheet.header>
                    <nav class="site-mobile__nav" aria-label="{{ $site['ui']['mobile_nav'] ?? '' }}">
                        <ul class="site-mobile__list">
                            @foreach ($site['nav'] ?? [] as $item)
                                <li>
                                    @if (! empty($item['children']))
                                        <x-ds.collapsible class="site-mobile__group">
                                            <x-ds.collapsible.trigger class="site-mobile__link">
                                                {{ $item['label'] ?? '' }}
                                                <x-site.icon name="chevron-down" :size="16" />
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
                        @if (! empty($site['account']))
                            <x-ds.button :href="$site['account']['url'] ?? ''" variant="outline" class="site-mobile__cta">
                                <x-site.icon name="user" :size="18" />
                                {{ $site['account']['label'] ?? '' }}
                            </x-ds.button>
                        @endif
                        <x-ds.button :href="$site['cta']['url'] ?? ''" variant="primary" class="site-mobile__cta">
                            {{ $site['cta']['label'] ?? '' }}
                        </x-ds.button>
                    </x-ds.sheet.footer>
                </x-ds.sheet.content>
            </x-ds.sheet>
        </div>
    </div>
</header>
