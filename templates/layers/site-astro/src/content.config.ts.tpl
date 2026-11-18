@@meta mode=replace when=vitrine@@
import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';
import { z } from 'astro/zod';

// Articles (src/content/blog) et services (src/content/services) : mêmes champs que les modèles Laravel
// (Post, Service) et les pages Wagtail (BlogPage, ServicePage).
const blog = defineCollection({
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
    category: z.string().default('Actualités'),
    author: z.string().default('La rédaction'),
  }),
});

const services = defineCollection({
  loader: glob({ base: './src/content/services', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    summary: z.string(),
    icon: z.string(),
    price: z.string(),
    order: z.number(),
    features: z.array(z.string()),
  }),
});

export const collections = { blog, services };
