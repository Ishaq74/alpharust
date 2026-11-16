@@meta when=auth@@
---
export const prerender = false;
import BaseLayout from '@layouts/BaseLayout.astro';
import SignInForm from '@components/auth/SignInForm.astro';
import { handleSignIn } from '@lib/auth-forms';
import { getMessages } from '@i18n/utils';

const result = await handleSignIn(Astro);
if (result.response) return result.response;
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.auth.signIn.title}>
  <SignInForm error={result.error} values={result.values} />
</BaseLayout>
