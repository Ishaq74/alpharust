@@meta when=!official@@
---
// Catalogue étendu — les 23 autres composants d'atomic. MÊME page pour Starwind (Tailwind) et pour le CSS pur :
// seuls les fichiers de src/components/atoms changent ; noms, props et attributs data-slot sont identiques.
import BaseLayout from '@layouts/BaseLayout.astro';
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@atoms/alert-dialog';
import { AspectRatio } from '@atoms/aspect-ratio';
import { Button } from '@atoms/button';
import { ButtonGroup, ButtonGroupSeparator, ButtonGroupText } from '@atoms/button-group';
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@atoms/carousel';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@atoms/collapsible';
import { Container } from '@atoms/container';
import { Dropdown, DropdownContent, DropdownItem, DropdownLabel, DropdownSeparator, DropdownShortcut, DropdownSub, DropdownSubContent, DropdownSubTrigger, DropdownTrigger } from '@atoms/dropdown';
import { Dropzone } from '@atoms/dropzone';
import { Image } from '@atoms/image';
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput, InputGroupText, InputGroupTextarea } from '@atoms/input-group';
import { InputOtp, InputOtpGroup, InputOtpSeparator, InputOtpSlot } from '@atoms/input-otp';
import { Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle } from '@atoms/item';
import { Label } from '@atoms/label';
import { Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle, PopoverTrigger } from '@atoms/popover';
import { Prose } from '@atoms/prose';
import { RadioGroup, RadioGroupItem } from '@atoms/radio-group';
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSearch, SelectSeparator, SelectTrigger, SelectValue } from '@atoms/select';
import { Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle, SheetTrigger } from '@atoms/sheet';
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInput, SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem, SidebarMenuSkeleton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSeparator, SidebarTrigger } from '@atoms/sidebar';
import { Slider } from '@atoms/slider';
import { ThemeToggle } from '@atoms/theme-toggle';
import { ToastDescription, Toaster, ToastItem, ToastTitle } from '@atoms/toast';
import { Toggle } from '@atoms/toggle';
import { Video } from '@atoms/video';

const slides = ['Concevoir', 'Construire', 'Vérifier', 'Publier'];
---

<BaseLayout title="Composants avancés" description="Les 23 autres composants d'atomic : même API en Tailwind + Starwind et en CSS pur.">
  <h1>Composants avancés</h1>
  <p class="meta">
    Suite du <a href="/components">catalogue</a> — kit <strong>@@kit@@</strong>. Mêmes composants, mêmes props et mêmes attributs
    <code>data-slot</code> en Starwind et en CSS pur.
  </p>

  <section aria-labelledby="x-overlays" data-demo="overlays">
    <h2 id="x-overlays">Fenêtres</h2>
    <div class="row">
      <AlertDialog>
        <AlertDialogTrigger asChild><Button variant="error">Supprimer le projet</Button></AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Supprimer définitivement ?</AlertDialogTitle>
            <AlertDialogDescription>Cette action est irréversible : le projet et ses fichiers seront effacés.</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Annuler</AlertDialogCancel>
            <AlertDialogAction>Supprimer</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <Sheet>
        <SheetTrigger asChild><Button variant="outline">Ouvrir le panneau</Button></SheetTrigger>
        <SheetContent side="right">
          <SheetHeader>
            <SheetTitle>Paramètres</SheetTitle>
            <SheetDescription>Un panneau latéral construit sur la boîte de dialogue native.</SheetDescription>
          </SheetHeader>
          <div class="pad">Contenu du panneau.</div>
          <SheetFooter><SheetClose asChild><Button>Fermer</Button></SheetClose></SheetFooter>
        </SheetContent>
      </Sheet>

      <Popover>
        <PopoverTrigger asChild><Button variant="outline">Popover</Button></PopoverTrigger>
        <PopoverContent>
          <PopoverHeader>
            <PopoverTitle>Dimensions</PopoverTitle>
            <PopoverDescription>Réglez la largeur du bloc.</PopoverDescription>
          </PopoverHeader>
        </PopoverContent>
      </Popover>

      <Dropdown>
        <DropdownTrigger asChild><Button variant="outline">Menu</Button></DropdownTrigger>
        <DropdownContent>
          <DropdownLabel>Mon compte</DropdownLabel>
          <DropdownSeparator />
          <DropdownItem>Profil <DropdownShortcut>⇧⌘P</DropdownShortcut></DropdownItem>
          <DropdownItem>Facturation</DropdownItem>
          <DropdownSub>
            <DropdownSubTrigger>Inviter</DropdownSubTrigger>
            <DropdownSubContent>
              <DropdownItem>Par e-mail</DropdownItem>
              <DropdownItem>Par lien</DropdownItem>
            </DropdownSubContent>
          </DropdownSub>
          <DropdownSeparator />
          <DropdownItem disabled>Déconnexion</DropdownItem>
        </DropdownContent>
      </Dropdown>
    </div>
  </section>

  <section aria-labelledby="x-inputs" data-demo="inputs">
    <h2 id="x-inputs">Saisie</h2>
    <div class="grid2">
      <div class="stack">
        <div class="stack-sm">
          <Label for="x-search">Recherche</Label>
          <InputGroup>
            <InputGroupAddon><InputGroupText>https://</InputGroupText></InputGroupAddon>
            <InputGroupInput id="x-search" placeholder="exemple.com" />
            <InputGroupAddon align="inline-end"><InputGroupButton>Aller</InputGroupButton></InputGroupAddon>
          </InputGroup>
        </div>
        <InputGroup>
          <InputGroupTextarea placeholder="Votre message…" aria-label="Message" />
          <InputGroupAddon align="block-end"><InputGroupText>0 / 280</InputGroupText></InputGroupAddon>
        </InputGroup>
        <div class="stack-sm">
          <Label for="x-otp">Code reçu par SMS</Label>
          <InputOtp maxLength={6} id="x-otp" name="otp">
            <InputOtpGroup>
              <InputOtpSlot index={0} />
              <InputOtpSlot index={1} />
              <InputOtpSlot index={2} />
            </InputOtpGroup>
            <InputOtpSeparator />
            <InputOtpGroup>
              <InputOtpSlot index={3} />
              <InputOtpSlot index={4} />
              <InputOtpSlot index={5} />
            </InputOtpGroup>
          </InputOtp>
        </div>
      </div>
      <div class="stack">
        <Select name="fruit">
          <SelectTrigger aria-label="Fruit"><SelectValue placeholder="Choisir un fruit" /></SelectTrigger>
          <SelectContent>
            <SelectSearch placeholder="Rechercher…" />
            <SelectGroup>
              <SelectLabel>Fruits</SelectLabel>
              <SelectItem value="pomme">Pomme</SelectItem>
              <SelectItem value="poire">Poire</SelectItem>
              <SelectItem value="abricot">Abricot</SelectItem>
            </SelectGroup>
            <SelectSeparator />
            <SelectGroup>
              <SelectLabel>Agrumes</SelectLabel>
              <SelectItem value="orange">Orange</SelectItem>
              <SelectItem value="citron" disabled>Citron</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
        <RadioGroup name="plan" defaultValue="pro" legend="Formule">
          <div class="row-tight"><RadioGroupItem value="free" id="plan-free" /><Label for="plan-free">Gratuit</Label></div>
          <div class="row-tight"><RadioGroupItem value="pro" id="plan-pro" /><Label for="plan-pro">Pro</Label></div>
          <div class="row-tight"><RadioGroupItem value="team" id="plan-team" /><Label for="plan-team">Équipe</Label></div>
        </RadioGroup>
        <Slider defaultValue={40} aria-label="Volume" />
        <Slider defaultValue={[20, 80]} aria-label="Fourchette de prix" />
        <Dropzone id="x-files" multiple />
      </div>
    </div>
  </section>

  <section aria-labelledby="x-actions" data-demo="actions">
    <h2 id="x-actions">Actions</h2>
    <div class="row">
      <ButtonGroup>
        <Button variant="outline">Archiver</Button>
        <Button variant="outline">Signaler</Button>
        <ButtonGroupSeparator />
        <Button variant="outline">Supprimer</Button>
      </ButtonGroup>
      <ButtonGroup>
        <ButtonGroupText>Lien</ButtonGroupText>
        <Button variant="outline">Copier</Button>
      </ButtonGroup>
      <Toggle aria-label="Gras" variant="outline"><strong>B</strong></Toggle>
      <Toggle aria-label="Italique" defaultPressed><em>I</em></Toggle>
      <ThemeToggle />
      <Button variant="primary" id="x-toast">Afficher une notification</Button>
    </div>
    <Collapsible>
      <CollapsibleTrigger>Afficher les détails</CollapsibleTrigger>
      <CollapsibleContent>Le contenu replié est masqué (attribut <code>hidden</code>) jusqu'au clic.</CollapsibleContent>
    </Collapsible>
  </section>

  <section aria-labelledby="x-content" data-demo="content">
    <h2 id="x-content">Contenu</h2>
    <div class="grid2">
      <ItemGroup>
        <Item variant="outline">
          <ItemMedia variant="icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M4 4h16v16H4z" /></svg>
          </ItemMedia>
          <ItemContent>
            <ItemTitle>Rapport mensuel</ItemTitle>
            <ItemDescription>Généré le 1er de chaque mois.</ItemDescription>
          </ItemContent>
          <ItemActions><Button size="sm" variant="outline">Ouvrir</Button></ItemActions>
        </Item>
        <ItemSeparator />
        <Item variant="muted" size="sm">
          <ItemHeader>En-tête</ItemHeader>
          <ItemContent><ItemTitle>Élément compact</ItemTitle></ItemContent>
          <ItemFooter>Pied</ItemFooter>
        </Item>
      </ItemGroup>
      <div class="stack">
        <AspectRatio ratio={16 / 9}><div class="ratio-fill">16 / 9</div></AspectRatio>
        <div class="img-box"><Image src="/favicon.svg" alt="Logo du site" width={48} height={48} /></div>
      </div>
    </div>
    <Carousel aria-label="Étapes">
      <CarouselContent>
        {slides.map((s, i) => <CarouselItem><div class="slide">{i + 1}. {s}</div></CarouselItem>)}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
    <Container size="md" paddingY="sm" background="muted" rounded="lg">
      <Prose>
        <h3>Prose</h3>
        <p>Typographie d'un contenu riche : <a href="#x-content">liens</a>, <code>code</code>, listes…</p>
        <ul><li>Mesure de lecture limitée</li><li>Tailles en <em>em</em></li></ul>
      </Prose>
    </Container>
    <div class="narrow">
      <Video src="https://www.youtube.com/watch?v=aqz-KE-bpKQ" title="Big Buck Bunny (Blender Foundation)" />
    </div>
  </section>

  <section aria-labelledby="x-layout" data-demo="layout">
    <h2 id="x-layout">Barre latérale</h2>
    <div class="sidebar-frame">
      <SidebarProvider>
        <Sidebar collapsible="icon">
          <SidebarHeader><strong>Mon espace</strong></SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupLabel>Navigation</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton href="#x-layout" isActive tooltip="Accueil">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M3 11l9-8 9 8v10H3z" /></svg>
                      <span>Accueil</span>
                    </SidebarMenuButton>
                    <SidebarMenuBadge>3</SidebarMenuBadge>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton tooltip="Projets">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M3 7h18v13H3zM3 7l3-4h6l2 4" /></svg>
                      <span>Projets</span>
                    </SidebarMenuButton>
                    <SidebarMenuAction showOnHover aria-label="Ajouter un projet">+</SidebarMenuAction>
                    <SidebarMenuSub>
                      <SidebarMenuSubItem><SidebarMenuSubButton href="#x-layout" isActive>Site vitrine</SidebarMenuSubButton></SidebarMenuSubItem>
                      <SidebarMenuSubItem><SidebarMenuSubButton href="#x-layout">Boutique</SidebarMenuSubButton></SidebarMenuSubItem>
                    </SidebarMenuSub>
                  </SidebarMenuItem>
                  <SidebarMenuItem><SidebarMenuSkeleton showIcon width="70%" /></SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
          <SidebarSeparator />
          <SidebarFooter><SidebarInput placeholder="Filtrer…" aria-label="Filtrer" /></SidebarFooter>
          <SidebarRail />
        </Sidebar>
        <SidebarInset>
          <div class="inset-head"><SidebarTrigger /><span>Contenu principal — <kbd>Ctrl</kbd> + <kbd>B</kbd> replie la barre.</span></div>
        </SidebarInset>
      </SidebarProvider>
    </div>
  </section>

  <section aria-labelledby="x-toasts" data-demo="toasts">
    <h2 id="x-toasts">Notifications</h2>
    <div class="toast-sample">
      <ToastItem variant="success">
        <ToastTitle variant="success">Enregistré</ToastTitle>
        <ToastDescription>Vos modifications sont en ligne.</ToastDescription>
      </ToastItem>
    </div>
    <Toaster position="bottom-right" />
  </section>
</BaseLayout>

<script>
  import { toast } from '@atoms/toast/toast-manager';
  document.getElementById('x-toast')?.addEventListener('click', () => {
    toast.success('Enregistré', { description: 'Vos modifications sont en ligne.' });
  });
</script>

<style>
  .row { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-3); margin-bottom: var(--space-4); }
  .row-tight { display: flex; align-items: center; gap: var(--space-2); }
  .grid2 { display: grid; gap: var(--space-6); grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr)); margin-bottom: var(--space-6); }
  .stack { display: grid; gap: var(--space-4); align-content: start; }
  .stack-sm { display: grid; gap: var(--space-2); }
  .narrow { max-width: 28rem; }
  .pad { padding: 0 1rem; }
  .ratio-fill { display: grid; place-items: center; width: 100%; height: 100%; border-radius: 0.5rem; background: linear-gradient(135deg, var(--brand, var(--primary)), color-mix(in oklab, var(--brand, var(--primary)) 35%, white)); color: var(--brand-contrast, var(--primary-foreground)); font-weight: 600; }
  .img-box { width: 3rem; }
  .img-box :global(img) { width: 3rem; height: 3rem; }
  .slide { display: grid; place-items: center; height: 8rem; border: 1px solid var(--border); border-radius: 0.5rem; font-size: 1.25rem; font-weight: 600; }
  /* Cadre de démonstration : `transform` contient les éléments fixes de la barre latérale dans la page. */
  .sidebar-frame { position: relative; height: 24rem; overflow: hidden; border: 1px solid var(--border); border-radius: 0.75rem; transform: translateZ(0); }
  .sidebar-frame :global([data-slot='sidebar-provider']) { min-height: 100%; height: 100%; }
  .inset-head { display: flex; align-items: center; gap: var(--space-2); padding: var(--space-3); }
  .toast-sample { position: relative; max-width: 24rem; }
  section { margin-bottom: var(--space-8); }
</style>
