@props(['asChild' => false])
@if ($asChild)
<div data-slot="popover-trigger" data-as-child>{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'aria-haspopup' => 'dialog', 'aria-expanded' => 'false', 'data-state' => 'closed', 'data-slot' => 'popover-trigger']) }}>{{ $slot }}</button>
@endif
