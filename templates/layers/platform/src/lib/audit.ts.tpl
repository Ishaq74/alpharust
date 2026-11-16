@@meta when=audit@@
import { getDrizzle } from '@database/drizzle';
import { auditLog } from '@database/schemas';
import { appendFile, mkdir } from 'node:fs/promises';
import { join } from 'node:path';
import { isIP } from 'node:net';

export type AuditAction =
  | 'SIGN_IN' | 'SIGN_IN_FAILED' | 'SIGN_UP' | 'SIGN_UP_FAILED' | 'SIGN_OUT'
  | 'PASSWORD_CHANGE' | 'PASSWORD_CHANGE_FAILED' | 'PASSWORD_RESET_REQUEST' | 'PASSWORD_RESET_COMPLETE'
  | 'USER_UPDATE' | 'USER_DELETE' | 'USER_BAN' | 'USER_UNBAN' | 'USER_ROLE_CHANGE'
  | 'IMPERSONATION_START' | 'IMPERSONATION_STOP'
  | 'ORG_CREATE' | 'ORG_UPDATE' | 'ORG_DELETE' | 'ORG_MEMBER_REMOVE' | 'ORG_MEMBER_ROLE_CHANGE'
  | 'ORG_INVITATION_SEND' | 'ORG_INVITATION_ACCEPT' | 'ORG_INVITATION_REJECT' | 'ORG_INVITATION_CANCEL';

export interface AuditEventInput {
  userId?: string | null;
  action: AuditAction;
  resource?: string | null;
  resourceId?: string | null;
  metadata?: Record<string, unknown> | null;
  ipAddress?: string | null;
  userAgent?: string | null;
}

const MAX_METADATA_SIZE = 10 * 1024;

function safeMetadata(metadata: Record<string, unknown> | null | undefined): Record<string, unknown> | null {
  if (!metadata) return null;
  const json = JSON.stringify(metadata);
  return json.length <= MAX_METADATA_SIZE ? metadata : { _truncated: true, _originalSize: json.length };
}

/**
 * Enregistre un événement d'audit. En cas d'échec de la base, l'événement est écrit dans
 * logs/audit-fallback-AAAA-MM-JJ.jsonl (même stratégie que le dépôt atomic).
 */
export async function logAuditEvent(input: AuditEventInput): Promise<void> {
  try {
    await getDrizzle().insert(auditLog).values({
      userId: input.userId ?? null,
      action: input.action,
      resource: input.resource ?? null,
      resourceId: input.resourceId ?? null,
      metadata: safeMetadata(input.metadata),
      ipAddress: input.ipAddress ?? null,
      userAgent: input.userAgent ?? null,
    });
  } catch (err) {
    console.error('[audit] Échec de l\'enregistrement :', err);
    const date = new Date().toISOString().slice(0, 10);
    const dir = join(process.cwd(), 'logs');
    const record = JSON.stringify({ timestamp: new Date().toISOString(), ...input });
    mkdir(dir, { recursive: true, mode: 0o700 })
      .then(() => appendFile(join(dir, `audit-fallback-${date}.jsonl`), record + '\n', { mode: 0o600 }))
      .catch((fileErr) => console.error('[audit] Repli sur fichier impossible :', fileErr));
  }
}

function isValidIp(ip: string): boolean {
  return isIP(ip) !== 0;
}

/** Adresse IP du client. Les en-têtes de proxy ne sont lus que si TRUST_PROXY=true. */
export function extractIp(headers: Headers, clientAddress?: string | null): string | null {
  if (process.env.TRUST_PROXY === 'true') {
    const forwarded = headers.get('x-forwarded-for')?.split(',')[0]?.trim();
    if (forwarded && isValidIp(forwarded)) return forwarded;
    const real = headers.get('x-real-ip');
    if (real && isValidIp(real)) return real;
  }
  return clientAddress && isValidIp(clientAddress) ? clientAddress : null;
}
