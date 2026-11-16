@@meta when=auth@@
import type { AstroGlobal } from 'astro';
import { DEFAULT_LOCALE, localizedPath } from '@i18n/config';

/** Exige un utilisateur connecté et non banni ; sinon redirige vers la connexion. */
export function requireAuth(astro: AstroGlobal) {
  const user = astro.locals.user;
  if (!user || (user as { banned?: boolean | null }).banned) {
    return { redirect: astro.redirect(localizedPath(astro.currentLocale ?? DEFAULT_LOCALE, '/auth/sign-in')) };
  }
  return { user };
}

@@if plugin_admin@@
/** Exige le rôle admin ; un utilisateur simple est renvoyé vers son tableau de bord. */
export function requireAdmin(astro: AstroGlobal) {
  const base = requireAuth(astro);
  if ('redirect' in base) return base;
  if ((base.user as { role?: string | null }).role !== 'admin') {
    return { redirect: astro.redirect(localizedPath(astro.currentLocale ?? DEFAULT_LOCALE, '/auth/dashboard')) };
  }
  return base;
}
@@end@@
