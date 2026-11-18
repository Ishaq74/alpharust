@props(['orientation' => 'horizontal'])
<div {{ $attributes->merge(['role' => 'separator', 'aria-orientation' => $orientation, 'data-slot' => 'separator']) }}></div>
