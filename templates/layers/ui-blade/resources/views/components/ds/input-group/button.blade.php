@props(['type' => 'button', 'variant' => 'ghost', 'size' => 'sm'])
<button {{ $attributes->merge(['type' => $type, 'data-size' => $size, 'data-input-group-button' => 'true', 'data-slot' => 'button', 'data-variant' => $variant]) }}>{{ $slot }}</button>
