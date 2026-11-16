@@meta when=auth@@
---
interface Props {
  label: string;
}
const { label } = Astro.props;
@@if starwind@@
import { Button } from '@/components/starwind/button';
@@end@@
@@if concordia@@
import Button from '@components/ui/Button.astro';
@@end@@
---

@@if starwind@@
<Button type="submit" variant="primary" class="submit">{label}</Button>
@@else@@
@@if concordia@@
<Button type="submit" color="primary" className="submit">{label}</Button>
@@else@@
<button type="submit" class="submit">{label}</button>
@@end@@
@@end@@

<style>
  .submit {
    width: 100%;
  }
@@if !starwind&!concordia@@
  .submit {
    font: inherit;
    font-weight: 600;
    padding: 0.65rem 1rem;
    border: 0;
    border-radius: 0.5rem;
    background: var(--accent);
    color: var(--accent-contrast);
    cursor: pointer;
  }
  .submit:hover {
    filter: brightness(1.1);
  }
@@end@@
</style>
