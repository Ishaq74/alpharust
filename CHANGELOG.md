# Journal des versions

Format : [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/), versions [SemVer](https://semver.org/lang/fr/).
Les notes de chaque release GitHub reprennent la section de sa version (cargo-dist).

## [0.3.0] - 2026-11-25

Version « qualité prouvée et publiée » : corrige les 19 constats de l'audit de novembre 2026 (C1–C3, I1–I8, M1–M9).

### Sécurité
- Projets Astro : la base SQLite (`data/*.db*`) n'entre plus jamais dans le premier commit Git ; un test le vérifie (C1).
- CLI Tailwind autonome : somme SHA-256 publiée avec la release vérifiée avant installation (I3).
- Versions des outils officiels épinglées (`create-astro`, `starwind`, Better Auth CLI, Wagtail) dans `src/scaffold/pins.rs` (I3) ; Dependabot pour Cargo et GitHub Actions.
- GUI : contrôle de l'en-tête `Host` sur toutes les routes, GET compris (M6).
- Même CSP de base sur Astro, Laravel et Wagtail en production ; `settings/production.py` Wagtail durci selon la liste de contrôle Django (M9).
- Dépendances à jour, `cargo deny` propre : ratatui 0.30, crossterm 0.29, axum 0.8, `serde_yaml` remplacé par `serde-saphyr` (I4).

### Site vitrine (6 combinaisons)
- Une seule palette par défaut pour les 6 combinaisons, texte de marque au contraste WCAG AA (C2, I1).
- Le formulaire de contact Astro enregistre les messages (table Drizzle `contact_message`), comme Laravel et Wagtail (C3).
- Référencement identique sur les 3 frameworks : canonical, hreflang + x-default, Open Graph, JSON-LD `Organization`/`BlogPosting`, `robots.txt`, `sitemap.xml` multilingue (I5).
- Wagtail CSS pur : `ds.bundle.css` produit par `manage.py ds_bundle` au lieu de 55 `@import` en cascade (I7).
- Design kit en propriétés logiques (RTL), nouvelle règle de lint `logical-properties` (M8).
- Astro : `allowScripts` npm déclaré (esbuild, better-sqlite3), composants repris d'atomic nettoyés (M7).

### CLI et outillage
- `--langs` est l'unique option de langues ; `--i18n` reste un alias déprécié ; `--db` accepte les mêmes noms partout (M4).
- La règle CSRF reconnaît `checkOrigin` d'Astro : plus de faux positifs en profil strict (M5).
- Assistant `alpharust new` testable (trait `Prompter`) et couvert par 10 tests scriptés ; e2e marqués `#[ignore]` et lancés avec `--ignored` en CI (M2).
- `scaffold/internal.rs` découpé par domaine (icônes, Tailwind, gettext, git) ; lints clippy ciblés (M3).
- `tools/dsgen` : vrais modules Python importés (plus d'`exec()`), ruff et pytest avec fichiers de référence (M1).
- Couches : `@@meta mode=delete@@` ; `-p` relatif corrigé.

### CI et publication
- Build et tests sur Windows et macOS, job MSRV (Rust 1.89), `cargo deny`, `cargo package` (I2).
- Workflow nocturne : 6 combinaisons × 4 langues avec `--auth`, parité (structure, SEO, pixels), parcours de compte, axe-core WCAG 2.1 AA clair/sombre, captures en artefacts (I2).
- Releases cargo-dist : binaires Linux, macOS (Intel et Apple Silicon) et Windows, installateurs shell et PowerShell (I6).
- Suppression automatique des branches fusionnées sur GitHub (I8).

## [0.2.0]

Site vitrine en 6 combinaisons (Astro, Laravel, Wagtail × Starwind, CSS pur), langues et RTL, comptes, icônes (voir les ADR 0014 à 0017).
