@props(['asChild' => false])
<div {{ $attributes->merge(['data-sidebar' => 'group-label', 'data-as-child' => ($asChild ? '' : null), 'data-slot' => 'sidebar-group-label']) }}>{{ $slot }}</div>
