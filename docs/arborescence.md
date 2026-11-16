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
├── templates/               # fichiers .tpl embarqués (bootstrap, wagtail, astro, laravel)
├── generators/
├── src/                     # parser, engine, checkers, generators, cli, tui, gui, sarif
├── tests/
├── examples/
├── docs/
├── adr/
├── todos/
└── .github/
```
