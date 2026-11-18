@@meta when=!official@@
---
import { getCollection } from 'astro:content';
import BaseLayout from '../../layouts/BaseLayout.astro';

const posts = (await getCollection('blog')).sort(
  (a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf(),
);
const format = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'long' });
---

<BaseLayout title="Blog" description="Tous les articles.">
  <h1>Blog</h1>
  <ul class="list">
    {
      posts.map((post) => (
        <li>
          <h2>
            <a href={`/blog/${post.id}/`}>{post.data.title}</a>
          </h2>
          <p class="meta">
            <time datetime={post.data.pubDate.toISOString()}>{format.format(post.data.pubDate)}</time>
          </p>
          <p>{post.data.description}</p>
        </li>
      ))
    }
  </ul>
</BaseLayout>

<style>
  .list {
    list-style: none;
    padding: 0;
    display: grid;
    gap: var(--space-8);
  }
  .list h2 {
    margin-block: 0 var(--space-1);
    font-size: 1.35rem;
  }
  .list li {
    padding-bottom: var(--space-6);
    border-bottom: 1px solid var(--border);
  }
  .meta {
    color: var(--text-muted);
    margin-block: 0;
  }
</style>
