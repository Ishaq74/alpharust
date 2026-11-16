@@meta when=auth@@
---
interface Props {
  message?: string | undefined;
}
const { message } = Astro.props;
@@if starwind@@
import { Alert } from '@/components/starwind/alert';
@@end@@
@@if concordia@@
import Alert from '@components/ui/Alert.astro';
@@end@@
---

{
  message && (
@@if starwind@@
    <Alert variant="error" class="notice">{message}</Alert>
@@else@@
@@if concordia@@
    <div class="notice"><Alert status="error" message={message} /></div>
@@else@@
    <p class="notice" role="alert">{message}</p>
@@end@@
@@end@@
  )
}

<style>
  .notice {
    margin: 0 0 var(--space-4);
@@if !starwind&!concordia@@
    padding: var(--space-3) var(--space-4);
    border-radius: 0.5rem;
    border: 1px solid #c0392b;
    background: color-mix(in srgb, #c0392b 12%, transparent);
    color: var(--text);
@@end@@
  }
</style>
