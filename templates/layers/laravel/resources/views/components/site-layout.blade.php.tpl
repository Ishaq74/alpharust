@props(['title' => '@@title@@'])
<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{{ $title }}</title>
@@if ui@@
    {{-- Assets du design kit (Vite + Tailwind v4) ; ignorés tant que `npm run build` n'a pas été lancé. --}}
    @if (file_exists(public_path('build/manifest.json')) || file_exists(public_path('hot')))
        @vite(['resources/css/app.css', 'resources/js/app.js'])
    @endif
    <style>
        header, footer { padding: 1rem 2rem; background: var(--brand); color: var(--brand-contrast); }
        header a { color: var(--brand-contrast); text-decoration: none; font-weight: 700; }
        header nav { display: flex; gap: 1.5rem; }
        main { max-width: var(--page-width); margin: 2rem auto; padding: 0 1rem; }
    </style>
@@else@@
    <style>
        :root { color-scheme: light dark; }
        body { font-family: system-ui, sans-serif; margin: 0; line-height: 1.6; }
        header, footer { padding: 1rem 2rem; background: #0b3d91; color: #ffffff; }
        header a { color: #ffffff; text-decoration: none; font-weight: 700; }
        main { max-width: 48rem; margin: 2rem auto; padding: 0 1rem; }
    </style>
@@end@@
</head>
<body>
@@if ui@@
    <header>
        <nav aria-label="Navigation principale">
            <a href="{{ url('/') }}">@@title@@</a>
            <a href="{{ route('pages.index') }}">Pages</a>
            <a href="{{ route('components') }}">Composants</a>
        </nav>
    </header>
@@else@@
    <header><a href="{{ url('/') }}">@@title@@</a></header>
@@end@@
    <main>{{ $slot }}</main>
    <footer><p>© @@title@@</p></footer>
</body>
</html>
