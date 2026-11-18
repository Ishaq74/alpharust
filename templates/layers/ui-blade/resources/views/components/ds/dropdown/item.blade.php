@props(['inset' => false, 'disabled' => false, 'as' => 'div'])
<{{ $as }} {{ $attributes->merge(['role' => 'menuitem', 'tabindex' => ($disabled ? '-1' : '0'), 'aria-disabled' => ($disabled ? 'true' : null), 'data-disabled' => ($disabled ? 'true' : null), 'data-inset' => ($inset ? '' : null), 'data-slot' => 'dropdown-item']) }}>{{ $slot }}</{{ $as }}>
