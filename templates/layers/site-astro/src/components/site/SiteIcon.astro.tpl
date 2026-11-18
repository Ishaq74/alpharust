@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// Icônes du site : src/data/icons.json (MDI par défaut, drapeaux Circle Flags ; `--icons <jeu>` le réécrit).
// Même fichier et même balisage qu’en Blade (<x-site.icon>) et Django ({% site_icon %}).
import data from '../../data/icons.json';

interface Props {
  name: string;
  size?: number;
}

const { name, size = 20 } = Astro.props;
const icon = (data.icons as Record<string, { body: string; width: number; height: number }>)[name];
---

{
  icon && (
    <svg width={size} height={size} viewBox={`0 0 ${icon.width} ${icon.height}`} fill="currentColor" aria-hidden="true" class="site-icon" set:html={icon.body} />
  )
}
