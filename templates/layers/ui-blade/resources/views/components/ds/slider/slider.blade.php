@props(['defaultValue' => 0, 'value' => null, 'min' => 0, 'max' => 100, 'step' => 1, 'largeStep' => 10, 'orientation' => 'horizontal', 'disabled' => false, 'name' => null, 'variant' => 'default', 'ariaLabel' => null, 'ariaLabelledby' => null])
@php
    $thumbs = \App\View\DesignKit::sliderThumbs($value ?? $defaultValue, $min, $max, $orientation, $name);
    $rangeStyle = \App\View\DesignKit::sliderRange($value ?? $defaultValue, $min, $max, $orientation);
@endphp
<div {{ $attributes->merge(['data-orientation' => $orientation, 'data-disabled' => ($disabled ? '' : null), 'data-variant' => $variant, 'role' => 'group', 'aria-labelledby' => $ariaLabelledby, 'data-min' => $min, 'data-max' => $max, 'data-step' => $step, 'data-large-step' => $largeStep, 'data-slot' => 'slider']) }}><div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'slider-control', 'data-orientation' => $orientation]) }}><div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'slider-track', 'data-orientation' => $orientation]) }}><div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'slider-range', 'data-orientation' => $orientation, 'style' => $rangeStyle]) }}></div></div>
@foreach ($thumbs as $t)
<div {{ new \Illuminate\View\ComponentAttributeBag(['data-slot' => 'slider-thumb', 'data-index' => $t['i'], 'data-orientation' => $orientation, 'style' => $t['style'], 'tabindex' => ($disabled ? '-1' : '0'), 'role' => 'slider', 'aria-label' => $ariaLabel, 'aria-labelledby' => $ariaLabelledby, 'aria-valuemin' => $min, 'aria-valuemax' => $max, 'aria-valuenow' => $t['v'], 'aria-orientation' => $orientation, 'aria-disabled' => ($disabled ? 'true' : 'false')]) }}><input {{ new \Illuminate\View\ComponentAttributeBag(['type' => 'range', 'min' => $min, 'max' => $max, 'step' => $step, 'value' => $t['v'], 'name' => $t['name'], 'disabled' => ($disabled ? '' : null), 'tabindex' => '-1', 'aria-hidden' => 'true', 'class' => 'sr-only']) }}></div>
@endforeach
</div></div>
