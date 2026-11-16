---
name: wagtail-migrations-required
apply: "*"
severity: error
checker: require-glob
args:
  any: ["**/migrations/0*.py"]
  when_any: ["manage.py"]
  when_contains: ["requirements.txt", "(?i)wagtail"]
  message: "projet Wagtail sans migration versionnée"
---

# Migrations Wagtail requises

Toute modification de modèle Wagtail doit être accompagnée d’une migration.
