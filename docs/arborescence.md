# Arborescence définitive AlphaRust

Validée et réalisée.

```text
alpharust/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── rules/
│   ├── core/
│   ├── languages/
│   ├── frameworks/
│   ├── domains/
│   │   ├── database/        # + mongodb/, redis/, planetscale/
│   │   ├── smtp/            # + sendgrid/, postmark/, brevo/, resend/, mailgun/
│   │   ├── payments/        # + adyen/, square/, braintree/, paddle/, stripe/
│   │   ├── cloud/           # AWS, GCP, GitHub, Slack, OpenAI, Twilio, Azure, clés privées
│   │   ├── styles/
│   │   ├── design/
│   │   ├── auth/            # + auth0/, clerk/, supabase/, firebase/
│   │   ├── docs/
│   │   ├── readme/
│   │   ├── checks/
│   │   └── tests/
│   ├── markdown/
│   └── _template.md
├── profiles/
├── sites/                   # projets générés (ignorés par git)
├── templates/layers/        # couches de modèles (.tpl) posées après l'outil officiel : common, astro, laravel, wagtail, rust
├── generators/
├── src/                     # parser, engine, checkers, scaffold (pipeline de génération), cli, tui, gui (+ assets/), sarif
├── tests/
├── examples/
├── docs/
├── adr/
├── todos/
└── .github/
```
