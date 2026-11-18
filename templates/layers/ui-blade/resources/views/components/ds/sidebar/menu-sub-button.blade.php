@props(['size' => null, 'isActive' => false])
<a {{ $attributes->merge(['data-sidebar' => 'menu-sub-button', 'data-size' => ($size ?: 'md'), 'data-active' => ($isActive ? 'true' : 'false'), 'aria-current' => ($isActive ? 'page' : null), 'data-slot' => 'sidebar-menu-sub-button']) }}>{{ $slot }}</a>
