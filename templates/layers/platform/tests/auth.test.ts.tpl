@@meta when=auth&tests@@
import { afterAll, describe, expect, it } from 'vitest';
import { eq } from 'drizzle-orm';
import { closeDb, getDrizzle } from '@database/drizzle';
import { auditLog, user } from '@database/schemas';
import { auth } from '@lib/auth';
import { prepareTestDb } from './helpers/db';

const ready = await prepareTestDb();
const email = `test-${Date.now()}@example.test`;
const password = `${crypto.randomUUID()}-Aa1`;
const wrong = `${crypto.randomUUID()}-faux`;

describe.skipIf(!ready)('authentification (Better Auth + Drizzle)', () => {
  afterAll(async () => {
    await closeDb();
  });

  it('inscrit un utilisateur et crée une session', async () => {
    const res = await auth.api.signUpEmail({
@@if plugin_username@@
      body: { name: 'Test', email, password, username: `test_${Date.now()}` },
@@else@@
      body: { name: 'Test', email, password },
@@end@@
    });
    expect(res.user.email).toBe(email);
    const rows = await getDrizzle().select().from(user).where(eq(user.email, email));
    expect(rows).toHaveLength(1);
  });

  it('connecte avec le bon mot de passe', async () => {
    const ok = await auth.api.signInEmail({ body: { email, password } });
    expect(ok.token).toBeTruthy();
  });

  it('refuse le mauvais mot de passe', async () => {
    const password = `${wrong}`;
    await expect(auth.api.signInEmail({ body: { email, password } })).rejects.toThrow();
  });

  it('refuse un mot de passe trop court', async () => {
    const password = `${'x'.repeat(3)}`;
    await expect(
      auth.api.signUpEmail({ body: { name: 'Court', email: `court-${Date.now()}@example.test`, password } }),
    ).rejects.toThrow();
  });

  it('expose la table d’audit', async () => {
    const rows = await getDrizzle().select().from(auditLog);
    expect(Array.isArray(rows)).toBe(true);
  });
});
