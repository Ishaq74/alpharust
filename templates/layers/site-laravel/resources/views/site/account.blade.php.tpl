@@meta when=auth@@
{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$page['title']">
    <section class="site-section site-auth">
        <div class="site-container site-container--narrow">
            <x-ds.card class="site-card site-auth__card">
                <x-ds.card.header>
                    <span class="site-icon-box">
                        <x-site.icon name="user" :size="22" />
                    </span>
                    <h1 class="site-auth__title">
                        {{ $page['title'] ?? '' }}
                    </h1>
                    <p class="site-lead">
                        {{ $page['lead'] ?? '' }}
                    </p>
                </x-ds.card.header>
                <x-ds.card.content>
                    <dl class="site-auth__details">
                        <dt>
                            {{ $site['ui']['field_name'] ?? '' }}
                        </dt>
                        <dd>
                            {{ $page['name'] ?? '' }}
                        </dd>
                        <dt>
                            {{ $site['ui']['field_email'] ?? '' }}
                        </dt>
                        <dd>
                            {{ $page['email'] ?? '' }}
                        </dd>
                    </dl>
                </x-ds.card.content>
                <x-ds.card.footer>
                    <form method="post" action="{{ $page['logout_action'] ?? '' }}">
                        @csrf
                        <input type="hidden" name="locale" value="{{ $site['locale'] ?? '' }}">
                        <x-ds.button type="submit" variant="outline">
                            <x-site.icon name="log-out" :size="16" />
                            {{ $site['ui']['sign_out'] ?? '' }}
                        </x-ds.button>
                    </form>
                </x-ds.card.footer>
            </x-ds.card>
        </div>
    </section>
</x-layouts.site>
