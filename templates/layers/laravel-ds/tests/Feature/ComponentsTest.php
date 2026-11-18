<?php

namespace Tests\Feature;

use Tests\TestCase;

class ComponentsTest extends TestCase
{
    public function test_the_component_catalog_renders_every_slot_of_the_contract(): void
    {
        $this->withoutVite();

        $response = $this->get('/components');

        $response->assertOk();
        foreach (['button', 'card', 'alert', 'tabs', 'dialog-content', 'native-select', 'checkbox-input', 'switch-button', 'table', 'pagination-link'] as $slot) {
            $response->assertSee('data-slot="'.$slot.'"', false);
        }
    }

    public function test_alert_role_follows_the_variant(): void
    {
        $this->withoutVite();

        $html = $this->get('/components')->getContent();

        $this->assertMatchesRegularExpression('/role="alert"[^>]*data-slot="alert"[^>]*data-variant="error"/', $html);
        $this->assertMatchesRegularExpression('/role="status"[^>]*data-slot="alert"[^>]*data-variant="info"/', $html);
    }

    public function test_button_renders_a_link_when_href_is_given(): void
    {
        $this->withoutVite();

        $this->get('/components')->assertSee('<a data-slot="button"', false);
    }

    public function test_the_advanced_catalog_renders_the_23_extended_components(): void
    {
        $this->withoutVite();

        $response = $this->get('/components/advanced');

        $response->assertOk();
        foreach (['alert-dialog', 'aspect-ratio', 'button-group', 'carousel', 'collapsible', 'container', 'dropdown', 'dropzone', 'image', 'input-group', 'input-otp', 'item', 'popover', 'prose', 'radio-group', 'select', 'sheet', 'sidebar', 'slider', 'theme-toggle', 'toast', 'toggle', 'video'] as $slot) {
            $response->assertSee('data-slot="'.$slot.'"', false);
        }
        // Sous-parties : en-têtes, contenus, déclencheurs (même balisage que les atomes Astro)
        foreach (['alert-dialog-content', 'select-content', 'select-item', 'sheet-content', 'sidebar-menu-button', 'toast-viewport', 'input-otp-slot', 'carousel-next'] as $slot) {
            $response->assertSee('data-slot="'.$slot.'"', false);
        }
    }

    public function test_extended_components_keep_their_accessibility_contract(): void
    {
        $this->withoutVite();

        $html = $this->get('/components/advanced')->getContent();

        // libellé passé par l'appelant, valeurs du curseur, page courante et vidéo sans cookies
        $this->assertMatchesRegularExpression('/role="combobox"[^>]*aria-label="Fruit"/', $html);
        $this->assertMatchesRegularExpression('/data-slot="slider-thumb"[^>]*role="slider"[^>]*aria-valuenow="40"/', $html);
        $this->assertMatchesRegularExpression('/<a [^>]*data-slot="sidebar-menu-button"[^>]*aria-current="page"/', $html);
        $this->assertStringContainsString('youtube-nocookie.com/embed/aqz-KE-bpKQ', $html);
        $this->assertStringContainsString('<template data-toast-template="success"', $html);
    }
}
