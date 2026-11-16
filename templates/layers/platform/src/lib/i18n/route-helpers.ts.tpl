@@meta when=concordia@@
/**
 * Adaptation AlphaRust de `@lib/i18n/route-helpers` (Concordia) : le composant `PageBreadcrumb` appelle
 * `getLocalizedUrl(locale, path)`. Ici la version s'appuie sur le routage natif d'Astro
 * (`prefixDefaultLocale: false`) au lieu de la table de slugs propre à l'application Concordia.
 */
@@if messages@@
import { localizedPath } from '@i18n/config';

export function getLocalizedUrl(locale: string, path: string): string {
  return localizedPath(locale, path);
}
@@else@@
export function getLocalizedUrl(_locale: string, path: string): string {
  return path.startsWith('/') ? path : `/${path}`;
}
@@end@@
