---
name: no-hardcoded-paths
apply: "*"
severity: warning
pattern: "[A-Z]:\\\\|(^|[^A-Za-z0-9_.@-])/(home|Users)/[A-Za-z0-9_.-]+"
examples:
  bad:
    - "open(\"/home/bob/data.txt\")"
    - "C:\\Users\\bob\\x"
    - "path = /Users/alice/project"
  good:
    - "open(path)"
    - "let p = dirs::home_dir();"
    - "templates(\"wagtail/home/models.py\")"
---

# Éviter les chemins absolus en dur
