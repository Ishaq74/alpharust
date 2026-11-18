@props(['variant' => 'default'])
@php
    $role = $attributes->get('role') ?? (in_array($variant, ['error', 'warning'], true) ? 'alert' : 'status');
@endphp
<div {{ $attributes->merge(['role' => $role, 'data-slot' => 'alert', 'data-variant' => $variant]) }}>{{ $slot }}</div>
