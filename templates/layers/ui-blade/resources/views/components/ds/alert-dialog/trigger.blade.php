@props(['asChild' => false, 'for' => null])
@if ($asChild)
<div {{ new \Illuminate\View\ComponentAttributeBag(['data-dialog-for' => $for, 'data-slot' => 'alert-dialog-trigger', 'data-as-child' => '']) }}>{{ $slot }}</div>
@else
<button {{ $attributes->merge(['type' => 'button', 'aria-haspopup' => 'dialog', 'data-dialog-for' => $for, 'data-slot' => 'alert-dialog-trigger']) }}>{{ $slot }}</button>
@endif
