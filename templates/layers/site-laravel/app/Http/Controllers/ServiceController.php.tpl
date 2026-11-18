<?php

namespace App\Http\Controllers;

use App\Models\Service;
use App\Support\SiteData;
use Illuminate\Support\Facades\App;
use Illuminate\View\View;

class ServiceController extends Controller
{
    public function show(string $slug): View
    {
        $service = Service::query()->where('locale', App::getLocale())->where('slug', $slug)->firstOrFail();

        return view('site.service', ['page' => $service->toCard() + [
            'description' => $service->summary,
            'body' => $service->body,
            'crumbs' => SiteData::crumbs(trans('site.ui.services'), '/services'),
        ]]);
    }
}
