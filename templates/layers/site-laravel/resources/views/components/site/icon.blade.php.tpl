{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
{{-- Icônes du site : resources/data/icons.json (MDI par défaut, drapeaux Circle Flags), comme Astro et Django. --}}
@props(['name', 'size' => 20])
@php($icon = \App\Support\SiteData::icon($name))
@if ($icon)
<svg width="{{ $size }}" height="{{ $size }}" viewBox="0 0 {{ $icon['width'] }} {{ $icon['height'] }}" fill="currentColor" aria-hidden="true" class="site-icon" data-icon="{{ $name }}">{!! $icon['body'] !!}</svg>
@endif
