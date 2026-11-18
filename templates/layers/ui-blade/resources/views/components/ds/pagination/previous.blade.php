<a {{ $attributes->merge(['data-slot' => 'pagination-previous', 'aria-label' => 'Go to previous page']) }}>@if ($slot->isEmpty()) ‹ Previous @else {{ $slot }} @endif</a>
