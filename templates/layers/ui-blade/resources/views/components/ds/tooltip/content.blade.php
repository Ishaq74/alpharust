@props(['side' => 'top'])
<span {{ $attributes->merge(['data-slot' => 'tooltip-content', 'data-side' => $side, 'role' => 'tooltip']) }}>{{ $slot }}</span>
