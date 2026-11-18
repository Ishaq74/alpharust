<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

/** Message envoyé depuis /contact (équivalent des FormSubmission de Wagtail). */
class ContactMessage extends Model
{
    protected $fillable = ['data'];

    protected function casts(): array
    {
        return ['data' => 'array'];
    }
}
