@@meta mode=replace when=!official@@
import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';
import { z } from 'astro/zod';

const blog = defineCollection({
  // Articles dans src/content/blog/
@@if mdx@@
  loader: glob({ base: './src/content/blog', pattern: '**/*.{md,mdx}' }),
@@else@@
  loader: glob({ base: './src/content/blog', pattern: '**/*.md' }),
@@end@@
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
  }),
});

export const collections = { blog };
