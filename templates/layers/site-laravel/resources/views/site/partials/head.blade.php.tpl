{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{{ $title ? $title.' · '.$site['name'] : $site['name'] }}</title>
<meta name="description" content="{{ $description ?: $site['description'] }}">
<link rel="canonical" href="{{ url()->current() }}">
<link rel="icon" type="image/svg+xml" href="/favicon.svg">
<meta property="og:type" content="website">
<meta property="og:title" content="{{ $title ? $title.' · '.$site['name'] : $site['name'] }}">
<meta property="og:description" content="{{ $description ?: $site['description'] }}">
<script>
    // Applique le thème choisi avant le premier rendu (évite le flash).
    try {
        const saved = localStorage.getItem('theme');
        const theme = saved === 'light' || saved === 'dark' ? saved : (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
        document.documentElement.dataset.theme = theme;
        document.documentElement.classList.toggle('dark', theme === 'dark');
    } catch (e) {}
</script>
@if (file_exists(public_path('build/manifest.json')) || file_exists(public_path('hot')))
    @vite(['resources/css/app.css', 'resources/js/app.js'])
@endif
