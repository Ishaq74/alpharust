---
name: csrf-protection
apply: "*.html,*.htm,*.php,*.twig,*.jsx,*.tsx,*.vue,*.astro,*.svelte,*.ejs,*.erb"
severity: error
checker: requires-companion
args:
  trigger: "(?i)<form[^>]*method\\s*=\\s*[\\\"']?post"
  require: "(?i)csrf|_token|authenticity_token"
  message: "formulaire POST sans jeton CSRF"
  # Astro : `security.checkOrigin` (doc Astro, « Configuration reference ») refuse les POST d'une autre origine.
  exempt_ext: ["astro"]
  exempt_files: ["astro.config.mjs", "astro.config.ts", "astro.config.mts", "astro.config.cjs"]
  exempt_pattern: "checkOrigin\\s*:\\s*true"
examples:
  bad:
    - "<form method=\"post\" action=\"/pay\"><button>Ok</button></form>"
  good:
    - "<form method=\"post\">{% csrf_token %}</form>"
    - "<form method=\"POST\">@csrf</form>"
    - "<form method=\"get\" action=\"/search\"></form>"
---

# Protection CSRF

Toutes les actions mutantes (POST, PUT, DELETE) doivent être protégées contre le CSRF.

Exception : une page `.astro` d'un projet dont `astro.config.*` active `security.checkOrigin: true`
(Astro vérifie alors l'en-tête `Origin` de chaque POST rendu à la demande, sans jeton dans le formulaire).
