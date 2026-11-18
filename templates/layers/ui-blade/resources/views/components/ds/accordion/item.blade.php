@props(['value'])
<details {{ $attributes->merge(['data-slot' => 'accordion-item', 'data-value' => $value]) }}>{{ $slot }}</details>
