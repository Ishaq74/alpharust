@props(['showIcon' => false, 'width' => null])
@php
    $skeletonWidth = $width ?? (random_int(50, 89).'%');
@endphp
<div {{ $attributes->merge(['data-sidebar' => 'menu-skeleton', 'data-slot' => 'sidebar-menu-skeleton']) }}>
@if ($showIcon)
<div data-sidebar="menu-skeleton-icon" data-slot="skeleton"></div>
@endif
<div {{ new \Illuminate\View\ComponentAttributeBag(['data-sidebar' => 'menu-skeleton-text', 'style' => ('--skeleton-width: ' . $skeletonWidth), 'data-slot' => 'skeleton']) }}></div></div>
