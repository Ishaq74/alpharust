@props(['size' => 'md', 'defaultValue' => ''])
<textarea {{ $attributes->merge(['data-slot' => 'textarea', 'data-size' => $size]) }}>{{ $defaultValue }}{{ $slot }}</textarea>
