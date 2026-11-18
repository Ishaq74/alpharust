@@meta when=auth@@
{{-- Connexion / inscription (comptes : --auth). --}}
{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$page['title']">
    <section class="site-section site-auth">
        <div class="site-container site-container--narrow">
            <x-ds.card class="site-card site-auth__card">
                <x-ds.card.header>
                    <h1 class="site-auth__title">
                        {{ $page['title'] ?? '' }}
                    </h1>
                    <p class="site-lead">
                        {{ $page['lead'] ?? '' }}
                    </p>
                </x-ds.card.header>
                <x-ds.card.content>
                    @if (! empty($page['error']))
                        <x-ds.alert variant="error" class="site-form__notice">
                            <x-ds.alert.title>
                                {{ $page['error'] ?? '' }}
                            </x-ds.alert.title>
                        </x-ds.alert>
                    @endif
                    <form class="site-form" method="post" action="{{ $page['action'] ?? '' }}">
                        @csrf
                        <input type="hidden" name="locale" value="{{ $site['locale'] ?? '' }}">
                        @foreach ($page['fields'] ?? [] as $field)
                            <div class="site-field">
                                <x-ds.label :for="$field['id'] ?? ''">
                                    {{ $field['label'] ?? '' }}
                                </x-ds.label>
                                <x-ds.input :id="$field['id'] ?? ''" :name="$field['name'] ?? ''" :type="$field['type'] ?? ''" :autocomplete="$field['autocomplete'] ?? ''" :value="$field['value'] ?? ''" :required="true" />
                            </div>
                        @endforeach
                        <x-ds.button type="submit" variant="primary" class="site-block">
                            {{ $page['submit'] ?? '' }}
                        </x-ds.button>
                    </form>
                </x-ds.card.content>
                <x-ds.card.footer class="site-auth__alt">
                    <span>
                        {{ $page['alt_text'] ?? '' }}
                    </span>
                    <a href="{{ $page['alt_url'] ?? '' }}">
                        {{ $page['alt_label'] ?? '' }}
                    </a>
                </x-ds.card.footer>
            </x-ds.card>
        </div>
    </section>
</x-layouts.site>
