/*
 * Entrée Tailwind CSS v4 du kit Starwind (Wagtail), compilée par la CLI autonome de Tailwind :
 *   .tailwind/tailwindcss -i tailwind/input.css -o @@pkg@@/static/css/tailwind.css --minify   (ajouter --watch en dev)
 * alpharust-ignore: no-hardcoded-urls (liens de documentation)
 * Doc : https://tailwindcss.com/blog/standalone-cli · https://tailwindcss.com/docs/installation/tailwind-cli
 * base.html charge la sortie avant css/site.css (gabarit du site, identique à Astro et Laravel).
 */
@import 'tailwindcss';
@import './starwind/theme.css';
@layer theme, base, components, utilities, overrides;

@import './starwind/colors.css';
@import '../ds/static/ds/css/tokens/typography.css';
@import '../ds/static/ds/css/tokens/spacing.css';
@import '../ds/static/ds/css/tokens/site.css';
@import '../ds/static/ds/css/themes/theme.css';
@import '../ds/static/ds/css/base.css' layer(base);
@import './starwind/base.css';
@import './starwind/components.css';

@source '../ds/templates';
@source '../@@pkg@@/templates';
