@@meta when=db@@
// Index de re-export des schémas Drizzle. Ajoutez vos tables dans ./schemas/ puis exportez-les ici.
@@if auth@@
export * from './schemas/auth.schema';
export * from './schemas/audit-log.schema';
@@end@@
export * from './schemas/note.schema';
@@if vitrine@@
export * from './schemas/contact-message.schema';
@@end@@
