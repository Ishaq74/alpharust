---
name: django-migrations-required
apply: "*"
severity: error
checker: require-glob
args:
  any: ["**/migrations/0*.py"]
  when_any: ["manage.py"]
  message: "projet Django sans migration versionnée"
---

# Migrations Django requises
