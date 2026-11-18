@@meta when=auth@@
import type { AstroGlobal } from 'astro';
import { APIError } from 'better-auth/api';
import { auth } from '@lib/auth';
import { DEFAULT_LOCALE, localizedPath } from '@i18n/config';
import { getMessages, type Messages } from '@i18n/utils';

export interface FormResult {
  /** Réponse à renvoyer telle quelle (redirection + cookies de session). */
  response?: Response;
  /** Message d'erreur déjà traduit. */
  error?: string;
  /** Valeurs à ré-afficher (jamais le mot de passe). */
  values: Record<string, string>;
}

type Errors = Messages['auth']['errors'];

/** Redirection 303 qui transmet les cookies posés par Better Auth. */
function redirect(location: string, source?: Headers): Response {
  const headers = new Headers({ Location: location });
  for (const cookie of source?.getSetCookie() ?? []) headers.append('Set-Cookie', cookie);
  return new Response(null, { status: 303, headers });
}

function translate(err: unknown, errors: Errors): string {
  if (err instanceof APIError) {
    const code = String((err.body as { code?: string } | undefined)?.code ?? '');
    if (code.includes('INVALID_EMAIL_OR_PASSWORD') || code.includes('INVALID_PASSWORD')) return errors.invalidCredentials;
    if (code.includes('USER_ALREADY_EXISTS')) return errors.userExists;
    if (code.includes('PASSWORD_TOO_SHORT')) return errors.passwordTooShort;
    if (code.includes('INVALID_EMAIL')) return errors.invalidEmail;
    if (err.statusCode === 429) return errors.rateLimited;
  }
  return errors.generic;
}

function text(form: FormData, key: string): string {
  const v = form.get(key);
  return typeof v === 'string' ? v.trim() : '';
}

/** Traite le formulaire de connexion (POST) côté serveur : aucun JavaScript client n'est requis. */
export async function handleSignIn(astro: AstroGlobal): Promise<FormResult> {
  const locale = astro.currentLocale ?? DEFAULT_LOCALE;
  const t = getMessages(locale);
  if (astro.locals.user) return { response: redirect(localizedPath(locale, '/auth/dashboard')), values: {} };
  if (astro.request.method !== 'POST') return { values: {} };

  const form = await astro.request.formData();
  const email = text(form, 'email');
  const password = String(form.get('password') ?? '');
  try {
    const { headers } = await auth.api.signInEmail({
      body: { email, password },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(localizedPath(locale, '/auth/dashboard'), headers), values: {} };
  } catch (err) {
    return { error: translate(err, t.auth.errors), values: { email } };
  }
}

/** Traite le formulaire d'inscription (POST) : crée le compte et ouvre la session. */
export async function handleSignUp(astro: AstroGlobal): Promise<FormResult> {
  const locale = astro.currentLocale ?? DEFAULT_LOCALE;
  const t = getMessages(locale);
  if (astro.locals.user) return { response: redirect(localizedPath(locale, '/auth/dashboard')), values: {} };
  if (astro.request.method !== 'POST') return { values: {} };

  const form = await astro.request.formData();
  const name = text(form, 'name');
  const email = text(form, 'email');
@@if plugin_username@@
  const username = text(form, 'username');
@@end@@
  const password = String(form.get('password') ?? '');
  try {
    const { headers } = await auth.api.signUpEmail({
      body: { name, email, password@@if plugin_username@@, username@@end@@ },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(localizedPath(locale, '/auth/dashboard'), headers), values: {} };
  } catch (err) {
    return { error: translate(err, t.auth.errors), values: { name, email@@if plugin_username@@, username@@end@@ } };
  }
}

/** Déconnexion (POST) : révoque la session et efface le cookie. */
export async function handleSignOut(astro: AstroGlobal): Promise<Response | null> {
  if (astro.request.method !== 'POST') return null;
  const locale = astro.currentLocale ?? DEFAULT_LOCALE;
  try {
    const { headers } = await auth.api.signOut({ headers: astro.request.headers, returnHeaders: true });
    return redirect(localizedPath(locale, '/'), headers);
  } catch {
    return redirect(localizedPath(locale, '/'));
  }
}
