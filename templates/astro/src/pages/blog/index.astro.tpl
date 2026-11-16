---
import { getCollection } from "astro:content";
import Layout from "../../layouts/Layout.astro";

const posts = await getCollection("blog");
---
<Layout title="Blog">
  <h1>Blog</h1>
  <ul>
    {posts.map((p) => (
      <li><a href={`/blog/${p.id}/`}>{p.data.title}</a> — {p.data.description}</li>
    ))}
  </ul>
</Layout>
