@@meta when=db@@
import { defineConfig } from 'drizzle-kit';
import { getDbUrl } from './src/database/env';
@@if sqlite@@
import { mkdirSync } from 'node:fs';
import { dirname } from 'node:path';
@@end@@

@@if sqlite@@
// drizzle-kit ouvre le fichier SQLite : son dossier doit exister.
mkdirSync(dirname(getDbUrl()), { recursive: true });

@@end@@
export default defineConfig({
  dialect: '@@drizzle_dialect@@',
  schema: './src/database/schemas.ts',
  out: './src/database/migrations',
  dbCredentials: {
    url: getDbUrl(),
  },
});
