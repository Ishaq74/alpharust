@@meta when=auth@@
<?php

namespace App\Actions\Fortify;

use App\Models\User;
use Illuminate\Support\Facades\Hash;
use Illuminate\Validation\ValidationException;
use Laravel\Fortify\Contracts\CreatesNewUsers;

/** Inscription (Fortify) : mêmes contrôles et messages qu'Astro (Better Auth) et Wagtail, dans le même ordre. */
class CreateNewUser implements CreatesNewUsers
{
    /** @param  array<string, mixed>  $input */
    public function create(array $input): User
    {
        $value = fn (string $k) => trim((string) ($input[$k] ?? ''));
        // Clé du libellé, traduite à l'affichage (AccountController) dans la langue de la page du formulaire.
        $fail = fn (string $key) => throw ValidationException::withMessages(['form' => 'site.ui.'.$key]);
        foreach (config('site.auth.fields.sign_up') as $f) {
            if ($value($f['name']) === '') {
                $fail('err_required');
            }
        }
        if (! filter_var($value('email'), FILTER_VALIDATE_EMAIL)) {
            $fail('invalid_email');
        }
        if (mb_strlen((string) $input['password']) < config('site.auth.password_min')) {
            $fail('err_password');
        }
        $email = mb_strtolower($value('email'));
        if (User::query()->where('email', $email)->exists()) {
            $fail('err_exists');
        }

        return User::create([
            'name' => $value('name'),
            'email' => $email,
            'password' => Hash::make((string) $input['password']),
        ]);
    }
}
