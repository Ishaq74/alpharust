@@meta when=audit@@
@@if postgres@@
import { relations } from 'drizzle-orm';
import { index, jsonb, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import { user } from './auth.schema';

export const auditLog = pgTable(
  'audit_log',
  {
    id: text('id')
      .primaryKey()
      .$defaultFn(() => crypto.randomUUID()),
    userId: text('user_id').references(() => user.id, { onDelete: 'set null' }),
    action: text('action').notNull(),
    resource: text('resource'),
    resourceId: text('resource_id'),
    metadata: jsonb('metadata'),
    ipAddress: text('ip_address'),
    userAgent: text('user_agent'),
    createdAt: timestamp('created_at').defaultNow().notNull(),
  },
  (table) => [
    index('audit_log_userId_idx').on(table.userId),
    index('audit_log_action_idx').on(table.action),
    index('audit_log_createdAt_idx').on(table.createdAt),
  ],
);
@@else@@
import { relations, sql } from 'drizzle-orm';
import { index, integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { user } from './auth.schema';

export const auditLog = sqliteTable(
  'audit_log',
  {
    id: text('id')
      .primaryKey()
      .$defaultFn(() => crypto.randomUUID()),
    userId: text('user_id').references(() => user.id, { onDelete: 'set null' }),
    action: text('action').notNull(),
    resource: text('resource'),
    resourceId: text('resource_id'),
    metadata: text('metadata', { mode: 'json' }),
    ipAddress: text('ip_address'),
    userAgent: text('user_agent'),
    createdAt: integer('created_at', { mode: 'timestamp_ms' })
      .default(sql`(cast(unixepoch('subsecond') * 1000 as integer))`)
      .notNull(),
  },
  (table) => [
    index('audit_log_userId_idx').on(table.userId),
    index('audit_log_action_idx').on(table.action),
    index('audit_log_createdAt_idx').on(table.createdAt),
  ],
);
@@end@@

export const auditLogRelations = relations(auditLog, ({ one }) => ({
  user: one(user, { fields: [auditLog.userId], references: [user.id] }),
}));
