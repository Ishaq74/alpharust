@@meta when=plugin_admin&i18n@@
---
export const prerender = false;
import BaseLayout from '@layouts/BaseLayout.astro';
import AdminUsers from '@components/auth/AdminUsers.astro';
import { auth } from '@lib/auth';
import { requireAdmin } from '@lib/auth-guards';
import { getMessages } from '@i18n/utils';
import { isLocale, DEFAULT_LOCALE } from '@i18n/config';

if (!isLocale(Astro.params.lang) || Astro.params.lang === DEFAULT_LOCALE) return new Response(null, { status: 404 });
const guard = requireAdmin(Astro);
if ('redirect' in guard) return guard.redirect;
const { users } = await auth.api.listUsers({
  query: { limit: 50, sortBy: 'createdAt', sortDirection: 'desc' },
  headers: Astro.request.headers,
});
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.auth.admin.title}>
  <AdminUsers users={users} />
</BaseLayout>
