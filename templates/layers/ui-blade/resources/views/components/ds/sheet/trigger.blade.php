@props(['for' => null, 'asChild' => false])
@if ($asChild)
<span {{ $attributes->merge(['data-slot' => 'dialog-trigger', 'data-as-child' => '', 'data-for' => $for]) }}>{{ $slot }}</span>
@else
<button {{ $attributes->merge(['type' => 'button', 'aria-haspopup' => 'dialog', 'data-slot' => 'dialog-trigger', 'data-for' => $for]) }}>{{ $slot }}</button>
@endif
