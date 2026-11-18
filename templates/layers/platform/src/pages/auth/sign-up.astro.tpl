@@meta when=auth@@
---
export const prerender = false;
import BaseLayout from '@layouts/BaseLayout.astro';
import SignUpForm from '@components/auth/SignUpForm.astro';
import { handleSignUp } from '@lib/auth-forms';
import { getMessages } from '@i18n/utils';

const result = await handleSignUp(Astro);
if (result.response) return result.response;
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.auth.signUp.title}>
  <SignUpForm error={result.error} values={result.values} />
</BaseLayout>
