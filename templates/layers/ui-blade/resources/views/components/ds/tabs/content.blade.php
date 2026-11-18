@props(['value', 'defaultActive' => false])
<div {{ $attributes->merge(['role' => 'tabpanel', 'data-slot' => 'tabs-content', 'data-value' => $value]) }} @unless ($defaultActive) hidden @endunless>{{ $slot }}</div>
