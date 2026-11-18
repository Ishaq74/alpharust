---
name: csrf-protection
apply: "*.html,*.htm,*.php,*.twig,*.jsx,*.tsx,*.vue,*.astro,*.svelte,*.ejs,*.erb"
severity: error
checker: requires-companion
args:
  trigger: "(?i)<form[^>]*method\\s*=\\s*[\\\"']?post"
  require: "(?i)csrf|_token|authenticity_token"
  message: "formulaire POST sans jeton CSRF"
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
