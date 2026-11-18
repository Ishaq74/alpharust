@@meta mode=replace when=!official@@
/*
 * Design system — ENTRÉE UNIQUE (importée par src/layouts/BaseLayout.astro).
 * Rien d'autre n'est importé par les pages : ajoutez vos fichiers ici, dans l'ordre voulu.
 *
 *   tokens/      variables : couleurs, typographie, espacements, composants
 *   themes/      palette de marque (générée avec contrôle WCAG : `--theme-*`)
 *   components/  un fichier par composant (et custom.css pour vos surcharges)
 */
@@if starwind@@
@import "./starwind.css"; /* généré par `starwind init` : Tailwind 4, tw-animate-css, jetons officiels */
@@else@@
@@if tailwind@@
@import "tailwindcss";
@@end@@
@@end@@
@@if tailwind@@
/* Les surcharges de composants passent APRÈS les utilitaires Tailwind. */
@layer theme, base, components, utilities, overrides;
@@end@@

/* 1. Jetons */
@@if concordia@@
@import "./base.css" layer(concordia); /* Concordia : tokens/{colors,typography,spacing,components}.css + variante initiale (couche `concordia` : les composants du kit passent après) */
@@else@@
@import "./tokens/colors.css";
@import "./tokens/typography.css";
@import "./tokens/spacing.css";
@import "./tokens/components.css";
@@if tailwind@@
@import "./tokens/tailwind.css";
@@end@@
@@end@@
@import "./tokens/site.css";

/* 2. Thème de marque */
@import "./themes/theme.css";

/* 3. Base et composants */
@@if concordia@@
@@if css_retro@@
@import "./components/retro.css" layer(concordia);
@@end@@
@@if css_modern@@
@import "./components/modern.css" layer(concordia);
@@end@@
@@if css_futuristic@@
@import "./components/futuristic.css" layer(concordia);
@@end@@
@@else@@
@import "./base.css";
@@end@@
@import "./components/layout.css";
@@if !concordia@@
@import "./components/button.css";
@import "./components/card.css";
@@end@@
@@if starwind@@
@import "./components/input.css";
@import "./components/textarea.css";
@import "./components/select.css";
@import "./components/badge.css";
@import "./components/alert.css";
@import "./components/dialog.css";
@import "./components/tabs.css";
@import "./components/table.css";
@import "./components/tooltip.css";
@@end@@
@@if concordia@@
/* Composants du design kit partagé : même balisage ([data-slot]) que Starwind, Blade et Django. */
@import "./components/utilities.css";
@import "./components/accordion.css";
@import "./components/alert.css";
@import "./components/avatar.css";
@import "./components/badge.css";
@import "./components/breadcrumb.css";
@import "./components/button.css";
@import "./components/card.css";
@import "./components/checkbox.css";
@import "./components/dialog.css";
@import "./components/input.css";
@import "./components/kbd.css";
@import "./components/label.css";
@import "./components/native-select.css";
@import "./components/pagination.css";
@import "./components/progress.css";
@import "./components/separator.css";
@import "./components/skeleton.css";
@import "./components/spinner.css";
@import "./components/switch.css";
@import "./components/table.css";
@import "./components/tabs.css";
@import "./components/textarea.css";
@import "./components/tooltip.css";
/* Composants étendus (mêmes noms et data-slot que les 23 autres atomes d'atomic). */
@import "./components/alert-dialog.css";
@import "./components/aspect-ratio.css";
@import "./components/button-group.css";
@import "./components/carousel.css";
@import "./components/collapsible.css";
@import "./components/container.css";
@import "./components/dropdown.css";
@import "./components/dropzone.css";
@import "./components/image.css";
@import "./components/input-group.css";
@import "./components/input-otp.css";
@import "./components/item.css";
@import "./components/popover.css";
@import "./components/prose.css";
@import "./components/radio-group.css";
@import "./components/select.css";
@import "./components/sheet.css";
@import "./components/sidebar.css";
@import "./components/slider.css";
@import "./components/theme-toggle.css";
@import "./components/toast.css";
@import "./components/toggle.css";
@import "./components/video.css";
@@end@@
@import "./components/custom.css";
