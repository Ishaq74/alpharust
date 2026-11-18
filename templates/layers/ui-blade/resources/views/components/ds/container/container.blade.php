@props(['as' => 'div', 'bleed' => false, 'size' => 'lg', 'gutter' => 'md', 'paddingY' => 'none', 'background' => 'none', 'border' => 'none', 'rounded' => 'none', 'minHeight' => 'none', 'layout' => null, 'centered' => false])
@php
    $layoutV = $centered ? 'centered' : ($layout ?? 'stack');
    $useWrapper = $bleed || $layoutV === 'split';
@endphp
@if ($useWrapper)
<{{ $as }} {{ $attributes->merge(['data-background' => $background, 'data-padding-y' => $paddingY, 'data-min-height' => $minHeight, 'data-border' => $border, 'data-rounded' => $rounded, 'data-bleed' => '', 'data-slot' => 'container']) }}><div {{ new \Illuminate\View\ComponentAttributeBag(['data-container-inner' => '', 'data-size' => $size, 'data-gutter' => $gutter, 'data-layout' => $layoutV]) }}><div data-container-main>{{ $slot }}</div><div data-container-media></div></div></{{ $as }}>
@else
<{{ $as }} {{ $attributes->merge(['data-size' => $size, 'data-gutter' => $gutter, 'data-background' => $background, 'data-padding-y' => $paddingY, 'data-min-height' => $minHeight, 'data-border' => $border, 'data-rounded' => $rounded, 'data-layout' => $layoutV, 'data-slot' => 'container']) }}>{{ $slot }}</{{ $as }}>
@endif
