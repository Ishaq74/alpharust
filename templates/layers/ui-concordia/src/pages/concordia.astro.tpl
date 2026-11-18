@@meta when=concordia@@
---
@@if ssr@@
export const prerender = false;
@@end@@
import BaseLayout from '../layouts/BaseLayout.astro';
import Alert from '../components/ui/Alert.astro';
import Badge from '../components/ui/Badge.astro';
import Button from '../components/ui/Button.astro';
import ProgressBar from '../components/ui/ProgressBar.astro';
import Card from '../components/ui/Card/Card.astro';
import CardContent from '../components/ui/Card/CardContent.astro';
import CardHeader from '../components/ui/Card/CardHeader.astro';
import Accordion from '../components/ui/Accordion/Accordion.astro';
import AccordionItem from '../components/ui/Accordion/AccordionItem.astro';
import Table from '../components/ui/Table/Table.astro';
import TableBody from '../components/ui/Table/TableBody.astro';
import TableCell from '../components/ui/Table/TableCell.astro';
import TableHead from '../components/ui/Table/TableHead.astro';
import TableHeader from '../components/ui/Table/TableHeader.astro';
import TableRow from '../components/ui/Table/TableRow.astro';

type Variant = 'initial' | 'retro' | 'modern' | 'futuristic';
const variants: { id: Variant; label: string }[] = [
  { id: 'initial', label: 'Initial' },
@@if css_retro@@
  { id: 'retro', label: 'Retro' },
@@end@@
@@if css_modern@@
  { id: 'modern', label: 'Modern' },
@@end@@
@@if css_futuristic@@
  { id: 'futuristic', label: 'Futuristic' },
@@end@@
];
const colors = ['default', 'primary', 'secondary', 'accent'] as const;
---

<BaseLayout title="Composants Concordia" description="Aperçu des composants du système CSS Concordia, par variante.">
  <h1>Composants Concordia</h1>
  <p class="lead">Aperçu des composants du dépôt Concordia, rendus avec vos couleurs de thème, pour chaque variante activée.</p>

  {
    variants.map((v) => (
      <section class="variant" aria-labelledby={`variant-${v.id}`}>
        <h2 id={`variant-${v.id}`}>{v.label}</h2>

        <h3>Boutons</h3>
        <div class="row">
          {colors.map((c) => (
            <Button variant={v.id} color={c}>
              {c}
            </Button>
          ))}
        </div>

        <h3>Badges</h3>
        <div class="row">
          {colors.map((c) => (
            <Badge variant={v.id} color={c} text={c} />
          ))}
        </div>

        <h3>Alertes</h3>
        <div class="stack">
          <Alert variant={v.id} status="info" title="Information" message="Un message informatif." />
          <Alert variant={v.id} status="success" title="Succès" message="L'opération a réussi." />
          <Alert variant={v.id} status="warning" title="Attention" message="Vérifiez ces informations." />
          <Alert variant={v.id} status="danger" title="Erreur" message="Une erreur est survenue." />
        </div>

        <h3>Progression</h3>
        <ProgressBar variant={v.id} color="primary" value={65} showLabel label="Avancement" />

        <h3>Carte</h3>
        <Card variant={v.id}>
          <CardHeader>
            <strong>Titre de carte</strong>
          </CardHeader>
          <CardContent>Contenu de la carte avec la variante {v.label}.</CardContent>
        </Card>

        <h3>Accordéon</h3>
        <Accordion variant={v.id} color="primary">
          <AccordionItem title="Première section" open>
            Contenu de la première section.
          </AccordionItem>
          <AccordionItem title="Deuxième section">Contenu de la deuxième section.</AccordionItem>
        </Accordion>

        <h3>Tableau</h3>
        <Table variant={v.id} striped>
          <TableHeader>
            <TableRow>
              <TableHead>Variante</TableHead>
              <TableHead>Couleur</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow>
              <TableCell>{v.label}</TableCell>
              <TableCell>primaire</TableCell>
            </TableRow>
            <TableRow>
              <TableCell>{v.label}</TableCell>
              <TableCell>secondaire</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </section>
    ))
  }
</BaseLayout>

<style>
  .lead {
    color: var(--text-muted);
  }
  .variant {
    margin-block: var(--space-8);
    padding-top: var(--space-4);
    border-top: 1px solid var(--border);
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
    align-items: center;
  }
  .stack {
    display: grid;
    gap: var(--space-3);
  }
</style>
