@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// Messages du formulaire de contact : enregistrés comme Laravel (table contact_messages) et Wagtail
// (soumissions du formulaire). Avec --db : table Drizzle `contact_message` ; sinon : data/contact-messages.jsonl
// (une ligne JSON par message, fichier local ignoré par Git).
@@if db@@
import { getDrizzle } from '@database/drizzle';
import { contactMessage } from '@database/schemas';
@@else@@
import { appendFile, mkdir } from 'node:fs/promises';
import { join } from 'node:path';
@@end@@

export async function saveContactMessage(data: Record<string, string>): Promise<void> {
@@if db@@
  await getDrizzle().insert(contactMessage).values({ data: JSON.stringify(data) });
@@else@@
  const dir = join(process.cwd(), 'data');
  await mkdir(dir, { recursive: true });
  const line = JSON.stringify({ data, created_at: new Date().toISOString() });
  await appendFile(join(dir, 'contact-messages.jsonl'), `${line}\n`, 'utf8');
@@end@@
}
