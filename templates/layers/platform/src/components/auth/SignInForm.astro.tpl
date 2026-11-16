@@meta when=auth@@
---
import AuthShell from './AuthShell.astro';
import Field from './Field.astro';
import Notice from './Notice.astro';
import SubmitButton from './SubmitButton.astro';
import { DEFAULT_LOCALE, localizedPath } from '@i18n/config';
import { getMessages } from '@i18n/utils';

interface Props {
  error?: string | undefined;
  values?: Record<string, string>;
}
const { error, values = {} } = Astro.props;
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(locale);
---

<AuthShell title={t.auth.signIn.title}>
  <Notice message={error} />
  <form method="post" novalidate={false}>
    <Field name="email" type="email" label={t.auth.fields.email} value={values['email'] ?? ''} autocomplete="email" />
    <Field name="password" type="password" label={t.auth.fields.password} autocomplete="current-password" />
    <SubmitButton label={t.auth.signIn.submit} />
  </form>
  <p class="alt">
    {t.auth.signIn.noAccount}{' '}<a href={localizedPath(locale, '/auth/sign-up')}>{t.auth.signIn.link}</a>
  </p>
</AuthShell>

<style>
  .alt {
    margin: var(--space-4) 0 0;
    text-align: center;
    font-size: 0.95rem;
  }
</style>
