@@meta when=auth@@
<?php

namespace Tests\Feature;

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

/** Comptes du site (Fortify) : mêmes pages, adresses et messages qu'Astro et Wagtail. */
class AuthTest extends TestCase
{
    use RefreshDatabase;

    public function test_the_header_links_to_the_sign_in_page(): void
    {
        $this->get('/')->assertOk()->assertSee('href="/auth/sign-in"', false)->assertSee('Connexion');
        $this->get('/en')->assertOk()->assertSee('href="/en/auth/sign-in"', false)->assertSee('Sign in');
    }

    public function test_auth_pages_render_in_the_site_layout(): void
    {
        $this->get('/auth/sign-in')->assertOk()->assertSee('Accédez à votre espace client')->assertSee('site-header', false);
        $this->get('/auth/sign-up')->assertOk()->assertSee('Créer mon compte')->assertSee('autocomplete="new-password"', false);
        $this->get('/en/auth/sign-up')->assertOk()->assertSee('Create my account');
        $this->get('/auth/dashboard')->assertRedirect('/auth/sign-in');
    }

    public function test_sign_up_sign_out_and_sign_in(): void
    {
        $this->post('/auth/sign-up', ['name' => 'Ada', 'email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'fr'])
            ->assertRedirect('/auth/dashboard');
        $this->assertAuthenticated();
        $this->get('/auth/dashboard')->assertOk()->assertSee('ada@example.com')->assertSee('href="/auth/dashboard"', false);
        $this->post('/auth/sign-out', ['locale' => 'en'])->assertRedirect('/en');
        $this->assertGuest();
        $this->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'en'])
            ->assertRedirect('/en/auth/dashboard');
        $this->assertAuthenticated();
    }

    public function test_errors_use_the_site_messages(): void
    {
        User::factory()->create(['email' => 'ada@example.com']);
        $this->from('/auth/sign-in')->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'mauvais'])
            ->assertRedirect('/auth/sign-in');
        $this->get('/auth/sign-in')->assertSee('E-mail ou mot de passe incorrect.');
        $this->from('/auth/sign-up')->post('/auth/sign-up', ['name' => 'Ada', 'email' => 'ada@example.com', 'password' => 'motdepasse']);
        $this->get('/auth/sign-up')->assertSee('Un compte existe déjà avec cette adresse.');
        $this->from('/auth/sign-up')->post('/auth/sign-up', ['name' => 'Bob', 'email' => 'bob@example.com', 'password' => 'court']);
        $this->get('/auth/sign-up')->assertSee('Le mot de passe doit contenir au moins 8 caractères.');
    }
}
