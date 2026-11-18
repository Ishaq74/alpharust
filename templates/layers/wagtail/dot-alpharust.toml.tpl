# Configuration AlphaRust du projet (https://github.com/Ishaq74/alpharust)
profile = "cms"
exclude = [
  ".venv/**",
  "media/**",
  "staticfiles/**",
  "requirements-lock.txt",
  "Dockerfile",
  ".dockerignore",
  "@@pkg@@/settings/**",
  "@@pkg@@/templates/**",
  "@@pkg@@/static/**",
  "home/templates/**",
  "home/static/**",
  "@@pkg@@/wsgi.py",
  "@@pkg@@/urls.py",
  "search/**",
]
fail_on = "error"
