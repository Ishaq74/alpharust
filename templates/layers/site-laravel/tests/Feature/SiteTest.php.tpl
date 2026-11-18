<?php

namespace Tests\Feature;

use App\Models\ContactMessage;
use Database\Seeders\SiteSeeder;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class SiteTest extends TestCase
{
    use RefreshDatabase;

    protected function setUp(): void
    {
        parent::setUp();
        $this->seed(SiteSeeder::class);
    }

    public static function pages(): array
    {
        return [
            'accueil' => ['/', 'Des sites rapides, accessibles'],
            'à propos' => ['/a-propos', 'Une petite équipe'],
            'services' => ['/services', 'Ce que nous faisons'],
            'service' => ['/services/accessibilite', 'Ce qui est inclus'],
            'blog' => ['/blog', 'Articles et conseils'],
            'article' => ['/blog/choisir-son-framework', 'Tous les articles'],
            'contact' => ['/contact', 'Parlons de votre projet'],
            'mentions légales' => ['/mentions-legales', 'Éditeur'],
            'confidentialité' => ['/confidentialite', 'Vos droits'],
        ];
    }

    /** @dataProvider pages */
    #[\PHPUnit\Framework\Attributes\DataProvider('pages')]
    public function test_every_page_renders_inside_the_shared_layout(string $url, string $text): void
    {
        $this->get($url)
            ->assertOk()
            ->assertSee($text)
            ->assertSee('href="#contenu"', false)
            ->assertSee('aria-label="Navigation principale"', false)
            ->assertSee('data-slot="theme-toggle"', false)
            ->assertSee('data-slot="sheet-content"', false)
            ->assertSee('<footer class="site-footer">', false);
    }

    public function test_the_current_page_is_marked_in_the_menu(): void
    {
        $this->get('/blog')->assertSee('href="/blog" aria-current="page"', false);
        $this->get('/services/maintenance')->assertSee('href="/services/maintenance" aria-current="page"', false);
    }

    public function test_unknown_pages_use_the_site_404(): void
    {
        $this->get('/nexiste-pas')->assertNotFound()->assertSee('Page introuvable')->assertSee('site-header', false);
        $this->get('/services/inconnu')->assertNotFound();
    }

    public function test_contact_form_validates_and_stores_messages(): void
    {
        $this->post('/contact', ['nom' => '', 'email' => 'x'])->assertRedirect()->assertSessionHasErrors(['nom', 'email', 'message']);
        $this->post('/contact', ['nom' => 'Ada', 'email' => 'ada@example.com', 'message' => 'Bonjour'])
            ->assertRedirect('/contact');
        $this->assertSame(1, ContactMessage::count());
        $this->withSession(['status' => 'sent'])->get('/contact')->assertSee('Votre message a bien été envoyé');
    }
}
