<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('posts', function (Blueprint $table) {
            $table->id();
            $table->string('locale', 8)->default('fr');
            $table->string('slug');
            $table->string('title');
            $table->text('excerpt');
            $table->string('category');
            $table->string('author');
            $table->date('published_at');
            $table->longText('body');
            $table->timestamps();
            $table->unique(['locale', 'slug']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('posts');
    }
};
