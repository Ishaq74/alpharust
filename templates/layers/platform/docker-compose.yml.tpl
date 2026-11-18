@@meta when=docker@@
# Démarrage : `docker compose up --build` puis http://localhost:4321
services:
  web:
    build: .
    ports:
      - "4321:4321"
    env_file:
      - .env
    environment:
      DB_ENV: PROD
@@if postgres@@
      DATABASE_URL_PROD: postgresql://postgres:${POSTGRES_PASSWORD:-postgres}@db:5432/@@db_name@@
@@else@@
      DATABASE_URL_PROD: /data/@@db_name@@.prod.db
@@end@@
@@if auth@@
      BETTER_AUTH_URL: http://localhost:4321
@@end@@
@@if sqlite@@
    volumes:
      - app-data:/data
@@end@@
@@if postgres@@
    depends_on:
      db:
        condition: service_healthy

  db:
    image: postgres:17-alpine
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-postgres}
      POSTGRES_DB: @@db_name@@
    volumes:
      - pg-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 10
@@end@@
@@if sqlite|postgres@@

volumes:
@@if sqlite@@
  app-data:
@@end@@
@@if postgres@@
  pg-data:
@@end@@
@@end@@
