---
name: no-raw-sql
apply: "*.py,*.php,*.js,*.jsx,*.ts,*.tsx,*.rb,*.go,*.java,*.rs"
severity: warning
pattern: "\\b(SELECT\\s.+\\sFROM|INSERT\\s+INTO|UPDATE\\s+\\w+\\s+SET|DELETE\\s+FROM)\\b"
examples:
  bad:
    - "db.query(\"SELECT id FROM users WHERE id = \" + id)"
    - "cur.execute(\"DELETE FROM users\")"
  good:
    - "User.objects.filter(id=id)"
    - "let sel = \"selected from list\";"
---

# Éviter le SQL brut non paramétré

Préférer les query builders ou ORM avec paramètres liés.
Le SQL brut doit être justifié et sécurisé.
