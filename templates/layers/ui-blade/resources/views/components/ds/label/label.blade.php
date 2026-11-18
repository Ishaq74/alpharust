@props(['size' => 'md'])
<label {{ $attributes->merge(['data-slot' => 'label', 'data-size' => $size]) }}>{{ $slot }}</label>
