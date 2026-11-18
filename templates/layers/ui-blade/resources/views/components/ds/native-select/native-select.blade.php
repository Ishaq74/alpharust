@props(['size' => 'md'])
<div data-slot="native-select-wrapper">
    <select {{ $attributes->merge(['data-slot' => 'native-select', 'data-size' => $size]) }}>{{ $slot }}</select>
    <span data-slot="native-select-icon" aria-hidden="true"></span>
</div>
