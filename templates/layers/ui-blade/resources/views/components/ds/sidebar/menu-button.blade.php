@props(['asChild' => false, 'isActive' => false, 'tooltip' => null, 'variant' => 'default', 'size' => null])
@php
    $tagV = $attributes->has('href') ? 'a' : 'button';
    $current = $isActive && $attributes->has('href') ? 'page' : null;
@endphp
@if ($tooltip)
<span data-sidebar-tooltip="true" data-slot="tooltip"><span data-slot="tooltip-trigger">
@if ($asChild)
<div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'sidebar-menu-button', 'data-sidebar' => 'menu-button', 'data-size' => ($size ?: 'default'), 'data-variant' => $variant, 'data-active' => ($isActive ? 'true' : 'false'), 'data-as-child' => '']) }}>{{ $slot }}</div>
@else
<{{ $tagV }} {{ $attributes->merge(['type' => (($tagV === 'button') ? 'button' : null), 'data-slot' => 'sidebar-menu-button', 'data-sidebar' => 'menu-button', 'data-size' => ($size ?: 'default'), 'data-variant' => $variant, 'data-active' => ($isActive ? 'true' : 'false'), 'aria-current' => $current]) }}>{{ $slot }}</{{ $tagV }}>
@endif
</span><span role="tooltip" data-slot="tooltip-content" data-side="right">{{ $tooltip }}</span></span>
@else
@if ($asChild)
<div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'sidebar-menu-button', 'data-sidebar' => 'menu-button', 'data-size' => ($size ?: 'default'), 'data-variant' => $variant, 'data-active' => ($isActive ? 'true' : 'false'), 'data-as-child' => '']) }}>{{ $slot }}</div>
@else
<{{ $tagV }} {{ $attributes->merge(['type' => (($tagV === 'button') ? 'button' : null), 'data-slot' => 'sidebar-menu-button', 'data-sidebar' => 'menu-button', 'data-size' => ($size ?: 'default'), 'data-variant' => $variant, 'data-active' => ($isActive ? 'true' : 'false'), 'aria-current' => $current]) }}>{{ $slot }}</{{ $tagV }}>
@endif
@endif
