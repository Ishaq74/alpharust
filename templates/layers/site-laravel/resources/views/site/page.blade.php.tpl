{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<x-layouts.site :title="$page['title'] ?? null" :description="$page['description'] ?? null">
    @foreach ($page['sections'] as $s)
        @include('site.sections.'.$s['type'], ['s' => $s])
    @endforeach
</x-layouts.site>
