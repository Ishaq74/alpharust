@@meta when=db&vitrine@@
@@if postgres@@
import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';

/** Messages du formulaire de contact du site (comme la table contact_messages de Laravel). */
export const contactMessage = pgTable('contact_message', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => crypto.randomUUID()),
  data: text('data').notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
});
@@else@@
import { sql } from 'drizzle-orm';
import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';

/** Messages du formulaire de contact du site (comme la table contact_messages de Laravel). */
export const contactMessage = sqliteTable('contact_message', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => crypto.randomUUID()),
  data: text('data').notNull(),
  createdAt: integer('created_at', { mode: 'timestamp_ms' })
    .default(sql`(cast(unixepoch('subsecond') * 1000 as integer))`)
    .notNull(),
});
@@end@@
