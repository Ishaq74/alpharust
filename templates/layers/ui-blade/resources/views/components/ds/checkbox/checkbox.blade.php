@props(['id' => null, 'label' => null, 'checked' => false, 'size' => 'md', 'variant' => 'default'])
<div data-slot="checkbox" data-size="{{ $size }}" data-variant="{{ $variant }}">
    <input type="checkbox" @if ($id) id="{{ $id }}" @endif @checked($checked) {{ $attributes->merge(['data-slot' => 'checkbox-input']) }}>
    @if ($label)
        <label @if ($id) for="{{ $id }}" @endif data-slot="checkbox-label">{{ $label }}</label>
    @endif
</div>
