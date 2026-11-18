---
name: migrations-required
apply: "*"
severity: warning
checker: require-glob
args:
  any: ["**/migrations/**", "prisma/migrations/**", "alembic/versions/**"]
  when_any: ["manage.py", "artisan", "prisma/schema.prisma", "alembic.ini", "knexfile.js", "knexfile.ts"]
  message: "projet avec ORM mais aucun dossier de migrations"
---

# Migrations requises

Toute modification de schéma doit passer par une migration versionnée.
Les bases ne doivent jamais être modifiées manuellement en production.
