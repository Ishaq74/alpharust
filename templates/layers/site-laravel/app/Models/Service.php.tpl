<?php

namespace App\Models;

use App\Support\SiteData;
use Illuminate\Database\Eloquent\Model;

class Service extends Model
{
    protected $fillable = ['locale', 'slug', 'title', 'summary', 'icon', 'price', 'features', 'body', 'position'];

    protected function casts(): array
    {
        return ['features' => 'array', 'position' => 'integer'];
    }

    /** @return array<string, mixed> */
    public function toCard(): array
    {
        $url = SiteData::href('/services/'.$this->slug);

        return [
            'title' => $this->title,
            'url' => $url,
            'summary' => $this->summary,
            'icon' => $this->icon,
            'price' => $this->price,
            'features' => $this->features ?? [],
            'more_label' => trans('site.ui.more_label').$this->title,
        ];
    }
}
