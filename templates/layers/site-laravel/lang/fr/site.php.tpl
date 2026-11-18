<?php

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
// Textes du site en « fr » : lus par App\Support\SiteData via trans('site').

return [
    'name' => '@@title@@',
    'tagline' => 'Des sites web rapides, accessibles et faciles à faire évoluer.',
    'description' => 'Agence web indépendante : création de sites, refonte, accessibilité et maintenance.',
    'cta' => [
        'label' => 'Demander un devis',
        'url' => '/contact',
    ],
    'contact' => [
        'email' => 'contact@exemple.fr',
        'phone' => '+33 1 23 45 67 89',
        'address' => '12 rue de la Paix, 75002 Paris',
        'hours' => 'Du lundi au vendredi, 9 h – 18 h',
    ],
    'social' => [
        [
            'label' => 'GitHub',
            'url' => 'https://github.com/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'github',
        ],
        [
            'label' => 'LinkedIn',
            'url' => 'https://www.linkedin.com/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'linkedin',
        ],
        [
            'label' => 'Mastodon',
            'url' => 'https://mastodon.social/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'mastodon',
        ],
    ],
    'nav' => [
        [
            'label' => 'Accueil',
            'url' => '/',
        ],
        [
            'label' => 'Services',
            'url' => '/services',
            'children' => 'services',
        ],
        [
            'label' => 'Blog',
            'url' => '/blog',
        ],
        [
            'label' => 'À propos',
            'url' => '/a-propos',
        ],
        [
            'label' => 'Contact',
            'url' => '/contact',
        ],
    ],
    'footer' => [
        [
            'title' => 'Navigation',
            'links' => [
                [
                    'label' => 'Accueil',
                    'url' => '/',
                ],
                [
                    'label' => 'Blog',
                    'url' => '/blog',
                ],
                [
                    'label' => 'À propos',
                    'url' => '/a-propos',
                ],
                [
                    'label' => 'Contact',
                    'url' => '/contact',
                ],
            ],
        ],
        [
            'title' => 'Services',
            'links' => 'services',
        ],
        [
            'title' => 'Informations',
            'links' => [
                [
                    'label' => 'Mentions légales',
                    'url' => '/mentions-legales',
                ],
                [
                    'label' => 'Politique de confidentialité',
                    'url' => '/confidentialite',
                ],
            ],
        ],
    ],
    'copyright' => 'Tous droits réservés.',
    'pages' => [
        'home' => [
            'url' => '/',
            'title' => 'Accueil',
            'seo_title' => null,
            'description' => 'Agence web indépendante : création de sites, refonte, accessibilité et maintenance.',
            'sections' => [
                [
                    'type' => 'hero',
                    'badge' => 'Agence web indépendante',
                    'title' => 'Des sites rapides, accessibles et faciles à faire évoluer',
                    'lead' => 'Nous concevons et maintenons des sites qui se chargent en moins d’une seconde, respectent le RGAA et restent simples à modifier.',
                    'primary' => [
                        'label' => 'Demander un devis',
                        'url' => '/contact',
                    ],
                    'secondary' => [
                        'label' => 'Voir nos services',
                        'url' => '/services',
                    ],
                ],
                [
                    'type' => 'stats',
                    'items' => [
                        [
                            'value' => '98/100',
                            'label' => 'Score Lighthouse moyen',
                        ],
                        [
                            'value' => 'AA',
                            'label' => 'Niveau RGAA visé',
                        ],
                        [
                            'value' => '< 1 s',
                            'label' => 'Affichage du contenu',
                        ],
                        [
                            'value' => '12 ans',
                            'label' => 'D’expérience',
                        ],
                    ],
                ],
                [
                    'type' => 'features',
                    'title' => 'Pourquoi nous choisir',
                    'lead' => 'Quatre engagements, sur chaque projet.',
                    'items' => [
                        [
                            'icon' => 'zap',
                            'title' => 'Performance',
                            'text' => 'Pages légères, images optimisées, scores mesurés à chaque livraison.',
                        ],
                        [
                            'icon' => 'accessibility',
                            'title' => 'Accessibilité',
                            'text' => 'Navigation au clavier, contrastes contrôlés, sémantique soignée.',
                        ],
                        [
                            'icon' => 'pencil',
                            'title' => 'Autonomie',
                            'text' => 'Vous modifiez vos contenus sans nous appeler.',
                        ],
                        [
                            'icon' => 'shield',
                            'title' => 'Sérénité',
                            'text' => 'Mises à jour, sauvegardes et surveillance incluses.',
                        ],
                    ],
                ],
                [
                    'type' => 'services',
                    'title' => 'Nos services',
                    'lead' => 'De la création à la maintenance.',
                    'limit' => 3,
                    'link' => [
                        'label' => 'Tous les services',
                        'url' => '/services',
                    ],
                ],
                [
                    'type' => 'testimonials',
                    'title' => 'Ils nous font confiance',
                    'items' => [
                        [
                            'quote' => 'Notre nouveau site est deux fois plus rapide et nos demandes de devis ont augmenté de 40 %.',
                            'name' => 'Sophie Laurent',
                            'role' => 'Directrice, Atelier Laurent',
                            'initials' => 'SL',
                        ],
                        [
                            'quote' => 'Un audit d’accessibilité clair, des corrections livrées en trois semaines.',
                            'name' => 'Marc Petit',
                            'role' => 'Responsable digital, Ville de Montreuil',
                            'initials' => 'MP',
                        ],
                        [
                            'quote' => 'Nous publions nos actualités en cinq minutes, sans aide.',
                            'name' => 'Léa Moreau',
                            'role' => 'Chargée de communication, Asso Horizon',
                            'initials' => 'LM',
                        ],
                    ],
                ],
                [
                    'type' => 'posts',
                    'title' => 'Derniers articles',
                    'lead' => 'Conseils et retours d’expérience.',
                    'limit' => 3,
                    'link' => [
                        'label' => 'Tout le blog',
                        'url' => '/blog',
                    ],
                ],
                [
                    'type' => 'faq',
                    'title' => 'Questions fréquentes',
                    'items' => [
                        [
                            'question' => 'Combien de temps faut-il pour créer un site ?',
                            'answer' => 'Entre quatre et huit semaines selon le nombre de pages et de fonctionnalités.',
                        ],
                        [
                            'question' => 'Pourrai-je modifier mon site moi-même ?',
                            'answer' => 'Oui : chaque site est livré avec un espace d’administration et une formation.',
                        ],
                        [
                            'question' => 'Mon site sera-t-il accessible ?',
                            'answer' => 'Nous visons le niveau AA du RGAA et livrons une déclaration d’accessibilité.',
                        ],
                        [
                            'question' => 'Proposez-vous la maintenance ?',
                            'answer' => 'Oui, avec un forfait mensuel qui inclut mises à jour, sauvegardes et petites évolutions.',
                        ],
                    ],
                ],
                [
                    'type' => 'cta',
                    'title' => 'Un projet en tête ?',
                    'text' => 'Parlons-en : réponse sous 24 heures ouvrées.',
                    'action' => [
                        'label' => 'Demander un devis',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'about' => [
            'url' => '/a-propos',
            'title' => 'À propos',
            'description' => 'Qui nous sommes, notre mission et nos valeurs.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'À propos',
                    'title' => 'Une petite équipe, de grandes exigences',
                    'lead' => 'Développeurs, designers et experts accessibilité réunis autour du web durable.',
                ],
                [
                    'type' => 'text',
                    'title' => 'Notre mission',
                    'body' => '<p>Nous croyons qu’un bon site est <strong>rapide, accessible et sobre</strong>. Rapide, parce que chaque seconde compte. Accessible, parce que le web est pour tout le monde. Sobre, parce que le meilleur code est celui qu’on n’écrit pas.</p>
<p>Depuis 2014, nous accompagnons artisans, associations, collectivités et PME, du premier atelier à la maintenance.</p>',
                ],
                [
                    'type' => 'features',
                    'title' => 'Nos valeurs',
                    'lead' => null,
                    'items' => [
                        [
                            'icon' => 'heart',
                            'title' => 'Transparence',
                            'text' => 'Devis détaillés, code ouvert, aucun verrouillage.',
                        ],
                        [
                            'icon' => 'leaf',
                            'title' => 'Sobriété',
                            'text' => 'Moins de dépendances, moins de données, moins d’énergie.',
                        ],
                        [
                            'icon' => 'users',
                            'title' => 'Proximité',
                            'text' => 'Un interlocuteur unique, joignable directement.',
                        ],
                    ],
                ],
                [
                    'type' => 'team',
                    'title' => 'L’équipe',
                    'lead' => 'Quatre personnes, toutes basées à Paris.',
                    'items' => [
                        [
                            'name' => 'Camille Martin',
                            'role' => 'Fondatrice, développeuse',
                            'initials' => 'CM',
                        ],
                        [
                            'name' => 'Inès Benali',
                            'role' => 'Experte accessibilité',
                            'initials' => 'IB',
                        ],
                        [
                            'name' => 'Thomas Roux',
                            'role' => 'Designer UI',
                            'initials' => 'TR',
                        ],
                        [
                            'name' => 'Hugo Lefèvre',
                            'role' => 'Développeur back-end',
                            'initials' => 'HL',
                        ],
                    ],
                ],
                [
                    'type' => 'cta',
                    'title' => 'Travaillons ensemble',
                    'text' => 'Présentez-nous votre projet, nous revenons vers vous rapidement.',
                    'action' => [
                        'label' => 'Nous contacter',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'services' => [
            'url' => '/services',
            'title' => 'Services',
            'description' => 'Création, refonte, accessibilité et maintenance de sites web.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Services',
                    'title' => 'Ce que nous faisons',
                    'lead' => 'Quatre offres claires, combinables selon votre projet.',
                ],
                [
                    'type' => 'services',
                    'title' => null,
                    'lead' => null,
                    'limit' => 0,
                    'link' => null,
                ],
                [
                    'type' => 'cta',
                    'title' => 'Besoin d’un conseil ?',
                    'text' => 'Nous vous aidons à choisir la bonne formule.',
                    'action' => [
                        'label' => 'Demander un devis',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'blog' => [
            'url' => '/blog',
            'title' => 'Blog',
            'description' => 'Conseils et retours d’expérience sur le web performant et accessible.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Blog',
                    'title' => 'Articles et conseils',
                    'lead' => 'Performance, accessibilité, choix techniques : ce que nous apprenons sur le terrain.',
                ],
                [
                    'type' => 'posts',
                    'title' => null,
                    'lead' => null,
                    'limit' => 0,
                    'link' => null,
                ],
            ],
        ],
        'contact' => [
            'url' => '/contact',
            'title' => 'Contact',
            'description' => 'Écrivez-nous : réponse sous 24 heures ouvrées.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Contact',
                    'title' => 'Parlons de votre projet',
                    'lead' => 'Remplissez le formulaire ou écrivez-nous directement, nous répondons sous 24 heures ouvrées.',
                ],
                [
                    'type' => 'contact',
                    'title' => 'Écrivez-nous',
                    'success' => 'Merci ! Votre message a bien été envoyé.',
                    'submit' => 'Envoyer le message',
                ],
            ],
        ],
        'legal' => [
            'url' => '/mentions-legales',
            'title' => 'Mentions légales',
            'description' => 'Mentions légales du site.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => null,
                    'title' => 'Mentions légales',
                    'lead' => null,
                ],
                [
                    'type' => 'text',
                    'title' => null,
                    'body' => '<h2 id="éditeur">Éditeur</h2>
<p>Ce site est édité par <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact : contact@exemple.fr.</p>
<h2 id="hébergement">Hébergement</h2>
<p>Le site est hébergé par votre prestataire d’hébergement ; ses coordonnées figurent dans le contrat de service.</p>
<h2 id="propriété-intellectuelle">Propriété intellectuelle</h2>
<p>Les textes, images et éléments graphiques de ce site sont protégés. Toute reproduction sans autorisation est interdite.</p>',
                ],
            ],
        ],
        'privacy' => [
            'url' => '/confidentialite',
            'title' => 'Politique de confidentialité',
            'description' => 'Comment nous traitons vos données personnelles.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => null,
                    'title' => 'Politique de confidentialité',
                    'lead' => null,
                ],
                [
                    'type' => 'text',
                    'title' => null,
                    'body' => '<h2 id="données-collectées">Données collectées</h2>
<p>Le formulaire de contact recueille votre nom, votre adresse e-mail et votre message, uniquement pour vous répondre.</p>
<h2 id="durée-de-conservation">Durée de conservation</h2>
<p>Les messages sont conservés douze mois, puis supprimés.</p>
<h2 id="vos-droits">Vos droits</h2>
<p>Vous pouvez demander l’accès, la rectification ou la suppression de vos données en écrivant à contact@exemple.fr.</p>',
                ],
            ],
        ],
    ],
    'contact_fields' => [
        [
            'name' => 'nom',
            'label' => 'Nom',
            'type' => 'text',
            'autocomplete' => 'name',
            'required' => true,
        ],
        [
            'name' => 'email',
            'label' => 'Adresse e-mail',
            'type' => 'email',
            'autocomplete' => 'email',
            'required' => true,
        ],
        [
            'name' => 'sujet',
            'label' => 'Sujet',
            'type' => 'text',
            'autocomplete' => 'off',
            'required' => false,
        ],
        [
            'name' => 'message',
            'label' => 'Message',
            'type' => 'textarea',
            'autocomplete' => 'off',
            'required' => true,
        ],
    ],
    'ui' => [
        'skip' => 'Aller au contenu',
        'main_nav' => 'Navigation principale',
        'mobile_nav' => 'Navigation mobile',
        'theme' => 'Changer de thème',
        'open_menu' => 'Ouvrir le menu',
        'social' => 'Réseaux sociaux',
        'language' => 'Langue',
        'contact' => 'Contact',
        'more' => 'En savoir plus',
        'more_label' => 'En savoir plus : ',
        'details' => 'Coordonnées',
        'included' => 'Ce qui est inclus',
        'all_posts' => 'Tous les articles',
        'home' => 'Accueil',
        'not_found' => 'Page introuvable',
        'not_found_text' => 'La page demandée n’existe pas ou a été déplacée.',
        'back_home' => 'Retour à l’accueil',
        'required' => 'Ce champ est obligatoire.',
        'invalid_email' => 'Adresse e-mail invalide.',
        'sign_in' => 'Connexion',
        'sign_in_lead' => 'Accédez à votre espace client.',
        'sign_in_submit' => 'Se connecter',
        'sign_up' => 'Créer un compte',
        'sign_up_lead' => 'Quelques secondes suffisent.',
        'sign_up_submit' => 'Créer mon compte',
        'account' => 'Mon compte',
        'account_lead' => 'Vous êtes connecté.',
        'sign_out' => 'Se déconnecter',
        'field_name' => 'Nom',
        'field_email' => 'E-mail',
        'field_password' => 'Mot de passe',
        'no_account' => 'Pas encore de compte ?',
        'have_account' => 'Déjà inscrit ?',
        'err_credentials' => 'E-mail ou mot de passe incorrect.',
        'err_exists' => 'Un compte existe déjà avec cette adresse.',
        'err_password' => 'Le mot de passe doit contenir au moins 8 caractères.',
        'err_required' => 'Tous les champs sont obligatoires.',
        'err_generic' => 'Une erreur est survenue, réessayez.',
    ],
];
