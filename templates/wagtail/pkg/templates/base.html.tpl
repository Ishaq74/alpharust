{% load static wagtailcore_tags wagtailuserbar %}
<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{% block title %}{% if page.seo_title %}{{ page.seo_title }}{% else %}{{ page.title }}{% endif %}{% endblock %} — @@title@@</title>
  <link rel="stylesheet" href="{% static 'css/site.css' %}">
</head>
<body>
  <header><a href="/">@@title@@</a></header>
  <main>
    {% block content %}{% endblock %}
  </main>
  <footer><p>© @@title@@ — propulsé par Wagtail</p></footer>
  {% wagtailuserbar %}
</body>
</html>
