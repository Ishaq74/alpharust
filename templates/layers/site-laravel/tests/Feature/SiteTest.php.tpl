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

    public function test_security_headers_and_csp_in_production(): void
    {
        $this->get('/')->assertHeader('X-Content-Type-Options', 'nosniff')->assertHeaderMissing('Content-Security-Policy');
        $this->app->detectEnvironment(fn () => 'production');
        $this->get('/')->assertHeader('Content-Security-Policy', \App\Http\Middleware\SecurityHeaders::CSP);
    }

    public function test_seo_robots_sitemap_and_structured_data(): void
    {
        $this->get('/robots.txt')
            ->assertOk()
            ->assertSee('Sitemap: '.url('/sitemap.xml'), false);
        $this->get('/sitemap.xml')
            ->assertOk()
            ->assertHeader('Content-Type', 'application/xml; charset=utf-8')
            ->assertSee('<loc>'.url('/blog/choisir-son-framework').'</loc>', false);
        $this->get('/blog/choisir-son-framework')
            ->assertOk()
            ->assertSee('<link rel="canonical" href="'.url('/blog/choisir-son-framework').'">', false)
            ->assertSee('"@type":"Organization"', false)
            ->assertSee('"@type":"BlogPosting"', false);
    }

@@if multilingual@@
    public static function translatedPages(): array
    {
        return [
@@if lang_en@@
            'en home' => ['en', 'ltr', '/en', 'Fast, accessible websites that are easy to grow', 'Main navigation'],
            'en about' => ['en', 'ltr', '/en/a-propos', 'A small team with high standards', 'Main navigation'],
            'en service' => ['en', 'ltr', '/en/services/accessibilite', 'What’s included', 'Main navigation'],
            'en article' => ['en', 'ltr', '/en/blog/choisir-son-framework', 'All articles', 'Main navigation'],
            'en contact' => ['en', 'ltr', '/en/contact', 'Let’s talk about your project', 'Main navigation'],
@@end@@
@@if lang_es@@
            'es home' => ['es', 'ltr', '/es', 'Sitios rápidos, accesibles y fáciles de hacer evolucionar', 'Navegación principal'],
            'es about' => ['es', 'ltr', '/es/a-propos', 'Un equipo pequeño, con grandes exigencias', 'Navegación principal'],
            'es service' => ['es', 'ltr', '/es/services/accessibilite', 'Qué incluye', 'Navegación principal'],
            'es article' => ['es', 'ltr', '/es/blog/choisir-son-framework', 'Todos los artículos', 'Navegación principal'],
            'es contact' => ['es', 'ltr', '/es/contact', 'Hablemos de su proyecto', 'Navegación principal'],
@@end@@
@@if lang_ar@@
            'ar home' => ['ar', 'rtl', '/ar', 'مواقع سريعة وسهلة الوصول ويسهل تطويرها', 'التنقل الرئيسي'],
            'ar about' => ['ar', 'rtl', '/ar/a-propos', 'فريق صغير بمعايير عالية', 'التنقل الرئيسي'],
            'ar service' => ['ar', 'rtl', '/ar/services/accessibilite', 'ما تتضمنه الخدمة', 'التنقل الرئيسي'],
            'ar article' => ['ar', 'rtl', '/ar/blog/choisir-son-framework', 'جميع المقالات', 'التنقل الرئيسي'],
            'ar contact' => ['ar', 'rtl', '/ar/contact', 'لنتحدث عن مشروعكم', 'التنقل الرئيسي'],
@@end@@
        ];
    }

    /** @dataProvider translatedPages */
    #[\PHPUnit\Framework\Attributes\DataProvider('translatedPages')]
    public function test_translated_pages(string $locale, string $dir, string $url, string $text, string $nav): void
    {
        $this->get($url)
            ->assertOk()
            ->assertSee('<html lang="'.$locale.'" dir="'.$dir.'">', false)
            ->assertSee($text)
            ->assertSee('aria-label="'.$nav.'"', false)
            ->assertSee('hreflang="fr"', false);
    }

    public static function switcherPages(): array
    {
        return [
@@if lang_en@@
            'en' => ['en', 'Page not found'],
@@end@@
@@if lang_es@@
            'es' => ['es', 'Página no encontrada'],
@@end@@
@@if lang_ar@@
            'ar' => ['ar', 'الصفحة غير موجودة'],
@@end@@
        ];
    }

    /** @dataProvider switcherPages */
    #[\PHPUnit\Framework\Attributes\DataProvider('switcherPages')]
    public function test_the_language_switcher_links_to_the_same_page(string $locale, string $notFound): void
    {
        $this->get('/blog')->assertSee('href="/'.$locale.'/blog"', false);
        $this->get('/'.$locale.'/blog')->assertSee('href="/blog"', false)
            ->assertSee('href="/'.$locale.'/blog" aria-current="page"', false);
        $this->get('/'.$locale.'/nope')->assertNotFound()->assertSee($notFound);
    }
@@end@@

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
