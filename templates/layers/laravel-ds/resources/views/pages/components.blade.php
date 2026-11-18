@php
    $variants = ['default', 'primary', 'secondary', 'outline', 'ghost', 'info', 'success', 'warning', 'error'];
    $pixel = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7';
@endphp
{{-- Catalogue de composants : même contenu que la page Astro (/components) et la page Django, mêmes data-slot. --}}
<x-layouts.site title="Composants">
<div class="ds-page site-main--page">
    <h1>Composants</h1>
    <p class="meta">
        Catalogue du kit <strong>Blade</strong> (<code>resources/views/components/ds</code>). L'allure se règle dans
        <code>resources/css/design/tokens/components.css</code>, <code>resources/css/design/components/*.css</code> et <code>themes/theme.css</code>.
        Les 23 composants étendus (fenêtres, saisie, barre latérale, notifications…) sont sur la page <a href="{{ url('/components/advanced') }}">composants avancés</a>.
    </p>

    <section aria-labelledby="c-buttons" data-demo="button">
        <h2 id="c-buttons">Boutons</h2>
        <div class="row">
            @foreach ($variants as $variant)
                <x-ds.button :variant="$variant">{{ $variant }}</x-ds.button>
            @endforeach
        </div>
        <div class="row">
            <x-ds.button size="sm" variant="primary">sm</x-ds.button>
            <x-ds.button size="md" variant="primary">md</x-ds.button>
            <x-ds.button size="lg" variant="primary">lg</x-ds.button>
            <x-ds.button variant="outline" href="#c-buttons">Lien</x-ds.button>
            <x-ds.button variant="primary" disabled>Désactivé</x-ds.button>
        </div>
    </section>

    <section aria-labelledby="c-badges" data-demo="badge">
        <h2 id="c-badges">Badges</h2>
        <div class="row">
            @foreach ($variants as $variant)
                <x-ds.badge :variant="$variant">{{ $variant }}</x-ds.badge>
            @endforeach
        </div>
    </section>

    <section aria-labelledby="c-alerts" data-demo="alert">
        <h2 id="c-alerts">Alertes</h2>
        <div class="stack">
            <x-ds.alert variant="info">
                <x-ds.alert.title>Information</x-ds.alert.title>
                <x-ds.alert.description>Les couleurs viennent de <code>tokens/colors.css</code>.</x-ds.alert.description>
            </x-ds.alert>
            <x-ds.alert variant="success"><x-ds.alert.title>Succès</x-ds.alert.title><x-ds.alert.description>L'opération a réussi.</x-ds.alert.description></x-ds.alert>
            <x-ds.alert variant="warning"><x-ds.alert.title>Attention</x-ds.alert.title><x-ds.alert.description>Vérifiez les données.</x-ds.alert.description></x-ds.alert>
            <x-ds.alert variant="error"><x-ds.alert.title>Erreur</x-ds.alert.title><x-ds.alert.description>Lisible en clair comme en sombre.</x-ds.alert.description></x-ds.alert>
        </div>
    </section>

    <section aria-labelledby="c-card" data-demo="card">
        <h2 id="c-card">Carte</h2>
        <div class="narrow">
            <x-ds.card>
                <x-ds.card.header>
                    <x-ds.card.title>Carte</x-ds.card.title>
                    <x-ds.card.description>Rayon : <code>--card-border-radius</code></x-ds.card.description>
                    <x-ds.card.action><x-ds.badge variant="primary">Nouveau</x-ds.badge></x-ds.card.action>
                </x-ds.card.header>
                <x-ds.card.content>
                    <x-ds.label for="demo-name">Nom</x-ds.label>
                    <x-ds.input id="demo-name" placeholder="Votre nom" />
                </x-ds.card.content>
                <x-ds.card.footer>
                    <x-ds.button variant="primary">Enregistrer</x-ds.button>
                    <x-ds.button variant="ghost">Annuler</x-ds.button>
                </x-ds.card.footer>
            </x-ds.card>
        </div>
    </section>

    <section aria-labelledby="c-forms" data-demo="form">
        <h2 id="c-forms">Formulaire</h2>
        <div class="narrow stack">
            <div class="stack-sm">
                <x-ds.label for="demo-email">Courriel</x-ds.label>
                <x-ds.input id="demo-email" type="email" placeholder="vous@exemple.fr" />
                <x-ds.input size="sm" placeholder="Petit" aria-label="Petit champ" />
                <x-ds.input size="lg" placeholder="Grand" aria-label="Grand champ" />
            </div>
            <div class="stack-sm">
                <x-ds.label for="demo-msg">Message</x-ds.label>
                <x-ds.textarea id="demo-msg" placeholder="Écrivez ici…" />
            </div>
            <div class="stack-sm">
                <x-ds.label for="demo-pick">Liste</x-ds.label>
                <x-ds.native-select id="demo-pick">
                    <x-ds.native-select.option value="a">Option A</x-ds.native-select.option>
                    <x-ds.native-select.option value="b">Option B</x-ds.native-select.option>
                </x-ds.native-select>
            </div>
            <x-ds.checkbox id="demo-check" label="J'accepte les conditions" variant="primary" :checked="true" />
            <x-ds.switch id="demo-switch" label="Notifications" variant="primary" :checked="true" />
        </div>
    </section>

    <section aria-labelledby="c-feedback" data-demo="feedback">
        <h2 id="c-feedback">Progression et chargement</h2>
        <div class="narrow stack">
            <x-ds.progress :value="60" variant="primary" aria-label="Progression" />
            <x-ds.progress variant="success" aria-label="Chargement" />
            <div class="row">
                <x-ds.skeleton class="sk" />
                <x-ds.spinner />
                <x-ds.avatar size="sm"><x-ds.avatar.fallback>AR</x-ds.avatar.fallback></x-ds.avatar>
                <x-ds.avatar><x-ds.avatar.image :src="$pixel" alt="" /><x-ds.avatar.fallback>AR</x-ds.avatar.fallback></x-ds.avatar>
                <x-ds.avatar size="lg"><x-ds.avatar.fallback>AR</x-ds.avatar.fallback></x-ds.avatar>
            </div>
        </div>
    </section>

    <section aria-labelledby="c-nav" data-demo="navigation">
        <h2 id="c-nav">Navigation</h2>
        <x-ds.breadcrumb>
            <x-ds.breadcrumb.list>
                <x-ds.breadcrumb.item><x-ds.breadcrumb.link href="/">Accueil</x-ds.breadcrumb.link></x-ds.breadcrumb.item>
                <x-ds.breadcrumb.separator />
                <x-ds.breadcrumb.item><x-ds.breadcrumb.link href="#c-nav">Composants</x-ds.breadcrumb.link></x-ds.breadcrumb.item>
                <x-ds.breadcrumb.separator />
                <x-ds.breadcrumb.item><x-ds.breadcrumb.page>Navigation</x-ds.breadcrumb.page></x-ds.breadcrumb.item>
            </x-ds.breadcrumb.list>
        </x-ds.breadcrumb>
        <x-ds.pagination>
            <x-ds.pagination.content>
                <x-ds.pagination.item><x-ds.pagination.previous href="#c-nav" /></x-ds.pagination.item>
                <x-ds.pagination.item><x-ds.pagination.link href="#c-nav">1</x-ds.pagination.link></x-ds.pagination.item>
                <x-ds.pagination.item><x-ds.pagination.link href="#c-nav" :is-active="true">2</x-ds.pagination.link></x-ds.pagination.item>
                <x-ds.pagination.item><x-ds.pagination.link href="#c-nav">3</x-ds.pagination.link></x-ds.pagination.item>
                <x-ds.pagination.item><x-ds.pagination.next href="#c-nav" /></x-ds.pagination.item>
            </x-ds.pagination.content>
        </x-ds.pagination>
    </section>

    <section aria-labelledby="c-table" data-demo="table">
        <h2 id="c-table">Tableau</h2>
        <x-ds.table>
            <x-ds.table.caption>Équipe</x-ds.table.caption>
            <x-ds.table.header>
                <x-ds.table.row><x-ds.table.head>Nom</x-ds.table.head><x-ds.table.head>Rôle</x-ds.table.head><x-ds.table.head>Statut</x-ds.table.head></x-ds.table.row>
            </x-ds.table.header>
            <x-ds.table.body>
                <x-ds.table.row><x-ds.table.cell>Ada</x-ds.table.cell><x-ds.table.cell>Ingénieure</x-ds.table.cell><x-ds.table.cell><x-ds.badge variant="success">Actif</x-ds.badge></x-ds.table.cell></x-ds.table.row>
                <x-ds.table.row><x-ds.table.cell>Linus</x-ds.table.cell><x-ds.table.cell>Mainteneur</x-ds.table.cell><x-ds.table.cell><x-ds.badge variant="warning">Absent</x-ds.badge></x-ds.table.cell></x-ds.table.row>
            </x-ds.table.body>
            <x-ds.table.foot>
                <x-ds.table.row><x-ds.table.cell>Total</x-ds.table.cell><x-ds.table.cell>2</x-ds.table.cell><x-ds.table.cell></x-ds.table.cell></x-ds.table.row>
            </x-ds.table.foot>
        </x-ds.table>
    </section>

    <section aria-labelledby="c-interactive" data-demo="interactive">
        <h2 id="c-interactive">Composants interactifs</h2>
        <x-ds.tabs default-value="one">
            <x-ds.tabs.list>
                <x-ds.tabs.trigger value="one" :default-active="true">Premier</x-ds.tabs.trigger>
                <x-ds.tabs.trigger value="two">Deuxième</x-ds.tabs.trigger>
                <x-ds.tabs.trigger value="three">Troisième</x-ds.tabs.trigger>
            </x-ds.tabs.list>
            <x-ds.tabs.content value="one" :default-active="true">Contenu du premier onglet (flèches ← → pour naviguer).</x-ds.tabs.content>
            <x-ds.tabs.content value="two">Contenu du deuxième onglet.</x-ds.tabs.content>
            <x-ds.tabs.content value="three">Contenu du troisième onglet.</x-ds.tabs.content>
        </x-ds.tabs>

        <x-ds.accordion type="single" default-value="a">
            <x-ds.accordion.item value="a"><x-ds.accordion.trigger>Qu'est-ce que le catalogue ?</x-ds.accordion.trigger><x-ds.accordion.content>Un jeu de composants au même balisage en Astro, Laravel et Django.</x-ds.accordion.content></x-ds.accordion.item>
            <x-ds.accordion.item value="b"><x-ds.accordion.trigger>Comment le personnaliser ?</x-ds.accordion.trigger><x-ds.accordion.content>Par les jetons de <code>resources/css/design/tokens</code>.</x-ds.accordion.content></x-ds.accordion.item>
        </x-ds.accordion>

        <div class="row">
            <x-ds.dialog id="demo-dialog">
                <x-ds.dialog.trigger :as-child="true"><x-ds.button variant="outline">Ouvrir la boîte de dialogue</x-ds.button></x-ds.dialog.trigger>
                <x-ds.dialog.content>
                    <x-ds.dialog.header>
                        <x-ds.dialog.title>Confirmer</x-ds.dialog.title>
                        <x-ds.dialog.description>Cette action est réversible.</x-ds.dialog.description>
                    </x-ds.dialog.header>
                    <x-ds.dialog.footer>
                        <x-ds.dialog.close :as-child="true"><x-ds.button variant="primary">Fermer</x-ds.button></x-ds.dialog.close>
                    </x-ds.dialog.footer>
                </x-ds.dialog.content>
            </x-ds.dialog>

            <x-ds.tooltip>
                <x-ds.tooltip.trigger><x-ds.button variant="outline">Survolez-moi</x-ds.button></x-ds.tooltip.trigger>
                <x-ds.tooltip.content>Info-bulle accessible au clavier</x-ds.tooltip.content>
            </x-ds.tooltip>

            <x-ds.kbd.group><x-ds.kbd>Ctrl</x-ds.kbd><x-ds.kbd>K</x-ds.kbd></x-ds.kbd.group>
        </div>
        <x-ds.separator />
    </section>

    <style>
        .row { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-3); margin-bottom: var(--space-4); }
        .narrow { max-width: 28rem; }
        .stack { display: grid; gap: var(--space-4); }
        .stack-sm { display: grid; gap: var(--space-2); }
        .sk { width: 6rem; height: 1rem; }
        section { margin-bottom: var(--space-8); }
    </style>
</div>
</x-layouts.site>
