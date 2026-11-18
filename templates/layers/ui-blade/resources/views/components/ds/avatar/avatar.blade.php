@props(['size' => 'md'])
<figure {{ $attributes->merge(['data-slot' => 'avatar', 'data-size' => $size]) }}>{{ $slot }}</figure>
