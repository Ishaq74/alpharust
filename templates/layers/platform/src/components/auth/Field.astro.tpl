@@meta when=auth@@
---
interface Props {
  name: string;
  label: string;
  type?: 'text' | 'email' | 'password';
  value?: string;
  autocomplete?: string;
  required?: boolean;
  minlength?: number;
}
const { name, label, type = 'text', value = '', autocomplete, required = true@@if !concordia@@, minlength@@end@@ } = Astro.props;
@@if starwind@@
import { Input } from '@/components/starwind/input';
import { Label } from '@/components/starwind/label';
@@end@@
@@if concordia@@
import FormGroup from '@components/ui/Form/FormGroup.astro';
import Input from '@components/ui/Form/Input.astro';
import Label from '@components/ui/Form/Label.astro';
@@end@@
const id = `field-${name}`;
---

<div class="field">
@@if starwind@@
  <Label for={id}>{label}</Label>
  <Input id={id} name={name} type={type} value={value} autocomplete={autocomplete} required={required} minlength={minlength} />
@@else@@
@@if concordia@@
  <FormGroup>
    <Label htmlFor={id}>{label}</Label>
    <Input id={id} name={name} type={type} value={value} autocomplete={autocomplete} required={required} />
  </FormGroup>
@@else@@
  <label for={id}>{label}</label>
  <input id={id} name={name} type={type} value={value} autocomplete={autocomplete} required={required} minlength={minlength} />
@@end@@
@@end@@
</div>

<style>
  .field {
    display: grid;
    gap: var(--space-1);
    margin-bottom: var(--space-4);
  }
@@if !starwind&!concordia@@
  label {
    font-weight: 600;
  }
  input {
    font: inherit;
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    background: var(--bg);
    color: var(--text);
  }
@@end@@
</style>
