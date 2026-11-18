<?php

namespace App\Http\Controllers;

use App\Models\ContactMessage;
use App\Support\SiteData;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\View\View;

class ContactController extends Controller
{
    public function __construct(private SiteData $data) {}

    public function create(Request $request): View
    {
        $bag = $request->session()->get('errors');
        $errors = $bag ? array_map(fn (array $m) => $m[0], $bag->getBag('default')->messages()) : [];

        return view('site.page', ['page' => $this->data->page('contact', [
            'sent' => $request->session()->get('status') === 'sent',
            'values' => $request->old(),
            'errors' => $errors,
        ])]);
    }

    public function store(Request $request): RedirectResponse
    {
        $rules = [];
        foreach (config('site.contact_fields') as $f) {
            $rules[$f['name']] = array_filter([
                $f['required'] ? 'required' : 'nullable', 'string', 'max:5000', $f['type'] === 'email' ? 'email' : null,
            ]);
        }
        $data = $request->validate($rules, [
            'required' => 'Ce champ est obligatoire.',
            'email' => 'Adresse e-mail invalide.',
        ]);
        ContactMessage::create(['data' => $data]);

        return redirect(SiteData::href('/contact'))->with('status', 'sent');
    }
}
