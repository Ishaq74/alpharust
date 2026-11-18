@props(['size' => 'md', 'index' => null])
<div {{ $attributes->merge(['data-otp-slot' => '', 'data-index' => $index, 'data-size' => $size, 'aria-hidden' => 'true', 'data-slot' => 'input-otp-slot']) }}><span data-otp-char></span><div data-otp-caret hidden><div data-otp-caret-line></div></div></div>
