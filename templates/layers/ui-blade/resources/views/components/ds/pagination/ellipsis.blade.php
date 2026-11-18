<span {{ $attributes->merge(['data-slot' => 'pagination-ellipsis', 'aria-hidden' => 'true']) }}>@if ($slot->isEmpty()) … @else {{ $slot }} @endif</span>
