---
name: powershell-no-plain-credentials
apply: "powershell"
severity: error
pattern: "ConvertTo-SecureString.*-AsPlainText"
examples:
  bad:
    - "$p = ConvertTo-SecureString \"x\" -AsPlainText -Force"
  good:
    - "$p = Read-Host -AsSecureString"
---

# Pas de credentials en clair en PowerShell
