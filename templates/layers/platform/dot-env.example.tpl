@@meta when=db@@
# ── Base de données ( LOCAL | TEST | PROD ) — mêmes conventions que le dépôt atomic ─────────
DB_ENV=LOCAL
@@if postgres@@
# Les identifiants ne sont pas dans les URL : le pilote `pg` lit les variables standard PGUSER / PGPASSWORD.
PGUSER=
PGPASSWORD=
DATABASE_URL_LOCAL=postgresql://localhost:5432/@@db_name@@
DATABASE_URL_TEST=postgresql://localhost:5432/@@db_name@@_test
DATABASE_URL_PROD=postgresql://hote:5432/@@db_name@@
@@else@@
DATABASE_URL_LOCAL=./data/@@db_name@@.db
DATABASE_URL_TEST=:memory:
DATABASE_URL_PROD=./data/@@db_name@@.prod.db
@@end@@
@@if auth@@

# ── Authentification (Better Auth) ────────────────────────────────────────────────────────
# Secret aléatoire d'au moins 32 caractères : `npx auth@latest secret`
BETTER_AUTH_SECRET=remplacez-moi-par-un-secret-aleatoire-d-au-moins-32-caracteres
BETTER_AUTH_URL=http://localhost:4321
# Exiger la vérification de l'e-mail avant la connexion (nécessite l'envoi d'e-mails)
AUTH_REQUIRE_EMAIL_VERIFICATION=false
@@if plugin_admin@@
# L'inscription avec cette adresse reçoit le rôle admin
ADMIN_EMAIL=
@@end@@
# À activer UNIQUEMENT derrière un proxy de confiance (nginx, Caddy…) : lit x-forwarded-for
# TRUST_PROXY=true
@@end@@

SITE_URL=http://localhost:4321
