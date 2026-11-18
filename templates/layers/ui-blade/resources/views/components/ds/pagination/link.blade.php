@props(['isActive' => false])
<a {{ $attributes->merge(['aria-current' => $isActive ? 'page' : null, 'data-slot' => 'pagination-link']) }}>{{ $slot }}</a>
