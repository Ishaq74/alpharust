<x-site-layout title="Pages — @@title@@">
    <h1>Pages</h1>
    <ul>
        @forelse ($pages as $page)
            <li><a href="{{ route('pages.show', $page->slug) }}">{{ $page->title }}</a></li>
        @empty
            <li>Aucune page pour le moment (<code>php artisan db:seed --class=PageSeeder</code>).</li>
        @endforelse
    </ul>
</x-site-layout>
