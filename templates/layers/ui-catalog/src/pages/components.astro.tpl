@@meta when=!official@@
---
// Catalogue de composants — MÊME page et MÊME API pour Starwind (Tailwind) et pour le CSS pur :
// les imports `@atoms/*` pointent vers src/components/atoms, dont les fichiers diffèrent selon le kit
// mais exposent les mêmes composants, les mêmes props et les mêmes attributs data-slot.
import BaseLayout from '@layouts/BaseLayout.astro';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@atoms/accordion';
import { Alert, AlertDescription, AlertTitle } from '@atoms/alert';
import { Avatar, AvatarFallback, AvatarImage } from '@atoms/avatar';
import { Badge } from '@atoms/badge';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@atoms/breadcrumb';
import { Button } from '@atoms/button';
import { Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@atoms/card';
import { Checkbox } from '@atoms/checkbox';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from '@atoms/dialog';
import { Input } from '@atoms/input';
import { Kbd, KbdGroup } from '@atoms/kbd';
import { Label } from '@atoms/label';
import { NativeSelect, NativeSelectOption } from '@atoms/native-select';
import { Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious } from '@atoms/pagination';
import { Progress } from '@atoms/progress';
import { Separator } from '@atoms/separator';
import { Skeleton } from '@atoms/skeleton';
import { Spinner } from '@atoms/spinner';
import { Switch } from '@atoms/switch';
import { Table, TableBody, TableCaption, TableCell, TableFoot, TableHead, TableHeader, TableRow } from '@atoms/table';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@atoms/tabs';
import { Textarea } from '@atoms/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '@atoms/tooltip';

const variants = ['default', 'primary', 'secondary', 'outline', 'ghost', 'info', 'success', 'warning', 'error'] as const;
const pixel = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7';
---

<BaseLayout title="Composants" description="Catalogue de composants : même API en Tailwind + Starwind et en CSS pur.">
  <h1>Composants</h1>
  <p class="meta">
    Catalogue du kit <strong>@@kit@@</strong> (<code>src/components/atoms</code>). L'allure se règle dans
    <code>src/styles/tokens/components.css</code>, <code>src/styles/components/*.css</code> et <code>themes/theme.css</code>.
  </p>

  <section aria-labelledby="c-buttons" data-demo="button">
    <h2 id="c-buttons">Boutons</h2>
    <div class="row">{variants.map((variant) => <Button variant={variant}>{variant}</Button>)}</div>
    <div class="row">
      <Button size="sm" variant="primary">sm</Button>
      <Button size="md" variant="primary">md</Button>
      <Button size="lg" variant="primary">lg</Button>
      <Button variant="outline" href="#c-buttons">Lien</Button>
      <Button variant="primary" disabled>Désactivé</Button>
    </div>
  </section>

  <section aria-labelledby="c-badges" data-demo="badge">
    <h2 id="c-badges">Badges</h2>
    <div class="row">{variants.map((variant) => <Badge variant={variant}>{variant}</Badge>)}</div>
  </section>

  <section aria-labelledby="c-alerts" data-demo="alert">
    <h2 id="c-alerts">Alertes</h2>
    <div class="stack">
      <Alert variant="info">
        <AlertTitle>Information</AlertTitle>
        <AlertDescription>Les couleurs viennent de <code>tokens/colors.css</code>.</AlertDescription>
      </Alert>
      <Alert variant="success"><AlertTitle>Succès</AlertTitle><AlertDescription>L'opération a réussi.</AlertDescription></Alert>
      <Alert variant="warning"><AlertTitle>Attention</AlertTitle><AlertDescription>Vérifiez les données.</AlertDescription></Alert>
      <Alert variant="error"><AlertTitle>Erreur</AlertTitle><AlertDescription>Lisible en clair comme en sombre.</AlertDescription></Alert>
    </div>
  </section>

  <section aria-labelledby="c-card" data-demo="card">
    <h2 id="c-card">Carte</h2>
    <div class="narrow">
      <Card>
        <CardHeader>
          <CardTitle>Carte</CardTitle>
          <CardDescription>Rayon : <code>--card-border-radius</code></CardDescription>
          <CardAction><Badge variant="primary">Nouveau</Badge></CardAction>
        </CardHeader>
        <CardContent>
          <Label for="demo-name">Nom</Label>
          <Input id="demo-name" placeholder="Votre nom" />
        </CardContent>
        <CardFooter>
          <Button variant="primary">Enregistrer</Button>
          <Button variant="ghost">Annuler</Button>
        </CardFooter>
      </Card>
    </div>
  </section>

  <section aria-labelledby="c-forms" data-demo="form">
    <h2 id="c-forms">Formulaire</h2>
    <div class="narrow stack">
      <div class="stack-sm">
        <Label for="demo-email">Courriel</Label>
        <Input id="demo-email" type="email" placeholder="vous@exemple.fr" />
        <Input size="sm" placeholder="Petit" aria-label="Petit champ" />
        <Input size="lg" placeholder="Grand" aria-label="Grand champ" />
      </div>
      <div class="stack-sm">
        <Label for="demo-msg">Message</Label>
        <Textarea id="demo-msg" placeholder="Écrivez ici…" />
      </div>
      <div class="stack-sm">
        <Label for="demo-pick">Liste</Label>
        <NativeSelect id="demo-pick">
          <NativeSelectOption value="a">Option A</NativeSelectOption>
          <NativeSelectOption value="b">Option B</NativeSelectOption>
        </NativeSelect>
      </div>
      <Checkbox id="demo-check" label="J'accepte les conditions" variant="primary" checked />
      <Switch id="demo-switch" label="Notifications" variant="primary" checked />
    </div>
  </section>

  <section aria-labelledby="c-feedback" data-demo="feedback">
    <h2 id="c-feedback">Progression et chargement</h2>
    <div class="narrow stack">
      <Progress value={60} variant="primary" aria-label="Progression" />
      <Progress variant="success" aria-label="Chargement" />
      <div class="row">
        <Skeleton class="sk" />
        <Spinner />
        <Avatar size="sm"><AvatarFallback>AR</AvatarFallback></Avatar>
        <Avatar><AvatarImage src={pixel} alt="" /><AvatarFallback>AR</AvatarFallback></Avatar>
        <Avatar size="lg"><AvatarFallback>AR</AvatarFallback></Avatar>
      </div>
    </div>
  </section>

  <section aria-labelledby="c-nav" data-demo="navigation">
    <h2 id="c-nav">Navigation</h2>
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem><BreadcrumbLink href="/">Accueil</BreadcrumbLink></BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem><BreadcrumbLink href="#c-nav">Composants</BreadcrumbLink></BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem><BreadcrumbPage>Navigation</BreadcrumbPage></BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
    <Pagination>
      <PaginationContent>
        <PaginationItem><PaginationPrevious href="#c-nav" /></PaginationItem>
        <PaginationItem><PaginationLink href="#c-nav">1</PaginationLink></PaginationItem>
        <PaginationItem><PaginationLink href="#c-nav" isActive>2</PaginationLink></PaginationItem>
        <PaginationItem><PaginationLink href="#c-nav">3</PaginationLink></PaginationItem>
        <PaginationItem><PaginationNext href="#c-nav" /></PaginationItem>
      </PaginationContent>
    </Pagination>
  </section>

  <section aria-labelledby="c-table" data-demo="table">
    <h2 id="c-table">Tableau</h2>
    <Table>
      <TableCaption>Équipe</TableCaption>
      <TableHeader>
        <TableRow><TableHead>Nom</TableHead><TableHead>Rôle</TableHead><TableHead>Statut</TableHead></TableRow>
      </TableHeader>
      <TableBody>
        <TableRow><TableCell>Ada</TableCell><TableCell>Ingénieure</TableCell><TableCell><Badge variant="success">Actif</Badge></TableCell></TableRow>
        <TableRow><TableCell>Linus</TableCell><TableCell>Mainteneur</TableCell><TableCell><Badge variant="warning">Absent</Badge></TableCell></TableRow>
      </TableBody>
      <TableFoot>
        <TableRow><TableCell>Total</TableCell><TableCell>2</TableCell><TableCell></TableCell></TableRow>
      </TableFoot>
    </Table>
  </section>

  <section aria-labelledby="c-interactive" data-demo="interactive">
    <h2 id="c-interactive">Composants interactifs</h2>
    <Tabs defaultValue="one">
      <TabsList>
        <TabsTrigger value="one" defaultActive>Premier</TabsTrigger>
        <TabsTrigger value="two">Deuxième</TabsTrigger>
        <TabsTrigger value="three">Troisième</TabsTrigger>
      </TabsList>
      <TabsContent value="one" defaultActive>Contenu du premier onglet (flèches ← → pour naviguer).</TabsContent>
      <TabsContent value="two">Contenu du deuxième onglet.</TabsContent>
      <TabsContent value="three">Contenu du troisième onglet.</TabsContent>
    </Tabs>

    <Accordion type="single" defaultValue="a">
      <AccordionItem value="a"><AccordionTrigger>Qu'est-ce que le catalogue ?</AccordionTrigger><AccordionContent>Un jeu de composants au même balisage en Astro, Laravel et Django.</AccordionContent></AccordionItem>
      <AccordionItem value="b"><AccordionTrigger>Comment le personnaliser ?</AccordionTrigger><AccordionContent>Par les jetons de <code>src/styles/tokens</code>.</AccordionContent></AccordionItem>
    </Accordion>

    <div class="row">
      <Dialog id="demo-dialog">
        <DialogTrigger asChild><Button variant="outline">Ouvrir la boîte de dialogue</Button></DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirmer</DialogTitle>
            <DialogDescription>Cette action est réversible.</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <DialogClose asChild><Button variant="primary">Fermer</Button></DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Tooltip>
        <TooltipTrigger><Button variant="outline">Survolez-moi</Button></TooltipTrigger>
        <TooltipContent>Info-bulle accessible au clavier</TooltipContent>
      </Tooltip>

      <KbdGroup><Kbd>Ctrl</Kbd><Kbd>K</Kbd></KbdGroup>
    </div>
    <Separator />
  </section>
</BaseLayout>

<style>
  .row { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-3); margin-bottom: var(--space-4); }
  .narrow { max-width: 28rem; }
  .stack { display: grid; gap: var(--space-4); }
  .stack-sm { display: grid; gap: var(--space-2); }
  .sk { width: 6rem; height: 1rem; }
  section { margin-bottom: var(--space-8); }
</style>
