@@meta when=auth&i18n@@
---
export const prerender = false;
import BaseLayout from '@layouts/BaseLayout.astro';
import SignUpForm from '@components/auth/SignUpForm.astro';
import { handleSignUp } from '@lib/auth-forms';
import { getMessages } from '@i18n/utils';
import { isLocale, DEFAULT_LOCALE } from '@i18n/config';

// Seules les langues non par défaut sont préfixées (prefixDefaultLocale: false).
if (!isLocale(Astro.params.lang) || Astro.params.lang === DEFAULT_LOCALE) return new Response(null, { status: 404 });
const result = await handleSignUp(Astro);
if (result.response) return result.response;
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.auth.signUp.title}>
  <SignUpForm error={result.error} values={result.values} />
</BaseLayout>
