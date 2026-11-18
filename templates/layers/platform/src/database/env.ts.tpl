@@meta when=db@@
import { config } from 'dotenv';
import path from 'node:path';

config({ path: path.resolve(process.cwd(), '.env'), quiet: true });

// ─── Environnements ───────────────────────────────────────────────────
export type DbEnv = 'PROD' | 'TEST' | 'LOCAL';

const DB_ENVS: DbEnv[] = ['PROD', 'TEST', 'LOCAL'];

const ENV_VAR_MAP: Record<DbEnv, string> = {
  PROD: 'DATABASE_URL_PROD',
  TEST: 'DATABASE_URL_TEST',
  LOCAL: 'DATABASE_URL_LOCAL',
};

export interface PoolConfig {
  max: number;
  idleTimeoutMillis: number;
  connectionTimeoutMillis: number;
  allowExitOnIdle?: boolean;
}

const POOL_CONFIGS: Record<DbEnv, PoolConfig> = {
  PROD: { max: Math.min(Math.max(parseInt(process.env.DB_POOL_MAX ?? '20', 10) || 20, 1), 100), idleTimeoutMillis: 30_000, connectionTimeoutMillis: 5_000, allowExitOnIdle: true },
  TEST: { max: 5, idleTimeoutMillis: 10_000, connectionTimeoutMillis: 3_000 },
  LOCAL: { max: 10, idleTimeoutMillis: 15_000, connectionTimeoutMillis: 5_000 },
};

/** Environnement actif (DB_ENV) ; LOCAL par défaut, erreur claire si la valeur est invalide. */
function activeEnv(): DbEnv {
  const raw = process.env.DB_ENV?.trim();
  if (!raw) return 'LOCAL';
  const normalized = raw.toUpperCase();
  if ((DB_ENVS as string[]).includes(normalized)) return normalized as DbEnv;
  throw new Error(`DB_ENV invalide : "${raw}". Valeurs acceptées : ${DB_ENVS.join(', ')}. Vérifiez votre fichier .env`);
}

export const getDbEnv = (): DbEnv => activeEnv();
export const isProd = (): boolean => activeEnv() === 'PROD';

export function getDbUrl(env: DbEnv = activeEnv()): string {
  const name = ENV_VAR_MAP[env];
  const url = process.env[name];
  if (!url) {
    throw new Error(`Variable d'environnement manquante : ${name} (environnement ${env}). Copiez .env.example vers .env et renseignez-la.`);
  }
  return url;
}

export const getPoolConfig = (env: DbEnv = activeEnv()): PoolConfig => POOL_CONFIGS[env];

/** Masque les identifiants d'une URL pour les journaux. */
export function maskUrl(url: string): string {
  return url.replace(/:\/\/[^@]+@/, '://***@');
}
