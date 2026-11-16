@@meta when=concordia&theme@@
// Contrôle d'accessibilité WCAG du thème : `npm run theme:check`.
// Même méthode que le dépôt themecsutomizer (luminance relative, ratio de contraste, AA ≥ 4,5 / AAA ≥ 7).
import { readFileSync } from 'node:fs';

const css = readFileSync(new URL('../src/styles/theme.css', import.meta.url), 'utf8');

function channel(v) {
  const s = v / 255;
  return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}
function luminance(hex) {
  const n = parseInt(hex.slice(1), 16);
  return 0.2126 * channel((n >> 16) & 255) + 0.7152 * channel((n >> 8) & 255) + 0.0722 * channel(n & 255);
}
function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

const blocks = css.split(/}\s*/).filter(Boolean);
let failed = false;
for (const block of blocks) {
  const mode = block.includes("data-theme='dark'") ? 'sombre' : 'clair';
  const read = (prefix) =>
    Object.fromEntries([...block.matchAll(new RegExp(`--${prefix}-(\\w+):\\s*(#[0-9a-fA-F]{6})`, 'g'))].map((m) => [m[1], m[2]]));
  const colors = read('color');
  const on = read('on');
  for (const name of Object.keys(colors)) {
    if (!colors[name] || !on[name]) continue;
    const ratio = contrast(colors[name], on[name]);
    const level = ratio >= 7 ? 'AAA' : ratio >= 4.5 ? 'AA' : 'insuffisant';
    console.log(`${mode.padEnd(6)} ${name.padEnd(10)} ${colors[name]} / ${on[name]}  ${ratio.toFixed(2)}:1  ${level}`);
    if (ratio < 4.5) failed = true;
  }
}
if (failed) {
  console.error('Contraste insuffisant (< 4,5:1) : ajustez les couleurs du thème.');
  process.exit(1);
}
