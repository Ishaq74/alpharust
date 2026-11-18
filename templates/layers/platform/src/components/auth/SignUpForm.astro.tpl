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

<AuthShell title={t.auth.signUp.title}>
  <Notice message={error} />
  <form method="post">
    <Field name="name" label={t.auth.fields.name} value={values['name'] ?? ''} autocomplete="name" />
@@if plugin_username@@
    <Field name="username" label={t.auth.fields.username} value={values['username'] ?? ''} autocomplete="username" minlength={3} />
@@end@@
    <Field name="email" type="email" label={t.auth.fields.email} value={values['email'] ?? ''} autocomplete="email" />
    <Field name="password" type="password" label={t.auth.fields.password} autocomplete="new-password" minlength={8} />
    <SubmitButton label={t.auth.signUp.submit} />
  </form>
  <p class="alt">
    {t.auth.signUp.haveAccount}{' '}<a href={localizedPath(locale, '/auth/sign-in')}>{t.auth.signUp.link}</a>
  </p>
</AuthShell>

<style>
  .alt {
    margin: var(--space-4) 0 0;
    text-align: center;
    font-size: 0.95rem;
  }
</style>
