@@meta when=!official@@
export const SITE_TITLE = '@@title@@';
export const SITE_DESCRIPTION = 'Site Astro créé avec AlphaRust.';

export const NAV_LINKS = [
  { href: '/', label: 'Accueil' },
  { href: '/blog/', label: 'Blog' },
  { href: '/about/', label: 'À propos' },
] as const;
