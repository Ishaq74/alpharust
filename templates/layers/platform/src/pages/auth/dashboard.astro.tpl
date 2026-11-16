@@meta when=auth@@
---
export const prerender = false;
import BaseLayout from '@layouts/BaseLayout.astro';
import Dashboard from '@components/auth/Dashboard.astro';
import { handleSignOut } from '@lib/auth-forms';
import { requireAuth } from '@lib/auth-guards';
import { getMessages } from '@i18n/utils';

const signedOut = await handleSignOut(Astro);
if (signedOut) return signedOut;
const guard = requireAuth(Astro);
if ('redirect' in guard) return guard.redirect;
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.auth.dashboard.title}>
  <Dashboard />
</BaseLayout>
