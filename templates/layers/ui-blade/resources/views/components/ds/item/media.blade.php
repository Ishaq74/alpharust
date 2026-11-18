@props(['variant' => 'default'])
<div {{ $attributes->merge(['data-variant' => $variant, 'data-slot' => 'item-media']) }}>{{ $slot }}</div>
