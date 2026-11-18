@@meta when=auth@@
<?php

namespace App\Http\Responses;

use App\Support\SiteData;
use Illuminate\Http\RedirectResponse;
use Laravel\Fortify\Contracts\LoginResponse;
use Laravel\Fortify\Contracts\LogoutResponse;
use Laravel\Fortify\Contracts\RegisterResponse;

/** Réponses de Fortify : « Mon compte » après connexion ou inscription, l'accueil après déconnexion, dans la langue du formulaire. */
class SiteAuthResponse implements LoginResponse, LogoutResponse, RegisterResponse
{
    public function toResponse($request): RedirectResponse
    {
        $locale = (string) $request->input('locale');
        $locale = array_key_exists($locale, config('site.locales')) ? $locale : config('site.locale');
        $to = $request->routeIs('logout') ? '/' : config('site.auth.routes.account');

        return redirect(SiteData::href($to, $locale));
    }
}
