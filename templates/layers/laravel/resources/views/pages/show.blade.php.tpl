<x-site-layout :title="$page->title.' — @@title@@'">
    <h1>{{ $page->title }}</h1>
    <p>{{ $page->body }}</p>
    <p><a href="{{ route('pages.index') }}">← Toutes les pages</a></p>
</x-site-layout>
