@props(['openOnHover' => false, 'closeDelay' => 200])
<div {{ $attributes->merge(['data-open-on-hover' => ($openOnHover ? 'true' : null), 'data-close-delay' => $closeDelay, 'data-slot' => 'dropdown']) }}>{{ $slot }}</div>
