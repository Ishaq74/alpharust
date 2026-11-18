@props(['size' => 'md'])
<input {{ $attributes->merge(['data-slot' => 'input', 'data-size' => $size]) }}>
