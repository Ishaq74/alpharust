---
name: django-settings-split
apply: "*"
severity: warning
checker: require-glob
args:
  any: ["**/settings/*.py"]
  when_any: ["manage.py"]
  message: "réglages non séparés (settings/base.py, dev.py, production.py)"
---

# Settings Django séparés recommandés
