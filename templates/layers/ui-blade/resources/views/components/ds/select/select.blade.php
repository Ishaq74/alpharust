@props(['name' => null, 'defaultValue' => null, 'required' => false])
<div {{ $attributes->merge(['data-name' => $name, 'data-value' => $defaultValue, 'data-required' => ($required ? 'true' : null), 'data-slot' => 'select']) }}>{{ $slot }}
@if ($name)
<input {{ new \Illuminate\View\ComponentAttributeBag(['type' => 'hidden', 'name' => $name, 'value' => ($defaultValue ?: ''), 'required' => ($required ? '' : null), 'data-select-input' => '']) }}>
@endif
</div>
