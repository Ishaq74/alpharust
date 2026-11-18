@props(['orientation' => 'horizontal'])
<div {{ $attributes->merge(['role' => 'group', 'data-orientation' => $orientation, 'data-slot' => 'button-group']) }}>{{ $slot }}</div>
