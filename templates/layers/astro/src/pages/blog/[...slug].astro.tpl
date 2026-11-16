@@meta when=!official@@
---
@@if ssr@@
import { getEntry, render } from 'astro:content';
@@else@@
import { type CollectionEntry, getCollection, render } from 'astro:content';
@@end@@
import BaseLayout from '../../layouts/BaseLayout.astro';

@@if ssr@@
// Rendu serveur : l'article est résolu à la demande.
export const prerender = false;

const post = await getEntry('blog', Astro.params.slug ?? '');
if (!post) return new Response(null, { status: 404 });
@@else@@
export async function getStaticPaths() {
  const posts = await getCollection('blog');
  return posts.map((post) => ({
    params: { slug: post.id },
    props: post,
  }));
}

type Props = CollectionEntry<'blog'>;

const post = Astro.props;
@@end@@
const { Content } = await render(post);
const format = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'long' });
---

<BaseLayout title={post.data.title} description={post.data.description}>
  <article>
    <h1>{post.data.title}</h1>
    <p class="meta">
      <time datetime={post.data.pubDate.toISOString()}>{format.format(post.data.pubDate)}</time>
    </p>
    <Content />
  </article>
</BaseLayout>

<style>
  .meta {
    color: var(--text-muted);
  }
</style>
