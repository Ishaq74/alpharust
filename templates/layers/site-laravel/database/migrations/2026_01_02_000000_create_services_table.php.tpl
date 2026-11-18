<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('services', function (Blueprint $table) {
            $table->id();
            $table->string('locale', 8)->default('fr');
            $table->string('slug');
            $table->string('title');
            $table->text('summary');
            $table->string('icon');
            $table->string('price');
            $table->json('features');
            $table->longText('body');
            $table->unsignedInteger('position')->default(0);
            $table->timestamps();
            $table->unique(['locale', 'slug']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('services');
    }
};
