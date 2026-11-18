@props(['ratio' => 1, 'as' => 'div'])
@php
    $pad = round(100 / (float) $ratio, 4);
@endphp
<div {{ new \Illuminate\View\ComponentAttributeBag(['style' => ('--ds-ratio: ' . $ratio . '; padding-bottom: ' . $pad . '%'), 'data-slot' => 'aspect-ratio-wrapper']) }}><{{ $as }} {{ $attributes->merge(['data-slot' => 'aspect-ratio']) }}>{{ $slot }}</{{ $as }}></div>
