@@meta when=!official@@
<button type="button" class="theme-toggle" id="theme-toggle" aria-label="Changer de thème (clair / sombre)">
  <span aria-hidden="true">◐</span>
</button>

<script>
  const button = document.getElementById('theme-toggle');
  const root = document.documentElement;

  const current = (): 'light' | 'dark' =>
    root.dataset['theme'] === 'dark' ||
    (!root.dataset['theme'] && window.matchMedia('(prefers-color-scheme: dark)').matches)
      ? 'dark'
      : 'light';

  button?.addEventListener('click', () => {
    const next = current() === 'dark' ? 'light' : 'dark';
    root.dataset['theme'] = next;
@@if starwind@@
    root.classList.toggle('dark', next === 'dark');
@@end@@
    try {
      localStorage.setItem('theme', next);
    } catch {}
  });
</script>

<style>
  .theme-toggle {
    background: transparent;
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 999px;
    width: 2.5rem;
    height: 2.5rem;
    font-size: 1.1rem;
    cursor: pointer;
  }
  .theme-toggle:hover {
    background: var(--surface);
  }
</style>
