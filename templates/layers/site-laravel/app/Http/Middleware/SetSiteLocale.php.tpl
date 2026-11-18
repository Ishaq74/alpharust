<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\App;
use Symfony\Component\HttpFoundation\Response;

/** Langue d'un groupe de routes du site (`site.locale:en`) : App::setLocale, comme le recommande la doc Laravel. */
class SetSiteLocale
{
    public function handle(Request $request, Closure $next, string $locale): Response
    {
        App::setLocale($locale);

        return $next($request);
    }
}
