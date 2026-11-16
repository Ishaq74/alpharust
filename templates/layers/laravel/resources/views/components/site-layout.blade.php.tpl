@props(['title' => '@@title@@'])
<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{{ $title }}</title>
    <style>
        :root { color-scheme: light dark; }
        body { font-family: system-ui, sans-serif; margin: 0; line-height: 1.6; }
        header, footer { padding: 1rem 2rem; background: #0b3d91; color: #ffffff; }
        header a { color: #ffffff; text-decoration: none; font-weight: 700; }
        main { max-width: 48rem; margin: 2rem auto; padding: 0 1rem; }
    </style>
</head>
<body>
    <header><a href="{{ url('/') }}">@@title@@</a></header>
    <main>{{ $slot }}</main>
    <footer><p>© @@title@@</p></footer>
</body>
</html>
