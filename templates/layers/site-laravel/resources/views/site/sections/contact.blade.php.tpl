{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<section class="site-section site-section--contact">
    <div class="site-container">
        <div class="site-contact__grid">
            <x-ds.card class="site-card">
                <x-ds.card.header>
                    <h2 class="site-card__title">
                        {{ $s['title'] ?? '' }}
                    </h2>
                </x-ds.card.header>
                <x-ds.card.content>
                    @if (! empty($s['sent']))
                        <x-ds.alert variant="success" class="site-form__notice">
                            <x-ds.alert.title>
                                {{ $s['success'] ?? '' }}
                            </x-ds.alert.title>
                        </x-ds.alert>
                    @endif
                    <form class="site-form" method="post" action="{{ $s['action'] ?? '' }}">
                        @csrf
                        @foreach ($s['fields'] ?? [] as $field)
                            <div class="site-field">
                                <x-ds.label :for="$field['id'] ?? ''">
                                    {{ $field['label'] ?? '' }}
                                    @if (! empty($field['required']))
                                        <span class="site-required" aria-hidden="true">
                                            *
                                        </span>
                                    @endif
                                </x-ds.label>
                                @if (! empty($field['textarea']))
                                    <x-ds.textarea :id="$field['id'] ?? ''" :name="$field['name'] ?? ''" rows="6" :required="$field['required'] ?? null" :aria-invalid="$field['invalid'] ?? null" :aria-describedby="$field['error_id'] ?? null" :default-value="$field['value'] ?? ''" />
                                @else
                                    <x-ds.input :id="$field['id'] ?? ''" :name="$field['name'] ?? ''" :type="$field['type'] ?? ''" :autocomplete="$field['autocomplete'] ?? ''" :value="$field['value'] ?? ''" :required="$field['required'] ?? null" :aria-invalid="$field['invalid'] ?? null" :aria-describedby="$field['error_id'] ?? null" />
                                @endif
                                @if (! empty($field['error']))
                                    <p class="site-field__error" id="{{ $field['error_id'] ?? '' }}">
                                        {{ $field['error'] ?? '' }}
                                    </p>
                                @endif
                            </div>
                        @endforeach
                        <x-ds.button type="submit" variant="primary">
                            {{ $s['submit'] ?? '' }}
                            <x-site.icon name="send" :size="16" />
                        </x-ds.button>
                    </form>
                </x-ds.card.content>
            </x-ds.card>
            <aside class="site-contact__info">
                <x-ds.card class="site-card">
                    <x-ds.card.header>
                        <h2 class="site-card__title">
                            {{ $site['ui']['details'] ?? '' }}
                        </h2>
                    </x-ds.card.header>
                    <x-ds.card.content>
                        <ul class="site-contact__list">
                            <li>
                                <x-site.icon name="mail" :size="18" />
                                <a href="{{ $site['contact']['email_url'] ?? '' }}">
                                    {{ $site['contact']['email'] ?? '' }}
                                </a>
                            </li>
                            <li>
                                <x-site.icon name="phone" :size="18" />
                                <a href="{{ $site['contact']['phone_url'] ?? '' }}">
                                    {{ $site['contact']['phone'] ?? '' }}
                                </a>
                            </li>
                            <li>
                                <x-site.icon name="map-pin" :size="18" />
                                <span>
                                    {{ $site['contact']['address'] ?? '' }}
                                </span>
                            </li>
                            <li>
                                <x-site.icon name="clock" :size="18" />
                                <span>
                                    {{ $site['contact']['hours'] ?? '' }}
                                </span>
                            </li>
                        </ul>
                    </x-ds.card.content>
                </x-ds.card>
            </aside>
        </div>
    </div>
</section>
