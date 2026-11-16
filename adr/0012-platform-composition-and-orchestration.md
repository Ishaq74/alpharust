# ADR 0012 — Plateforme Astro composable et orchestration des projets

Statut : accepté

## Contexte

Les générateurs créaient un site propre mais nu. Les dépôts de référence de l'utilisateur (`atomic`,
`astro-css-drizzle-better-auth`, `concordia`, `themecsutomizer`) contiennent des briques éprouvées : Starwind, Drizzle,
Better Auth, i18n, audit, Docker, CI, système CSS complet. Il fallait les rendre **composables** sans les recopier
approximativement ni écrire des fichiers « au hasard ».

## Décision

1. **Outil officiel d'abord.** Starwind : `starwind init` + `starwind add` (remplacé par les composants d'atomic, ADR 0013) ; intégrations : `astro add` ; schéma Better
   Auth : `auth@latest generate` (le paquet `@better-auth/cli` est déprécié) ; migrations : `drizzle-kit generate/migrate`.
   Les dépendances sont installées par le gestionnaire de paquets, jamais écrites à la main dans `package.json`.
2. **Couche `platform/` calquée sur `atomic`** : environnements `LOCAL|TEST|PROD`, connexion Drizzle paresseuse, journal
   d'audit avec repli fichier, middleware de session + en-têtes, `/api/health`, Dockerfile, CI. Les gabarits sont
   conditionnels (`when=`, `@@if@@`) : une seule couche pour SQLite et PostgreSQL.
3. **Concordia copié sans réécriture** (`templates/layers/ui-concordia`) ; les seules adaptations sont listées ci-dessous.
4. **Thème** : l'algorithme WCAG de `themecsutomizer` est porté en Rust (`scaffold/theme.rs`) et exposé en CLI, GUI
   (contrôle en direct) et dans le projet (`theme:check`). Le dépôt `blank` est vide : non utilisé.
5. **Orchestration** : le manifeste `.alpharust-project.json` suffit à choisir la bonne commande native
   (`scaffold/projects.rs`) ; CLI `projects`/`run`, GUI (cartes de projets, jobs, historique), TUI (liste).
6. **Secrets** : aucune URL avec mot de passe dans les fichiers versionnés ni dans le manifeste (masquage `://***@`).
7. **WordPress reporté** : `wp-cli` impose PHP + MySQL réels ; sans base de test fiable, la vérification « réelle »
   promise par le pipeline ne serait pas tenable. À reprendre avec un conteneur MySQL en CI.

## Corrections apportées aux sources reprises

| Source | Correction | Raison |
|--------|-----------|--------|
| concordia `SearchBar.astro` | suppression d'un `?? ''` final inatteignable | erreur `ts(2869)` en TypeScript 6 strict |
| concordia `PageBreadcrumb` | shim `src/lib/i18n/route-helpers.ts` (`getLocalizedUrl`) | dépendance à la table de slugs propre à l'application |
| concordia `global.css` | remplacé par `concordia.css` (import conditionnel des variantes) | choisir les variantes à la génération |
| concordia (icônes) | `astro-icon` + `@iconify-json/mdi` installés, intégration `astro-icon` ajoutée à `astro.config` | les composants importent `astro-icon` |
| themecsutomizer | sélecteur sombre `:root[data-theme='dark']` | doit battre la spécificité des jetons Concordia |
| atomic | `@better-auth/cli` → `auth@latest` ; secret aléatoire par projet ; SMTP non repris | CLI dépréciée ; pas de service d'e-mail à vérifier |

## Conséquences

* Les projets générés passent `astro check`, `astro build`, `vitest` (base, auth, i18n) et le lint AlphaRust ;
  des tests `e2e_astro_platform_*` le démontrent (SQLite, Concordia, PostgreSQL réel).
* Limites assumées : `docker build` n'est pas exécuté ; le blog reste monolingue sous i18n ; pas d'envoi d'e-mails.
