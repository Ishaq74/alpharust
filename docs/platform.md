# Briques plateforme (Astro)

Au-delà du squelette, `alpharust generate astro` compose une **plateforme** à partir de briques indépendantes. Chacune
est installée avec l'**outil officiel** quand il existe, puis une couche de modèles pose le code de liaison ; l'ensemble
est ensuite **vérifié pour de vrai** (`astro check`, `astro build`, `vitest`, lint).

| Brique | Option | Outil officiel / source | Ce qui est vérifié |
|--------|--------|-------------------------|--------------------|
| Tailwind + **Starwind UI** | `--ui starwind [--ui-component …]` | `starwind@latest init --defaults` puis `starwind add … --yes` | composants présents, build |
| **Concordia** (CSS pur) | `--ui concordia [--css-variant …] [--theme-*]` | copie fidèle du dépôt `Ishaq74/concordia` (jetons, variantes, **tous** les composants) | `astro check` sur les 80+ composants, page de démonstration `/components` |
| Base de données | `--db sqlite\|postgres [--db-url …]` | `drizzle-orm` + `drizzle-kit generate` / `migrate` | migrations appliquées, test d'insertion/lecture |
| Authentification | `--auth [--auth-plugin admin,username,organization]` | **Better Auth** ; schéma Drizzle produit par `auth@latest generate` | inscription, connexion, mauvais mot de passe (tests) + parcours navigateur |
| Langues | `--i18n fr,en,es,ar` | routage i18n natif d'Astro (`prefixDefaultLocale: false`), arabe en RTL | parité des clés de messages (test) |
| Déploiement | `--docker`, `--ci` | adaptateur `@astrojs/node` (`astro add node`) | Dockerfile, compose, workflow GitHub Actions |

Préréglages : `alpharust presets` (`vitrine`, `blog-i18n`, `app-auth`, `saas-pg`, `concordia`) →
`alpharust generate astro mon-site --preset saas-pg`. Les options explicites complètent ou remplacent le préréglage.

Règles de compatibilité (refusées avant toute exécution) : briques plateforme ⇒ template `alpharust` et installation ;
`--auth` ⇒ base de données ; `--i18n` ⇒ au moins deux langues ; Concordia ⇒ pas de Tailwind ; Docker ⇒ npm/pnpm/yarn.

## Ce que contient un projet généré

```text
src/
  database/        env.ts (LOCAL | TEST | PROD), drizzle.ts (connexion paresseuse), schemas/, migrations/
  lib/             auth.ts (Better Auth + plugins + journal d'audit), audit.ts, auth-forms.ts, auth-guards.ts
  i18n/            config.ts, utils.ts, messages/{fr,en,es,ar}.json
  middleware.ts    session → Astro.locals + en-têtes de sécurité
  pages/           auth/{sign-in,sign-up,dashboard}, admin, api/health, api/auth/[...all], [lang]/…
  components/      auth/ (Starwind, Concordia ou HTML simple), LanguageSwitcher
tests/             base de données (TEST), authentification, parité i18n
```

* **Environnements de base** : `DB_ENV=LOCAL|TEST|PROD`, une URL par environnement (`DATABASE_URL_*`). Les tests utilisent
  toujours `TEST` (SQLite en mémoire ou base `<nom>_test`).
* **Authentification** : sessions Better Auth, redirections 303 avec cookies, limites de débit, journal d'audit en base
  (repli fichier), rôle `admin` attribué à `ADMIN_EMAIL`. Le secret `BETTER_AUTH_SECRET` est aléatoire (32 octets) dans `.env`.
* **Aucun identifiant en clair dans les fichiers versionnés** : `.env.example` s'appuie sur `PGUSER`/`PGPASSWORD`, la CI et
  compose lisent leurs mots de passe via variables, et le manifeste `.alpharust-project.json` masque les URL (`://***@`).

## Concordia et themecsutomizer

`--ui concordia` copie sans réécriture `src/styles/{base.css,tokens,components/*}` et `src/components/ui/**`.
Corrections minimales apportées (liste complète dans l'ADR 0012) : `SearchBar` (TypeScript strict), shim
`@lib/i18n/route-helpers`, `concordia.css` (sélection des variantes), `astro-icon` + `@iconify-json/mdi` installés.

`--theme-primary/-secondary/-accent` génère `src/styles/theme.css` avec l'algorithme WCAG du dépôt
`themecsutomizer` (luminance relative, contraste, AA ≥ 4,5 / AAA ≥ 7, texte lisible blanc ou `#111827`, variante sombre
recalculée sur `#0f172a`). La GUI affiche ce contrôle **en direct** ; `npm run theme:check` le rejoue dans le projet.

## Orchestration

```bash
alpharust projects                       # état : briques, dépendances installées, git, tâches possibles
alpharust run mon-site build             # install | dev | build | check | test | lint | update
alpharust run mon-site update --json     # événements JSON
```

Chaque tâche lance la commande native d'après le manifeste (`pnpm run build`, `php artisan test`, `cargo test`…). La GUI
propose les mêmes actions (page *Projets*, historique des exécutions) ; le serveur de développement reste réservé à la CLI.
