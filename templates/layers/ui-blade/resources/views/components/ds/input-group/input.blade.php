@props(['size' => 'md'])
<input {{ $attributes->merge(['data-slot' => 'input-group-control', 'data-size' => $size]) }}>
