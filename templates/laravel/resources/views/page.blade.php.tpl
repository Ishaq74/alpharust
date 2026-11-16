@extends('layouts.app')

@section('title', $page->title.' — @@title@@')

@section('content')
    <h1>{{ $page->title }}</h1>
    <p>{{ $page->body }}</p>
@endsection
