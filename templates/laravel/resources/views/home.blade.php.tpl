@extends('layouts.app')

@section('title', 'Accueil — @@title@@')

@section('content')
    <h1>@@title@@</h1>
    <p>Projet Laravel généré par AlphaRust.</p>
    <ul>
        @forelse ($pages as $page)
            <li><a href="{{ route('pages.show', $page->slug) }}">{{ $page->title }}</a></li>
        @empty
            <li>Aucune page pour le moment (<code>php artisan db:seed</code>).</li>
        @endforelse
    </ul>
@endsection
