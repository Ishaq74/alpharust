<span {{ $attributes->merge(['data-slot' => 'breadcrumb-ellipsis', 'role' => 'presentation', 'aria-hidden' => 'true']) }}>@if ($slot->isEmpty()) … @else {{ $slot }} @endif</span>
