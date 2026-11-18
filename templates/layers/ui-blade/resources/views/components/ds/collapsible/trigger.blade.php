@props(['asChild' => false])
@if ($asChild)
<div aria-expanded="false" data-slot="collapsible-trigger" data-as-child>{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'aria-expanded' => 'false', 'data-slot' => 'collapsible-trigger']) }}>{{ $slot }}</button>
@endif
