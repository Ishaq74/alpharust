<?php

namespace App\Http\Controllers;

use App\Support\SiteData;
use Illuminate\View\View;

class SiteController extends Controller
{
    public function __construct(private SiteData $data) {}

    public function home(): View
    {
        return view('site.page', ['page' => ['title' => null] + $this->data->page('home')]);
    }

    /** Pages composées de sections : à propos, mentions légales, confidentialité, index services/blog. */
    public function page(string $key): View
    {
        return view('site.page', ['page' => $this->data->page($key)]);
    }
}
