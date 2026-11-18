---
name: mysql-connection-string
apply: "mysql"
severity: error
pattern: "mysql://[^/\\s:@'\\\"$]+:[^@\\s'\\\"$]+@"
examples:
  bad:
    - "url = \"mysql://root:p4ss@db:3306/app\""
  good:
    - "url = \"mysql://${USER}:${PASS}@db/app\""
    - "url = os.environ[\"DATABASE_URL\"]"
---

# Connection string MySQL sans credentials en clair
