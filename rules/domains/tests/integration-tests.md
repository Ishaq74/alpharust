---
name: integration-tests
apply: "*"
severity: warning
checker: require-glob
args:
  any: ["tests/*.rs", "**/integration*/**", "**/Feature/**", "tests/**/*.py", "**/e2e/**", "**/*.integration.*", "**/test_*.py", "**/tests.py", "**/tests/**"]
  when_any: ["Cargo.toml", "package.json", "composer.json", "pyproject.toml", "manage.py", "requirements.txt", "go.mod", "pom.xml"]
  message: "aucun test d'intégration trouvé"
---

# Tests d’intégration requis
