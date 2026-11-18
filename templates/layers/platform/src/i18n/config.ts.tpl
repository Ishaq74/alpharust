@@meta when=messages@@
export const LOCALES = [@@locales_ts@@] as const;
export type Locale = (typeof LOCALES)[number];
export const DEFAULT_LOCALE: Locale = '@@default_locale@@';
export const RTL_LOCALES: readonly string[] = ['ar'];

export const LOCALE_LABELS: Record<string, string> = {
  fr: 'Français',
  en: 'English',
  es: 'Español',
  ar: 'العربية',
};

export function isLocale(value: string | undefined): value is Locale {
  return !!value && (LOCALES as readonly string[]).includes(value);
}

export function toLocale(value: string | undefined): Locale {
  return isLocale(value) ? value : DEFAULT_LOCALE;
}

export function dirOf(locale: string): 'rtl' | 'ltr' {
  return RTL_LOCALES.includes(locale) ? 'rtl' : 'ltr';
}

/** `/auth/sign-in` → `/auth/sign-in` pour la langue par défaut, `/en/auth/sign-in` sinon (prefixDefaultLocale: false). */
export function localizedPath(locale: string, path: string): string {
  const normalized = path.startsWith('/') ? path : `/${path}`;
  if (locale === DEFAULT_LOCALE) return normalized;
  return normalized === '/' ? `/${locale}/` : `/${locale}${normalized}`;
}
