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
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="m22 2-7 20-4-9-9-4Z"/><path d="M22 2 11 13"/></svg>
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
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
                                <a href="{{ $site['contact']['email_url'] ?? '' }}">
                                    {{ $site['contact']['email'] ?? '' }}
                                </a>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.13.96.36 1.9.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.91.34 1.85.57 2.81.7A2 2 0 0 1 22 16.92z"/></svg>
                                <a href="{{ $site['contact']['phone_url'] ?? '' }}">
                                    {{ $site['contact']['phone'] ?? '' }}
                                </a>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/><circle cx="12" cy="10" r="3"/></svg>
                                <span>
                                    {{ $site['contact']['address'] ?? '' }}
                                </span>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
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
