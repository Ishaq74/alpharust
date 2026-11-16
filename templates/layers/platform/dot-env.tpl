@@meta when=db@@
# Fichier local généré par AlphaRust — ne pas versionner (ignoré par .gitignore).
DB_ENV=LOCAL
DATABASE_URL_LOCAL=@@db_url@@
DATABASE_URL_TEST=@@db_url_test@@
@@if auth@@
BETTER_AUTH_SECRET=@@secret@@
BETTER_AUTH_URL=http://localhost:4321
AUTH_REQUIRE_EMAIL_VERIFICATION=false
@@if plugin_admin@@
ADMIN_EMAIL=
@@end@@
@@end@@
SITE_URL=http://localhost:4321
