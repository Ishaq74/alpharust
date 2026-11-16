---
name: functional-tests
apply: "*"
severity: warning
checker: require-glob
args:
  any: ["**/e2e/**", "**/functional/**", "**/Feature/**", "**/features/**", "tests/*.rs", "tests/**/*.py", "**/cypress/**", "**/playwright*", "**/*.e2e.*", "**/test_*.py", "**/tests.py"]
  when_any: ["Cargo.toml", "package.json", "composer.json", "pyproject.toml", "manage.py", "requirements.txt", "go.mod", "pom.xml"]
  message: "aucun test fonctionnel / end-to-end trouvé"
---

# Tests fonctionnels requis
