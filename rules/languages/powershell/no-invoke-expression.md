---
name: no-invoke-expression
apply: "powershell"
severity: error
pattern: "(?i)\\bInvoke-Expression\\b|\\biex\\s"
examples:
  bad:
    - "Invoke-Expression $cmd"
    - "iex $cmd"
  good:
    - "Invoke-Command -ScriptBlock { 1 }"
    - "$x = Get-Index"
---

# Interdiction d’Invoke-Expression
