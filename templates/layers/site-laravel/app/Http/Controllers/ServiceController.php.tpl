<?php

namespace App\Http\Controllers;

use App\Models\Service;
use App\Support\SiteData;
use Illuminate\View\View;

class ServiceController extends Controller
{
    public function show(Service $service): View
    {
        return view('site.service', ['page' => $service->toCard() + [
            'description' => $service->summary,
            'body' => $service->body,
            'crumbs' => SiteData::crumbs('Services', '/services'),
        ]]);
    }
}
