@props(['variant' => 'outline', 'size' => 'icon'])
<button {{ $attributes->merge(['type' => 'button', 'data-variant' => $variant, 'data-size' => $size, 'aria-label' => 'Previous slide', 'data-slot' => 'carousel-previous']) }}><svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-left"><path d="M5 12h14M5 12l6 6M5 12l6-6"/></svg>
@if (trim((string) $slot) === '')
<span class="sr-only">Previous slide</span>
@else
{{ $slot }}
@endif
</button>
