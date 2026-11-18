@props(['size' => 'md'])
<input {{ $attributes->merge(['data-sidebar' => 'input', 'data-slot' => 'sidebar-input', 'data-size' => $size]) }}>
