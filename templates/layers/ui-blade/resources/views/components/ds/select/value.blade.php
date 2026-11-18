@props(['placeholder' => 'select'])
<span {{ $attributes->merge(['data-placeholder' => $placeholder, 'data-slot' => 'select-value']) }}>{{ $placeholder }}</span>
