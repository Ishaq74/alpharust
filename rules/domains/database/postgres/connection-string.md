---
name: postgres-connection-string
apply: "postgres"
severity: error
pattern: "postgres(ql)?://[^/\\s:@'\\\"$]+:[^@\\s'\\\"$]+@"
examples:
  bad:
    - "DATABASE_URL = \"postgres://admin:secret@db/app\""
  good:
    - "DATABASE_URL = os.environ[\"DATABASE_URL\"]"
    - "postgres://${U}:${P}@db/app"
---

# Connection string PostgreSQL sans credentials en clair
