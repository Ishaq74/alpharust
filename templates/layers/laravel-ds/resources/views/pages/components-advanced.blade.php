@php
    $slides = ['Concevoir', 'Construire', 'Vérifier', 'Publier'];
    $logo = 'data:image/svg+xml,'.rawurlencode('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><rect width="48" height="48" rx="10" fill="#3b4cca"/><path d="M15 36 24 12l9 24m-15-8h12" stroke="#fff" stroke-width="4" fill="none"/></svg>'); // alpharust-ignore: no-hardcoded-urls
@endphp
{{-- Catalogue étendu : mêmes composants, props et data-slot que la page Astro /components/advanced et la page Django. --}}
<x-site-layout title="Composants avancés">
    <h1>Composants avancés</h1>
    <p class="meta">
        Suite du <a href="{{ url('/components') }}">catalogue</a> — kit <strong>Blade</strong>. Mêmes composants, mêmes props et mêmes attributs
        <code>data-slot</code> qu'en Astro (Starwind et CSS pur) et en Django.
    </p>

    <section aria-labelledby="x-overlays" data-demo="overlays">
        <h2 id="x-overlays">Fenêtres</h2>
        <div class="row">
            <x-ds.alert-dialog>
                <x-ds.alert-dialog.trigger :as-child="true"><x-ds.button variant="error">Supprimer le projet</x-ds.button></x-ds.alert-dialog.trigger>
                <x-ds.alert-dialog.content>
                    <x-ds.alert-dialog.header>
                        <x-ds.alert-dialog.title>Supprimer définitivement ?</x-ds.alert-dialog.title>
                        <x-ds.alert-dialog.description>Cette action est irréversible : le projet et ses fichiers seront effacés.</x-ds.alert-dialog.description>
                    </x-ds.alert-dialog.header>
                    <x-ds.alert-dialog.footer>
                        <x-ds.alert-dialog.cancel>Annuler</x-ds.alert-dialog.cancel>
                        <x-ds.alert-dialog.action>Supprimer</x-ds.alert-dialog.action>
                    </x-ds.alert-dialog.footer>
                </x-ds.alert-dialog.content>
            </x-ds.alert-dialog>

            <x-ds.sheet>
                <x-ds.sheet.trigger :as-child="true"><x-ds.button variant="outline">Ouvrir le panneau</x-ds.button></x-ds.sheet.trigger>
                <x-ds.sheet.content side="right">
                    <x-ds.sheet.header>
                        <x-ds.sheet.title>Paramètres</x-ds.sheet.title>
                        <x-ds.sheet.description>Un panneau latéral construit sur la boîte de dialogue native.</x-ds.sheet.description>
                    </x-ds.sheet.header>
                    <div class="pad">Contenu du panneau.</div>
                    <x-ds.sheet.footer><x-ds.sheet.close :as-child="true"><x-ds.button>Fermer</x-ds.button></x-ds.sheet.close></x-ds.sheet.footer>
                </x-ds.sheet.content>
            </x-ds.sheet>

            <x-ds.popover>
                <x-ds.popover.trigger :as-child="true"><x-ds.button variant="outline">Popover</x-ds.button></x-ds.popover.trigger>
                <x-ds.popover.content>
                    <x-ds.popover.header>
                        <x-ds.popover.title>Dimensions</x-ds.popover.title>
                        <x-ds.popover.description>Réglez la largeur du bloc.</x-ds.popover.description>
                    </x-ds.popover.header>
                </x-ds.popover.content>
            </x-ds.popover>

            <x-ds.dropdown>
                <x-ds.dropdown.trigger :as-child="true"><x-ds.button variant="outline">Menu</x-ds.button></x-ds.dropdown.trigger>
                <x-ds.dropdown.content>
                    <x-ds.dropdown.label>Mon compte</x-ds.dropdown.label>
                    <x-ds.dropdown.separator />
                    <x-ds.dropdown.item>Profil <x-ds.dropdown.shortcut>⇧⌘P</x-ds.dropdown.shortcut></x-ds.dropdown.item>
                    <x-ds.dropdown.item>Facturation</x-ds.dropdown.item>
                    <x-ds.dropdown.sub>
                        <x-ds.dropdown.sub-trigger>Inviter</x-ds.dropdown.sub-trigger>
                        <x-ds.dropdown.sub-content>
                            <x-ds.dropdown.item>Par e-mail</x-ds.dropdown.item>
                            <x-ds.dropdown.item>Par lien</x-ds.dropdown.item>
                        </x-ds.dropdown.sub-content>
                    </x-ds.dropdown.sub>
                    <x-ds.dropdown.separator />
                    <x-ds.dropdown.item :disabled="true">Déconnexion</x-ds.dropdown.item>
                </x-ds.dropdown.content>
            </x-ds.dropdown>
        </div>
    </section>

    <section aria-labelledby="x-inputs" data-demo="inputs">
        <h2 id="x-inputs">Saisie</h2>
        <div class="grid2">
            <div class="stack">
                <div class="stack-sm">
                    <x-ds.label for="x-search">Recherche</x-ds.label>
                    <x-ds.input-group>
                        <x-ds.input-group.addon><x-ds.input-group.text>https://</x-ds.input-group.text></x-ds.input-group.addon>
                        <x-ds.input-group.input id="x-search" placeholder="exemple.com" />
                        <x-ds.input-group.addon align="inline-end"><x-ds.input-group.button>Aller</x-ds.input-group.button></x-ds.input-group.addon>
                    </x-ds.input-group>
                </div>
                <x-ds.input-group>
                    <x-ds.input-group.textarea placeholder="Votre message…" aria-label="Message" />
                    <x-ds.input-group.addon align="block-end"><x-ds.input-group.text>0 / 280</x-ds.input-group.text></x-ds.input-group.addon>
                </x-ds.input-group>
                <div class="stack-sm">
                    <x-ds.label for="x-otp">Code reçu par SMS</x-ds.label>
                    <x-ds.input-otp :max-length="6" id="x-otp" name="otp">
                        <x-ds.input-otp.group>
                            <x-ds.input-otp.slot :index="0" />
                            <x-ds.input-otp.slot :index="1" />
                            <x-ds.input-otp.slot :index="2" />
                        </x-ds.input-otp.group>
                        <x-ds.input-otp.separator />
                        <x-ds.input-otp.group>
                            <x-ds.input-otp.slot :index="3" />
                            <x-ds.input-otp.slot :index="4" />
                            <x-ds.input-otp.slot :index="5" />
                        </x-ds.input-otp.group>
                    </x-ds.input-otp>
                </div>
            </div>
            <div class="stack">
                <x-ds.select name="fruit">
                    <x-ds.select.trigger aria-label="Fruit"><x-ds.select.value placeholder="Choisir un fruit" /></x-ds.select.trigger>
                    <x-ds.select.content>
                        <x-ds.select.search placeholder="Rechercher…" />
                        <x-ds.select.group>
                            <x-ds.select.label>Fruits</x-ds.select.label>
                            <x-ds.select.item value="pomme">Pomme</x-ds.select.item>
                            <x-ds.select.item value="poire">Poire</x-ds.select.item>
                            <x-ds.select.item value="abricot">Abricot</x-ds.select.item>
                        </x-ds.select.group>
                        <x-ds.select.separator />
                        <x-ds.select.group>
                            <x-ds.select.label>Agrumes</x-ds.select.label>
                            <x-ds.select.item value="orange">Orange</x-ds.select.item>
                            <x-ds.select.item value="citron" :disabled="true">Citron</x-ds.select.item>
                        </x-ds.select.group>
                    </x-ds.select.content>
                </x-ds.select>
                <x-ds.radio-group name="plan" default-value="pro" legend="Formule">
                    <div class="row-tight"><x-ds.radio-group.item value="free" id="plan-free" /><x-ds.label for="plan-free">Gratuit</x-ds.label></div>
                    <div class="row-tight"><x-ds.radio-group.item value="pro" id="plan-pro" /><x-ds.label for="plan-pro">Pro</x-ds.label></div>
                    <div class="row-tight"><x-ds.radio-group.item value="team" id="plan-team" /><x-ds.label for="plan-team">Équipe</x-ds.label></div>
                </x-ds.radio-group>
                <x-ds.slider :default-value="40" aria-label="Volume" />
                <x-ds.slider :default-value="[20, 80]" aria-label="Fourchette de prix" />
                <x-ds.dropzone id="x-files" multiple />
            </div>
        </div>
    </section>

    <section aria-labelledby="x-actions" data-demo="actions">
        <h2 id="x-actions">Actions</h2>
        <div class="row">
            <x-ds.button-group>
                <x-ds.button variant="outline">Archiver</x-ds.button>
                <x-ds.button variant="outline">Signaler</x-ds.button>
                <x-ds.button-group.separator />
                <x-ds.button variant="outline">Supprimer</x-ds.button>
            </x-ds.button-group>
            <x-ds.button-group>
                <x-ds.button-group.text>Lien</x-ds.button-group.text>
                <x-ds.button variant="outline">Copier</x-ds.button>
            </x-ds.button-group>
            <x-ds.toggle aria-label="Gras" variant="outline"><strong>B</strong></x-ds.toggle>
            <x-ds.toggle aria-label="Italique" :default-pressed="true"><em>I</em></x-ds.toggle>
            <x-ds.theme-toggle />
            <x-ds.button variant="primary" id="x-toast">Afficher une notification</x-ds.button>
        </div>
        <x-ds.collapsible>
            <x-ds.collapsible.trigger>Afficher les détails</x-ds.collapsible.trigger>
            <x-ds.collapsible.content>Le contenu replié est masqué (attribut <code>hidden</code>) jusqu'au clic.</x-ds.collapsible.content>
        </x-ds.collapsible>
    </section>

    <section aria-labelledby="x-content" data-demo="content">
        <h2 id="x-content">Contenu</h2>
        <div class="grid2">
            <x-ds.item.group>
                <x-ds.item variant="outline">
                    <x-ds.item.media variant="icon">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M4 4h16v16H4z" /></svg>
                    </x-ds.item.media>
                    <x-ds.item.content>
                        <x-ds.item.title>Rapport mensuel</x-ds.item.title>
                        <x-ds.item.description>Généré le 1er de chaque mois.</x-ds.item.description>
                    </x-ds.item.content>
                    <x-ds.item.actions><x-ds.button size="sm" variant="outline">Ouvrir</x-ds.button></x-ds.item.actions>
                </x-ds.item>
                <x-ds.item.separator />
                <x-ds.item variant="muted" size="sm">
                    <x-ds.item.header>En-tête</x-ds.item.header>
                    <x-ds.item.content><x-ds.item.title>Élément compact</x-ds.item.title></x-ds.item.content>
                    <x-ds.item.footer>Pied</x-ds.item.footer>
                </x-ds.item>
            </x-ds.item.group>
            <div class="stack">
                <x-ds.aspect-ratio :ratio="16 / 9"><div class="ratio-fill">16 / 9</div></x-ds.aspect-ratio>
                <div class="img-box"><x-ds.image :src="$logo" alt="Logo du site" width="48" height="48" /></div>
            </div>
        </div>
        <x-ds.carousel aria-label="Étapes">
            <x-ds.carousel.content>
                @foreach ($slides as $i => $s)
                    <x-ds.carousel.item><div class="slide">{{ $i + 1 }}. {{ $s }}</div></x-ds.carousel.item>
                @endforeach
            </x-ds.carousel.content>
            <x-ds.carousel.previous />
            <x-ds.carousel.next />
        </x-ds.carousel>
        <x-ds.container size="md" padding-y="sm" background="muted" rounded="lg">
            <x-ds.prose>
                <h3>Prose</h3>
                <p>Typographie d'un contenu riche : <a href="#x-content">liens</a>, <code>code</code>, listes…</p>
                <ul><li>Mesure de lecture limitée</li><li>Tailles en <em>em</em></li></ul>
            </x-ds.prose>
        </x-ds.container>
        <div class="narrow">
            {{-- alpharust-ignore: no-hardcoded-urls --}}
            <x-ds.video src="https://www.youtube.com/watch?v=aqz-KE-bpKQ" title="Big Buck Bunny (Blender Foundation)" />
        </div>
    </section>

    <section aria-labelledby="x-layout" data-demo="layout">
        <h2 id="x-layout">Barre latérale</h2>
        <div class="sidebar-frame">
            <x-ds.sidebar.provider>
                <x-ds.sidebar collapsible="icon">
                    <x-ds.sidebar.header><strong>Mon espace</strong></x-ds.sidebar.header>
                    <x-ds.sidebar.content>
                        <x-ds.sidebar.group>
                            <x-ds.sidebar.group-label>Navigation</x-ds.sidebar.group-label>
                            <x-ds.sidebar.group-content>
                                <x-ds.sidebar.menu>
                                    <x-ds.sidebar.menu-item>
                                        <x-ds.sidebar.menu-button href="#x-layout" :is-active="true" tooltip="Accueil">
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M3 11l9-8 9 8v10H3z" /></svg>
                                            <span>Accueil</span>
                                        </x-ds.sidebar.menu-button>
                                        <x-ds.sidebar.menu-badge>3</x-ds.sidebar.menu-badge>
                                    </x-ds.sidebar.menu-item>
                                    <x-ds.sidebar.menu-item>
                                        <x-ds.sidebar.menu-button tooltip="Projets">
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M3 7h18v13H3zM3 7l3-4h6l2 4" /></svg>
                                            <span>Projets</span>
                                        </x-ds.sidebar.menu-button>
                                        <x-ds.sidebar.menu-action :show-on-hover="true" aria-label="Ajouter un projet">+</x-ds.sidebar.menu-action>
                                        <x-ds.sidebar.menu-sub>
                                            <x-ds.sidebar.menu-sub-item><x-ds.sidebar.menu-sub-button href="#x-layout" :is-active="true">Site vitrine</x-ds.sidebar.menu-sub-button></x-ds.sidebar.menu-sub-item>
                                            <x-ds.sidebar.menu-sub-item><x-ds.sidebar.menu-sub-button href="#x-layout">Boutique</x-ds.sidebar.menu-sub-button></x-ds.sidebar.menu-sub-item>
                                        </x-ds.sidebar.menu-sub>
                                    </x-ds.sidebar.menu-item>
                                    <x-ds.sidebar.menu-item><x-ds.sidebar.menu-skeleton :show-icon="true" width="70%" /></x-ds.sidebar.menu-item>
                                </x-ds.sidebar.menu>
                            </x-ds.sidebar.group-content>
                        </x-ds.sidebar.group>
                    </x-ds.sidebar.content>
                    <x-ds.sidebar.separator />
                    <x-ds.sidebar.footer><x-ds.sidebar.input placeholder="Filtrer…" aria-label="Filtrer" /></x-ds.sidebar.footer>
                    <x-ds.sidebar.rail />
                </x-ds.sidebar>
                <x-ds.sidebar.inset>
                    <div class="inset-head"><x-ds.sidebar.trigger /><span>Contenu principal — <kbd>Ctrl</kbd> + <kbd>B</kbd> replie la barre.</span></div>
                </x-ds.sidebar.inset>
            </x-ds.sidebar.provider>
        </div>
    </section>

    <section aria-labelledby="x-toasts" data-demo="toasts">
        <h2 id="x-toasts">Notifications</h2>
        <div class="toast-sample">
            <x-ds.toast.item variant="success">
                <x-ds.toast.title variant="success">Enregistré</x-ds.toast.title>
                <x-ds.toast.description>Vos modifications sont en ligne.</x-ds.toast.description>
            </x-ds.toast.item>
        </div>
        <x-ds.toast.toaster position="bottom-right" />
    </section>

    <script>
        document.getElementById('x-toast')?.addEventListener('click', () => {
            window.__starwind__.toast.add({ title: 'Enregistré', description: 'Vos modifications sont en ligne.', variant: 'success' });
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
        .ratio-fill { display: grid; place-items: center; width: 100%; height: 100%; border-radius: 0.5rem; background: linear-gradient(135deg, var(--brand), color-mix(in oklab, var(--brand) 35%, white)); color: var(--brand-contrast); font-weight: 600; }
        .img-box img { width: 3rem; height: 3rem; }
        .slide { display: grid; place-items: center; height: 8rem; border: 1px solid var(--border, #d0d4de); border-radius: 0.5rem; font-size: 1.25rem; font-weight: 600; }
        .sidebar-frame { position: relative; height: 24rem; overflow: hidden; border: 1px solid var(--border, #d0d4de); border-radius: 0.75rem; transform: translateZ(0); }
        .sidebar-frame [data-slot='sidebar-provider'] { min-height: 100%; height: 100%; }
        .inset-head { display: flex; align-items: center; gap: var(--space-2); padding: var(--space-3); }
        .toast-sample { position: relative; max-width: 24rem; }
        section { margin-bottom: var(--space-8); }
    </style>
</x-site-layout>
