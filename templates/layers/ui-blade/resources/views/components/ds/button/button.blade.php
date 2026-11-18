@props(['variant' => 'default', 'size' => 'md'])
@if ($attributes->has('href'))
    <a {{ $attributes->merge(['data-slot' => 'button', 'data-variant' => $variant, 'data-size' => $size]) }}>{{ $slot }}</a>
@else
    <button {{ $attributes->merge(['data-slot' => 'button', 'data-variant' => $variant, 'data-size' => $size, 'type' => 'button']) }}>{{ $slot }}</button>
@endif
