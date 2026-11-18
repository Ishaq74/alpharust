@props(['orientation' => 'horizontal'])
<div {{ $attributes->merge(['role' => 'none', 'data-orientation' => $orientation, 'data-slot' => 'item-separator']) }}></div>
