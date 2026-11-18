@props(['variant' => 'default', 'size' => 'default', 'as' => 'div'])
<{{ $as }} {{ $attributes->merge(['data-variant' => $variant, 'data-size' => $size, 'data-slot' => 'item']) }}>{{ $slot }}</{{ $as }}>
