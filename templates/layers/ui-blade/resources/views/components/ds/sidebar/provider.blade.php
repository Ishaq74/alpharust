@props(['defaultOpen' => true, 'keyboardShortcut' => 'b'])
<div {{ $attributes->merge(['data-state' => ($defaultOpen ? 'expanded' : 'collapsed'), 'data-mobile-open' => 'false', 'data-keyboard-shortcut' => $keyboardShortcut, 'style' => '--sidebar-width: 18rem; --sidebar-width-icon: 3.5rem;', 'data-slot' => 'sidebar-provider']) }}>{{ $slot }}</div>
