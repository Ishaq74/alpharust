@@meta when=auth@@
---
interface Props {
  title: string;
}
const { title } = Astro.props;
@@if starwind@@
import { Card, CardContent, CardHeader, CardTitle } from '@/components/starwind/card';
@@end@@
@@if concordia@@
import Card from '@components/ui/Card/Card.astro';
@@end@@
---

@@if starwind@@
<div class="auth-wrap">
  <Card class="auth-card">
    <CardHeader>
      <CardTitle><h1 class="auth-title">{title}</h1></CardTitle>
    </CardHeader>
    <CardContent>
      <slot />
    </CardContent>
  </Card>
</div>
@@else@@
@@if concordia@@
<section class="auth-wrap">
  <Card className="auth-card" elevation="md">
    <h1 class="auth-title">{title}</h1>
    <slot />
  </Card>
</section>
@@else@@
<section class="auth-wrap">
  <div class="auth-card">
    <h1 class="auth-title">{title}</h1>
    <slot />
  </div>
</section>
@@end@@
@@end@@

<style>
  .auth-wrap {
    display: grid;
    place-items: start center;
    padding-block: var(--space-6);
  }
  .auth-card {
    width: min(100%, 26rem);
@@if !starwind&!concordia@@
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: var(--space-6);
@@end@@
  }
  .auth-title {
    font-size: 1.5rem;
    margin: 0 0 var(--space-4);
  }
</style>
