@props(['defaultPressed' => false, 'syncGroup' => null, 'variant' => 'default', 'size' => 'md'])
<button {{ $attributes->merge(['type' => 'button', 'data-state' => ($defaultPressed ? 'on' : 'off'), 'data-sync-group' => $syncGroup, 'data-variant' => $variant, 'data-size' => $size, 'aria-pressed' => ($defaultPressed ? 'true' : 'false'), 'data-slot' => 'toggle']) }}>{{ $slot }}</button>
