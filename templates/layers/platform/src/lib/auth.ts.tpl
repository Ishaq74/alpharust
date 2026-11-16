@@meta when=auth@@
import { betterAuth } from 'better-auth';
import { createAuthMiddleware } from 'better-auth/api';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
@@if plugin_username|plugin_admin|plugin_organization@@
import {
@@if plugin_username@@
  username,
@@end@@
@@if plugin_admin@@
  admin,
@@end@@
@@if plugin_organization@@
  organization,
@@end@@
} from 'better-auth/plugins';
@@end@@
import { getLazyDrizzle, schema } from '@database/drizzle';
import { logAuditEvent, extractIp, type AuditAction } from '@lib/audit';

/** Chemins Better Auth journalisés → action d'audit. */
const AUDITED: Record<string, { action: AuditAction; resource: string }> = {
  '/sign-in/email': { action: 'SIGN_IN', resource: 'session' },
  '/sign-up/email': { action: 'SIGN_UP', resource: 'user' },
  '/sign-out': { action: 'SIGN_OUT', resource: 'session' },
  '/change-password': { action: 'PASSWORD_CHANGE', resource: 'user' },
  '/update-user': { action: 'USER_UPDATE', resource: 'user' },
  '/delete-user': { action: 'USER_DELETE', resource: 'user' },
@@if plugin_admin@@
  '/admin/ban-user': { action: 'USER_BAN', resource: 'user' },
  '/admin/unban-user': { action: 'USER_UNBAN', resource: 'user' },
  '/admin/set-role': { action: 'USER_ROLE_CHANGE', resource: 'user' },
  '/admin/impersonate-user': { action: 'IMPERSONATION_START', resource: 'session' },
  '/admin/stop-impersonating': { action: 'IMPERSONATION_STOP', resource: 'session' },
  '/admin/remove-user': { action: 'USER_DELETE', resource: 'user' },
@@end@@
@@if plugin_organization@@
  '/organization/create': { action: 'ORG_CREATE', resource: 'organization' },
  '/organization/update': { action: 'ORG_UPDATE', resource: 'organization' },
  '/organization/delete': { action: 'ORG_DELETE', resource: 'organization' },
  '/organization/invite-member': { action: 'ORG_INVITATION_SEND', resource: 'invitation' },
  '/organization/accept-invitation': { action: 'ORG_INVITATION_ACCEPT', resource: 'invitation' },
  '/organization/reject-invitation': { action: 'ORG_INVITATION_REJECT', resource: 'invitation' },
  '/organization/cancel-invitation': { action: 'ORG_INVITATION_CANCEL', resource: 'invitation' },
  '/organization/remove-member': { action: 'ORG_MEMBER_REMOVE', resource: 'member' },
  '/organization/update-member-role': { action: 'ORG_MEMBER_ROLE_CHANGE', resource: 'member' },
@@end@@
};

/** Champs sans donnée personnelle conservés dans les métadonnées d'audit. */
const SAFE_FIELDS = new Set(['userId', 'organizationId', 'memberId', 'invitationId', 'role', 'slug']);

export const auth = betterAuth({
  database: drizzleAdapter(getLazyDrizzle(), { provider: '@@dialect@@', schema }),
  baseURL: process.env.BETTER_AUTH_URL,
  secret: process.env.BETTER_AUTH_SECRET,
  rateLimit: {
    enabled: true,
    window: 60,
    max: 100,
    customRules: {
      '/sign-in/email': { window: 10, max: 5 },
      '/sign-up/email': { window: 60, max: 3 },
      '/change-password': { window: 60, max: 5 },
@@if plugin_organization@@
      '/organization/create': { window: 60, max: 5 },
@@end@@
    },
  },
  emailAndPassword: {
    enabled: true,
    minPasswordLength: 8,
    // Mettre AUTH_REQUIRE_EMAIL_VERIFICATION=true une fois l'envoi d'e-mails configuré.
    requireEmailVerification: process.env.AUTH_REQUIRE_EMAIL_VERIFICATION === 'true',
  },
@@if plugin_admin@@
  databaseHooks: {
    user: {
      create: {
        // L'adresse ADMIN_EMAIL reçoit le rôle admin à l'inscription.
        before: async (user) => {
          const adminEmail = process.env.ADMIN_EMAIL?.trim().toLowerCase();
          if (adminEmail && user.email.toLowerCase() === adminEmail) {
            return { data: { ...user, role: 'admin' } };
          }
          return { data: user };
        },
      },
    },
  },
@@end@@
  hooks: {
    after: createAuthMiddleware(async (ctx) => {
      const mapping = AUDITED[ctx.path];
      if (!mapping) return;

      const returned = ctx.context.returned;
      const failed =
        returned instanceof Error ||
        (!!returned && typeof returned === 'object' && 'status' in returned && Number((returned as { status: number }).status) >= 400);
      const ip = ctx.headers ? extractIp(ctx.headers) : null;
      const userAgent = ctx.headers?.get('user-agent') ?? null;

      if (failed) {
        if (['/sign-in/email', '/sign-up/email', '/change-password'].includes(ctx.path)) {
          ctx.context.runInBackground(
            logAuditEvent({
              action: `${mapping.action}_FAILED` as AuditAction,
              resource: mapping.resource,
              metadata: { path: ctx.path },
              ipAddress: ip,
              userAgent,
            }),
          );
        }
        return;
      }

      const metadata: Record<string, string> = {};
      if (ctx.body && typeof ctx.body === 'object') {
        for (const [key, value] of Object.entries(ctx.body as Record<string, unknown>)) {
          if (SAFE_FIELDS.has(key) && typeof value === 'string' && value.length <= 255) metadata[key] = value;
        }
      }
      ctx.context.runInBackground(
        logAuditEvent({
          userId: ctx.context.newSession?.user?.id ?? ctx.context.session?.user?.id ?? null,
          action: mapping.action,
          resource: mapping.resource,
          metadata: Object.keys(metadata).length > 0 ? metadata : null,
          ipAddress: ip,
          userAgent,
        }),
      );
    }),
  },
@@if plugin_username|plugin_admin|plugin_organization@@
  plugins: [
@@if plugin_username@@
    username(),
@@end@@
@@if plugin_admin@@
    admin(),
@@end@@
@@if plugin_organization@@
    organization({
      // Sans service d'e-mail, l'invitation est seulement journalisée (à remplacer par votre envoi).
      async sendInvitationEmail(data) {
        console.info(`[auth] Invitation ${data.id} pour ${data.email} dans ${data.organization.name}`);
      },
    }),
@@end@@
  ],
@@end@@
});
