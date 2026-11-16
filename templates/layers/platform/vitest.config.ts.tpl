@@meta when=tests@@
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const src = (dir: string) => fileURLToPath(new URL(`./src/${dir}`, import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      '@lib': src('lib'),
      '@database': src('database'),
      '@i18n': src('i18n'),
      '@components': src('components'),
      '@layouts': src('layouts'),
      '@styles': src('styles'),
    },
  },
  test: {
    environment: 'node',
    include: ['tests/**/*.test.ts'],
    // Les tests utilisent toujours la base TEST (SQLite en mémoire ou base `_test` PostgreSQL).
    env: {
      DB_ENV: 'TEST',
      BETTER_AUTH_SECRET: `${crypto.randomUUID()}${crypto.randomUUID()}`,
      BETTER_AUTH_URL: 'http://localhost:4321',
    },
    testTimeout: 20_000,
    hookTimeout: 30_000,
  },
});
