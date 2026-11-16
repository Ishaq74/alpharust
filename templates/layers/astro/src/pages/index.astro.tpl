@@meta mode=replace when=!official@@
---
import { getCollection } from 'astro:content';
import BaseLayout from '../layouts/BaseLayout.astro';
import { SITE_DESCRIPTION, SITE_TITLE } from '../consts';
@@if react@@
import Counter from '../components/Counter.tsx';
@@end@@
@@if vue@@
import Counter from '../components/Counter.vue';
@@end@@
@@if svelte@@
import Counter from '../components/Counter.svelte';
@@end@@

const posts = (await getCollection('blog'))
  .sort((a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf())
  .slice(0, 3);
const format = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'long' });
---

<BaseLayout title={SITE_TITLE} description={SITE_DESCRIPTION}>
  <section class="hero">
    <h1>{SITE_TITLE}</h1>
    <p class="lead">{SITE_DESCRIPTION}</p>
    <div class="actions">
      <a class="button" href="/blog/">Lire le blog</a>
      <a class="button secondary" href="/about/">À propos</a>
    </div>
  </section>

  <section aria-labelledby="latest">
    <h2 id="latest">Derniers articles</h2>
    <ul class="posts">
      {
        posts.map((post) => (
          <li class="card">
            <h3>
              <a href={`/blog/${post.id}/`}>{post.data.title}</a>
            </h3>
            <p class="meta">
              <time datetime={post.data.pubDate.toISOString()}>{format.format(post.data.pubDate)}</time>
            </p>
            <p>{post.data.description}</p>
          </li>
        ))
      }
    </ul>
  </section>
@@if react@@

  <section aria-labelledby="island">
    <h2 id="island">Îlot React</h2>
    <Counter client:visible />
  </section>
@@end@@
@@if vue@@

  <section aria-labelledby="island">
    <h2 id="island">Îlot Vue</h2>
    <Counter client:visible />
  </section>
@@end@@
@@if svelte@@

  <section aria-labelledby="island">
    <h2 id="island">Îlot Svelte</h2>
    <Counter client:visible />
  </section>
@@end@@
</BaseLayout>

<style>
  .hero {
    padding: var(--space-12) var(--space-8);
    border: 1px solid var(--border);
    border-radius: calc(var(--radius) * 2);
    background: linear-gradient(135deg, color-mix(in srgb, var(--brand) 14%, var(--bg)), var(--bg) 70%);
  }
  .lead {
    font-size: 1.25rem;
    color: var(--text-muted);
    margin-bottom: 0;
  }
  .posts {
    list-style: none;
    padding: 0;
    display: grid;
    gap: var(--space-4);
    grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));
  }
  .posts h3 {
    margin-top: 0;
    margin-bottom: var(--space-1);
  }
  .posts .meta {
    margin-bottom: var(--space-2);
  }
</style>
