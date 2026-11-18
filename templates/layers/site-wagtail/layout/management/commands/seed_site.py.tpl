# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Crée l'arborescence et le contenu du site — mêmes données qu'Astro (src/data, src/content) et Laravel (config/site.php,
SiteSeeder). Idempotent : une page existante (même slug) est conservée.

    python manage.py seed_site
"""
import json
from datetime import date

from django.core.management.base import BaseCommand
from django.db import transaction
from wagtail.models import Locale, Site

from blog.models import BlogIndexPage, BlogPage
from contact.models import ContactPage, FormField
from flex.models import FlexPage
from home.models import HomePage
from layout.models import FooterSettings, HeaderSettings, NavigationItem, NavigationMenu
from service.models import ServiceIndexPage, ServicePage
from site_settings.models import ContactSettings, OrganisationSettings, SocialMediaSettings

PAGES = json.loads(r"""{
 "home": {
  "title": "Accueil",
  "description": "Agence web indépendante : création de sites, refonte, accessibilité et maintenance.",
  "slug": "",
  "body": [
   {
    "type": "hero",
    "value": {
     "badge": "Agence web indépendante",
     "title": "Des sites rapides, accessibles et faciles à faire évoluer",
     "lead": "Nous concevons et maintenons des sites qui se chargent en moins d’une seconde, respectent le RGAA et restent simples à modifier.",
     "primary": {
      "label": "Demander un devis",
      "page": null,
      "url": "/contact"
     },
     "secondary": {
      "label": "Voir nos services",
      "page": null,
      "url": "/services"
     }
    }
   },
   {
    "type": "stats",
    "value": {
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
    }
   },
   {
    "type": "features",
    "value": {
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
    }
   },
   {
    "type": "services",
    "value": {
     "title": "Nos services",
     "lead": "De la création à la maintenance.",
     "limit": 3,
     "link": {
      "label": "Tous les services",
      "page": null,
      "url": "/services"
     }
    }
   },
   {
    "type": "testimonials",
    "value": {
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
    }
   },
   {
    "type": "posts",
    "value": {
     "title": "Derniers articles",
     "lead": "Conseils et retours d’expérience.",
     "limit": 3,
     "link": {
      "label": "Tout le blog",
      "page": null,
      "url": "/blog"
     }
    }
   },
   {
    "type": "faq",
    "value": {
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
    }
   },
   {
    "type": "cta",
    "value": {
     "title": "Un projet en tête ?",
     "text": "Parlons-en : réponse sous 24 heures ouvrées.",
     "action": {
      "label": "Demander un devis",
      "page": null,
      "url": "/contact"
     }
    }
   }
  ]
 },
 "about": {
  "title": "À propos",
  "description": "Qui nous sommes, notre mission et nos valeurs.",
  "slug": "a-propos",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "À propos",
     "title": "Une petite équipe, de grandes exigences",
     "lead": "Développeurs, designers et experts accessibilité réunis autour du web durable."
    }
   },
   {
    "type": "text",
    "value": {
     "title": "Notre mission",
     "body": "<p>Nous croyons qu’un bon site est <strong>rapide, accessible et sobre</strong>. Rapide, parce que chaque seconde compte. Accessible, parce que le web est pour tout le monde. Sobre, parce que le meilleur code est celui qu’on n’écrit pas.</p>\n<p>Depuis 2014, nous accompagnons artisans, associations, collectivités et PME, du premier atelier à la maintenance.</p>"
    }
   },
   {
    "type": "features",
    "value": {
     "title": "Nos valeurs",
     "lead": "",
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
    }
   },
   {
    "type": "team",
    "value": {
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
    }
   },
   {
    "type": "cta",
    "value": {
     "title": "Travaillons ensemble",
     "text": "Présentez-nous votre projet, nous revenons vers vous rapidement.",
     "action": {
      "label": "Nous contacter",
      "page": null,
      "url": "/contact"
     }
    }
   }
  ]
 },
 "services": {
  "title": "Services",
  "description": "Création, refonte, accessibilité et maintenance de sites web.",
  "slug": "services",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "Services",
     "title": "Ce que nous faisons",
     "lead": "Quatre offres claires, combinables selon votre projet."
    }
   },
   {
    "type": "services",
    "value": {
     "title": "",
     "lead": "",
     "limit": 0,
     "link": {
      "label": "",
      "page": null,
      "url": ""
     }
    }
   },
   {
    "type": "cta",
    "value": {
     "title": "Besoin d’un conseil ?",
     "text": "Nous vous aidons à choisir la bonne formule.",
     "action": {
      "label": "Demander un devis",
      "page": null,
      "url": "/contact"
     }
    }
   }
  ]
 },
 "blog": {
  "title": "Blog",
  "description": "Conseils et retours d’expérience sur le web performant et accessible.",
  "slug": "blog",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "Blog",
     "title": "Articles et conseils",
     "lead": "Performance, accessibilité, choix techniques : ce que nous apprenons sur le terrain."
    }
   },
   {
    "type": "posts",
    "value": {
     "title": "",
     "lead": "",
     "limit": 0,
     "link": {
      "label": "",
      "page": null,
      "url": ""
     }
    }
   }
  ]
 },
 "contact": {
  "title": "Contact",
  "description": "Écrivez-nous : réponse sous 24 heures ouvrées.",
  "slug": "contact",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "Contact",
     "title": "Parlons de votre projet",
     "lead": "Remplissez le formulaire ou écrivez-nous directement, nous répondons sous 24 heures ouvrées."
    }
   },
   {
    "type": "contact",
    "value": {
     "title": "Écrivez-nous",
     "success": "Merci ! Votre message a bien été envoyé.",
     "submit": "Envoyer le message"
    }
   }
  ]
 },
 "legal": {
  "title": "Mentions légales",
  "description": "Mentions légales du site.",
  "slug": "mentions-legales",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "",
     "title": "Mentions légales",
     "lead": ""
    }
   },
   {
    "type": "text",
    "value": {
     "title": "",
     "body": "<h2 id=\"éditeur\">Éditeur</h2>\n<p>Ce site est édité par <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact : contact@exemple.fr.</p>\n<h2 id=\"hébergement\">Hébergement</h2>\n<p>Le site est hébergé par votre prestataire d’hébergement ; ses coordonnées figurent dans le contrat de service.</p>\n<h2 id=\"propriété-intellectuelle\">Propriété intellectuelle</h2>\n<p>Les textes, images et éléments graphiques de ce site sont protégés. Toute reproduction sans autorisation est interdite.</p>"
    }
   }
  ]
 },
 "privacy": {
  "title": "Politique de confidentialité",
  "description": "Comment nous traitons vos données personnelles.",
  "slug": "confidentialite",
  "body": [
   {
    "type": "page_header",
    "value": {
     "eyebrow": "",
     "title": "Politique de confidentialité",
     "lead": ""
    }
   },
   {
    "type": "text",
    "value": {
     "title": "",
     "body": "<h2 id=\"données-collectées\">Données collectées</h2>\n<p>Le formulaire de contact recueille votre nom, votre adresse e-mail et votre message, uniquement pour vous répondre.</p>\n<h2 id=\"durée-de-conservation\">Durée de conservation</h2>\n<p>Les messages sont conservés douze mois, puis supprimés.</p>\n<h2 id=\"vos-droits\">Vos droits</h2>\n<p>Vous pouvez demander l’accès, la rectification ou la suppression de vos données en écrivant à contact@exemple.fr.</p>"
    }
   }
  ]
 }
}""")
SERVICES = json.loads(r"""[
 {
  "slug": "creation-site-web",
  "title": "Création de site web",
  "summary": "Un site sur mesure, rapide et référencé, livré avec son espace d’administration.",
  "icon": "code",
  "price": "À partir de 2 400 €",
  "features": [
   "Maquettes et design system",
   "Contenus éditables",
   "SEO technique et données structurées",
   "Mise en ligne et formation"
  ],
  "body": "<p>Nous concevons votre site à partir de vos objectifs : qui doit le trouver, que doit-il y faire, comment le mesurer.</p>\n<h2 id=\"notre-méthode\">Notre méthode</h2>\n<p>Ateliers de cadrage, maquettes, puis développement par itérations courtes. Chaque version est <strong>mise en ligne sur un environnement de recette</strong> pour que vous puissiez la tester.</p>\n<h2 id=\"ce-qui-est-livré\">Ce qui est livré</h2>\n<ul>\n<li>un site responsive, accessible et performant ;</li>\n<li>un espace d’administration pour modifier vos contenus ;</li>\n<li>la documentation et une formation d’une demi-journée.</li>\n</ul>"
 },
 {
  "slug": "refonte-performance",
  "title": "Refonte et performance",
  "summary": "Un site existant plus rapide, plus clair et mieux classé, sans perdre votre référencement.",
  "icon": "zap",
  "price": "À partir de 1 900 €",
  "features": [
   "Audit Lighthouse et Core Web Vitals",
   "Plan de redirections",
   "Optimisation des images",
   "Suivi après mise en ligne"
  ],
  "body": "<p>Une refonte réussie commence par un audit : temps de chargement, parcours, contenus qui convertissent.</p>\n<h2 id=\"garder-le-référencement\">Garder le référencement</h2>\n<p>Toutes les anciennes adresses sont <strong>redirigées</strong> et suivies pendant trois mois après la mise en ligne.</p>\n<h2 id=\"mesurer-le-gain\">Mesurer le gain</h2>\n<ul>\n<li>scores Lighthouse avant / après ;</li>\n<li>Core Web Vitals sur les pages clés ;</li>\n<li>taux de conversion des formulaires.</li>\n</ul>"
 },
 {
  "slug": "accessibilite",
  "title": "Accessibilité (RGAA)",
  "summary": "Audit et mise en conformité RGAA / WCAG 2.2 AA, déclaration d’accessibilité comprise.",
  "icon": "accessibility",
  "price": "À partir de 1 200 €",
  "features": [
   "Audit sur échantillon de pages",
   "Corrections priorisées",
   "Déclaration d’accessibilité",
   "Formation des rédacteurs"
  ],
  "body": "<p>L’accessibilité profite à tous : navigation au clavier, contrastes, lecteurs d’écran, formulaires compréhensibles.</p>\n<h2 id=\"audit\">Audit</h2>\n<p>Nous évaluons un échantillon représentatif de pages selon le <strong>RGAA 4.1</strong> et livrons un rapport priorisé.</p>\n<h2 id=\"mise-en-conformité\">Mise en conformité</h2>\n<ul>\n<li>corrections dans le code et les gabarits ;</li>\n<li>contrôle des contrastes du thème ;</li>\n<li>rédaction de la déclaration d’accessibilité.</li>\n</ul>"
 },
 {
  "slug": "maintenance",
  "title": "Maintenance et hébergement",
  "summary": "Mises à jour, sauvegardes, surveillance et petites évolutions, sans surprise.",
  "icon": "shield",
  "price": "À partir de 90 € / mois",
  "features": [
   "Mises à jour de sécurité",
   "Sauvegardes quotidiennes",
   "Surveillance 24 h / 24",
   "Une heure d’évolution par mois"
  ],
  "body": "<p>Un site est un logiciel : il doit être mis à jour, sauvegardé et surveillé.</p>\n<h2 id=\"ce-qui-est-compris\">Ce qui est compris</h2>\n<ul>\n<li>mises à jour du framework et des dépendances ;</li>\n<li>sauvegardes quotidiennes conservées 30 jours ;</li>\n<li>surveillance de la disponibilité et des certificats.</li>\n</ul>\n<h2 id=\"évolutions\">Évolutions</h2>\n<p>Chaque mois, une heure d’évolution est incluse : nouvelle section, formulaire, page.</p>"
 }
]""")
POSTS = json.loads(r"""[
 {
  "slug": "performance-images",
  "title": "Des images légères sans perte de qualité",
  "excerpt": "Formats modernes, dimensions explicites, chargement différé : le trio gagnant.",
  "category": "Performance",
  "author": "Camille Martin",
  "date": "2026-08-28",
  "body": "<p>Les images représentent souvent plus de la moitié du poids d’une page.</p>\n<h2 id=\"les-bons-formats\">Les bons formats</h2>\n<p>AVIF et WebP divisent le poids par deux à cinq par rapport au JPEG, à qualité égale.</p>\n<h2 id=\"les-bons-attributs\">Les bons attributs</h2>\n<ul>\n<li><code>width</code> et <code>height</code> pour éviter les décalages de mise en page ;</li>\n<li><code>loading=\"lazy\"</code> sous la ligne de flottaison ;</li>\n<li><code>srcset</code> pour servir la bonne taille.</li>\n</ul>"
 },
 {
  "slug": "accessibilite-checklist",
  "title": "L’accessibilité en 10 points",
  "excerpt": "Une liste courte et concrète pour éviter les erreurs les plus fréquentes.",
  "category": "Accessibilité",
  "author": "Inès Benali",
  "date": "2026-09-15",
  "body": "<p>Ces dix points couvrent la majorité des problèmes rencontrés en audit.</p>\n<h2 id=\"structure\">Structure</h2>\n<ul>\n<li>un seul <code>h1</code> par page et des titres hiérarchisés ;</li>\n<li>des repères : <code>header</code>, <code>nav</code>, <code>main</code>, <code>footer</code> ;</li>\n<li>un lien d’évitement vers le contenu.</li>\n</ul>\n<h2 id=\"interaction\">Interaction</h2>\n<ul>\n<li>tout est utilisable <strong>au clavier</strong> ;</li>\n<li>le focus est toujours visible ;</li>\n<li>les champs de formulaire ont un libellé.</li>\n</ul>"
 },
 {
  "slug": "choisir-son-framework",
  "title": "Astro, Laravel ou Wagtail : choisir son framework",
  "excerpt": "Trois excellents outils, trois façons de travailler. Nos critères pour choisir sans regret.",
  "category": "Technique",
  "author": "Camille Martin",
  "date": "2026-10-02",
  "body": "<p>Le bon framework est celui que votre équipe saura faire vivre. Voici nos critères.</p>\n<h2 id=\"astro-pour-les-sites-de-contenu\">Astro pour les sites de contenu</h2>\n<p>Pages statiques par défaut, îlots interactifs à la demande : idéal pour un site vitrine ou un blog très rapide.</p>\n<h2 id=\"laravel-pour-les-applications\">Laravel pour les applications</h2>\n<p>Authentification, files d’attente, e-mails, paiements : <strong>tout est prévu</strong> pour une application métier.</p>\n<h2 id=\"wagtail-pour-les-rédactions\">Wagtail pour les rédactions</h2>\n<p>Un vrai CMS éditorial : arborescence de pages, StreamField, révisions et prévisualisation.</p>"
 }
]""")
SITE = json.loads(r"""{
 "tagline": "Des sites web rapides, accessibles et faciles à faire évoluer.",
 "description": "Agence web indépendante : création de sites, refonte, accessibilité et maintenance.",
 "copyright": "Tous droits réservés.",
 "cta": {
  "label": "Demander un devis",
  "url": "/contact"
 },
 "contact": {
  "email": "contact@exemple.fr",
  "phone": "+33 1 23 45 67 89",
  "address": "12 rue de la Paix, 75002 Paris",
  "hours": "Du lundi au vendredi, 9 h – 18 h"
 }
}""")
NAV = json.loads(r"""[
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
]""")
FOOTER = json.loads(r"""[
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
]""")
SOCIAL = [
    {"icon": "github", "url": "https://github.com/"},  # alpharust-ignore: no-hardcoded-urls
    {"icon": "linkedin", "url": "https://www.linkedin.com/"},  # alpharust-ignore: no-hardcoded-urls
    {"icon": "mastodon", "url": "https://mastodon.social/"},  # alpharust-ignore: no-hardcoded-urls
]
FIELDS = json.loads(r"""[
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
]""")
FIELD_TYPES = {"text": "singleline", "email": "email", "textarea": "multiline"}
# Traduction anglaise (vitrine/i18n.py) : mêmes slugs, pages liées aux pages françaises par translation_key.
EN = json.loads(r"""{
 "pages": {
  "home": {
   "title": "Home",
   "description": "Independent web agency: website creation, redesign, accessibility and maintenance.",
   "slug": "",
   "body": [
    {
     "type": "hero",
     "value": {
      "badge": "Independent web agency",
      "title": "Fast, accessible websites that are easy to grow",
      "lead": "We design and maintain websites that load in under a second, meet the RGAA and stay easy to edit.",
      "primary": {
       "label": "Request a quote",
       "page": null,
       "url": "/contact"
      },
      "secondary": {
       "label": "See our services",
       "page": null,
       "url": "/services"
      }
     }
    },
    {
     "type": "stats",
     "value": {
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
     }
    },
    {
     "type": "features",
     "value": {
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
     }
    },
    {
     "type": "services",
     "value": {
      "title": "Our services",
      "lead": "From creation to maintenance.",
      "limit": 3,
      "link": {
       "label": "All services",
       "page": null,
       "url": "/services"
      }
     }
    },
    {
     "type": "testimonials",
     "value": {
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
     }
    },
    {
     "type": "posts",
     "value": {
      "title": "Latest articles",
      "lead": "Advice and lessons learned.",
      "limit": 3,
      "link": {
       "label": "The whole blog",
       "page": null,
       "url": "/blog"
      }
     }
    },
    {
     "type": "faq",
     "value": {
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
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "Have a project in mind?",
      "text": "Let’s talk: reply within 24 business hours.",
      "action": {
       "label": "Request a quote",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "about": {
   "title": "About",
   "description": "Who we are, our mission and our values.",
   "slug": "a-propos",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "About",
      "title": "A small team with high standards",
      "lead": "Developers, designers and accessibility experts united around a sustainable web."
     }
    },
    {
     "type": "text",
     "value": {
      "title": "Our mission",
      "body": "<p>We believe a good website is <strong>fast, accessible and frugal</strong>. Fast, because every second counts. Accessible, because the web is for everyone. Frugal, because the best code is the code you don’t write.</p>\n<p>Since 2014, we have been supporting craftspeople, non-profits, local authorities and SMEs, from the first workshop to maintenance.</p>"
     }
    },
    {
     "type": "features",
     "value": {
      "title": "Our values",
      "lead": "",
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
     }
    },
    {
     "type": "team",
     "value": {
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
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "Let’s work together",
      "text": "Tell us about your project, we will get back to you quickly.",
      "action": {
       "label": "Contact us",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "services": {
   "title": "Services",
   "description": "Website creation, redesign, accessibility and maintenance.",
   "slug": "services",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Services",
      "title": "What we do",
      "lead": "Four clear offers, to combine as your project requires."
     }
    },
    {
     "type": "services",
     "value": {
      "title": "",
      "lead": "",
      "limit": 0,
      "link": {
       "label": "",
       "page": null,
       "url": ""
      }
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "Need advice?",
      "text": "We help you choose the right plan.",
      "action": {
       "label": "Request a quote",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "blog": {
   "title": "Blog",
   "description": "Advice and lessons learned on fast, accessible websites.",
   "slug": "blog",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Blog",
      "title": "Articles and advice",
      "lead": "Performance, accessibility, technical choices: what we learn in the field."
     }
    },
    {
     "type": "posts",
     "value": {
      "title": "",
      "lead": "",
      "limit": 0,
      "link": {
       "label": "",
       "page": null,
       "url": ""
      }
     }
    }
   ]
  },
  "contact": {
   "title": "Contact",
   "description": "Write to us: reply within 24 business hours.",
   "slug": "contact",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Contact",
      "title": "Let’s talk about your project",
      "lead": "Fill in the form or write to us directly, we reply within 24 business hours."
     }
    },
    {
     "type": "contact",
     "value": {
      "title": "Write to us",
      "success": "Thank you! Your message has been sent.",
      "submit": "Send message"
     }
    }
   ]
  },
  "legal": {
   "title": "Legal notice",
   "description": "Site legal notice.",
   "slug": "mentions-legales",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "Legal notice",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"publisher\">Publisher</h2>\n<p>This website is published by <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact: contact@exemple.fr.</p>\n<h2 id=\"hosting\">Hosting</h2>\n<p>The website is hosted by your hosting provider; their details are in the service contract.</p>\n<h2 id=\"intellectual-property\">Intellectual property</h2>\n<p>The texts, images and graphic elements of this website are protected. Any reproduction without permission is prohibited.</p>"
     }
    }
   ]
  },
  "privacy": {
   "title": "Privacy policy",
   "description": "How we process your personal data.",
   "slug": "confidentialite",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "Privacy policy",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"data-collected\">Data collected</h2>\n<p>The contact form collects your name, e-mail address and message, solely to reply to you.</p>\n<h2 id=\"retention-period\">Retention period</h2>\n<p>Messages are kept for twelve months, then deleted.</p>\n<h2 id=\"your-rights\">Your rights</h2>\n<p>You can request access to, correction or deletion of your data by writing to contact@exemple.fr.</p>"
     }
    }
   ]
  }
 },
 "services": [
  {
   "slug": "creation-site-web",
   "title": "Website creation",
   "summary": "A custom website, fast and well ranked, delivered with its admin area.",
   "icon": "code",
   "price": "From €2,400",
   "features": [
    "Mockups and design system",
    "Editable content",
    "Technical SEO and structured data",
    "Launch and training"
   ],
   "body": "<p>We design your website around your goals: who needs to find it, what they should do there, how to measure it.</p>\n<h2 id=\"our-method\">Our method</h2>\n<p>Scoping workshops, mockups, then development in short iterations. Each version is <strong>deployed to a staging environment</strong> so you can test it.</p>\n<h2 id=\"what-is-delivered\">What is delivered</h2>\n<ul>\n<li>a responsive, accessible and fast website;</li>\n<li>an admin area to edit your content;</li>\n<li>documentation and a half-day training session.</li>\n</ul>"
  },
  {
   "slug": "refonte-performance",
   "title": "Redesign and performance",
   "summary": "An existing website made faster, clearer and better ranked, without losing your SEO.",
   "icon": "zap",
   "price": "From €1,900",
   "features": [
    "Lighthouse and Core Web Vitals audit",
    "Redirect plan",
    "Image optimization",
    "Post-launch monitoring"
   ],
   "body": "<p>A successful redesign starts with an audit: load times, user journeys, content that converts.</p>\n<h2 id=\"keeping-your-rankings\">Keeping your rankings</h2>\n<p>Every old address is <strong>redirected</strong> and monitored for three months after launch.</p>\n<h2 id=\"measuring-the-gain\">Measuring the gain</h2>\n<ul>\n<li>Lighthouse scores before / after;</li>\n<li>Core Web Vitals on key pages;</li>\n<li>form conversion rates.</li>\n</ul>"
  },
  {
   "slug": "accessibilite",
   "title": "Accessibility (RGAA)",
   "summary": "RGAA / WCAG 2.2 AA audit and remediation, accessibility statement included.",
   "icon": "accessibility",
   "price": "From €1,200",
   "features": [
    "Audit on a sample of pages",
    "Prioritized fixes",
    "Accessibility statement",
    "Editor training"
   ],
   "body": "<p>Accessibility benefits everyone: keyboard navigation, contrast, screen readers, understandable forms.</p>\n<h2 id=\"audit\">Audit</h2>\n<p>We assess a representative sample of pages against <strong>RGAA 4.1</strong> and deliver a prioritized report.</p>\n<h2 id=\"remediation\">Remediation</h2>\n<ul>\n<li>fixes in the code and templates;</li>\n<li>theme contrast checks;</li>\n<li>writing the accessibility statement.</li>\n</ul>"
  },
  {
   "slug": "maintenance",
   "title": "Maintenance and hosting",
   "summary": "Updates, backups, monitoring and small improvements, with no surprises.",
   "icon": "shield",
   "price": "From €90 / month",
   "features": [
    "Security updates",
    "Daily backups",
    "24/7 monitoring",
    "One hour of improvements per month"
   ],
   "body": "<p>A website is software: it must be updated, backed up and monitored.</p>\n<h2 id=\"what-is-included\">What is included</h2>\n<ul>\n<li>framework and dependency updates;</li>\n<li>daily backups kept for 30 days;</li>\n<li>uptime and certificate monitoring.</li>\n</ul>\n<h2 id=\"improvements\">Improvements</h2>\n<p>Every month, one hour of improvements is included: a new section, form or page.</p>"
  }
 ],
 "posts": [
  {
   "slug": "performance-images",
   "title": "Lightweight images without quality loss",
   "excerpt": "Modern formats, explicit dimensions, lazy loading: the winning trio.",
   "category": "Performance",
   "author": "Camille Martin",
   "date": "2026-08-28",
   "body": "<p>Images often account for more than half of a page’s weight.</p>\n<h2 id=\"the-right-formats\">The right formats</h2>\n<p>AVIF and WebP cut the weight by two to five compared with JPEG, at equal quality.</p>\n<h2 id=\"the-right-attributes\">The right attributes</h2>\n<ul>\n<li><code>width</code> and <code>height</code> to avoid layout shifts;</li>\n<li><code>loading=\"lazy\"</code> below the fold;</li>\n<li><code>srcset</code> to serve the right size.</li>\n</ul>"
  },
  {
   "slug": "accessibilite-checklist",
   "title": "Accessibility in 10 points",
   "excerpt": "A short, concrete list to avoid the most common mistakes.",
   "category": "Accessibility",
   "author": "Inès Benali",
   "date": "2026-09-15",
   "body": "<p>These ten points cover most of the issues found in audits.</p>\n<h2 id=\"structure\">Structure</h2>\n<ul>\n<li>a single <code>h1</code> per page and hierarchical headings;</li>\n<li>landmarks: <code>header</code>, <code>nav</code>, <code>main</code>, <code>footer</code>;</li>\n<li>a skip link to the content.</li>\n</ul>\n<h2 id=\"interaction\">Interaction</h2>\n<ul>\n<li>everything is usable <strong>with the keyboard</strong>;</li>\n<li>focus is always visible;</li>\n<li>form fields have a label.</li>\n</ul>"
  },
  {
   "slug": "choisir-son-framework",
   "title": "Astro, Laravel or Wagtail: choosing your framework",
   "excerpt": "Three excellent tools, three ways of working. Our criteria for choosing without regret.",
   "category": "Technical",
   "author": "Camille Martin",
   "date": "2026-10-02",
   "body": "<p>The right framework is the one your team can keep alive. Here are our criteria.</p>\n<h2 id=\"astro-for-content-sites\">Astro for content sites</h2>\n<p>Static pages by default, interactive islands on demand: ideal for a very fast showcase site or blog.</p>\n<h2 id=\"laravel-for-applications\">Laravel for applications</h2>\n<p>Authentication, queues, e-mails, payments: <strong>everything is built in</strong> for a business application.</p>\n<h2 id=\"wagtail-for-editorial-teams\">Wagtail for editorial teams</h2>\n<p>A real editorial CMS: page tree, StreamField, revisions and previews.</p>"
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
 "fields": [
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
 ]
}""")


def publish(page):
    page.save_revision().publish()
    return page


def child(parent, model, slug, **fields):
    existing = parent.get_children().filter(slug=slug).first()
    if existing:
        return existing.specific
    page = model(slug=slug, **fields)
    parent.add_child(instance=page)
    return publish(page)


def section_page(parent, model, key, data=None, **extra):
    p = (data or PAGES)[key]
    return child(parent, model, p["slug"], title=p["title"], search_description=p["description"],
                 body=json.dumps(p["body"]), **extra)


def translated(page, locale):
    """Champs d'une traduction : même translation_key que la page française (internationalisation de Wagtail)."""
    return {"locale": locale, "translation_key": page.translation_key}


def seed_english(home, pages):
    """Arborescence anglaise (/en/…) : traduction de chaque page et de chaque menu."""
    en, _ = Locale.objects.get_or_create(language_code="en")
    en_home = home.get_translation_or_none(en)
    if en_home is None:
        en_home = home.copy_for_translation(en)
    en_home = en_home.specific
    if en_home.title != EN["pages"]["home"]["title"] or not en_home.live:
        en_home.title = EN["pages"]["home"]["title"]
        en_home.search_description = EN["pages"]["home"]["description"]
        en_home.body = json.dumps(EN["pages"]["home"]["body"])
        publish(en_home)
    out = {"home": en_home}

    def tr(key, parent, model, **extra):
        return section_page(parent, model, key, EN["pages"], **translated(pages[key], en), **extra)

    out["services"] = tr("services", en_home, ServiceIndexPage)
    for s in EN["services"]:
        out[s["slug"]] = child(
            out["services"], ServicePage, s["slug"], title=s["title"], search_description=s["summary"],
            summary=s["summary"], icon=s["icon"], price=s["price"], body=s["body"],
            features=json.dumps([{"type": "feature", "value": f} for f in s["features"]]), **translated(pages[s["slug"]], en),
        )
    out["blog"] = tr("blog", en_home, BlogIndexPage)
    fr_posts = {p.slug: p for p in BlogPage.objects.filter(locale__language_code=home.locale.language_code)}
    for p in EN["posts"]:
        child(
            out["blog"], BlogPage, p["slug"], title=p["title"], search_description=p["excerpt"],
            date=date.fromisoformat(p["date"]), category=p["category"], author=p["author"], excerpt=p["excerpt"],
            body=p["body"], **translated(fr_posts[p["slug"]], en),
        )
    out["about"] = tr("about", en_home, FlexPage)
    if not en_home.get_children().filter(slug=EN["pages"]["contact"]["slug"]).exists():
        page = tr("contact", en_home, ContactPage, subject="New message from the website")
        for i, f in enumerate(EN["fields"]):
            FormField.objects.create(
                page=page, sort_order=i, label=f["label"], clean_name=f["name"], field_type=FIELD_TYPES[f["type"]],
                required=f["required"], autocomplete=f["autocomplete"],
            )
    out["contact"] = en_home.get_children().get(slug=EN["pages"]["contact"]["slug"]).specific
    out["legal"] = tr("legal", en_home, FlexPage, show_in_menus=False)
    out["privacy"] = tr("privacy", en_home, FlexPage, show_in_menus=False)
    return en, out


def translate_menus(en, en_pages):
    """Copie anglaise des menus (même translation_key) avec les libellés et les pages anglaises."""
    by_url = {("/" + EN["pages"][k]["slug"]) if EN["pages"][k]["slug"] else "/": p for k, p in en_pages.items() if k in EN["pages"]}
    services = [en_pages[s["slug"]] for s in EN["services"]]
    fr_menus = list(NavigationMenu.objects.exclude(locale=en).order_by("pk"))
    specs = [EN["nav"]] + [c["links"] for c in EN["footer"]]
    titles = ["Main"] + [c["title"] for c in EN["footer"]]
    for fr_menu, entries, title in zip(fr_menus, specs, titles):
        if NavigationMenu.objects.filter(locale=en, translation_key=fr_menu.translation_key).exists():
            continue
        menu = NavigationMenu.objects.create(title=title, **translated(fr_menu, en))
        for i, entry in enumerate(entries):
            page = by_url.get(entry["url"])
            parent = NavigationItem.objects.create(
                menu=menu, label=entry["label"], page=page, custom_url="" if page else entry["url"], sort_order=i,
            )
            if entry.get("children") == "services":
                for j, s in enumerate(services):
                    NavigationItem.objects.create(menu=menu, label=s.title, page=s, parent=parent, sort_order=100 + j)


class Command(BaseCommand):
    help = "Crée les pages, menus et réglages du site (mêmes contenus qu'Astro et Laravel)."

    @transaction.atomic
    def handle(self, *args, **options):
        site = Site.objects.get(is_default_site=True)
        home = site.root_page.specific
        if not isinstance(home, HomePage):
            self.stderr.write("La page racine du site n'est pas une HomePage : rien à faire.")
            return
        if not home.body:
            home.title = PAGES["home"]["title"]
            home.search_description = PAGES["home"]["description"]
            home.body = json.dumps(PAGES["home"]["body"])
            publish(home)
        site.site_name = site.site_name or "@@title@@"
        site.save()

        pages = {"home": home}
        pages["services"] = section_page(home, ServiceIndexPage, "services")
        for s in SERVICES:
            pages[s["slug"]] = child(
                pages["services"], ServicePage, s["slug"], title=s["title"], search_description=s["summary"],
                summary=s["summary"], icon=s["icon"], price=s["price"], body=s["body"],
                features=json.dumps([{"type": "feature", "value": f} for f in s["features"]]),
            )
        pages["blog"] = section_page(home, BlogIndexPage, "blog")
        for p in POSTS:
            child(
                pages["blog"], BlogPage, p["slug"], title=p["title"], search_description=p["excerpt"],
                date=date.fromisoformat(p["date"]), category=p["category"], author=p["author"], excerpt=p["excerpt"],
                body=p["body"],
            )
        pages["about"] = section_page(home, FlexPage, "about")
        contact = PAGES["contact"]
        if not home.get_children().filter(slug=contact["slug"]).exists():
            page = section_page(home, ContactPage, "contact", subject="Nouveau message depuis le site")
            for i, f in enumerate(FIELDS):
                FormField.objects.create(
                    page=page, sort_order=i, label=f["label"], clean_name=f["name"], field_type=FIELD_TYPES[f["type"]],
                    required=f["required"], autocomplete=f["autocomplete"],
                )
        pages["contact"] = home.get_children().get(slug=contact["slug"]).specific
        pages["legal"] = section_page(home, FlexPage, "legal", show_in_menus=False)
        pages["privacy"] = section_page(home, FlexPage, "privacy", show_in_menus=False)

        by_url = {"/" + p.url.strip("/") if p.url != "/" else "/": p for p in pages.values() if hasattr(p, "url")}

        def item(menu, entry, order, parent=None):
            page = by_url.get(entry["url"])
            return NavigationItem.objects.create(
                menu=menu, label=entry["label"], page=page, custom_url="" if page else entry["url"],
                parent=parent, sort_order=order,
            )

        if not NavigationMenu.objects.exists():
            main = NavigationMenu.objects.create(title="Principal")
            for i, entry in enumerate(NAV):
                parent = item(main, entry, i)
                if entry.get("children") == "services":
                    for j, s in enumerate(SERVICES):
                        NavigationItem.objects.create(
                            menu=main, label=s["title"], page=pages[s["slug"]], parent=parent, sort_order=100 + j
                        )
            columns = []
            for col in FOOTER:
                menu = NavigationMenu.objects.create(title=col["title"])
                for i, entry in enumerate(col["links"]):
                    item(menu, entry, i)
                columns.append(menu)
            header = HeaderSettings.load()
            header.selected_menu = main
            header.cta_label = SITE["cta"]["label"]
            header.cta_page = by_url.get(SITE["cta"]["url"])
            header.save()
            footer = FooterSettings.load()
            footer.selected_menu_1, footer.selected_menu_2 = columns[0], columns[-1]
            footer.save()

        org = OrganisationSettings.load()
        if not org.tagline:
            org.tagline, org.description, org.copyright = SITE["tagline"], SITE["description"], SITE["copyright"]
            org.save()
        contact_settings = ContactSettings.load()
        if not contact_settings.email:
            for k in ("email", "phone", "address", "hours"):
                setattr(contact_settings, k, SITE["contact"][k])
            contact_settings.save()
        social = SocialMediaSettings.load()
        if not (social.github or social.linkedin or social.mastodon):
            for s in SOCIAL:
                setattr(social, s["icon"], s["url"])
            social.save()
        en, en_pages = seed_english(home, pages)
        translate_menus(en, en_pages)
        self.stdout.write(self.style.SUCCESS("Site prêt : %d pages." % home.get_descendants(inclusive=True).count()))
