# ADR 0018 — v0.3.0 : corrections de l'audit, parité prouvée en CI, sécurité de production et releases

## Statut

Accepté (complète les ADR 0011, 0015, 0016 et 0017)

## Contexte

L'audit de novembre 2026 a relevé 19 constats : 3 critiques (base SQLite committée, contraste insuffisant par défaut
sur Astro + Starwind, contact Astro qui jetait les messages), 8 importants (palette différente selon le framework, CI
sans parité ni accessibilité, téléchargements non vérifiés, dépendances vieillissantes, SEO inégal, aucune release,
cascade de 55 `@import` côté Wagtail, branches fusionnées) et 9 mineurs (outillage, tests, CLI, faux positifs, GUI,
RTL, production). La promesse « même site sur les 6 combinaisons » n'était vérifiée qu'à la main.

## Décision

1. **Parité vérifiée par la machine, pas par la relecture.** Un workflow `nightly.yml` (chaque nuit, à la demande et
   sur les PR qui touchent le site) génère les 6 combinaisons avec `--auth --langs fr,en,es,ar` et la palette par
   défaut (`docs/screenshots/scripts/serve_combos.sh`), puis lance `site_parity.py` (structure, SEO, pixels ≤ 1 %),
   `site_auth.py` (parcours de compte) et `site_axe.py` (axe-core, WCAG 2.1 A/AA, thèmes clair et sombre ; échec sur
   tout impact sérieux ou critique). Les captures et rapports partent en artefacts.
2. **Une seule palette par défaut** produite par `tools/dsgen` ; le texte de marque utilise un jeton « texte »
   (contraste ≥ 4,5:1), jamais la couleur de fond de marque.
3. **Le référencement fait partie de la spec commune** : canonical, alternates hreflang + `x-default`, Open Graph,
   JSON-LD `Organization` et `BlogPosting`, `robots.txt`, `sitemap.xml` multilingue. Mécanismes officiels quand ils
   existent : `wagtail.contrib.sitemaps` (alternates ajoutés par Wagtail en i18n) ; endpoint Astro dédié car
   `@astrojs/sitemap` ignore les routes rendues côté serveur ; route Laravel.
4. **Une CSP de base identique** dans les 3 frameworks, active en production seulement : Astro par `<meta>` dans le
   layout (sans `frame-ancestors`, ignoré en meta), Laravel par un middleware `SecurityHeaders` ajouté au groupe `web`
   dans `bootstrap/app.php`, Django par son middleware CSP intégré (`SECURE_CSP`, Django ≥ 6.0). `'unsafe-inline'`
   reste autorisé pour les scripts et styles : l'admin Wagtail et le script anti-flash du thème en ont besoin.
   Wagtail reçoit un `settings/production.py` durci (variables d'environnement obligatoires, cookies sécurisés, HSTS,
   redirection HTTPS) ; `check --deploy` ne garde que W005/W021, propres à chaque hébergement.
5. **Chaînes d'approvisionnement** : versions des outils officiels épinglées dans `src/scaffold/pins.rs` ; CLI
   Tailwind vérifiée par SHA-256 ; `allowScripts` npm explicite ; `cargo deny` et Dependabot.
6. **Publication** par cargo-dist : binaires Linux, macOS (x86_64, arm64) et Windows, installateurs shell et
   PowerShell, notes tirées de `CHANGELOG.md`. La publication sur crates.io demande un jeton du propriétaire :
   `cargo install --git https://github.com/Ishaq74/alpharust --tag v0.3.0` en attendant.
7. **Qualité du code** : lints clippy ciblés (`uninlined_format_args`, `redundant_closure_for_method_calls`,
   `semicolon_if_nothing_returned`) plutôt que tout `clippy::pedantic` (558 avertissements surtout documentaires) ;
   l'assistant passe par un trait `Prompter` pour être testé sans terminal ; `tools/dsgen` est un vrai paquet Python
   (ruff, pytest, fichiers de référence).

## Conséquences

- Une régression de parité, de contraste ou de SEO casse la CI au lieu d'attendre une relecture de captures.
- La nightly dure une vingtaine de minutes ; elle ne bloque que les PR qui touchent le site ou les générateurs.
- Resserrer la CSP (nonces, suppression de `'unsafe-inline'`) reste possible projet par projet.
- `--i18n` disparaîtra dans une version majeure ; des sous-commandes `generate astro|laravel|wagtail` restent à l'étude.
