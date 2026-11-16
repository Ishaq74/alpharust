# Documentation — @@title@@

## Génération

Ce projet a été créé par AlphaRust (v@@version@@) avec les outils officiels :

1. `create-astro` — template `@@template@@` ;
@@if tailwind|mdx|sitemap|react|vue|svelte@@
2. `astro add` — intégrations choisies ;
@@end@@
3. couche AlphaRust (structure, thème, documentation, configuration qualité) ;
4. vérification : build@@if typecheck@@, `astro check`@@end@@ et lint AlphaRust.

La configuration exacte est conservée dans `.alpharust-project.json`.

## Stack

- Astro (rendu statique), TypeScript `astro/tsconfigs/@@typescript@@`
@@if tailwind@@
- Tailwind CSS (plugin Vite officiel), importé dans `src/styles/global.css`
@@end@@
@@if mdx@@
- MDX pour le contenu (`src/content/blog/*.mdx`)
@@end@@
@@if sitemap@@
- Sitemap XML (`site` défini dans `astro.config.mjs`)
@@end@@
@@if react@@
- Îlots React (`src/components/Counter.tsx`)
@@end@@
@@if vue@@
- Îlots Vue (`src/components/Counter.vue`)
@@end@@
@@if svelte@@
- Îlots Svelte (`src/components/Counter.svelte`)
@@end@@

## Contenu

Les articles vivent dans `src/content/blog/` ; leur frontmatter est validé par le schéma de `src/content.config.ts`
(`title`, `description`, `pubDate`, `updatedDate?`).

## Accessibilité

Lien d'évitement, hiérarchie de titres, contrastes AA, `prefers-color-scheme` et `prefers-reduced-motion` respectés.
