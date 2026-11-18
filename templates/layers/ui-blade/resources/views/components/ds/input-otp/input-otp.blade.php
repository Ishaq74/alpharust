@props(['maxLength' => 6, 'value' => null, 'defaultValue' => null, 'disabled' => false, 'pattern' => null, 'name' => null, 'id' => null, 'required' => false])
@php
    $patternStr = $pattern ?? '^\\d+$';
    $inputMode = (str_contains($patternStr, '\\d') || str_contains($patternStr, '0-9')) && ! str_contains($patternStr, 'a-z') && ! str_contains($patternStr, 'A-Z') && ! str_contains($patternStr, '\\w') ? 'numeric' : 'text';
    $initial = $value ?: ($defaultValue ?: '');
@endphp
<div {{ $attributes->merge(['data-max-length' => $maxLength, 'data-value' => $initial, 'data-disabled' => ($disabled ? 'true' : 'false'), 'data-pattern' => $patternStr, 'role' => 'group', 'aria-label' => ('OTP input, ' . $maxLength . ' digits'), 'data-slot' => 'input-otp']) }}><input {{ new \Illuminate\View\ComponentAttributeBag(['autocomplete' => 'one-time-code', 'inputmode' => $inputMode, 'name' => $name, 'id' => $id, 'class' => 'sr-only', 'required' => ($required ? '' : null), 'maxlength' => $maxLength, 'value' => $initial, 'disabled' => ($disabled ? '' : null), 'aria-label' => ('OTP, ' . $maxLength), 'data-otp-hidden-input' => '']) }}>{{ $slot }}</div>
