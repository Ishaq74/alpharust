<a {{ $attributes->merge(['data-slot' => 'pagination-next', 'aria-label' => 'Go to next page']) }}>@if ($slot->isEmpty()) Next › @else {{ $slot }} @endif</a>
