#!/usr/bin/env bash
# Génère (si absent) et sert le même site sur les 6 combinaisons, aux ports attendus par site_parity.py,
# site_auth.py et site_axe.py : 4401 Astro Starwind, 4402 Astro CSS, 4403 Laravel CSS, 4404 Wagtail CSS,
# 4405 Laravel Starwind, 4406 Wagtail Starwind. Toutes avec --auth et les 4 langues, sans --theme-* (palette par défaut).
#
# Usage : serve_combos.sh <dossier> [binaire alpharust]   (SITE_STACKS=clé1,clé2 pour n'en garder que certaines)
# Journaux : <dossier>/<combo>.log (génération) et <dossier>/<combo>.serve.log (serveur).
set -euo pipefail
DIR=$(mkdir -p "$1" && cd "$1" && pwd)
BIN=${2:-alpharust}
LANGS=fr,en,es,ar

want() { [[ -z "${SITE_STACKS:-}" || ",$SITE_STACKS," == *",$1,"* ]]; }

gen() { # combo framework options…
  local combo=$1 fw=$2; shift 2
  want "$combo" || return 0
  if [[ ! -f "$DIR/$combo/vitrine/.alpharust-project.json" ]]; then
    echo "→ génération $combo"
    "$BIN" generate "$fw" vitrine -p "$DIR/$combo/vitrine" --force --langs "$LANGS" --auth "$@" > "$DIR/$combo.log" 2>&1 \
      || { tail -n 40 "$DIR/$combo.log"; exit 1; }
  fi
}

serve() { # combo port commande…
  local combo=$1 port=$2; shift 2
  want "$combo" || return 0
  (cd "$DIR/$combo/vitrine" && nohup "$@" > "$DIR/$combo.serve.log" 2>&1 &)
  for _ in $(seq 1 60); do
    curl -s -o /dev/null "http://127.0.0.1:$port/" && { echo "✔ $combo sur le port $port"; return 0; }
    sleep 1
  done
  echo "✘ $combo ne répond pas sur le port $port"; tail -n 40 "$DIR/$combo.serve.log"; exit 1
}

gen astro-starwind astro --ui starwind --db sqlite
gen astro-css astro --ui concordia --db sqlite
gen laravel-css laravel --ui css
gen wagtail-css wagtail --ui css
gen laravel-starwind laravel --ui starwind
gen wagtail-starwind wagtail --ui starwind

serve astro-starwind 4401 env HOST=0.0.0.0 PORT=4401 node dist/server/entry.mjs
serve astro-css 4402 env HOST=0.0.0.0 PORT=4402 node dist/server/entry.mjs
serve laravel-css 4403 php artisan serve --host=127.0.0.1 --port=4403
serve wagtail-css 4404 .venv/bin/python manage.py runserver 127.0.0.1:4404 --noreload
serve laravel-starwind 4405 php artisan serve --host=127.0.0.1 --port=4405
serve wagtail-starwind 4406 .venv/bin/python manage.py runserver 127.0.0.1:4406 --noreload
