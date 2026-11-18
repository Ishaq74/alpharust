<?php

namespace App\Http\Controllers;

use App\Models\Post;
use App\Support\SiteData;
use Illuminate\View\View;

class BlogController extends Controller
{
    public function show(Post $post): View
    {
        return view('site.post', ['page' => $post->toCard() + [
            'description' => $post->excerpt,
            'body' => $post->body,
            'crumbs' => SiteData::crumbs('Blog', '/blog'),
            'back_url' => SiteData::href('/blog'),
        ]]);
    }
}
