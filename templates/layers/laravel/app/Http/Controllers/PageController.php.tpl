<?php

namespace App\Http\Controllers;

use App\Models\Page;
use Illuminate\View\View;

class PageController extends Controller
{
    public function index(): View
    {
        return view('pages.index', [
            'pages' => Page::query()->orderBy('title')->get(),
        ]);
    }

    public function show(string $slug): View
    {
        $page = Page::query()->where('slug', $slug)->firstOrFail();

        return view('pages.show', ['page' => $page]);
    }
}
