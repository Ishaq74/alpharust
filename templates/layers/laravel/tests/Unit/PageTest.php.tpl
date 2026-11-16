<?php

namespace Tests\Unit;

use App\Models\Page;
use PHPUnit\Framework\TestCase;

class PageTest extends TestCase
{
    public function test_fillable_attributes(): void
    {
        $this->assertSame(['title', 'slug', 'body'], (new Page)->getFillable());
    }
}
