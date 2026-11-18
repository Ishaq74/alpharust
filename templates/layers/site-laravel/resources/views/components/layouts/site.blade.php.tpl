{{-- Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Wagtail. --}}
{{-- Gabarit commun du site (modèle manecms : composant « shell » + partials head/header/footer). --}}
@props(['title' => null, 'description' => null])
<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}" dir="{{ $site['dir'] }}">
<head>
    @include('site.partials.head', ['title' => $title, 'description' => $description])
</head>
<body class="site-body">
    <a class="site-skip" href="#contenu">{{ $site['ui']['skip'] }}</a>
    @include('site.partials.header')
    <main id="contenu" class="site-main" tabindex="-1">
        {{ $slot }}
    </main>
    @include('site.partials.footer')
</body>
</html>
