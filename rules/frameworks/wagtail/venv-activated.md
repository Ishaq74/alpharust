---
name: wagtail-venv-activated
apply: "*"
severity: warning
checker: file-contains
args:
  file: ".gitignore"
  any: ["(?m)^/?\\.?venv/?\\s*$"]
  when_any: ["manage.py"]
  when_contains: ["requirements.txt", "(?i)wagtail"]
  message: ".gitignore doit ignorer l'environnement virtuel (.venv/)"
---

# Environnement virtuel ignoré par git

L'environnement virtuel Python (`.venv/` ou `venv/`) ne doit jamais être versionné : le `.gitignore` du projet doit l'ignorer.
