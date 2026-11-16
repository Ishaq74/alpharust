<?php

namespace Tests\Feature;

use App\Models\Page;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class PageTest extends TestCase
{
    use RefreshDatabase;

    public function test_home_page_responds(): void
    {
        $this->get('/')->assertStatus(200)->assertSee('@@title@@');
    }

    public function test_page_is_displayed_by_slug(): void
    {
        Page::create(['title' => 'Contact', 'slug' => 'contact', 'body' => 'Écrivez-nous.']);

        $this->get('/pages/contact')->assertStatus(200)->assertSee('Écrivez-nous.');
    }

    public function test_unknown_page_is_404(): void
    {
        $this->get('/pages/inconnue')->assertStatus(404);
    }
}
