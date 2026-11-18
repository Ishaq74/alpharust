# Configuration AlphaRust du projet (https://github.com/Ishaq74/alpharust)
profile = "web"
exclude = [
  "dist/**",
  ".astro/**",
  "node_modules/**",
  "package-lock.json",
  "pnpm-lock.yaml",
  "yarn.lock",
  "bun.lock",
  "bun.lockb",
  "astro.config.*",
  ".env",
@@if platform@@
  "src/database/migrations/**",
@@end@@
@@if starwind@@
  # composants Starwind d'atomic (atoms/) et utilitaires : code tiers repris tel quel
  "src/components/atoms/**",
  "src/components/wow/**",
  "src/lib/starwind/**",
  "src/styles/starwind.css",
  "src/styles/tokens/colors.css",
@@end@@
@@if concordia@@
  # système CSS et composants Concordia (copie fidèle du dépôt d'origine)
  "src/components/ui/**",
  "src/styles/base.css",
  "src/styles/tokens/**",
  "src/styles/components/**",
@@end@@
@@if official@@
  # fichiers fournis par le template officiel create-astro (non modifiés par AlphaRust)
  "src/layouts/Layout.astro",
  "src/styles/global.css",
@@end@@
]
@@if official@@
# le template officiel n'utilise pas de content collections
disable = ["astro-content-collections"]
@@end@@
fail_on = "error"
