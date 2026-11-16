---
import { getCollection } from "astro:content";
import Layout from "../layouts/Layout.astro";

const posts = (await getCollection("blog")).sort(
  (a, b) => b.data.pubDate.getTime() - a.data.pubDate.getTime(),
);
---
<Layout title="Accueil">
  <h1>@@title@@</h1>
  <p>Site Astro généré par AlphaRust.</p>
  <h2>Derniers articles</h2>
  <ul>
    {posts.map((p) => <li><a href={`/blog/${p.id}/`}>{p.data.title}</a></li>)}
  </ul>
</Layout>
