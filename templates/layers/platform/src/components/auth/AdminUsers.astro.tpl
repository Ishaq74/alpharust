@@meta when=plugin_admin@@
---
import { DEFAULT_LOCALE } from '@i18n/config';
import { getMessages } from '@i18n/utils';

interface UserRow {
  id: string;
  name: string;
  email: string;
  role?: string | null | undefined;
  createdAt: Date;
}
interface Props {
  users: UserRow[];
}
const { users } = Astro.props;
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(locale);
const date = new Intl.DateTimeFormat(locale, { dateStyle: 'medium' });
---

<h1>{t.auth.admin.title}</h1>
<h2>{t.auth.admin.users} ({users.length})</h2>
<div class="scroll">
  <table>
    <thead>
      <tr>
        <th scope="col">{t.auth.fields.name}</th>
        <th scope="col">{t.auth.admin.email}</th>
        <th scope="col">{t.auth.admin.role}</th>
        <th scope="col">{t.auth.admin.created}</th>
      </tr>
    </thead>
    <tbody>
      {
        users.map((u) => (
          <tr>
            <td>{u.name}</td>
            <td>{u.email}</td>
            <td>{u.role ?? t.auth.dashboard.defaultRole}</td>
            <td>{date.format(u.createdAt)}</td>
          </tr>
        ))
      }
    </tbody>
  </table>
</div>

<style>
  .scroll {
    overflow-x: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th,
  td {
    text-align: start;
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--border);
  }
</style>
