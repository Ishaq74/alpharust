@@meta when=db@@
import type { APIRoute } from 'astro';
import { checkConnection } from '@database/drizzle';

export const prerender = false;

const startedAt = Date.now();

/** Sonde de santé (Docker HEALTHCHECK, orchestrateur) : état de la base et durée de fonctionnement. */
export const GET: APIRoute = async () => {
  const db = await checkConnection();
  const body = {
    status: db.ok ? 'ok' : 'degraded',
    uptime: Math.floor((Date.now() - startedAt) / 1000),
    timestamp: new Date().toISOString(),
    db,
  };
  return new Response(JSON.stringify(body), {
    status: db.ok ? 200 : 503,
    headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' },
  });
};
