@props(['inset' => false])
<div {{ $attributes->merge(['data-inset' => ($inset ? '' : null), 'data-slot' => 'dropdown-label']) }}>{{ $slot }}</div>
