@props(['value', 'defaultActive' => false])
<button {{ $attributes->merge(['type' => 'button', 'role' => 'tab', 'aria-selected' => $defaultActive ? 'true' : 'false', 'tabindex' => $defaultActive ? 0 : -1, 'data-slot' => 'tabs-trigger', 'data-value' => $value]) }}>{{ $slot }}</button>
