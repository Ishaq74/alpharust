<?php

namespace App\View\Composers;

use App\Support\SiteData;
use Illuminate\View\View;

/** Partage `$site` (navigation, pied de page, coordonnées) avec le gabarit et les vues du site. */
class SiteComposer
{
    public function __construct(private SiteData $data) {}

    public function compose(View $view): void
    {
        $view->with('site', $this->data->site(request()->path()));
    }
}
