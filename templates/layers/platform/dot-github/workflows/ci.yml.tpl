@@meta when=ci@@
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  verify:
    runs-on: ubuntu-latest
@@if postgres@@
    services:
      postgres:
        image: postgres:17-alpine
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: @@db_name@@_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres"
          --health-interval 5s
          --health-timeout 5s
          --health-retries 10
    env:
      # Identifiants lus par le pilote `pg` (PGUSER / PGPASSWORD) : aucun mot de passe dans l'URL.
      PGUSER: postgres
      PGPASSWORD: postgres
      DB_ENV: TEST
      DATABASE_URL_TEST: postgresql://localhost:5432/@@db_name@@_test
@@end@@
    steps:
      - uses: actions/checkout@v4
@@if pnpm@@
      - uses: pnpm/action-setup@v4
@@end@@
      - uses: actions/setup-node@v4
        with:
          node-version: 24
@@if !bun@@
          cache: @@pm@@
@@end@@
@@if bun@@
      - uses: oven-sh/setup-bun@v2
@@end@@
      - run: @@install_ci@@
@@if typecheck@@
      - run: @@run@@ check
@@end@@
@@if tests@@
      - run: @@run@@ test
@@end@@
      - run: @@run@@ build
