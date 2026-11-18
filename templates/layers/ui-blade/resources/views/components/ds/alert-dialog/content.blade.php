@props(['animationDuration' => 200])
<div data-state="closed" data-slot="alert-dialog-backdrop" aria-hidden="true"></div><dialog {{ $attributes->merge(['data-state' => 'closed', 'role' => 'alertdialog', 'style' => ('--ds-duration: ' . $animationDuration . 'ms'), 'data-slot' => 'alert-dialog-content']) }}>{{ $slot }}</dialog>
