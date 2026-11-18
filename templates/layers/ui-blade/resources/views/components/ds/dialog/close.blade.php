@props(['asChild' => false])
@if ($asChild)
    <span {{ $attributes->merge(['data-slot' => 'dialog-close', 'data-as-child' => true]) }}>{{ $slot }}</span>
@else
    <button {{ $attributes->merge(['type' => 'button', 'data-slot' => 'dialog-close']) }}>{{ $slot }}</button>
@endif
