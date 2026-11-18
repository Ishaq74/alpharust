@@meta when=db@@
@@if postgres@@
import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';

/** Table d'exemple : à remplacer par votre modèle de données. */
export const note = pgTable('note', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => crypto.randomUUID()),
  title: text('title').notNull(),
  body: text('body').notNull().default(''),
  createdAt: timestamp('created_at').defaultNow().notNull(),
});
@@else@@
import { sql } from 'drizzle-orm';
import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';

/** Table d'exemple : à remplacer par votre modèle de données. */
export const note = sqliteTable('note', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => crypto.randomUUID()),
  title: text('title').notNull(),
  body: text('body').notNull().default(''),
  createdAt: integer('created_at', { mode: 'timestamp_ms' })
    .default(sql`(cast(unixepoch('subsecond') * 1000 as integer))`)
    .notNull(),
});
@@end@@
