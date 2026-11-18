# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Crée l'arborescence et le contenu du site — mêmes données qu'Astro (src/data, src/content) et Laravel (config/site.php,
SiteSeeder). Idempotent : une page existante (même slug) est conservée.

    python manage.py seed_site
"""
import json
from datetime import date

from django.conf import settings
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
# Traductions (vitrine/locales, langues choisies : --langs) : mêmes slugs, pages liées aux pages françaises par translation_key.
TRANSLATIONS = {}
@@if lang_en@@
TRANSLATIONS["en"] = json.loads(r"""{
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
 ],
 "subject": "New message from the website",
 "menu": "Main"
}""")
@@end@@
@@if lang_es@@
TRANSLATIONS["es"] = json.loads(r"""{
 "pages": {
  "home": {
   "title": "Inicio",
   "description": "Agencia web independiente: creación de sitios, rediseño, accesibilidad y mantenimiento.",
   "slug": "",
   "body": [
    {
     "type": "hero",
     "value": {
      "badge": "Agencia web independiente",
      "title": "Sitios rápidos, accesibles y fáciles de hacer evolucionar",
      "lead": "Diseñamos y mantenemos sitios que cargan en menos de un segundo, cumplen el RGAA y siguen siendo fáciles de modificar.",
      "primary": {
       "label": "Solicitar un presupuesto",
       "page": null,
       "url": "/contact"
      },
      "secondary": {
       "label": "Ver nuestros servicios",
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
        "label": "Puntuación media en Lighthouse"
       },
       {
        "value": "AA",
        "label": "Nivel RGAA objetivo"
       },
       {
        "value": "< 1 s",
        "label": "Visualización del contenido"
       },
       {
        "value": "12 años",
        "label": "De experiencia"
       }
      ]
     }
    },
    {
     "type": "features",
     "value": {
      "title": "Por qué elegirnos",
      "lead": "Cuatro compromisos en cada proyecto.",
      "items": [
       {
        "icon": "zap",
        "title": "Rendimiento",
        "text": "Páginas ligeras, imágenes optimizadas y puntuaciones medidas en cada entrega."
       },
       {
        "icon": "accessibility",
        "title": "Accesibilidad",
        "text": "Navegación con teclado, contrastes controlados y una semántica cuidada."
       },
       {
        "icon": "pencil",
        "title": "Autonomía",
        "text": "Usted modifica sus contenidos sin tener que llamarnos."
       },
       {
        "icon": "shield",
        "title": "Tranquilidad",
        "text": "Actualizaciones, copias de seguridad y supervisión incluidas."
       }
      ]
     }
    },
    {
     "type": "services",
     "value": {
      "title": "Nuestros servicios",
      "lead": "De la creación al mantenimiento.",
      "limit": 3,
      "link": {
       "label": "Todos los servicios",
       "page": null,
       "url": "/services"
      }
     }
    },
    {
     "type": "testimonials",
     "value": {
      "title": "Confían en nosotros",
      "items": [
       {
        "quote": "Nuestro nuevo sitio es el doble de rápido y nuestras solicitudes de presupuesto han aumentado un 40 %.",
        "name": "Sophie Laurent",
        "role": "Directora, Atelier Laurent",
        "initials": "SL"
       },
       {
        "quote": "Una auditoría de accesibilidad clara y correcciones entregadas en tres semanas.",
        "name": "Marc Petit",
        "role": "Responsable digital, Ayuntamiento de Montreuil",
        "initials": "MP"
       },
       {
        "quote": "Publicamos nuestras noticias en cinco minutos, sin ayuda.",
        "name": "Léa Moreau",
        "role": "Responsable de comunicación, Asociación Horizon",
        "initials": "LM"
       }
      ]
     }
    },
    {
     "type": "posts",
     "value": {
      "title": "Últimos artículos",
      "lead": "Consejos y experiencias.",
      "limit": 3,
      "link": {
       "label": "Todo el blog",
       "page": null,
       "url": "/blog"
      }
     }
    },
    {
     "type": "faq",
     "value": {
      "title": "Preguntas frecuentes",
      "items": [
       {
        "question": "¿Cuánto tiempo se tarda en crear un sitio?",
        "answer": "Entre cuatro y ocho semanas, según el número de páginas y de funcionalidades."
       },
       {
        "question": "¿Podré modificar mi sitio yo mismo?",
        "answer": "Sí: cada sitio se entrega con un panel de administración y una formación."
       },
       {
        "question": "¿Será accesible mi sitio?",
        "answer": "Apuntamos al nivel AA del RGAA y entregamos una declaración de accesibilidad."
       },
       {
        "question": "¿Ofrecen mantenimiento?",
        "answer": "Sí, con una cuota mensual que incluye actualizaciones, copias de seguridad y pequeñas mejoras."
       }
      ]
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "¿Tiene un proyecto en mente?",
      "text": "Hablemos: respuesta en 24 horas laborables.",
      "action": {
       "label": "Solicitar un presupuesto",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "about": {
   "title": "Quiénes somos",
   "description": "Quiénes somos, nuestra misión y nuestros valores.",
   "slug": "a-propos",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Quiénes somos",
      "title": "Un equipo pequeño, con grandes exigencias",
      "lead": "Desarrolladores, diseñadores y expertos en accesibilidad unidos en torno a una web sostenible."
     }
    },
    {
     "type": "text",
     "value": {
      "title": "Nuestra misión",
      "body": "<p>Creemos que un buen sitio es <strong>rápido, accesible y sobrio</strong>. Rápido, porque cada segundo cuenta. Accesible, porque la web es para todo el mundo. Sobrio, porque el mejor código es el que no se escribe.</p>\n<p>Desde 2014 acompañamos a artesanos, asociaciones, administraciones locales y pymes, desde el primer taller hasta el mantenimiento.</p>"
     }
    },
    {
     "type": "features",
     "value": {
      "title": "Nuestros valores",
      "lead": "",
      "items": [
       {
        "icon": "heart",
        "title": "Transparencia",
        "text": "Presupuestos detallados, código abierto y ninguna dependencia de proveedor."
       },
       {
        "icon": "leaf",
        "title": "Sobriedad",
        "text": "Menos dependencias, menos datos, menos energía."
       },
       {
        "icon": "users",
        "title": "Cercanía",
        "text": "Un único interlocutor, con contacto directo."
       }
      ]
     }
    },
    {
     "type": "team",
     "value": {
      "title": "El equipo",
      "lead": "Cuatro personas, todas en París.",
      "items": [
       {
        "name": "Camille Martin",
        "role": "Fundadora, desarrolladora",
        "initials": "CM"
       },
       {
        "name": "Inès Benali",
        "role": "Experta en accesibilidad",
        "initials": "IB"
       },
       {
        "name": "Thomas Roux",
        "role": "Diseñador UI",
        "initials": "TR"
       },
       {
        "name": "Hugo Lefèvre",
        "role": "Desarrollador back-end",
        "initials": "HL"
       }
      ]
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "Trabajemos juntos",
      "text": "Cuéntenos su proyecto y le responderemos rápidamente.",
      "action": {
       "label": "Contactar",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "services": {
   "title": "Servicios",
   "description": "Creación, rediseño, accesibilidad y mantenimiento de sitios web.",
   "slug": "services",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Servicios",
      "title": "Lo que hacemos",
      "lead": "Cuatro ofertas claras, combinables según su proyecto."
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
      "title": "¿Necesita consejo?",
      "text": "Le ayudamos a elegir la fórmula adecuada.",
      "action": {
       "label": "Solicitar un presupuesto",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "blog": {
   "title": "Blog",
   "description": "Consejos y experiencias sobre una web rápida y accesible.",
   "slug": "blog",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Blog",
      "title": "Artículos y consejos",
      "lead": "Rendimiento, accesibilidad, decisiones técnicas: lo que aprendemos sobre el terreno."
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
   "title": "Contacto",
   "description": "Escríbanos: respuesta en 24 horas laborables.",
   "slug": "contact",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "Contacto",
      "title": "Hablemos de su proyecto",
      "lead": "Rellene el formulario o escríbanos directamente; respondemos en 24 horas laborables."
     }
    },
    {
     "type": "contact",
     "value": {
      "title": "Escríbanos",
      "success": "¡Gracias! Su mensaje se ha enviado correctamente.",
      "submit": "Enviar el mensaje"
     }
    }
   ]
  },
  "legal": {
   "title": "Aviso legal",
   "description": "Aviso legal del sitio.",
   "slug": "mentions-legales",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "Aviso legal",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"editor\">Editor</h2>\n<p>Este sitio lo edita <strong>@@title@@</strong>, 12 rue de la Paix, 75002 París. Contacto: contact@exemple.fr.</p>\n<h2 id=\"alojamiento\">Alojamiento</h2>\n<p>El sitio está alojado por su proveedor de alojamiento; sus datos figuran en el contrato de servicio.</p>\n<h2 id=\"propiedad-intelectual\">Propiedad intelectual</h2>\n<p>Los textos, imágenes y elementos gráficos de este sitio están protegidos. Queda prohibida cualquier reproducción sin autorización.</p>"
     }
    }
   ]
  },
  "privacy": {
   "title": "Política de privacidad",
   "description": "Cómo tratamos sus datos personales.",
   "slug": "confidentialite",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "Política de privacidad",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"datos-recogidos\">Datos recogidos</h2>\n<p>El formulario de contacto recoge su nombre, su dirección de correo electrónico y su mensaje, únicamente para responderle.</p>\n<h2 id=\"plazo-de-conservación\">Plazo de conservación</h2>\n<p>Los mensajes se conservan durante doce meses y después se eliminan.</p>\n<h2 id=\"sus-derechos\">Sus derechos</h2>\n<p>Puede solicitar el acceso, la rectificación o la supresión de sus datos escribiendo a contact@exemple.fr.</p>"
     }
    }
   ]
  }
 },
 "services": [
  {
   "slug": "creation-site-web",
   "title": "Creación de sitios web",
   "summary": "Un sitio a medida, rápido y bien posicionado, entregado con su panel de administración.",
   "icon": "code",
   "price": "Desde 2.400 €",
   "features": [
    "Maquetas y design system",
    "Contenidos editables",
    "SEO técnico y datos estructurados",
    "Puesta en línea y formación"
   ],
   "body": "<p>Diseñamos su sitio a partir de sus objetivos: quién debe encontrarlo, qué debe hacer en él y cómo medirlo.</p>\n<h2 id=\"nuestro-método\">Nuestro método</h2>\n<p>Talleres de definición, maquetas y, después, desarrollo en iteraciones cortas. Cada versión se <strong>publica en un entorno de pruebas</strong> para que pueda probarla.</p>\n<h2 id=\"qué-se-entrega\">Qué se entrega</h2>\n<ul>\n<li>un sitio adaptable, accesible y rápido;</li>\n<li>un panel de administración para modificar sus contenidos;</li>\n<li>la documentación y una formación de media jornada.</li>\n</ul>"
  },
  {
   "slug": "refonte-performance",
   "title": "Rediseño y rendimiento",
   "summary": "Su sitio actual, más rápido, más claro y mejor posicionado, sin perder su visibilidad en los buscadores.",
   "icon": "zap",
   "price": "Desde 1.900 €",
   "features": [
    "Auditoría Lighthouse y Core Web Vitals",
    "Plan de redirecciones",
    "Optimización de imágenes",
    "Seguimiento tras la puesta en línea"
   ],
   "body": "<p>Un buen rediseño empieza por una auditoría: tiempos de carga, recorridos de usuario y contenidos que convierten.</p>\n<h2 id=\"conservar-el-posicionamiento\">Conservar el posicionamiento</h2>\n<p>Todas las direcciones antiguas se <strong>redirigen</strong> y se supervisan durante tres meses tras la puesta en línea.</p>\n<h2 id=\"medir-la-mejora\">Medir la mejora</h2>\n<ul>\n<li>puntuaciones Lighthouse antes / después;</li>\n<li>Core Web Vitals en las páginas clave;</li>\n<li>tasas de conversión de los formularios.</li>\n</ul>"
  },
  {
   "slug": "accessibilite",
   "title": "Accesibilidad (RGAA)",
   "summary": "Auditoría y adecuación a RGAA / WCAG 2.2 AA, declaración de accesibilidad incluida.",
   "icon": "accessibility",
   "price": "Desde 1.200 €",
   "features": [
    "Auditoría sobre una muestra de páginas",
    "Correcciones priorizadas",
    "Declaración de accesibilidad",
    "Formación de redactores"
   ],
   "body": "<p>La accesibilidad beneficia a todos: navegación con teclado, contrastes, lectores de pantalla y formularios comprensibles.</p>\n<h2 id=\"auditoría\">Auditoría</h2>\n<p>Evaluamos una muestra representativa de páginas según el <strong>RGAA 4.1</strong> y entregamos un informe priorizado.</p>\n<h2 id=\"adecuación\">Adecuación</h2>\n<ul>\n<li>correcciones en el código y en las plantillas;</li>\n<li>control de los contrastes del tema;</li>\n<li>redacción de la declaración de accesibilidad.</li>\n</ul>"
  },
  {
   "slug": "maintenance",
   "title": "Mantenimiento y alojamiento",
   "summary": "Actualizaciones, copias de seguridad, supervisión y pequeñas mejoras, sin sorpresas.",
   "icon": "shield",
   "price": "Desde 90 € / mes",
   "features": [
    "Actualizaciones de seguridad",
    "Copias de seguridad diarias",
    "Supervisión 24/7",
    "Una hora de mejoras al mes"
   ],
   "body": "<p>Un sitio web es software: hay que actualizarlo, hacer copias de seguridad y supervisarlo.</p>\n<h2 id=\"qué-incluye\">Qué incluye</h2>\n<ul>\n<li>actualizaciones del framework y de las dependencias;</li>\n<li>copias de seguridad diarias conservadas durante 30 días;</li>\n<li>supervisión de la disponibilidad y de los certificados.</li>\n</ul>\n<h2 id=\"mejoras\">Mejoras</h2>\n<p>Cada mes se incluye una hora de mejoras: una nueva sección, un formulario o una página.</p>"
  }
 ],
 "posts": [
  {
   "slug": "performance-images",
   "title": "Imágenes ligeras sin perder calidad",
   "excerpt": "Formatos modernos, dimensiones explícitas y carga diferida: el trío ganador.",
   "category": "Rendimiento",
   "author": "Camille Martin",
   "date": "2026-08-28",
   "body": "<p>Las imágenes suelen representar más de la mitad del peso de una página.</p>\n<h2 id=\"los-formatos-adecuados\">Los formatos adecuados</h2>\n<p>AVIF y WebP dividen el peso entre dos y cinco respecto al JPEG, con la misma calidad.</p>\n<h2 id=\"los-atributos-adecuados\">Los atributos adecuados</h2>\n<ul>\n<li><code>width</code> y <code>height</code> para evitar desplazamientos del diseño;</li>\n<li><code>loading=\"lazy\"</code> por debajo de la línea de flotación;</li>\n<li><code>srcset</code> para servir el tamaño adecuado.</li>\n</ul>"
  },
  {
   "slug": "accessibilite-checklist",
   "title": "La accesibilidad en 10 puntos",
   "excerpt": "Una lista breve y concreta para evitar los errores más frecuentes.",
   "category": "Accesibilidad",
   "author": "Inès Benali",
   "date": "2026-09-15",
   "body": "<p>Estos diez puntos cubren la mayoría de los problemas que encontramos en las auditorías.</p>\n<h2 id=\"estructura\">Estructura</h2>\n<ul>\n<li>un solo <code>h1</code> por página y títulos jerarquizados;</li>\n<li>puntos de referencia: <code>header</code>, <code>nav</code>, <code>main</code>, <code>footer</code>;</li>\n<li>un enlace para saltar al contenido.</li>\n</ul>\n<h2 id=\"interacción\">Interacción</h2>\n<ul>\n<li>todo se puede usar <strong>con el teclado</strong>;</li>\n<li>el foco siempre es visible;</li>\n<li>los campos de formulario tienen una etiqueta.</li>\n</ul>"
  },
  {
   "slug": "choisir-son-framework",
   "title": "Astro, Laravel o Wagtail: cómo elegir framework",
   "excerpt": "Tres herramientas excelentes, tres formas de trabajar. Nuestros criterios para elegir sin arrepentirse.",
   "category": "Técnica",
   "author": "Camille Martin",
   "date": "2026-10-02",
   "body": "<p>El framework adecuado es el que su equipo sabrá mantener vivo. Estos son nuestros criterios.</p>\n<h2 id=\"astro-para-sitios-de-contenido\">Astro para sitios de contenido</h2>\n<p>Páginas estáticas por defecto e islas interactivas cuando hacen falta: ideal para un sitio corporativo o un blog muy rápido.</p>\n<h2 id=\"laravel-para-aplicaciones\">Laravel para aplicaciones</h2>\n<p>Autenticación, colas, correos electrónicos, pagos: <strong>todo está previsto</strong> para una aplicación de negocio.</p>\n<h2 id=\"wagtail-para-equipos-editoriales\">Wagtail para equipos editoriales</h2>\n<p>Un verdadero CMS editorial: árbol de páginas, StreamField, revisiones y previsualización.</p>"
  }
 ],
 "nav": [
  {
   "label": "Inicio",
   "url": "/"
  },
  {
   "label": "Servicios",
   "url": "/services",
   "children": "services"
  },
  {
   "label": "Blog",
   "url": "/blog"
  },
  {
   "label": "Quiénes somos",
   "url": "/a-propos"
  },
  {
   "label": "Contacto",
   "url": "/contact"
  }
 ],
 "footer": [
  {
   "title": "Navegación",
   "links": [
    {
     "label": "Inicio",
     "url": "/"
    },
    {
     "label": "Blog",
     "url": "/blog"
    },
    {
     "label": "Quiénes somos",
     "url": "/a-propos"
    },
    {
     "label": "Contacto",
     "url": "/contact"
    }
   ]
  },
  {
   "title": "Información",
   "links": [
    {
     "label": "Aviso legal",
     "url": "/mentions-legales"
    },
    {
     "label": "Política de privacidad",
     "url": "/confidentialite"
    }
   ]
  }
 ],
 "fields": [
  {
   "name": "nom",
   "label": "Nombre",
   "type": "text",
   "autocomplete": "name",
   "required": true
  },
  {
   "name": "email",
   "label": "Correo electrónico",
   "type": "email",
   "autocomplete": "email",
   "required": true
  },
  {
   "name": "sujet",
   "label": "Asunto",
   "type": "text",
   "autocomplete": "off",
   "required": false
  },
  {
   "name": "message",
   "label": "Mensaje",
   "type": "textarea",
   "autocomplete": "off",
   "required": true
  }
 ],
 "subject": "Nuevo mensaje desde el sitio web",
 "menu": "Principal"
}""")
@@end@@
@@if lang_ar@@
TRANSLATIONS["ar"] = json.loads(r"""{
 "pages": {
  "home": {
   "title": "الرئيسية",
   "description": "وكالة ويب مستقلة: إنشاء المواقع، وإعادة التصميم، وإمكانية الوصول، والصيانة.",
   "slug": "",
   "body": [
    {
     "type": "hero",
     "value": {
      "badge": "وكالة ويب مستقلة",
      "title": "مواقع سريعة وسهلة الوصول ويسهل تطويرها",
      "lead": "نصمم مواقع تُحمَّل في أقل من ثانية، وتحترم معايير RGAA، وتبقى سهلة التعديل، ونتولى صيانتها.",
      "primary": {
       "label": "اطلبوا عرض سعر",
       "page": null,
       "url": "/contact"
      },
      "secondary": {
       "label": "اكتشفوا خدماتنا",
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
        "label": "متوسط نتيجة Lighthouse"
       },
       {
        "value": "AA",
        "label": "مستوى RGAA المستهدف"
       },
       {
        "value": "< 1 ث",
        "label": "عرض المحتوى"
       },
       {
        "value": "12 عامًا",
        "label": "من الخبرة"
       }
      ]
     }
    },
    {
     "type": "features",
     "value": {
      "title": "لماذا تختاروننا",
      "lead": "أربعة التزامات في كل مشروع.",
      "items": [
       {
        "icon": "zap",
        "title": "الأداء",
        "text": "صفحات خفيفة، وصور محسّنة، ونتائج تُقاس عند كل تسليم."
       },
       {
        "icon": "accessibility",
        "title": "إمكانية الوصول",
        "text": "تنقل بلوحة المفاتيح، وتباين مضبوط، وبنية دلالية متقنة."
       },
       {
        "icon": "pencil",
        "title": "الاستقلالية",
        "text": "تعدّلون محتواكم دون الحاجة إلى الاتصال بنا."
       },
       {
        "icon": "shield",
        "title": "راحة البال",
        "text": "التحديثات والنسخ الاحتياطية والمراقبة مشمولة."
       }
      ]
     }
    },
    {
     "type": "services",
     "value": {
      "title": "خدماتنا",
      "lead": "من الإنشاء إلى الصيانة.",
      "limit": 3,
      "link": {
       "label": "جميع الخدمات",
       "page": null,
       "url": "/services"
      }
     }
    },
    {
     "type": "testimonials",
     "value": {
      "title": "يثقون بنا",
      "items": [
       {
        "quote": "موقعنا الجديد أسرع بمرتين، وارتفعت طلبات عروض الأسعار لدينا بنسبة 40%.",
        "name": "Sophie Laurent",
        "role": "مديرة، Atelier Laurent",
        "initials": "SL"
       },
       {
        "quote": "تدقيق واضح لإمكانية الوصول، وتصحيحات سُلّمت خلال ثلاثة أسابيع.",
        "name": "Marc Petit",
        "role": "مسؤول التحول الرقمي، بلدية مونتروي",
        "initials": "MP"
       },
       {
        "quote": "ننشر أخبارنا في خمس دقائق، دون مساعدة.",
        "name": "Léa Moreau",
        "role": "مسؤولة الاتصال، جمعية Horizon",
        "initials": "LM"
       }
      ]
     }
    },
    {
     "type": "posts",
     "value": {
      "title": "أحدث المقالات",
      "lead": "نصائح وخبرات من الميدان.",
      "limit": 3,
      "link": {
       "label": "المدونة كاملة",
       "page": null,
       "url": "/blog"
      }
     }
    },
    {
     "type": "faq",
     "value": {
      "title": "الأسئلة الشائعة",
      "items": [
       {
        "question": "كم يستغرق إنشاء موقع؟",
        "answer": "من أربعة إلى ثمانية أسابيع حسب عدد الصفحات والوظائف."
       },
       {
        "question": "هل سأتمكن من تعديل موقعي بنفسي؟",
        "answer": "نعم: يُسلَّم كل موقع مع لوحة إدارة ودورة تدريبية."
       },
       {
        "question": "هل سيكون موقعي سهل الوصول؟",
        "answer": "نستهدف المستوى AA من معايير RGAA ونسلّم بيانًا لإمكانية الوصول."
       },
       {
        "question": "هل تقدمون خدمة الصيانة؟",
        "answer": "نعم، باشتراك شهري يشمل التحديثات والنسخ الاحتياطية والتحسينات الصغيرة."
       }
      ]
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "لديكم مشروع؟",
      "text": "لنتحدث: نرد خلال 24 ساعة عمل.",
      "action": {
       "label": "اطلبوا عرض سعر",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "about": {
   "title": "من نحن",
   "description": "من نحن، ومهمتنا، وقيمنا.",
   "slug": "a-propos",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "من نحن",
      "title": "فريق صغير بمعايير عالية",
      "lead": "مطورون ومصممون وخبراء في إمكانية الوصول، يجمعهم الويب المستدام."
     }
    },
    {
     "type": "text",
     "value": {
      "title": "مهمتنا",
      "body": "<p>نؤمن بأن الموقع الجيد <strong>سريع وسهل الوصول ومقتصد</strong>. سريع، لأن كل ثانية مهمة. سهل الوصول، لأن الويب للجميع. مقتصد، لأن أفضل شيفرة هي تلك التي لا نكتبها.</p>\n<p>منذ عام 2014 نرافق الحرفيين والجمعيات والجماعات المحلية والشركات الصغيرة والمتوسطة، من ورشة العمل الأولى حتى الصيانة.</p>"
     }
    },
    {
     "type": "features",
     "value": {
      "title": "قيمنا",
      "lead": "",
      "items": [
       {
        "icon": "heart",
        "title": "الشفافية",
        "text": "عروض أسعار مفصلة، وشيفرة مفتوحة، ودون أي ارتباط إلزامي بمزوّد."
       },
       {
        "icon": "leaf",
        "title": "الاعتدال",
        "text": "تبعيات أقل، وبيانات أقل، وطاقة أقل."
       },
       {
        "icon": "users",
        "title": "القرب",
        "text": "محاور واحد يمكن التواصل معه مباشرة."
       }
      ]
     }
    },
    {
     "type": "team",
     "value": {
      "title": "الفريق",
      "lead": "أربعة أشخاص، جميعهم في باريس.",
      "items": [
       {
        "name": "Camille Martin",
        "role": "مؤسِّسة ومطوِّرة",
        "initials": "CM"
       },
       {
        "name": "Inès Benali",
        "role": "خبيرة في إمكانية الوصول",
        "initials": "IB"
       },
       {
        "name": "Thomas Roux",
        "role": "مصمم واجهات المستخدم",
        "initials": "TR"
       },
       {
        "name": "Hugo Lefèvre",
        "role": "مطوّر الواجهة الخلفية",
        "initials": "HL"
       }
      ]
     }
    },
    {
     "type": "cta",
     "value": {
      "title": "لنعمل معًا",
      "text": "عرّفونا بمشروعكم، وسنعود إليكم سريعًا.",
      "action": {
       "label": "تواصلوا معنا",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "services": {
   "title": "الخدمات",
   "description": "إنشاء المواقع الإلكترونية، وإعادة تصميمها، وإمكانية الوصول، والصيانة.",
   "slug": "services",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "الخدمات",
      "title": "ما نقدمه",
      "lead": "أربعة عروض واضحة يمكن الجمع بينها حسب مشروعكم."
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
      "title": "تحتاجون إلى نصيحة؟",
      "text": "نساعدكم على اختيار الصيغة المناسبة.",
      "action": {
       "label": "اطلبوا عرض سعر",
       "page": null,
       "url": "/contact"
      }
     }
    }
   ]
  },
  "blog": {
   "title": "المدونة",
   "description": "نصائح وخبرات حول الويب السريع وسهل الوصول.",
   "slug": "blog",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "المدونة",
      "title": "مقالات ونصائح",
      "lead": "الأداء، وإمكانية الوصول، والخيارات التقنية: ما نتعلمه في الميدان."
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
   "title": "اتصل بنا",
   "description": "راسلونا: نرد خلال 24 ساعة عمل.",
   "slug": "contact",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "اتصل بنا",
      "title": "لنتحدث عن مشروعكم",
      "lead": "املؤوا النموذج أو راسلونا مباشرة، ونرد خلال 24 ساعة عمل."
     }
    },
    {
     "type": "contact",
     "value": {
      "title": "راسلونا",
      "success": "شكرًا! تم إرسال رسالتكم بنجاح.",
      "submit": "إرسال الرسالة"
     }
    }
   ]
  },
  "legal": {
   "title": "الإشعار القانوني",
   "description": "الإشعار القانوني للموقع.",
   "slug": "mentions-legales",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "الإشعار القانوني",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"الناشر\">الناشر</h2>\n<p>ينشر هذا الموقع <strong>@@title@@</strong>، 12 rue de la Paix, 75002 Paris. للتواصل: contact@exemple.fr.</p>\n<h2 id=\"الاستضافة\">الاستضافة</h2>\n<p>يستضيف الموقع مزود الاستضافة الخاص بكم، وترد بياناته في عقد الخدمة.</p>\n<h2 id=\"الملكية-الفكرية\">الملكية الفكرية</h2>\n<p>النصوص والصور والعناصر الرسومية في هذا الموقع محمية. يُمنع أي نسخ دون إذن.</p>"
     }
    }
   ]
  },
  "privacy": {
   "title": "سياسة الخصوصية",
   "description": "كيف نعالج بياناتكم الشخصية.",
   "slug": "confidentialite",
   "body": [
    {
     "type": "page_header",
     "value": {
      "eyebrow": "",
      "title": "سياسة الخصوصية",
      "lead": ""
     }
    },
    {
     "type": "text",
     "value": {
      "title": "",
      "body": "<h2 id=\"البيانات-المجمعة\">البيانات المجمعة</h2>\n<p>يجمع نموذج الاتصال اسمكم وبريدكم الإلكتروني ورسالتكم، وذلك فقط للرد عليكم.</p>\n<h2 id=\"مدة-الاحتفاظ\">مدة الاحتفاظ</h2>\n<p>تُحفظ الرسائل اثني عشر شهرًا، ثم تُحذف.</p>\n<h2 id=\"حقوقكم\">حقوقكم</h2>\n<p>يمكنكم طلب الاطلاع على بياناتكم أو تصحيحها أو حذفها بمراسلتنا على contact@exemple.fr.</p>"
     }
    }
   ]
  }
 },
 "services": [
  {
   "slug": "creation-site-web",
   "title": "إنشاء المواقع الإلكترونية",
   "summary": "موقع مصمم حسب الطلب، سريع ومهيأ لمحركات البحث، يُسلَّم مع لوحة الإدارة الخاصة به.",
   "icon": "code",
   "price": "ابتداءً من 2400 €",
   "features": [
    "نماذج التصميم ونظام التصميم",
    "محتوى قابل للتعديل",
    "تحسين تقني لمحركات البحث وبيانات منظمة",
    "الإطلاق والتدريب"
   ],
   "body": "<p>نصمم موقعكم انطلاقًا من أهدافكم: من يجب أن يجده، وماذا ينبغي أن يفعل فيه، وكيف يُقاس نجاحه.</p>\n<h2 id=\"منهجيتنا\">منهجيتنا</h2>\n<p>ورش عمل لتحديد النطاق، ثم نماذج التصميم، ثم تطوير على مراحل قصيرة. تُنشر كل نسخة <strong>على بيئة اختبار</strong> لتتمكنوا من تجربتها.</p>\n<h2 id=\"ما-نسلمه\">ما نسلمه</h2>\n<ul>\n<li>موقع متجاوب وسهل الوصول وعالي الأداء؛</li>\n<li>لوحة إدارة لتعديل محتواكم؛</li>\n<li>التوثيق ودورة تدريبية لمدة نصف يوم.</li>\n</ul>"
  },
  {
   "slug": "refonte-performance",
   "title": "إعادة التصميم والأداء",
   "summary": "موقعكم الحالي أسرع وأوضح وفي ترتيب أفضل، دون أن يفقد ظهوره في محركات البحث.",
   "icon": "zap",
   "price": "ابتداءً من 1900 €",
   "features": [
    "تدقيق Lighthouse وCore Web Vitals",
    "خطة إعادة التوجيه",
    "تحسين الصور",
    "متابعة بعد الإطلاق"
   ],
   "body": "<p>تبدأ إعادة التصميم الناجحة بالتدقيق: أزمنة التحميل، ومسارات المستخدمين، والمحتوى الذي يحقق التحويل.</p>\n<h2 id=\"الحفاظ-على-ترتيبكم-في-محركات-البحث\">الحفاظ على ترتيبكم في محركات البحث</h2>\n<p>يُعاد <strong>توجيه</strong> جميع العناوين القديمة وتُتابَع لمدة ثلاثة أشهر بعد الإطلاق.</p>\n<h2 id=\"قياس-التحسن\">قياس التحسن</h2>\n<ul>\n<li>نتائج Lighthouse قبل التحسين وبعده؛</li>\n<li>مؤشرات Core Web Vitals على الصفحات الرئيسية؛</li>\n<li>معدلات التحويل في النماذج.</li>\n</ul>"
  },
  {
   "slug": "accessibilite",
   "title": "إمكانية الوصول (RGAA)",
   "summary": "تدقيق ومطابقة لمعايير RGAA / WCAG 2.2 AA، مع بيان إمكانية الوصول.",
   "icon": "accessibility",
   "price": "ابتداءً من 1200 €",
   "features": [
    "تدقيق على عينة من الصفحات",
    "تصحيحات حسب الأولوية",
    "بيان إمكانية الوصول",
    "تدريب المحررين"
   ],
   "body": "<p>إمكانية الوصول تفيد الجميع: التنقل بلوحة المفاتيح، والتباين، وقارئات الشاشة، ونماذج مفهومة.</p>\n<h2 id=\"التدقيق\">التدقيق</h2>\n<p>نقيّم عينة تمثيلية من الصفحات وفق <strong>RGAA 4.1</strong> ونسلّم تقريرًا مرتبًا حسب الأولوية.</p>\n<h2 id=\"المطابقة\">المطابقة</h2>\n<ul>\n<li>تصحيحات في الشيفرة والقوالب؛</li>\n<li>التحقق من تباين ألوان المظهر؛</li>\n<li>صياغة بيان إمكانية الوصول.</li>\n</ul>"
  },
  {
   "slug": "maintenance",
   "title": "الصيانة والاستضافة",
   "summary": "تحديثات ونسخ احتياطية ومراقبة وتحسينات صغيرة، دون مفاجآت.",
   "icon": "shield",
   "price": "ابتداءً من 90 € شهريًا",
   "features": [
    "تحديثات الأمان",
    "نسخ احتياطية يومية",
    "مراقبة على مدار الساعة",
    "ساعة تطوير واحدة شهريًا"
   ],
   "body": "<p>الموقع برنامج: يجب تحديثه ونسخه احتياطيًا ومراقبته.</p>\n<h2 id=\"ما-تشمله-الخدمة\">ما تشمله الخدمة</h2>\n<ul>\n<li>تحديثات إطار العمل والمكتبات التي يعتمد عليها؛</li>\n<li>نسخ احتياطية يومية تُحفظ لمدة 30 يومًا؛</li>\n<li>مراقبة توفر الموقع وصلاحية الشهادات.</li>\n</ul>\n<h2 id=\"التطوير\">التطوير</h2>\n<p>كل شهر، تُخصَّص ساعة للتطوير: قسم جديد أو نموذج أو صفحة.</p>"
  }
 ],
 "posts": [
  {
   "slug": "performance-images",
   "title": "صور خفيفة دون فقدان الجودة",
   "excerpt": "صيغ حديثة، وأبعاد صريحة، وتحميل مؤجل: الثلاثي الرابح.",
   "category": "الأداء",
   "author": "كاميل مارتان",
   "date": "2026-08-28",
   "body": "<p>تمثل الصور غالبًا أكثر من نصف حجم الصفحة.</p>\n<h2 id=\"الصيغ-المناسبة\">الصيغ المناسبة</h2>\n<p>تقلّص صيغتا AVIF وWebP الحجم بمقدار مرتين إلى خمس مرات مقارنةً بصيغة JPEG، بالجودة نفسها.</p>\n<h2 id=\"السمات-المناسبة\">السمات المناسبة</h2>\n<ul>\n<li><code>width</code> و<code>height</code> لتجنب إزاحة التخطيط؛</li>\n<li><code>loading=\"lazy\"</code> للصور الواقعة أسفل الجزء المرئي من الصفحة؛</li>\n<li><code>srcset</code> لتقديم الحجم المناسب.</li>\n</ul>"
  },
  {
   "slug": "accessibilite-checklist",
   "title": "إمكانية الوصول في 10 نقاط",
   "excerpt": "قائمة قصيرة وعملية لتجنب الأخطاء الأكثر شيوعًا.",
   "category": "إمكانية الوصول",
   "author": "إيناس بن علي",
   "date": "2026-09-15",
   "body": "<p>تغطي هذه النقاط العشر معظم المشكلات التي نصادفها في عمليات التدقيق.</p>\n<h2 id=\"البنية\">البنية</h2>\n<ul>\n<li>عنوان <code>h1</code> واحد في كل صفحة وعناوين متدرجة؛</li>\n<li>معالم واضحة: <code>header</code> و<code>nav</code> و<code>main</code> و<code>footer</code>؛</li>\n<li>رابط تخطٍّ إلى المحتوى.</li>\n</ul>\n<h2 id=\"التفاعل\">التفاعل</h2>\n<ul>\n<li>كل شيء قابل للاستخدام <strong>بلوحة المفاتيح</strong>؛</li>\n<li>مؤشر التركيز ظاهر دائمًا؛</li>\n<li>لكل حقل في النماذج تسمية.</li>\n</ul>"
  },
  {
   "slug": "choisir-son-framework",
   "title": "Astro أو Laravel أو Wagtail: كيف تختار إطار العمل",
   "excerpt": "ثلاث أدوات ممتازة، وثلاث طرق للعمل. معاييرنا لاختيار لا ندم بعده.",
   "category": "تقنية",
   "author": "كاميل مارتان",
   "date": "2026-10-02",
   "body": "<p>إطار العمل المناسب هو الذي يستطيع فريقكم المحافظة عليه وتطويره. إليكم معاييرنا.</p>\n<h2 id=\"astro-لمواقع-المحتوى\">Astro لمواقع المحتوى</h2>\n<p>صفحات ثابتة افتراضيًا، وجزر تفاعلية عند الحاجة: مثالي لموقع تعريفي أو مدونة فائقة السرعة.</p>\n<h2 id=\"laravel-للتطبيقات\">Laravel للتطبيقات</h2>\n<p>المصادقة، وقوائم الانتظار، والبريد الإلكتروني، والمدفوعات: <strong>كل شيء جاهز</strong> لتطبيقات الأعمال.</p>\n<h2 id=\"wagtail-لفرق-التحرير\">Wagtail لفرق التحرير</h2>\n<p>نظام إدارة محتوى تحريري حقيقي: شجرة صفحات، وStreamField، ومراجعات، ومعاينة.</p>"
  }
 ],
 "nav": [
  {
   "label": "الرئيسية",
   "url": "/"
  },
  {
   "label": "الخدمات",
   "url": "/services",
   "children": "services"
  },
  {
   "label": "المدونة",
   "url": "/blog"
  },
  {
   "label": "من نحن",
   "url": "/a-propos"
  },
  {
   "label": "اتصل بنا",
   "url": "/contact"
  }
 ],
 "footer": [
  {
   "title": "التنقل",
   "links": [
    {
     "label": "الرئيسية",
     "url": "/"
    },
    {
     "label": "المدونة",
     "url": "/blog"
    },
    {
     "label": "من نحن",
     "url": "/a-propos"
    },
    {
     "label": "اتصل بنا",
     "url": "/contact"
    }
   ]
  },
  {
   "title": "معلومات",
   "links": [
    {
     "label": "الإشعار القانوني",
     "url": "/mentions-legales"
    },
    {
     "label": "سياسة الخصوصية",
     "url": "/confidentialite"
    }
   ]
  }
 ],
 "fields": [
  {
   "name": "nom",
   "label": "الاسم",
   "type": "text",
   "autocomplete": "name",
   "required": true
  },
  {
   "name": "email",
   "label": "البريد الإلكتروني",
   "type": "email",
   "autocomplete": "email",
   "required": true
  },
  {
   "name": "sujet",
   "label": "الموضوع",
   "type": "text",
   "autocomplete": "off",
   "required": false
  },
  {
   "name": "message",
   "label": "الرسالة",
   "type": "textarea",
   "autocomplete": "off",
   "required": true
  }
 ],
 "subject": "رسالة جديدة من الموقع",
 "menu": "القائمة الرئيسية"
}""")
@@end@@


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


def seed_translation(home, pages, code, data):
    """Arborescence d'une langue (/<code>/…) : traduction de chaque page (internationalisation de Wagtail)."""
    loc, _ = Locale.objects.get_or_create(language_code=code)
    t_home = home.get_translation_or_none(loc)
    if t_home is None:
        t_home = home.copy_for_translation(loc)
    t_home = t_home.specific
    if t_home.title != data["pages"]["home"]["title"] or not t_home.live:
        t_home.title = data["pages"]["home"]["title"]
        t_home.search_description = data["pages"]["home"]["description"]
        t_home.body = json.dumps(data["pages"]["home"]["body"])
        publish(t_home)
    out = {"home": t_home}

    def tr(key, parent, model, **extra):
        return section_page(parent, model, key, data["pages"], **translated(pages[key], loc), **extra)

    out["services"] = tr("services", t_home, ServiceIndexPage)
    for s in data["services"]:
        out[s["slug"]] = child(
            out["services"], ServicePage, s["slug"], title=s["title"], search_description=s["summary"],
            summary=s["summary"], icon=s["icon"], price=s["price"], body=s["body"],
            features=json.dumps([{"type": "feature", "value": f} for f in s["features"]]), **translated(pages[s["slug"]], loc),
        )
    out["blog"] = tr("blog", t_home, BlogIndexPage)
    fr_posts = {p.slug: p for p in BlogPage.objects.filter(locale__language_code=home.locale.language_code)}
    for p in data["posts"]:
        child(
            out["blog"], BlogPage, p["slug"], title=p["title"], search_description=p["excerpt"],
            date=date.fromisoformat(p["date"]), category=p["category"], author=p["author"], excerpt=p["excerpt"],
            body=p["body"], **translated(fr_posts[p["slug"]], loc),
        )
    out["about"] = tr("about", t_home, FlexPage)
    if not t_home.get_children().filter(slug=data["pages"]["contact"]["slug"]).exists():
        page = tr("contact", t_home, ContactPage, subject=data["subject"])
        for i, f in enumerate(data["fields"]):
            FormField.objects.create(
                page=page, sort_order=i, label=f["label"], clean_name=f["name"], field_type=FIELD_TYPES[f["type"]],
                required=f["required"], autocomplete=f["autocomplete"],
            )
    out["contact"] = t_home.get_children().get(slug=data["pages"]["contact"]["slug"]).specific
    out["legal"] = tr("legal", t_home, FlexPage, show_in_menus=False)
    out["privacy"] = tr("privacy", t_home, FlexPage, show_in_menus=False)
    return loc, out


def translate_menus(loc, t_pages, data):
    """Copie des menus dans une langue (même translation_key) avec ses libellés et ses pages."""
    by_url = {("/" + data["pages"][k]["slug"]) if data["pages"][k]["slug"] else "/": p for k, p in t_pages.items() if k in data["pages"]}
    services = [t_pages[s["slug"]] for s in data["services"]]
    fr_menus = list(NavigationMenu.objects.filter(locale__language_code=settings.LANGUAGE_CODE).order_by("pk"))
    specs = [data["nav"]] + [c["links"] for c in data["footer"]]
    titles = [data["menu"]] + [c["title"] for c in data["footer"]]
    for fr_menu, entries, title in zip(fr_menus, specs, titles):
        if NavigationMenu.objects.filter(locale=loc, translation_key=fr_menu.translation_key).exists():
            continue
        menu = NavigationMenu.objects.create(title=title, **translated(fr_menu, loc))
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
        for code, _ in settings.LANGUAGES:
            if code != settings.LANGUAGE_CODE and code in TRANSLATIONS:
                loc, t_pages = seed_translation(home, pages, code, TRANSLATIONS[code])
                translate_menus(loc, t_pages, TRANSLATIONS[code])
        self.stdout.write(self.style.SUCCESS("Site prêt : %d pages." % home.get_descendants(inclusive=True).count()))
