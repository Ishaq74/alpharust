@props(['animationDuration' => 150])
<div {{ $attributes->merge(['data-sub-content' => '', 'role' => 'menu', 'data-side' => 'right', 'data-align' => 'start', 'data-state' => 'closed', 'tabindex' => '-1', 'aria-orientation' => 'vertical', 'hidden' => '', 'style' => ('--ds-duration: ' . $animationDuration . 'ms'), 'data-slot' => 'dropdown-content']) }}>{{ $slot }}</div>
