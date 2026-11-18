{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<footer class="site-footer">
    <div class="site-container site-footer__grid">
        <div class="site-footer__about">
            <a class="site-brand" href="{{ $site['home_url'] ?? '' }}">
                <span class="site-brand__mark" aria-hidden="true">
                    {{ $site['initial'] ?? '' }}
                </span>
                <span class="site-brand__name">
                    {{ $site['name'] ?? '' }}
                </span>
            </a>
            <p class="site-footer__tagline">
                {{ $site['tagline'] ?? '' }}
            </p>
            <ul class="site-social" aria-label="Réseaux sociaux">
                @foreach ($site['social'] ?? [] as $link)
                    <li>
                        <a href="{{ $link['url'] ?? '' }}" aria-label="{{ $link['label'] ?? '' }}" rel="me noopener">
                            <x-site.icon :name="$link['icon'] ?? ''" :size="20" />
                        </a>
                    </li>
                @endforeach
            </ul>
        </div>
        @foreach ($site['footer'] ?? [] as $col)
            <nav class="site-footer__col" aria-label="{{ $col['title'] ?? '' }}">
                <h2 class="site-footer__title">
                    {{ $col['title'] ?? '' }}
                </h2>
                <ul class="site-footer__links">
                    @foreach ($col['links'] ?? [] as $link)
                        <li>
                            <a href="{{ $link['url'] ?? '' }}">
                                {{ $link['label'] ?? '' }}
                            </a>
                        </li>
                    @endforeach
                </ul>
            </nav>
        @endforeach
        <div class="site-footer__col">
            <h2 class="site-footer__title">
                Contact
            </h2>
            <address class="site-footer__contact">
                <a href="{{ $site['contact']['email_url'] ?? '' }}">
                    {{ $site['contact']['email'] ?? '' }}
                </a>
                <a href="{{ $site['contact']['phone_url'] ?? '' }}">
                    {{ $site['contact']['phone'] ?? '' }}
                </a>
                <span>
                    {{ $site['contact']['address'] ?? '' }}
                </span>
            </address>
        </div>
    </div>
    <div class="site-container site-footer__bottom">
        <p>
            ©
            {{ $site['year'] ?? '' }}
            {{ $site['name'] ?? '' }}
            ·
            {{ $site['copyright'] ?? '' }}
        </p>
    </div>
</footer>
