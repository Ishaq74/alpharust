@props(['variant' => 'default', 'size' => 'md'])
@if ($attributes->has('href'))
    <a {{ $attributes->merge(['data-slot' => 'badge', 'data-variant' => $variant, 'data-size' => $size]) }}>{{ $slot }}</a>
@else
    <div {{ $attributes->merge(['data-slot' => 'badge', 'data-variant' => $variant, 'data-size' => $size]) }}>{{ $slot }}</div>
@endif
