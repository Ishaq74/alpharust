@props(['value' => null, 'max' => 100, 'variant' => 'default'])
@php
    $progress = ['data-slot' => 'progress', 'data-variant' => $variant, 'max' => $max];
    if (! is_null($value)) {
        $progress['value'] = $value;
    }
@endphp
<progress {{ $attributes->merge($progress) }}></progress>
