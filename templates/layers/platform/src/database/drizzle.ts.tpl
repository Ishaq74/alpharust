@@meta when=db@@
@@if postgres@@
import { drizzle, type NodePgDatabase } from 'drizzle-orm/node-postgres';
import { Pool } from 'pg';
import * as schema from './schemas';
import { getDbUrl, getPoolConfig } from './env';

export { schema };
export type DrizzleDB = NodePgDatabase<typeof schema>;

let instance: { pool: Pool; db: DrizzleDB; healthy: boolean } | null = null;

function create() {
  const url = getDbUrl();
  const pool = new Pool({
    connectionString: url,
    ssl: url.includes('sslmode=require') ? { rejectUnauthorized: true } : undefined,
    ...getPoolConfig(),
  });
  const entry = { pool, db: drizzle(pool, { schema }), healthy: true };
  pool.on('error', (err) => {
    console.error('[DB] Erreur de pool inattendue :', err.message);
    entry.healthy = false;
  });
  return entry;
}

/** Connexion paresseuse : rien n'est ouvert tant qu'une requête n'est pas faite. */
export function getDrizzle(): DrizzleDB {
  if (instance && !instance.healthy) {
    void instance.pool.end().catch(() => undefined);
    instance = null;
  }
  instance ??= create();
  return instance.db;
}

export async function checkConnection(): Promise<{ ok: boolean }> {
  try {
    getDrizzle();
    await instance!.pool.query('SELECT 1');
    return { ok: true };
  } catch {
    return { ok: false };
  }
}

export async function closeDb(): Promise<void> {
  if (instance) await instance.pool.end();
  instance = null;
}
@@else@@
import Database from 'better-sqlite3';
import { drizzle, type BetterSQLite3Database } from 'drizzle-orm/better-sqlite3';
import { mkdirSync } from 'node:fs';
import { dirname } from 'node:path';
import * as schema from './schemas';
import { getDbUrl } from './env';

export { schema };
export type DrizzleDB = BetterSQLite3Database<typeof schema>;

let instance: { sqlite: Database.Database; db: DrizzleDB } | null = null;

function create() {
  const url = getDbUrl();
  if (url !== ':memory:') mkdirSync(dirname(url), { recursive: true });
  const sqlite = new Database(url);
  sqlite.pragma('journal_mode = WAL');
  sqlite.pragma('foreign_keys = ON');
  return { sqlite, db: drizzle(sqlite, { schema }) };
}

/** Ouverture paresseuse : le fichier n'est ouvert qu'à la première requête. */
export function getDrizzle(): DrizzleDB {
  instance ??= create();
  return instance.db;
}

export async function checkConnection(): Promise<{ ok: boolean }> {
  try {
    getDrizzle();
    instance!.sqlite.prepare('SELECT 1').get();
    return { ok: true };
  } catch {
    return { ok: false };
  }
}

export async function closeDb(): Promise<void> {
  instance?.sqlite.close();
  instance = null;
}
@@end@@

/** Proxy paresseux pour les bibliothèques qui reçoivent la base à l'initialisation (Better Auth). */
export function getLazyDrizzle(): DrizzleDB {
  return new Proxy({} as DrizzleDB, {
    get: (_target, prop) => Reflect.get(getDrizzle(), prop),
  });
}
