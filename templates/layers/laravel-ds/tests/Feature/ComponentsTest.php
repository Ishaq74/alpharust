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
}
