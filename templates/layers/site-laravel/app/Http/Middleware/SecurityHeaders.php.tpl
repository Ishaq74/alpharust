<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * En-têtes de sécurité du site, comme Astro (src/middleware.ts) et Wagtail (settings/production.py).
 * Branché sur le groupe web par bootstrap/app.php (`$middleware->web(append: [...])`).
 * La CSP n'est envoyée qu'en production : le serveur Vite de développement (npm run dev) sert depuis un autre port.
 */
class SecurityHeaders
{
    public const CSP = "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https:; font-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'self'";

    public function handle(Request $request, Closure $next): Response
    {
        $response = $next($request);
        $response->headers->set('X-Content-Type-Options', 'nosniff');
        $response->headers->set('Referrer-Policy', 'same-origin');
        if (app()->isProduction()) {
            $response->headers->set('Content-Security-Policy', self::CSP);
        }

        return $response;
    }
}
