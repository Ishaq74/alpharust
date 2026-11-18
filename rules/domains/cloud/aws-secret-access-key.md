---
name: aws-secret-access-key
apply: "*"
severity: error
pattern: "(?i)aws_?secret_?access_?key\\s*[:=]\\s*['\\\"]?[A-Za-z0-9/+=]{40}\\b"
examples:
  bad:
    - "aws_secret_access_key = \"wJalrXUtnFEMI/K7MDENG/bPxRfiCY¦EXAMPLEKEY\""
  good:
    - "aws_secret_access_key = os.environ[\"AWS_SECRET_ACCESS_KEY\"]"
---

# Clé secrète AWS en clair

La clé secrète AWS ne doit pas être versionnée.
