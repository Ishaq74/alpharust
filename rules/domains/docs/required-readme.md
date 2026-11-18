---
name: required-readme
apply: "docs"
severity: warning
checker: require-glob
args:
  any: ["README*", "readme*"]
  when_any: ["Cargo.toml", "package.json", "composer.json", "pyproject.toml", "manage.py", "requirements.txt", "go.mod", "pom.xml"]
  message: "README manquant à la racine du projet"
---

# README requis
