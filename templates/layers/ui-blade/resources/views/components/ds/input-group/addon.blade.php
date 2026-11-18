@props(['align' => 'inline-start'])
<div {{ $attributes->merge(['role' => 'group', 'data-align' => $align, 'data-slot' => 'input-group-addon']) }}>{{ $slot }}</div>
