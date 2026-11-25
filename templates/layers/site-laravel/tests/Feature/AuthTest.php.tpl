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
@@if lang_en@@
        $this->get('/en')->assertOk()->assertSee('href="/en/auth/sign-in"', false)->assertSee('Sign in');
@@end@@
@@if lang_ar@@
        $this->get('/ar')->assertOk()->assertSee('href="/ar/auth/sign-in"', false)->assertSee('تسجيل الدخول');
@@end@@
    }

    public function test_auth_pages_render_in_the_site_layout(): void
    {
        $this->get('/auth/sign-in')->assertOk()->assertSee('Accédez à votre espace client')->assertSee('site-header', false);
        $this->get('/auth/sign-up')->assertOk()->assertSee('Créer mon compte')->assertSee('autocomplete="new-password"', false);
@@if lang_en@@
        $this->get('/en/auth/sign-up')->assertOk()->assertSee('Create my account');
@@end@@
@@if lang_es@@
        $this->get('/es/auth/sign-up')->assertOk()->assertSee('Crear mi cuenta');
@@end@@
        $this->get('/auth/dashboard')->assertRedirect('/auth/sign-in');
    }

    public function test_sign_up_sign_out_and_sign_in(): void
    {
        $this->post('/auth/sign-up', ['name' => 'Ada', 'email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'fr'])
            ->assertRedirect('/auth/dashboard');
        $this->assertAuthenticated();
        $this->get('/auth/dashboard')->assertOk()->assertSee('ada@example.com')->assertSee('href="/auth/dashboard"', false);
@@if lang_en@@
        $this->post('/auth/sign-out', ['locale' => 'en'])->assertRedirect('/en');
        $this->assertGuest();
        $this->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'en'])
            ->assertRedirect('/en/auth/dashboard');
@@else@@
        $this->post('/auth/sign-out', ['locale' => 'fr'])->assertRedirect('/');
        $this->assertGuest();
        $this->post('/auth/sign-in', ['email' => 'ada@example.com', 'password' => 'motdepasse', 'locale' => 'fr'])
            ->assertRedirect('/auth/dashboard');
@@end@@
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
