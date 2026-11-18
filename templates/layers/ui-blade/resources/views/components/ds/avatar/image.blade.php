@props(['src' => null, 'alt' => ''])
@if ($src)
    <img src="{{ $src }}" alt="{{ $alt }}" {{ $attributes->merge(['data-slot' => 'avatar-image']) }}>
@endif
