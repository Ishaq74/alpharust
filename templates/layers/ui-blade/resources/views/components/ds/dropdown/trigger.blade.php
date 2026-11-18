@props(['asChild' => false])
@if ($asChild)
<div data-slot="dropdown-trigger" data-as-child aria-haspopup="true" aria-expanded="false">{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'aria-haspopup' => 'true', 'aria-expanded' => 'false', 'data-state' => 'closed', 'data-slot' => 'dropdown-trigger']) }}>{{ $slot }}</button>
@endif
