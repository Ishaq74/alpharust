@props(['openOnHover' => false, 'closeDelay' => 200, 'defaultOpen' => false])
<div {{ $attributes->merge(['data-open-on-hover' => ($openOnHover ? 'true' : null), 'data-close-delay' => $closeDelay, 'data-state' => ($defaultOpen ? 'open' : 'closed'), 'data-default-open' => ($defaultOpen ? 'true' : null), 'data-slot' => 'popover']) }}>{{ $slot }}</div>
