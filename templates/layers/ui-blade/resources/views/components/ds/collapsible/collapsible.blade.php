@props(['defaultOpen' => false, 'disabled' => false])
<div {{ $attributes->merge(['data-state' => ($defaultOpen ? 'open' : 'closed'), 'data-disabled' => ($disabled ? '' : null), 'data-slot' => 'collapsible']) }}>{{ $slot }}</div>
