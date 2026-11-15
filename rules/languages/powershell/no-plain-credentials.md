---
name: powershell-no-plain-credentials
apply: powershell
severity: error
pattern: "ConvertTo-SecureString.*-AsPlainText"
---

# Pas de credentials en clair en PowerShell
