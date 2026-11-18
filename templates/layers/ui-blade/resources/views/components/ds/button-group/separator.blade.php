@props(['orientation' => 'vertical'])
<div {{ $attributes->merge(['role' => 'separator', 'aria-orientation' => $orientation, 'data-orientation' => $orientation, 'data-slot' => 'button-group-separator']) }}></div>
