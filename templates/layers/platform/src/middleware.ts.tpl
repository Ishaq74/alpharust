@@meta when=auth@@
import { defineMiddleware } from 'astro:middleware';
import { auth } from '@lib/auth';

const SECURITY_HEADERS: Record<string, string> = {
  'X-Content-Type-Options': 'nosniff',
  'X-Frame-Options': 'DENY',
  'Referrer-Policy': 'strict-origin-when-cross-origin',
  'Permissions-Policy': 'camera=(), microphone=(), geolocation=()',
  'Cross-Origin-Opener-Policy': 'same-origin',
};

export const onRequest = defineMiddleware(async (context, next) => {
  context.locals.user = null;
  context.locals.session = null;

  // Les pages prérendues (blog, à propos…) n'ont ni en-têtes ni session au build : on ne les interroge pas.
  if (!context.isPrerendered) {
    try {
      const result = await auth.api.getSession({ headers: context.request.headers });
      if (result) {
        context.locals.user = result.user;
        context.locals.session = result.session;
      }
    } catch (err) {
      console.error('[middleware] Lecture de la session impossible :', err);
    }
  }

  const response = await next();
  for (const [key, value] of Object.entries(SECURITY_HEADERS)) response.headers.set(key, value);
  return response;
});
