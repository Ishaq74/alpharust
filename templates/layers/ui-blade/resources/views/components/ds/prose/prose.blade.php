@props(['size' => 'default'])
<div {{ $attributes->merge(['data-size' => $size, 'data-slot' => 'prose']) }}>{{ $slot }}</div>
