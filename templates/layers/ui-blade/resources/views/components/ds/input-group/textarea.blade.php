@props(['size' => 'md', 'defaultValue' => ''])
<textarea {{ $attributes->merge(['data-slot' => 'input-group-control', 'data-size' => $size]) }}>{{ $defaultValue }}</textarea>
