@@meta mode=replace@@
/*
 * Feuille d'entrée Vite (laravel-vite-plugin) — Tailwind CSS v4 + kit Starwind (classes des atomes d'atomic).
 * Même ordre qu'Astro + Starwind : jetons, thème de marque, base, composants, puis gabarit du site.
 * alpharust-ignore: no-hardcoded-urls (liens de documentation)
 * Docs : https://laravel.com/docs/vite#working-with-stylesheets · https://tailwindcss.com/docs/installation/framework-guides/laravel/vite
 */
@import 'tailwindcss';
@import './starwind/theme.css';
@layer theme, base, components, utilities, overrides;

@import './starwind/colors.css';
@import './design/tokens/typography.css';
@import './design/tokens/spacing.css';
@import './design/tokens/site.css';
@import './design/themes/theme.css';
@import './design/base.css' layer(base);
@import './starwind/base.css';
@import './starwind/components.css';
@import './site.css'; /* gabarit du site (en-tête, pied de page, sections) : identique à Astro et Wagtail */

@source '../../vendor/laravel/framework/src/Illuminate/Pagination/resources/views/*.blade.php';
@source '../../storage/framework/views/*.php';
