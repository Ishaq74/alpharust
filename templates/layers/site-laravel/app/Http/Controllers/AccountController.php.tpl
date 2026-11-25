@@meta when=auth@@
<?php

namespace App\Http\Controllers;

use App\Models\User;
use App\Support\SiteData;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\View\View;

/** Pages de compte du site ; les formulaires sont traités par Fortify (config/fortify.php). */
class AccountController extends Controller
{
    public function signIn(Request $request): View|RedirectResponse
    {
        if ($request->user()) {
            return redirect(SiteData::href(config('site.auth.routes.account')));
        }
        // Erreur de Fortify (champ manquant, identifiants refusés) -> libellé du site, comme Astro et Wagtail.
        $bag = $request->session()->get('errors')?->getBag('default');
        $error = null;
        if ($bag && $bag->any()) {
            $email = trim((string) $request->old('email'));
            $error = trans(match (true) {
                $email === '' || $bag->has('password') => 'site.ui.err_required',
                ! filter_var($email, FILTER_VALIDATE_EMAIL) => 'site.ui.invalid_email',
                default => 'site.ui.err_credentials',
            });
        }

        return view('site.auth', ['page' => SiteData::authPage('sign_in', $error, $request->old())]);
    }

    public function signUp(Request $request): View|RedirectResponse
    {
        if ($request->user()) {
            return redirect(SiteData::href(config('site.auth.routes.account')));
        }
        $bag = $request->session()->get('errors')?->getBag('default');

        $key = $bag?->first();

        return view('site.auth', ['page' => SiteData::authPage('sign_up', $key ? trans($key) : null, $request->old())]);
    }

    public function show(Request $request): View|RedirectResponse
    {
        $user = $request->user();
        if (! $user instanceof User) {
            return redirect(SiteData::href(config('site.auth.routes.sign_in')));
        }

        return view('site.account', ['page' => SiteData::accountPage($user)]);
    }
}
