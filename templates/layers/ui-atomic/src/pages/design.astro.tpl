---
// Vitrine du design system : composants Starwind d'atomic (src/components/atoms) lisant les jetons de src/styles/.
import BaseLayout from '@layouts/BaseLayout.astro';
import { Alert } from '@atoms/alert';
import { Badge } from '@atoms/badge';
import { Button } from '@atoms/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@atoms/card';
import { Input } from '@atoms/input';
import { Label } from '@atoms/label';

const variants = ['default', 'primary', 'secondary', 'outline', 'ghost', 'info', 'success', 'warning', 'error'] as const;
---

<BaseLayout title="Design system" description="Composants Starwind et jetons de src/styles/.">
  <h1>Design system</h1>
  <p class="meta">
    Les composants viennent de <code>src/components/atoms</code> ; leur allure se règle dans
    <code>src/styles/tokens/components.css</code> et <code>src/styles/components/*.css</code>.
  </p>

  <section aria-labelledby="buttons">
    <h2 id="buttons">Boutons</h2>
    <div class="row" data-demo="buttons">
      {variants.map((variant) => <Button variant={variant}>{variant}</Button>)}
    </div>
    <div class="row">
      <Button size="sm" variant="primary">sm</Button>
      <Button size="md" variant="primary">md</Button>
      <Button size="lg" variant="primary">lg</Button>
    </div>
  </section>

  <section aria-labelledby="badges">
    <h2 id="badges">Badges et alertes</h2>
    <div class="row">
      {variants.map((variant) => <Badge variant={variant}>{variant}</Badge>)}
    </div>
    <Alert variant="info">Information : les couleurs viennent de <code>tokens/colors.css</code>.</Alert>
    <Alert variant="error">Erreur : lisible en clair comme en sombre.</Alert>
  </section>

  <section aria-labelledby="cards">
    <h2 id="cards">Carte et champ</h2>
    <Card class="max-w-md">
      <CardHeader>
        <CardTitle>Carte</CardTitle>
        <CardDescription>Rayon : <code>--card-border-radius</code></CardDescription>
      </CardHeader>
      <CardContent>
        <Label for="demo-name">Nom</Label>
        <Input id="demo-name" placeholder="Votre nom" />
      </CardContent>
    </Card>
  </section>
</BaseLayout>

<style>
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
    margin-bottom: var(--space-4);
  }
</style>
