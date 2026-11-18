@@meta when=db&tests@@
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { eq } from 'drizzle-orm';
import { closeDb, getDrizzle } from '@database/drizzle';
import { note } from '@database/schemas';
import { prepareTestDb } from './helpers/db';

const ready = await prepareTestDb();

describe.skipIf(!ready)('base de données (@@dialect@@)', () => {
  beforeAll(() => {
    expect(ready).toBe(true);
  });
  afterAll(async () => {
    await closeDb();
  });

  it('insère puis relit une note', async () => {
    const db = getDrizzle();
    const [created] = await db.insert(note).values({ title: 'Bonjour', body: 'Premier test' }).returning();
    expect(created?.id).toBeTruthy();
    const rows = await db.select().from(note).where(eq(note.id, created!.id));
    expect(rows).toHaveLength(1);
    expect(rows[0]?.title).toBe('Bonjour');
    expect(rows[0]?.createdAt).toBeInstanceOf(Date);
  });
});
