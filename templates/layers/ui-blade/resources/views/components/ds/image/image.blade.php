@props(['src' => null, 'alt' => ''])
@if ($src)
<img {{ $attributes->merge(['src' => $src, 'alt' => $alt, 'loading' => 'lazy', 'decoding' => 'async', 'data-slot' => 'image']) }}>
@endif
