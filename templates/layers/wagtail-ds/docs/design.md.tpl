# Composants (design kit)

Les composants de ce projet viennent du **design kit AlphaRust**, partagé avec les projets Astro (Starwind / CSS) et Laravel :
une seule source pour les jetons et le CSS, un même balisage (`data-slot`, `data-variant`, `data-size`) partout.

## Où est quoi

```text
ds/                              # application Django « ds » (INSTALLED_APPS)
├── templatetags/ds.py           # balises {% ds_button %}, {% ds_card_header %}…
├── templates/ds/components.html # la page /components/ (23 composants core)
├── templates/ds/components_advanced.html # la page /components/advanced/ (23 composants étendus)
├── views.py  urls.py  tests.py
└── static/ds/
    ├── css/
    │   ├── index.css            # importe jetons, thème, base et composants (ordre fixe)
    │   ├── tokens/              # colors, typography, spacing, components, site
    │   ├── themes/theme.css     # palette de marque (à remplir ou générer)
    │   └── components/*.css     # un fichier par composant ; custom.css = vos surcharges (chargé en dernier)
    └── js/ds.js                 # onglets, dialogue, accordéon, interrupteur, info-bulle (HTML natif + ARIA)
```

`base.html` charge `{% static 'ds/css/index.css' %}` et `{% static 'ds/js/ds.js' %}` ; Wagtail n'a donc besoin d'aucun outil de
build : ce sont de simples fichiers statiques (Tailwind n'est pas utilisé ici).

## Utiliser un composant

```django
{% load ds %}
{% ds_button variant="primary" size="lg" %}Enregistrer{% endds_button %}
{% ds_button href="/search/" variant="outline" %}Lien{% endds_button %}
{% ds_card %}
  {% ds_card_header %}{% ds_card_title %}Titre{% endds_card_title %}{% endds_card_header %}
  {% ds_card_content %}…{% endds_card_content %}
{% endds_card %}
```

Les attributs HTML supplémentaires sont des arguments nommés (`aria_label="x"` → `aria-label="x"`, `class_="mt-4"` → `class`).
Les valeurs sont échappées par `format_html` ; le contenu des blocs est celui rendu par le moteur de gabarits.

## Personnaliser

1. **Couleurs** : `ds/static/ds/css/themes/theme.css` (ou `tokens/colors.css`).
2. **Allure d'un composant** : son jeton dans `tokens/components.css` (`--button-border-radius: 9999px;`).
3. **Autre chose** : `components/custom.css`, jamais les fichiers des composants.

En production : `python manage.py collectstatic` (voir la documentation Django sur le déploiement des fichiers statiques).

## Documentation officielle

- Balises de gabarit personnalisées (`simple_tag`, `simple_block_tag`) : <https://docs.djangoproject.com/en/stable/howto/custom-template-tags/>
- Fichiers statiques : <https://docs.djangoproject.com/en/stable/howto/static-files/>
- `format_html`, `format_html_join` : <https://docs.djangoproject.com/en/stable/ref/utils/#module-django.utils.html>
- Wagtail : <https://docs.wagtail.org/en/stable/>
