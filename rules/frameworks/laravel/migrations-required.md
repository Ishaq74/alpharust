---
name: laravel-migrations-required
apply: "*"
severity: error
checker: require-glob
args:
  any: ["database/migrations/*.php"]
  when_any: ["artisan"]
  message: "projet Laravel sans migration"
---

# Migrations Laravel requises

Toute modification de schéma doit passer par une migration Artisan.
