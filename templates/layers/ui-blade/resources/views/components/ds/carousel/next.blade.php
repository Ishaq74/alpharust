@props(['variant' => 'outline', 'size' => 'icon'])
<button {{ $attributes->merge(['type' => 'button', 'data-variant' => $variant, 'data-size' => $size, 'aria-label' => 'Next slide', 'data-slot' => 'carousel-next']) }}><svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-right"><path d="M5 12h14M13 18l6-6M13 6l6 6"/></svg>
@if (trim((string) $slot) === '')
<span class="sr-only">Next slide</span>
@else
{{ $slot }}
@endif
</button>
