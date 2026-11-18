"""Site vitrine complet — SOURCE UNIQUE du contenu, de la navigation et des pages.

Rendu à l'identique en Astro (layout + pages + collections), Laravel (layout Blade + routes/contrôleurs + migrations/seeders)
et Wagtail (modèles de page + StreamField + snippets/réglages + base.html) par tools/dsgen/site/write.py.
Le nom du site est celui du projet (`@@title@@`, substitué à la génération).
Les URL sont écrites sans barre finale ; chaque framework applique sa convention (Astro/Wagtail : `/blog/`, Laravel : `/blog`).
"""

SITE = {
    'tagline': 'Des sites web rapides, accessibles et faciles à faire évoluer.',
    'description': "Agence web indépendante : création de sites, refonte, accessibilité et maintenance.",
    'cta': {'label': 'Demander un devis', 'url': '/contact'},
    'contact': {
        'email': 'contact@exemple.fr',
        'phone': '+33 1 23 45 67 89',
        'address': '12 rue de la Paix, 75002 Paris',
        'hours': 'Du lundi au vendredi, 9 h – 18 h',
    },
    'social': [
        {'label': 'GitHub', 'url': 'https://github.com/', 'icon': 'github'},  # alpharust-ignore: no-hardcoded-urls
        {'label': 'LinkedIn', 'url': 'https://www.linkedin.com/', 'icon': 'linkedin'},  # alpharust-ignore: no-hardcoded-urls
        {'label': 'Mastodon', 'url': 'https://mastodon.social/', 'icon': 'mastodon'},  # alpharust-ignore: no-hardcoded-urls
    ],
    # Menu principal : l'entrée `services` reçoit les services en sous-menu (liste déroulante, repli mobile).
    'nav': [
        {'label': 'Accueil', 'url': '/'},
        {'label': 'Services', 'url': '/services', 'children': 'services'},
        {'label': 'Blog', 'url': '/blog'},
        {'label': 'À propos', 'url': '/a-propos'},
        {'label': 'Contact', 'url': '/contact'},
    ],
    'footer': [
        {'title': 'Navigation', 'links': [
            {'label': 'Accueil', 'url': '/'}, {'label': 'Blog', 'url': '/blog'},
            {'label': 'À propos', 'url': '/a-propos'}, {'label': 'Contact', 'url': '/contact'}]},
        {'title': 'Services', 'links': 'services'},
        {'title': 'Informations', 'links': [
            {'label': 'Mentions légales', 'url': '/mentions-legales'},
            {'label': 'Politique de confidentialité', 'url': '/confidentialite'}]},
    ],
    'copyright': 'Tous droits réservés.',
}

SERVICES = [
    {'slug': 'creation-site-web', 'title': 'Création de site web', 'icon': 'code', 'price': 'À partir de 2 400 €',
     'summary': "Un site sur mesure, rapide et référencé, livré avec son espace d'administration.",
     'features': ['Maquettes et design system', 'Contenus éditables', 'SEO technique et données structurées', 'Mise en ligne et formation'],
     'body': """Nous concevons votre site à partir de vos objectifs : qui doit le trouver, que doit-il y faire, comment le mesurer.

## Notre méthode

Ateliers de cadrage, maquettes, puis développement par itérations courtes. Chaque version est **mise en ligne sur un
environnement de recette** pour que vous puissiez la tester.

## Ce qui est livré

- un site responsive, accessible et performant ;
- un espace d'administration pour modifier vos contenus ;
- la documentation et une formation d'une demi-journée."""},
    {'slug': 'refonte-performance', 'title': 'Refonte et performance', 'icon': 'zap', 'price': 'À partir de 1 900 €',
     'summary': 'Un site existant plus rapide, plus clair et mieux classé, sans perdre votre référencement.',
     'features': ['Audit Lighthouse et Core Web Vitals', 'Plan de redirections', 'Optimisation des images', 'Suivi après mise en ligne'],
     'body': """Une refonte réussie commence par un audit : temps de chargement, parcours, contenus qui convertissent.

## Garder le référencement

Toutes les anciennes adresses sont **redirigées** et suivies pendant trois mois après la mise en ligne.

## Mesurer le gain

- scores Lighthouse avant / après ;
- Core Web Vitals sur les pages clés ;
- taux de conversion des formulaires."""},
    {'slug': 'accessibilite', 'title': 'Accessibilité (RGAA)', 'icon': 'accessibility', 'price': 'À partir de 1 200 €',
     'summary': 'Audit et mise en conformité RGAA / WCAG 2.2 AA, déclaration d\'accessibilité comprise.',
     'features': ['Audit sur échantillon de pages', 'Corrections priorisées', "Déclaration d'accessibilité", 'Formation des rédacteurs'],
     'body': """L'accessibilité profite à tous : navigation au clavier, contrastes, lecteurs d'écran, formulaires compréhensibles.

## Audit

Nous évaluons un échantillon représentatif de pages selon le **RGAA 4.1** et livrons un rapport priorisé.

## Mise en conformité

- corrections dans le code et les gabarits ;
- contrôle des contrastes du thème ;
- rédaction de la déclaration d'accessibilité."""},
    {'slug': 'maintenance', 'title': 'Maintenance et hébergement', 'icon': 'shield', 'price': 'À partir de 90 € / mois',
     'summary': 'Mises à jour, sauvegardes, surveillance et petites évolutions, sans surprise.',
     'features': ['Mises à jour de sécurité', 'Sauvegardes quotidiennes', 'Surveillance 24 h / 24', 'Une heure d\'évolution par mois'],
     'body': """Un site est un logiciel : il doit être mis à jour, sauvegardé et surveillé.

## Ce qui est compris

- mises à jour du framework et des dépendances ;
- sauvegardes quotidiennes conservées 30 jours ;
- surveillance de la disponibilité et des certificats.

## Évolutions

Chaque mois, une heure d'évolution est incluse : nouvelle section, formulaire, page."""},
]

POSTS = [
    {'slug': 'choisir-son-framework', 'title': 'Astro, Laravel ou Wagtail : choisir son framework', 'date': '2026-10-02',
     'category': 'Technique', 'author': 'Camille Martin',
     'excerpt': 'Trois excellents outils, trois façons de travailler. Nos critères pour choisir sans regret.',
     'body': """Le bon framework est celui que votre équipe saura faire vivre. Voici nos critères.

## Astro pour les sites de contenu

Pages statiques par défaut, îlots interactifs à la demande : idéal pour un site vitrine ou un blog très rapide.

## Laravel pour les applications

Authentification, files d'attente, e-mails, paiements : **tout est prévu** pour une application métier.

## Wagtail pour les rédactions

Un vrai CMS éditorial : arborescence de pages, StreamField, révisions et prévisualisation."""},
    {'slug': 'accessibilite-checklist', 'title': "L'accessibilité en 10 points", 'date': '2026-09-15',
     'category': 'Accessibilité', 'author': 'Inès Benali',
     'excerpt': 'Une liste courte et concrète pour éviter les erreurs les plus fréquentes.',
     'body': """Ces dix points couvrent la majorité des problèmes rencontrés en audit.

## Structure

- un seul `h1` par page et des titres hiérarchisés ;
- des repères : `header`, `nav`, `main`, `footer` ;
- un lien d'évitement vers le contenu.

## Interaction

- tout est utilisable **au clavier** ;
- le focus est toujours visible ;
- les champs de formulaire ont un libellé."""},
    {'slug': 'performance-images', 'title': 'Des images légères sans perte de qualité', 'date': '2026-08-28',
     'category': 'Performance', 'author': 'Camille Martin',
     'excerpt': "Formats modernes, dimensions explicites, chargement différé : le trio gagnant.",
     'body': """Les images représentent souvent plus de la moitié du poids d'une page.

## Les bons formats

AVIF et WebP divisent le poids par deux à cinq par rapport au JPEG, à qualité égale.

## Les bons attributs

- `width` et `height` pour éviter les décalages de mise en page ;
- `loading="lazy"` sous la ligne de flottaison ;
- `srcset` pour servir la bonne taille."""},
]

# ---------------------------------------------------------------------------------------------------------------
# Pages composées de sections (StreamField côté Wagtail, config/site.php côté Laravel, src/data/site.json côté Astro).
# Types de section : hero, page_header, stats, features, services, testimonials, posts, faq, cta, text, team, contact.

PAGES = {
    'home': {'url': '/', 'title': 'Accueil', 'seo_title': None, 'description': SITE['description'], 'sections': [
        {'type': 'hero', 'badge': 'Agence web indépendante',
         'title': 'Des sites rapides, accessibles et faciles à faire évoluer',
         'lead': "Nous concevons et maintenons des sites qui se chargent en moins d'une seconde, respectent le RGAA et "
                 "restent simples à modifier.",
         'primary': {'label': 'Demander un devis', 'url': '/contact'},
         'secondary': {'label': 'Voir nos services', 'url': '/services'}},
        {'type': 'stats', 'items': [
            {'value': '98/100', 'label': 'Score Lighthouse moyen'}, {'value': 'AA', 'label': 'Niveau RGAA visé'},
            {'value': '< 1 s', 'label': 'Affichage du contenu'}, {'value': '12 ans', 'label': "D'expérience"}]},
        {'type': 'features', 'title': 'Pourquoi nous choisir', 'lead': 'Quatre engagements, sur chaque projet.', 'items': [
            {'icon': 'zap', 'title': 'Performance', 'text': 'Pages légères, images optimisées, scores mesurés à chaque livraison.'},
            {'icon': 'accessibility', 'title': 'Accessibilité', 'text': 'Navigation au clavier, contrastes contrôlés, sémantique soignée.'},
            {'icon': 'pencil', 'title': 'Autonomie', 'text': 'Vous modifiez vos contenus sans nous appeler.'},
            {'icon': 'shield', 'title': 'Sérénité', 'text': 'Mises à jour, sauvegardes et surveillance incluses.'}]},
        {'type': 'services', 'title': 'Nos services', 'lead': 'De la création à la maintenance.', 'limit': 3,
         'link': {'label': 'Tous les services', 'url': '/services'}},
        {'type': 'testimonials', 'title': 'Ils nous font confiance', 'items': [
            {'quote': 'Notre nouveau site est deux fois plus rapide et nos demandes de devis ont augmenté de 40 %.',
             'name': 'Sophie Laurent', 'role': 'Directrice, Atelier Laurent', 'initials': 'SL'},
            {'quote': "Un audit d'accessibilité clair, des corrections livrées en trois semaines.",
             'name': 'Marc Petit', 'role': 'Responsable digital, Ville de Montreuil', 'initials': 'MP'},
            {'quote': 'Nous publions nos actualités en cinq minutes, sans aide.',
             'name': 'Léa Moreau', 'role': 'Chargée de communication, Asso Horizon', 'initials': 'LM'}]},
        {'type': 'posts', 'title': 'Derniers articles', 'lead': 'Conseils et retours d\'expérience.', 'limit': 3,
         'link': {'label': 'Tout le blog', 'url': '/blog'}},
        {'type': 'faq', 'title': 'Questions fréquentes', 'items': [
            {'question': 'Combien de temps faut-il pour créer un site ?', 'answer': 'Entre quatre et huit semaines selon le nombre de pages et de fonctionnalités.'},
            {'question': 'Pourrai-je modifier mon site moi-même ?', 'answer': "Oui : chaque site est livré avec un espace d'administration et une formation."},
            {'question': 'Mon site sera-t-il accessible ?', 'answer': 'Nous visons le niveau AA du RGAA et livrons une déclaration d\'accessibilité.'},
            {'question': 'Proposez-vous la maintenance ?', 'answer': 'Oui, avec un forfait mensuel qui inclut mises à jour, sauvegardes et petites évolutions.'}]},
        {'type': 'cta', 'title': 'Un projet en tête ?', 'text': 'Parlons-en : réponse sous 24 heures ouvrées.',
         'action': {'label': 'Demander un devis', 'url': '/contact'}},
    ]},
    'about': {'url': '/a-propos', 'title': 'À propos', 'description': "Qui nous sommes, notre mission et nos valeurs.", 'sections': [
        {'type': 'page_header', 'eyebrow': 'À propos', 'title': 'Une petite équipe, de grandes exigences',
         'lead': 'Développeurs, designers et experts accessibilité réunis autour du web durable.'},
        {'type': 'text', 'title': 'Notre mission', 'body': """Nous croyons qu'un bon site est **rapide, accessible et sobre**. Rapide, parce que chaque seconde
compte. Accessible, parce que le web est pour tout le monde. Sobre, parce que le meilleur code est celui qu'on n'écrit pas.

Depuis 2014, nous accompagnons artisans, associations, collectivités et PME, du premier atelier à la maintenance."""},
        {'type': 'features', 'title': 'Nos valeurs', 'lead': None, 'items': [
            {'icon': 'heart', 'title': 'Transparence', 'text': 'Devis détaillés, code ouvert, aucun verrouillage.'},
            {'icon': 'leaf', 'title': 'Sobriété', 'text': 'Moins de dépendances, moins de données, moins d\'énergie.'},
            {'icon': 'users', 'title': 'Proximité', 'text': 'Un interlocuteur unique, joignable directement.'}]},
        {'type': 'team', 'title': "L'équipe", 'lead': 'Quatre personnes, toutes basées à Paris.', 'items': [
            {'name': 'Camille Martin', 'role': 'Fondatrice, développeuse', 'initials': 'CM'},
            {'name': 'Inès Benali', 'role': 'Experte accessibilité', 'initials': 'IB'},
            {'name': 'Thomas Roux', 'role': 'Designer UI', 'initials': 'TR'},
            {'name': 'Hugo Lefèvre', 'role': 'Développeur back-end', 'initials': 'HL'}]},
        {'type': 'cta', 'title': 'Travaillons ensemble', 'text': 'Présentez-nous votre projet, nous revenons vers vous rapidement.',
         'action': {'label': 'Nous contacter', 'url': '/contact'}},
    ]},
    'services': {'url': '/services', 'title': 'Services', 'description': 'Création, refonte, accessibilité et maintenance de sites web.', 'sections': [
        {'type': 'page_header', 'eyebrow': 'Services', 'title': 'Ce que nous faisons',
         'lead': 'Quatre offres claires, combinables selon votre projet.'},
        {'type': 'services', 'title': None, 'lead': None, 'limit': 0, 'link': None},
        {'type': 'cta', 'title': 'Besoin d\'un conseil ?', 'text': 'Nous vous aidons à choisir la bonne formule.',
         'action': {'label': 'Demander un devis', 'url': '/contact'}},
    ]},
    'blog': {'url': '/blog', 'title': 'Blog', 'description': 'Conseils et retours d\'expérience sur le web performant et accessible.', 'sections': [
        {'type': 'page_header', 'eyebrow': 'Blog', 'title': 'Articles et conseils',
         'lead': 'Performance, accessibilité, choix techniques : ce que nous apprenons sur le terrain.'},
        {'type': 'posts', 'title': None, 'lead': None, 'limit': 0, 'link': None},
    ]},
    'contact': {'url': '/contact', 'title': 'Contact', 'description': 'Écrivez-nous : réponse sous 24 heures ouvrées.', 'sections': [
        {'type': 'page_header', 'eyebrow': 'Contact', 'title': 'Parlons de votre projet',
         'lead': 'Remplissez le formulaire ou écrivez-nous directement, nous répondons sous 24 heures ouvrées.'},
        {'type': 'contact', 'title': 'Écrivez-nous', 'success': 'Merci ! Votre message a bien été envoyé.',
         'submit': 'Envoyer le message'},
    ]},
    'legal': {'url': '/mentions-legales', 'title': 'Mentions légales', 'description': 'Mentions légales du site.', 'sections': [
        {'type': 'page_header', 'eyebrow': None, 'title': 'Mentions légales', 'lead': None},
        {'type': 'text', 'title': None, 'body': """## Éditeur

Ce site est édité par **@@title@@**, 12 rue de la Paix, 75002 Paris. Contact : contact@exemple.fr.

## Hébergement

Le site est hébergé par votre prestataire d'hébergement ; ses coordonnées figurent dans le contrat de service.

## Propriété intellectuelle

Les textes, images et éléments graphiques de ce site sont protégés. Toute reproduction sans autorisation est interdite."""},
    ]},
    'privacy': {'url': '/confidentialite', 'title': 'Politique de confidentialité', 'description': 'Comment nous traitons vos données personnelles.', 'sections': [
        {'type': 'page_header', 'eyebrow': None, 'title': 'Politique de confidentialité', 'lead': None},
        {'type': 'text', 'title': None, 'body': """## Données collectées

Le formulaire de contact recueille votre nom, votre adresse e-mail et votre message, uniquement pour vous répondre.

## Durée de conservation

Les messages sont conservés douze mois, puis supprimés.

## Vos droits

Vous pouvez demander l'accès, la rectification ou la suppression de vos données en écrivant à contact@exemple.fr."""},
    ]},
}

NOT_FOUND = {'title': 'Page introuvable', 'code': '404', 'text': "La page demandée n'existe pas ou a été déplacée.",
             'action': {'label': "Retour à l'accueil", 'url': '/'}}

CONTACT_FIELDS = [
    {'name': 'nom', 'label': 'Nom', 'type': 'text', 'autocomplete': 'name', 'required': True},
    {'name': 'email', 'label': 'Adresse e-mail', 'type': 'email', 'autocomplete': 'email', 'required': True},
    {'name': 'sujet', 'label': 'Sujet', 'type': 'text', 'autocomplete': 'off', 'required': False},
    {'name': 'message', 'label': 'Message', 'type': 'textarea', 'autocomplete': 'off', 'required': True},
]
