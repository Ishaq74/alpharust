<li {{ $attributes->merge(['data-slot' => 'breadcrumb-separator', 'role' => 'presentation', 'aria-hidden' => 'true']) }}>@if ($slot->isEmpty()) › @else {{ $slot }} @endif</li>
