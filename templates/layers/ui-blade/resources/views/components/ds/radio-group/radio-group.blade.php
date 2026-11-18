@props(['value' => null, 'defaultValue' => null, 'legend' => null, 'disabled' => false, 'orientation' => 'vertical'])
<fieldset {{ $attributes->merge(['role' => 'radiogroup', 'data-value' => ($value ?: $defaultValue), 'data-orientation' => $orientation, 'disabled' => ($disabled ? '' : null), 'data-slot' => 'radio-group']) }}>
@if ($legend)
<legend class="sr-only">{{ $legend }}</legend>
@endif
{{ $slot }}</fieldset>
