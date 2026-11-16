---
name: supabase-service-role-key-env
apply: "*"
severity: error
pattern: "(?i)SUPABASE_SERVICE_ROLE_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]|NEXT_PUBLIC_SUPABASE_SERVICE\\w*|\\bPUBLIC_SUPABASE_SERVICE\\w*"
examples:
  bad:
    - "SUPABASE_SERVICE_ROLE_KEY = \"abcdefgh12345678\""
    - "NEXT_PUBLIC_SUPABASE_SERVICE_KEY=x"
  good:
    - "SUPABASE_SERVICE_ROLE_KEY = process.env.SUPABASE_SERVICE_ROLE_KEY"
---

# Clé service_role Supabase exposée

La clé `service_role` contourne la sécurité par ligne : jamais dans le code ni dans une variable publique.
