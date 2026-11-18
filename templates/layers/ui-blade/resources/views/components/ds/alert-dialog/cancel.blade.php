@props(['asChild' => false])
@if ($asChild)
<div data-slot="alert-dialog-cancel" data-as-child>{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'data-variant' => 'outline', 'data-slot' => 'alert-dialog-cancel']) }}>{{ $slot }}</button>
@endif
