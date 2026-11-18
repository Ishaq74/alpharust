---
name: api-docs-present
apply: "docs"
severity: warning
checker: require-glob
args:
  any: ["docs/**", "openapi.*", "**/openapi.*", "swagger.*", "**/swagger.*"]
  when_any: ["Cargo.toml", "package.json", "composer.json", "pyproject.toml", "manage.py", "requirements.txt", "go.mod", "pom.xml"]
  message: "aucun dossier docs/ ni spécification OpenAPI"
---

# Documentation API requise
