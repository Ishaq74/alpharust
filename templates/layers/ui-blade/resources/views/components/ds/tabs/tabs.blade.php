@props(['defaultValue' => null])
<div {{ $attributes->merge(['data-slot' => 'tabs', 'data-default' => $defaultValue]) }}>{{ $slot }}</div>
