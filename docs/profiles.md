# Profils

Un profil (`profiles/<nom>.toml`) sélectionne un sous-ensemble de règles.

```toml
name = "strict"
description = "Core + domaines critiques"
rules = [
  "core/*",                 # glob sur l'id de la règle (« * » traverse les dossiers)
  "domains/database/*",
  "core/no-eval",           # id exact
]
```

Un motif correspond à l'**id** (`core/no-eval`) ou au **nom** de la règle. Un profil sans motif sélectionne
toutes les règles. `alpharust check-rules` signale tout motif qui ne correspond à aucune règle.

| Profil | Usage |
|--------|-------|
| `core` | Règles universelles minimales (eval, secrets, injection de commandes) |
| `starter` | Core + README + documentation |
| `strict` | Core + base de données, e-mail, paiements, cloud, authentification |
| `web` | Core + styles, accessibilité, design, base de données, e-mail, Astro / Next.js / Laravel |
| `ecommerce` | Paiements + authentification + base de données + e-mail + secrets cloud |
| `cms` | Core + base de données, documentation, README, tests, Wagtail / Django |
| `secrets` | Détection de secrets : cloud + services tiers |
| `third-party` | Règles dédiées aux services tiers (Stripe, SendGrid, MongoDB, Supabase…) |

```bash
alpharust list --profiles
alpharust lint --profile strict
```
