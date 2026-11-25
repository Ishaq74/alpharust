<?php

namespace App\View\Composers;

use App\Support\SiteData;
use Illuminate\Support\Facades\App;
use Illuminate\View\View;

/** Partage `$site` (navigation, pied de page, coordonnées) avec le gabarit et les vues du site. */
class SiteComposer
{
    public function __construct(private SiteData $data) {}

    public function compose(View $view): void
    {
        // Les pages d'erreur (404) ne passent pas par le middleware de langue : la langue suit l'adresse.
        App::setLocale(SiteData::localeOf(request()->path()));
        $view->with('site', $this->data->site(request()->path()));
    }
}
