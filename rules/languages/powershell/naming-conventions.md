---
name: powershell-naming
apply: "powershell"
severity: warning
pattern: "(?i)^\\s*function\\s+[A-Za-z0-9_]+\\s*[{(]"
examples:
  bad:
    - "function Deploy {"
    - "function  build("
  good:
    - "function Get-Thing {"
    - "function Invoke-Build {"
---

# Conventions de nommage PowerShell
