@props(['showOnHover' => false, 'variant' => 'ghost', 'size' => 'icon-sm'])
<button {{ $attributes->merge(['type' => 'button', 'data-variant' => $variant, 'data-size' => $size, 'data-show-on-hover' => ($showOnHover ? '' : null), 'data-sidebar' => 'menu-action', 'data-slot' => 'sidebar-menu-action']) }}>{{ $slot }}</button>
