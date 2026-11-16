@@meta when=db&tests@@
import { fileURLToPath } from 'node:url';
@@if postgres@@
import { migrate } from 'drizzle-orm/node-postgres/migrator';
@@else@@
import { migrate } from 'drizzle-orm/better-sqlite3/migrator';
@@end@@
import { checkConnection, getDrizzle } from '@database/drizzle';

const migrationsFolder = fileURLToPath(new URL('../../src/database/migrations', import.meta.url));

/**
 * Prépare la base TEST : applique les migrations générées par `drizzle-kit generate`.
 * Retourne false si la base n'est pas joignable (PostgreSQL non démarré) : les tests sont alors ignorés.
 */
export async function prepareTestDb(): Promise<boolean> {
  const status = await checkConnection();
  if (!status.ok) return false;
@@if postgres@@
  await migrate(getDrizzle(), { migrationsFolder });
@@else@@
  migrate(getDrizzle(), { migrationsFolder });
@@end@@
  return true;
}
