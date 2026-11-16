<?php

namespace Tests\Feature;

use App\Models\Page;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class PageTest extends TestCase
{
    use RefreshDatabase;

    public function test_index_lists_pages(): void
    {
        Page::factory()->create(['title' => 'Contact', 'slug' => 'contact']);

        $this->get('/pages')->assertOk()->assertSee('Contact');
    }

    public function test_page_is_displayed_by_slug(): void
    {
        Page::create(['title' => 'Contact', 'slug' => 'contact', 'body' => 'Écrivez-nous.']);

        $this->get('/pages/contact')->assertOk()->assertSee('Écrivez-nous.');
    }

    public function test_unknown_page_is_404(): void
    {
        $this->get('/pages/inconnue')->assertNotFound();
    }
}
