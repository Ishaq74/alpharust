@@meta when=!official@@
/* Variables de thème : clair par défaut, sombre selon le système ou le choix de l'utilisateur. */
:root {
  color-scheme: light dark;

  --font-sans: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-mono: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;

  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  --space-6: 1.5rem;
  --space-8: 2rem;
  --space-12: 3rem;

  --radius: 0.75rem;
  --measure: 68ch;
  --page-width: 64rem;

  --bg: #ffffff;
  --surface: #f5f6fa;
  --text: #151821;
  --text-muted: #4a5163;
  --border: #dde0ea;
  --accent: #3b4cca;
  --accent-contrast: #ffffff;
  --focus: #ff9f1a;
}

@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    --bg: #0f1117;
    --surface: #181b25;
    --text: #eceef5;
    --text-muted: #a9b0c4;
    --border: #2a2f3f;
    --accent: #8fa0ff;
    --accent-contrast: #0f1117;
  }
}

:root[data-theme="dark"] {
  --bg: #0f1117;
  --surface: #181b25;
  --text: #eceef5;
  --text-muted: #a9b0c4;
  --border: #2a2f3f;
  --accent: #8fa0ff;
  --accent-contrast: #0f1117;
}
