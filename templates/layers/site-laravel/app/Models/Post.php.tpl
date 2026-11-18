<?php

namespace App\Models;

use App\Support\SiteData;
use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    protected $fillable = ['slug', 'title', 'excerpt', 'category', 'author', 'published_at', 'body'];

    protected function casts(): array
    {
        return ['published_at' => 'date'];
    }

    /** @return array<string, mixed> */
    public function toCard(): array
    {
        return [
            'title' => $this->title,
            'url' => SiteData::href('/blog/'.$this->slug),
            'excerpt' => $this->excerpt,
            'category' => $this->category,
            'author' => $this->author,
            'date_iso' => $this->published_at->toDateString(),
            'date_label' => $this->published_at->locale('fr')->translatedFormat('j F Y'),
        ];
    }
}
