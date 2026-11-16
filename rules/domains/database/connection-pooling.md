---
name: connection-pooling
apply: "*.js,*.ts,*.py,*.php,*.rb,*.go,*.java"
severity: warning
checker: requires-companion
args:
  trigger: "\\b(createConnection|psycopg2\\.connect|mysql\\.connector\\.connect|mysqli_connect|sqlite3\\.connect)\\s*\\("
  require: "(?i)pool"
  message: "connexion directe sans pool"
examples:
  bad:
    - "conn = psycopg2.connect(dsn)"
  good:
    - "pool = ThreadedConnectionPool(1, 5, dsn)\nconn = psycopg2.connect(dsn)"
    - "const pool = mysql.createPool(cfg);"
---

# Pool de connexions

Les connexions à la base de données doivent utiliser un pool pour éviter l’épuisement des connexions et améliorer les performances.
