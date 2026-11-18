@props(['variant' => 'outline', 'size' => 'md', 'ariaLabel' => 'Toggle theme'])
<button {{ $attributes->merge(['type' => 'button', 'data-state' => 'off', 'data-variant' => $variant, 'data-size' => $size, 'aria-pressed' => 'false', 'aria-label' => $ariaLabel, 'data-slot' => 'theme-toggle']) }}>
@if (trim((string) $slot) === '')
<span data-theme-icon-wrapper><svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="light"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg><svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="dark"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9z"/></svg></span>
@else
{{ $slot }}
@endif
</button>
