@@meta when=docker@@
# syntax=docker/dockerfile:1
# Image de production : construction puis exécution (adapter Node d'Astro, mode standalone).
FROM node:24-slim AS build
WORKDIR /app
@@if corepack@@
RUN corepack enable
@@end@@
COPY package.json @@lockfile@@ ./
RUN @@install_ci@@
COPY . .
RUN @@run@@ build

FROM node:24-slim AS runtime
WORKDIR /app
ENV NODE_ENV=production HOST=0.0.0.0 PORT=4321
COPY --from=build /app/dist ./dist
COPY --from=build /app/node_modules ./node_modules
COPY --from=build /app/package.json ./package.json
@@if db@@
COPY --from=build /app/src/database ./src/database
COPY --from=build /app/drizzle.config.ts ./drizzle.config.ts
@@end@@
USER node
EXPOSE 4321
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD node -e "fetch('http://127.0.0.1:'+process.env.PORT+'/api/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"
CMD ["node", "./dist/server/entry.mjs"]
