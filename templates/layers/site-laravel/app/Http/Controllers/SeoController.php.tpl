<?php

namespace App\Http\Controllers;

use App\Support\SiteData;
use Illuminate\Http\Response;

/** robots.txt et sitemap.xml (protocole sitemaps.org, alternatives hreflang), comme Astro et Wagtail. */
class SeoController extends Controller
{
    public function robots(): Response
    {
        return response("User-agent: *\nAllow: /\n\nSitemap: ".url('/sitemap.xml')."\n", 200)
            ->header('Content-Type', 'text/plain; charset=utf-8');
    }

    public function sitemap(SiteData $data): Response
    {
        return response()->view('site.sitemap', ['entries' => $data->sitemap()])
            ->header('Content-Type', 'application/xml; charset=utf-8');
    }
}
