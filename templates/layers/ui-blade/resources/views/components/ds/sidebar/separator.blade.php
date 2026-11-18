@props(['orientation' => 'horizontal'])
<div {{ $attributes->merge(['role' => 'none', 'data-orientation' => $orientation, 'data-sidebar' => 'separator', 'data-slot' => 'sidebar-separator']) }}></div>
