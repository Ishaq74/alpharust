@@meta mode=replace when=vitrine@@
{
  "locales": [
    {
      "code": "fr",
      "name": "Français",
      "label": "FR"
    },
    {
      "code": "en",
      "name": "English",
      "label": "EN"
    }
  ],
  "default": "fr",
  "data": {
    "fr": {
      "site": {
        "name": "@@title@@",
        "tagline": "Des sites web rapides, accessibles et faciles à faire évoluer.",
        "description": "Agence web indépendante : création de sites, refonte, accessibilité et maintenance.",
        "cta": {
          "label": "Demander un devis",
          "url": "/contact"
        },
        "contact": {
          "email": "contact@exemple.fr",
          "phone": "+33 1 23 45 67 89",
          "address": "12 rue de la Paix, 75002 Paris",
          "hours": "Du lundi au vendredi, 9 h – 18 h"
        },
        "social": [
          {
            "label": "GitHub",
            "url": "https://github.com/",
            "icon": "github"
          },
          {
            "label": "LinkedIn",
            "url": "https://www.linkedin.com/",
            "icon": "linkedin"
          },
          {
            "label": "Mastodon",
            "url": "https://mastodon.social/",
            "icon": "mastodon"
          }
        ],
        "nav": [
          {
            "label": "Accueil",
            "url": "/"
          },
          {
            "label": "Services",
            "url": "/services",
            "children": "services"
          },
          {
            "label": "Blog",
            "url": "/blog"
          },
          {
            "label": "À propos",
            "url": "/a-propos"
          },
          {
            "label": "Contact",
            "url": "/contact"
          }
        ],
        "footer": [
          {
            "title": "Navigation",
            "links": [
              {
                "label": "Accueil",
                "url": "/"
              },
              {
                "label": "Blog",
                "url": "/blog"
              },
              {
                "label": "À propos",
                "url": "/a-propos"
              },
              {
                "label": "Contact",
                "url": "/contact"
              }
            ]
          },
          {
            "title": "Services",
            "links": "services"
          },
          {
            "title": "Informations",
            "links": [
              {
                "label": "Mentions légales",
                "url": "/mentions-legales"
              },
              {
                "label": "Politique de confidentialité",
                "url": "/confidentialite"
              }
            ]
          }
        ],
        "copyright": "Tous droits réservés."
      },
      "pages": {
        "home": {
          "url": "/",
          "title": "Accueil",
          "seo_title": null,
          "description": "Agence web indépendante : création de sites, refonte, accessibilité et maintenance.",
          "sections": [
            {
              "type": "hero",
              "badge": "Agence web indépendante",
              "title": "Des sites rapides, accessibles et faciles à faire évoluer",
              "lead": "Nous concevons et maintenons des sites qui se chargent en moins d’une seconde, respectent le RGAA et restent simples à modifier.",
              "primary": {
                "label": "Demander un devis",
                "url": "/contact"
              },
              "secondary": {
                "label": "Voir nos services",
                "url": "/services"
              }
            },
            {
              "type": "stats",
              "items": [
                {
                  "value": "98/100",
                  "label": "Score Lighthouse moyen"
                },
                {
                  "value": "AA",
                  "label": "Niveau RGAA visé"
                },
                {
                  "value": "< 1 s",
                  "label": "Affichage du contenu"
                },
                {
                  "value": "12 ans",
                  "label": "D’expérience"
                }
              ]
            },
            {
              "type": "features",
              "title": "Pourquoi nous choisir",
              "lead": "Quatre engagements, sur chaque projet.",
              "items": [
                {
                  "icon": "zap",
                  "title": "Performance",
                  "text": "Pages légères, images optimisées, scores mesurés à chaque livraison."
                },
                {
                  "icon": "accessibility",
                  "title": "Accessibilité",
                  "text": "Navigation au clavier, contrastes contrôlés, sémantique soignée."
                },
                {
                  "icon": "pencil",
                  "title": "Autonomie",
                  "text": "Vous modifiez vos contenus sans nous appeler."
                },
                {
                  "icon": "shield",
                  "title": "Sérénité",
                  "text": "Mises à jour, sauvegardes et surveillance incluses."
                }
              ]
            },
            {
              "type": "services",
              "title": "Nos services",
              "lead": "De la création à la maintenance.",
              "limit": 3,
              "link": {
                "label": "Tous les services",
                "url": "/services"
              }
            },
            {
              "type": "testimonials",
              "title": "Ils nous font confiance",
              "items": [
                {
                  "quote": "Notre nouveau site est deux fois plus rapide et nos demandes de devis ont augmenté de 40 %.",
                  "name": "Sophie Laurent",
                  "role": "Directrice, Atelier Laurent",
                  "initials": "SL"
                },
                {
                  "quote": "Un audit d’accessibilité clair, des corrections livrées en trois semaines.",
                  "name": "Marc Petit",
                  "role": "Responsable digital, Ville de Montreuil",
                  "initials": "MP"
                },
                {
                  "quote": "Nous publions nos actualités en cinq minutes, sans aide.",
                  "name": "Léa Moreau",
                  "role": "Chargée de communication, Asso Horizon",
                  "initials": "LM"
                }
              ]
            },
            {
              "type": "posts",
              "title": "Derniers articles",
              "lead": "Conseils et retours d’expérience.",
              "limit": 3,
              "link": {
                "label": "Tout le blog",
                "url": "/blog"
              }
            },
            {
              "type": "faq",
              "title": "Questions fréquentes",
              "items": [
                {
                  "question": "Combien de temps faut-il pour créer un site ?",
                  "answer": "Entre quatre et huit semaines selon le nombre de pages et de fonctionnalités."
                },
                {
                  "question": "Pourrai-je modifier mon site moi-même ?",
                  "answer": "Oui : chaque site est livré avec un espace d’administration et une formation."
                },
                {
                  "question": "Mon site sera-t-il accessible ?",
                  "answer": "Nous visons le niveau AA du RGAA et livrons une déclaration d’accessibilité."
                },
                {
                  "question": "Proposez-vous la maintenance ?",
                  "answer": "Oui, avec un forfait mensuel qui inclut mises à jour, sauvegardes et petites évolutions."
                }
              ]
            },
            {
              "type": "cta",
              "title": "Un projet en tête ?",
              "text": "Parlons-en : réponse sous 24 heures ouvrées.",
              "action": {
                "label": "Demander un devis",
                "url": "/contact"
              }
            }
          ]
        },
        "about": {
          "url": "/a-propos",
          "title": "À propos",
          "description": "Qui nous sommes, notre mission et nos valeurs.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "À propos",
              "title": "Une petite équipe, de grandes exigences",
              "lead": "Développeurs, designers et experts accessibilité réunis autour du web durable."
            },
            {
              "type": "text",
              "title": "Notre mission",
              "body": "<p>Nous croyons qu’un bon site est <strong>rapide, accessible et sobre</strong>. Rapide, parce que chaque seconde compte. Accessible, parce que le web est pour tout le monde. Sobre, parce que le meilleur code est celui qu’on n’écrit pas.</p>\n<p>Depuis 2014, nous accompagnons artisans, associations, collectivités et PME, du premier atelier à la maintenance.</p>"
            },
            {
              "type": "features",
              "title": "Nos valeurs",
              "lead": null,
              "items": [
                {
                  "icon": "heart",
                  "title": "Transparence",
                  "text": "Devis détaillés, code ouvert, aucun verrouillage."
                },
                {
                  "icon": "leaf",
                  "title": "Sobriété",
                  "text": "Moins de dépendances, moins de données, moins d’énergie."
                },
                {
                  "icon": "users",
                  "title": "Proximité",
                  "text": "Un interlocuteur unique, joignable directement."
                }
              ]
            },
            {
              "type": "team",
              "title": "L’équipe",
              "lead": "Quatre personnes, toutes basées à Paris.",
              "items": [
                {
                  "name": "Camille Martin",
                  "role": "Fondatrice, développeuse",
                  "initials": "CM"
                },
                {
                  "name": "Inès Benali",
                  "role": "Experte accessibilité",
                  "initials": "IB"
                },
                {
                  "name": "Thomas Roux",
                  "role": "Designer UI",
                  "initials": "TR"
                },
                {
                  "name": "Hugo Lefèvre",
                  "role": "Développeur back-end",
                  "initials": "HL"
                }
              ]
            },
            {
              "type": "cta",
              "title": "Travaillons ensemble",
              "text": "Présentez-nous votre projet, nous revenons vers vous rapidement.",
              "action": {
                "label": "Nous contacter",
                "url": "/contact"
              }
            }
          ]
        },
        "services": {
          "url": "/services",
          "title": "Services",
          "description": "Création, refonte, accessibilité et maintenance de sites web.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Services",
              "title": "Ce que nous faisons",
              "lead": "Quatre offres claires, combinables selon votre projet."
            },
            {
              "type": "services",
              "title": null,
              "lead": null,
              "limit": 0,
              "link": null
            },
            {
              "type": "cta",
              "title": "Besoin d’un conseil ?",
              "text": "Nous vous aidons à choisir la bonne formule.",
              "action": {
                "label": "Demander un devis",
                "url": "/contact"
              }
            }
          ]
        },
        "blog": {
          "url": "/blog",
          "title": "Blog",
          "description": "Conseils et retours d’expérience sur le web performant et accessible.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Blog",
              "title": "Articles et conseils",
              "lead": "Performance, accessibilité, choix techniques : ce que nous apprenons sur le terrain."
            },
            {
              "type": "posts",
              "title": null,
              "lead": null,
              "limit": 0,
              "link": null
            }
          ]
        },
        "contact": {
          "url": "/contact",
          "title": "Contact",
          "description": "Écrivez-nous : réponse sous 24 heures ouvrées.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Contact",
              "title": "Parlons de votre projet",
              "lead": "Remplissez le formulaire ou écrivez-nous directement, nous répondons sous 24 heures ouvrées."
            },
            {
              "type": "contact",
              "title": "Écrivez-nous",
              "success": "Merci ! Votre message a bien été envoyé.",
              "submit": "Envoyer le message"
            }
          ]
        },
        "legal": {
          "url": "/mentions-legales",
          "title": "Mentions légales",
          "description": "Mentions légales du site.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": null,
              "title": "Mentions légales",
              "lead": null
            },
            {
              "type": "text",
              "title": null,
              "body": "<h2 id=\"éditeur\">Éditeur</h2>\n<p>Ce site est édité par <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact : contact@exemple.fr.</p>\n<h2 id=\"hébergement\">Hébergement</h2>\n<p>Le site est hébergé par votre prestataire d’hébergement ; ses coordonnées figurent dans le contrat de service.</p>\n<h2 id=\"propriété-intellectuelle\">Propriété intellectuelle</h2>\n<p>Les textes, images et éléments graphiques de ce site sont protégés. Toute reproduction sans autorisation est interdite.</p>"
            }
          ]
        },
        "privacy": {
          "url": "/confidentialite",
          "title": "Politique de confidentialité",
          "description": "Comment nous traitons vos données personnelles.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": null,
              "title": "Politique de confidentialité",
              "lead": null
            },
            {
              "type": "text",
              "title": null,
              "body": "<h2 id=\"données-collectées\">Données collectées</h2>\n<p>Le formulaire de contact recueille votre nom, votre adresse e-mail et votre message, uniquement pour vous répondre.</p>\n<h2 id=\"durée-de-conservation\">Durée de conservation</h2>\n<p>Les messages sont conservés douze mois, puis supprimés.</p>\n<h2 id=\"vos-droits\">Vos droits</h2>\n<p>Vous pouvez demander l’accès, la rectification ou la suppression de vos données en écrivant à contact@exemple.fr.</p>"
            }
          ]
        }
      },
      "contactFields": [
        {
          "name": "nom",
          "label": "Nom",
          "type": "text",
          "autocomplete": "name",
          "required": true
        },
        {
          "name": "email",
          "label": "Adresse e-mail",
          "type": "email",
          "autocomplete": "email",
          "required": true
        },
        {
          "name": "sujet",
          "label": "Sujet",
          "type": "text",
          "autocomplete": "off",
          "required": false
        },
        {
          "name": "message",
          "label": "Message",
          "type": "textarea",
          "autocomplete": "off",
          "required": true
        }
      ],
      "ui": {
        "skip": "Aller au contenu",
        "main_nav": "Navigation principale",
        "mobile_nav": "Navigation mobile",
        "theme": "Changer de thème",
        "open_menu": "Ouvrir le menu",
        "social": "Réseaux sociaux",
        "language": "Langue",
        "contact": "Contact",
        "more": "En savoir plus",
        "more_label": "En savoir plus : ",
        "details": "Coordonnées",
        "included": "Ce qui est inclus",
        "all_posts": "Tous les articles",
        "home": "Accueil",
        "not_found": "Page introuvable",
        "not_found_text": "La page demandée n’existe pas ou a été déplacée.",
        "back_home": "Retour à l’accueil",
        "required": "Ce champ est obligatoire.",
        "invalid_email": "Adresse e-mail invalide.",
        "sign_in": "Connexion",
        "sign_in_lead": "Accédez à votre espace client.",
        "sign_in_submit": "Se connecter",
        "sign_up": "Créer un compte",
        "sign_up_lead": "Quelques secondes suffisent.",
        "sign_up_submit": "Créer mon compte",
        "account": "Mon compte",
        "account_lead": "Vous êtes connecté.",
        "sign_out": "Se déconnecter",
        "field_name": "Nom",
        "field_email": "E-mail",
        "field_password": "Mot de passe",
        "no_account": "Pas encore de compte ?",
        "have_account": "Déjà inscrit ?",
        "err_credentials": "E-mail ou mot de passe incorrect.",
        "err_exists": "Un compte existe déjà avec cette adresse.",
        "err_password": "Le mot de passe doit contenir au moins 8 caractères.",
        "err_required": "Tous les champs sont obligatoires.",
        "err_generic": "Une erreur est survenue, réessayez."
      }
    },
    "en": {
      "site": {
        "name": "@@title@@",
        "tagline": "Fast, accessible websites that are easy to grow.",
        "description": "Independent web agency: website creation, redesign, accessibility and maintenance.",
        "cta": {
          "label": "Request a quote",
          "url": "/contact"
        },
        "contact": {
          "email": "contact@exemple.fr",
          "phone": "+33 1 23 45 67 89",
          "address": "12 rue de la Paix, 75002 Paris",
          "hours": "Monday to Friday, 9 am – 6 pm"
        },
        "social": [
          {
            "label": "GitHub",
            "url": "https://github.com/",
            "icon": "github"
          },
          {
            "label": "LinkedIn",
            "url": "https://www.linkedin.com/",
            "icon": "linkedin"
          },
          {
            "label": "Mastodon",
            "url": "https://mastodon.social/",
            "icon": "mastodon"
          }
        ],
        "nav": [
          {
            "label": "Home",
            "url": "/"
          },
          {
            "label": "Services",
            "url": "/services",
            "children": "services"
          },
          {
            "label": "Blog",
            "url": "/blog"
          },
          {
            "label": "About",
            "url": "/a-propos"
          },
          {
            "label": "Contact",
            "url": "/contact"
          }
        ],
        "footer": [
          {
            "title": "Navigation",
            "links": [
              {
                "label": "Home",
                "url": "/"
              },
              {
                "label": "Blog",
                "url": "/blog"
              },
              {
                "label": "About",
                "url": "/a-propos"
              },
              {
                "label": "Contact",
                "url": "/contact"
              }
            ]
          },
          {
            "title": "Services",
            "links": "services"
          },
          {
            "title": "Information",
            "links": [
              {
                "label": "Legal notice",
                "url": "/mentions-legales"
              },
              {
                "label": "Privacy policy",
                "url": "/confidentialite"
              }
            ]
          }
        ],
        "copyright": "All rights reserved."
      },
      "pages": {
        "home": {
          "url": "/",
          "title": "Home",
          "seo_title": null,
          "description": "Independent web agency: website creation, redesign, accessibility and maintenance.",
          "sections": [
            {
              "type": "hero",
              "badge": "Independent web agency",
              "title": "Fast, accessible websites that are easy to grow",
              "lead": "We design and maintain websites that load in under a second, meet the RGAA and stay easy to edit.",
              "primary": {
                "label": "Request a quote",
                "url": "/contact"
              },
              "secondary": {
                "label": "See our services",
                "url": "/services"
              }
            },
            {
              "type": "stats",
              "items": [
                {
                  "value": "98/100",
                  "label": "Average Lighthouse score"
                },
                {
                  "value": "AA",
                  "label": "Target RGAA level"
                },
                {
                  "value": "< 1 s",
                  "label": "Content display"
                },
                {
                  "value": "12 years",
                  "label": "Of experience"
                }
              ]
            },
            {
              "type": "features",
              "title": "Why choose us",
              "lead": "Four commitments, on every project.",
              "items": [
                {
                  "icon": "zap",
                  "title": "Performance",
                  "text": "Light pages, optimized images, scores measured at every delivery."
                },
                {
                  "icon": "accessibility",
                  "title": "Accessibility",
                  "text": "Keyboard navigation, checked contrast, careful semantics."
                },
                {
                  "icon": "pencil",
                  "title": "Autonomy",
                  "text": "You edit your content without calling us."
                },
                {
                  "icon": "shield",
                  "title": "Peace of mind",
                  "text": "Updates, backups and monitoring included."
                }
              ]
            },
            {
              "type": "services",
              "title": "Our services",
              "lead": "From creation to maintenance.",
              "limit": 3,
              "link": {
                "label": "All services",
                "url": "/services"
              }
            },
            {
              "type": "testimonials",
              "title": "They trust us",
              "items": [
                {
                  "quote": "Our new website is twice as fast and our quote requests have grown by 40%.",
                  "name": "Sophie Laurent",
                  "role": "Director, Atelier Laurent",
                  "initials": "SL"
                },
                {
                  "quote": "A clear accessibility audit, fixes delivered in three weeks.",
                  "name": "Marc Petit",
                  "role": "Digital manager, City of Montreuil",
                  "initials": "MP"
                },
                {
                  "quote": "We publish our news in five minutes, without help.",
                  "name": "Léa Moreau",
                  "role": "Communications officer, Asso Horizon",
                  "initials": "LM"
                }
              ]
            },
            {
              "type": "posts",
              "title": "Latest articles",
              "lead": "Advice and lessons learned.",
              "limit": 3,
              "link": {
                "label": "The whole blog",
                "url": "/blog"
              }
            },
            {
              "type": "faq",
              "title": "Frequently asked questions",
              "items": [
                {
                  "question": "How long does it take to build a website?",
                  "answer": "Between four and eight weeks depending on the number of pages and features."
                },
                {
                  "question": "Will I be able to edit my website myself?",
                  "answer": "Yes: every website comes with an admin area and training."
                },
                {
                  "question": "Will my website be accessible?",
                  "answer": "We target RGAA level AA and deliver an accessibility statement."
                },
                {
                  "question": "Do you offer maintenance?",
                  "answer": "Yes, with a monthly plan that includes updates, backups and small improvements."
                }
              ]
            },
            {
              "type": "cta",
              "title": "Have a project in mind?",
              "text": "Let’s talk: reply within 24 business hours.",
              "action": {
                "label": "Request a quote",
                "url": "/contact"
              }
            }
          ]
        },
        "about": {
          "url": "/a-propos",
          "title": "About",
          "description": "Who we are, our mission and our values.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "About",
              "title": "A small team with high standards",
              "lead": "Developers, designers and accessibility experts united around a sustainable web."
            },
            {
              "type": "text",
              "title": "Our mission",
              "body": "<p>We believe a good website is <strong>fast, accessible and frugal</strong>. Fast, because every second counts. Accessible, because the web is for everyone. Frugal, because the best code is the code you don’t write.</p>\n<p>Since 2014, we have been supporting craftspeople, non-profits, local authorities and SMEs, from the first workshop to maintenance.</p>"
            },
            {
              "type": "features",
              "title": "Our values",
              "lead": null,
              "items": [
                {
                  "icon": "heart",
                  "title": "Transparency",
                  "text": "Detailed quotes, open code, no lock-in."
                },
                {
                  "icon": "leaf",
                  "title": "Frugality",
                  "text": "Fewer dependencies, less data, less energy."
                },
                {
                  "icon": "users",
                  "title": "Closeness",
                  "text": "A single point of contact, reachable directly."
                }
              ]
            },
            {
              "type": "team",
              "title": "The team",
              "lead": "Four people, all based in Paris.",
              "items": [
                {
                  "name": "Camille Martin",
                  "role": "Founder, developer",
                  "initials": "CM"
                },
                {
                  "name": "Inès Benali",
                  "role": "Accessibility expert",
                  "initials": "IB"
                },
                {
                  "name": "Thomas Roux",
                  "role": "UI designer",
                  "initials": "TR"
                },
                {
                  "name": "Hugo Lefèvre",
                  "role": "Back-end developer",
                  "initials": "HL"
                }
              ]
            },
            {
              "type": "cta",
              "title": "Let’s work together",
              "text": "Tell us about your project, we will get back to you quickly.",
              "action": {
                "label": "Contact us",
                "url": "/contact"
              }
            }
          ]
        },
        "services": {
          "url": "/services",
          "title": "Services",
          "description": "Website creation, redesign, accessibility and maintenance.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Services",
              "title": "What we do",
              "lead": "Four clear offers, to combine as your project requires."
            },
            {
              "type": "services",
              "title": null,
              "lead": null,
              "limit": 0,
              "link": null
            },
            {
              "type": "cta",
              "title": "Need advice?",
              "text": "We help you choose the right plan.",
              "action": {
                "label": "Request a quote",
                "url": "/contact"
              }
            }
          ]
        },
        "blog": {
          "url": "/blog",
          "title": "Blog",
          "description": "Advice and lessons learned on fast, accessible websites.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Blog",
              "title": "Articles and advice",
              "lead": "Performance, accessibility, technical choices: what we learn in the field."
            },
            {
              "type": "posts",
              "title": null,
              "lead": null,
              "limit": 0,
              "link": null
            }
          ]
        },
        "contact": {
          "url": "/contact",
          "title": "Contact",
          "description": "Write to us: reply within 24 business hours.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": "Contact",
              "title": "Let’s talk about your project",
              "lead": "Fill in the form or write to us directly, we reply within 24 business hours."
            },
            {
              "type": "contact",
              "title": "Write to us",
              "success": "Thank you! Your message has been sent.",
              "submit": "Send message"
            }
          ]
        },
        "legal": {
          "url": "/mentions-legales",
          "title": "Legal notice",
          "description": "Site legal notice.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": null,
              "title": "Legal notice",
              "lead": null
            },
            {
              "type": "text",
              "title": null,
              "body": "<h2 id=\"publisher\">Publisher</h2>\n<p>This website is published by <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact: contact@exemple.fr.</p>\n<h2 id=\"hosting\">Hosting</h2>\n<p>The website is hosted by your hosting provider; their details are in the service contract.</p>\n<h2 id=\"intellectual-property\">Intellectual property</h2>\n<p>The texts, images and graphic elements of this website are protected. Any reproduction without permission is prohibited.</p>"
            }
          ]
        },
        "privacy": {
          "url": "/confidentialite",
          "title": "Privacy policy",
          "description": "How we process your personal data.",
          "sections": [
            {
              "type": "page_header",
              "eyebrow": null,
              "title": "Privacy policy",
              "lead": null
            },
            {
              "type": "text",
              "title": null,
              "body": "<h2 id=\"data-collected\">Data collected</h2>\n<p>The contact form collects your name, e-mail address and message, solely to reply to you.</p>\n<h2 id=\"retention-period\">Retention period</h2>\n<p>Messages are kept for twelve months, then deleted.</p>\n<h2 id=\"your-rights\">Your rights</h2>\n<p>You can request access to, correction or deletion of your data by writing to contact@exemple.fr.</p>"
            }
          ]
        }
      },
      "contactFields": [
        {
          "name": "nom",
          "label": "Name",
          "type": "text",
          "autocomplete": "name",
          "required": true
        },
        {
          "name": "email",
          "label": "E-mail address",
          "type": "email",
          "autocomplete": "email",
          "required": true
        },
        {
          "name": "sujet",
          "label": "Subject",
          "type": "text",
          "autocomplete": "off",
          "required": false
        },
        {
          "name": "message",
          "label": "Message",
          "type": "textarea",
          "autocomplete": "off",
          "required": true
        }
      ],
      "ui": {
        "skip": "Skip to content",
        "main_nav": "Main navigation",
        "mobile_nav": "Mobile navigation",
        "theme": "Toggle theme",
        "open_menu": "Open menu",
        "social": "Social networks",
        "language": "Language",
        "contact": "Contact",
        "more": "Learn more",
        "more_label": "Learn more: ",
        "details": "Contact details",
        "included": "What’s included",
        "all_posts": "All articles",
        "home": "Home",
        "not_found": "Page not found",
        "not_found_text": "The requested page does not exist or has been moved.",
        "back_home": "Back to home",
        "required": "This field is required.",
        "invalid_email": "Invalid e-mail address.",
        "sign_in": "Sign in",
        "sign_in_lead": "Access your client area.",
        "sign_in_submit": "Sign in",
        "sign_up": "Create an account",
        "sign_up_lead": "It only takes a few seconds.",
        "sign_up_submit": "Create my account",
        "account": "My account",
        "account_lead": "You are signed in.",
        "sign_out": "Sign out",
        "field_name": "Name",
        "field_email": "E-mail",
        "field_password": "Password",
        "no_account": "No account yet?",
        "have_account": "Already registered?",
        "err_credentials": "Incorrect e-mail or password.",
        "err_exists": "An account already exists with this e-mail.",
        "err_password": "The password must be at least 8 characters.",
        "err_required": "All fields are required.",
        "err_generic": "Something went wrong, please try again."
      }
    }
  },
  "auth": {
    "routes": {
      "sign_in": "/auth/sign-in",
      "sign_up": "/auth/sign-up",
      "account": "/auth/dashboard"
    },
    "fields": {
      "sign_in": [
        {
          "name": "email",
          "type": "email",
          "autocomplete": "email"
        },
        {
          "name": "password",
          "type": "password",
          "autocomplete": "current-password"
        }
      ],
      "sign_up": [
        {
          "name": "name",
          "type": "text",
          "autocomplete": "name"
        },
        {
          "name": "email",
          "type": "email",
          "autocomplete": "email"
        },
        {
          "name": "password",
          "type": "password",
          "autocomplete": "new-password"
        }
      ]
    },
    "passwordMin": 8
  }
}
