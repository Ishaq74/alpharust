@props(['asChild' => false])
@if ($asChild)
<div data-slot="alert-dialog-action" data-as-child>{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'data-variant' => 'primary', 'data-slot' => 'alert-dialog-action']) }}>{{ $slot }}</button>
@endif
