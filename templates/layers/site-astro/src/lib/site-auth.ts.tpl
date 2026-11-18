@@meta mode=replace when=vitrine&auth@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// Comptes du site avec Better Auth (API serveur : aucun JavaScript client), comme Fortify (Laravel) et l'auth Django.
import type { AstroGlobal } from 'astro';
import { APIError } from 'better-auth/api';
import { auth } from '@lib/auth';

/** Redirection 303 qui transmet les cookies de session posés par Better Auth. */
function redirect(location: string, source?: Headers): Response {
  const headers = new Headers({ Location: location });
  for (const cookie of source?.getSetCookie() ?? []) headers.append('Set-Cookie', cookie);
  return new Response(null, { status: 303, headers });
}

/** Clé de libellé (site.ui) d'une erreur Better Auth. */
function errorKey(err: unknown, fallback: string): string {
  if (err instanceof APIError) {
    const code = String((err.body as { code?: string } | undefined)?.code ?? '');
    if (code.includes('USER_ALREADY_EXISTS')) return 'err_exists';
    if (code.includes('PASSWORD_TOO_SHORT')) return 'err_password';
    if (code.includes('INVALID_EMAIL_OR_PASSWORD') || code.includes('INVALID_PASSWORD')) return 'err_credentials';
    if (code.includes('INVALID_EMAIL')) return 'invalid_email';
  }
  return fallback;
}

export type AuthResult = { response?: Response; error?: string };

export async function signIn(astro: AstroGlobal, values: Record<string, string>, next: string): Promise<AuthResult> {
  try {
    const { headers } = await auth.api.signInEmail({
      body: { email: values.email.trim(), password: values.password },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(next, headers) };
  } catch (err) {
    return { error: errorKey(err, 'err_credentials') };
  }
}

export async function signUp(astro: AstroGlobal, values: Record<string, string>, next: string): Promise<AuthResult> {
  try {
    const { headers } = await auth.api.signUpEmail({
      body: { name: values.name.trim(), email: values.email.trim(), password: values.password },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(next, headers) };
  } catch (err) {
    return { error: errorKey(err, 'err_generic') };
  }
}

export async function signOut(astro: AstroGlobal, next: string): Promise<Response> {
  try {
    const { headers } = await auth.api.signOut({ headers: astro.request.headers, returnHeaders: true });
    return redirect(next, headers);
  } catch {
    return redirect(next);
  }
}
