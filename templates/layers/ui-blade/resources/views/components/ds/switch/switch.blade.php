@props(['id', 'label' => null, 'checked' => false, 'size' => 'md', 'variant' => 'default'])
@php
    $ariaLabel = $attributes->get('aria-label') ?? $label ?? 'switch';
@endphp
<div data-slot="switch">
    <button type="button" id="{{ $id }}" role="switch" aria-checked="{{ $checked ? 'true' : 'false' }}" aria-label="{{ $ariaLabel }}" {{ $attributes->except('aria-label')->merge(['data-slot' => 'switch-button', 'data-size' => $size, 'data-variant' => $variant]) }}>
        <span data-slot="switch-toggle"></span>
    </button>
    @if ($label)
        <label for="{{ $id }}" data-slot="switch-label">{{ $label }}</label>
    @endif
</div>
