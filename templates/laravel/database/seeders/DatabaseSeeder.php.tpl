<?php

namespace Database\Seeders;

use App\Models\Page;
use Illuminate\Database\Seeder;

class DatabaseSeeder extends Seeder
{
    public function run(): void
    {
        Page::query()->firstOrCreate(
            ['slug' => 'bienvenue'],
            ['title' => 'Bienvenue', 'body' => 'Bienvenue sur @@title@@.'],
        );
    }
}
