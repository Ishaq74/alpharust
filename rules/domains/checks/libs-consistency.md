---
name: libs-consistency
apply: "package.json,composer.json,Cargo.toml,requirements.txt"
severity: warning
checker: deps-consistency
examples:
  file: "package.json"
  bad:
    - "{\"dependencies\": {\"left-pad\": \"*\"}}"
    - "{\"dependencies\": {\"a\": \"^1\"}, \"devDependencies\": {\"a\": \"^1\"}}"
  good:
    - "{\"dependencies\": {\"a\": \"^1\"}, \"devDependencies\": {\"b\": \"^2\"}}"
---

# Cohérence des librairies

Les versions de dépendances doivent être cohérentes et à jour dans la mesure du possible.
