---
name: unit-tests-required
apply: "*"
severity: warning
checker: require-glob
args:
  any: ["tests/**", "test/**", "**/test_*.py", "**/*_test.*", "**/*.test.*", "**/*.spec.*", "**/tests.py", "**/__tests__/**", "**/Tests/**"]
  when_any: ["Cargo.toml", "package.json", "composer.json", "pyproject.toml", "manage.py", "requirements.txt", "go.mod", "pom.xml"]
  message: "aucun test trouvé"
---

# Tests unitaires requis

Le code métier doit être couvert par des tests unitaires.
