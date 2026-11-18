@props(['type' => 'single', 'defaultValue' => null])
<div {{ $attributes->merge(['data-slot' => 'accordion', 'data-type' => $type, 'data-default' => $defaultValue]) }}>{{ $slot }}</div>
