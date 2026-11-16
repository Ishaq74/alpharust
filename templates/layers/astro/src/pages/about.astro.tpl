@@meta when=!official@@
---
import BaseLayout from '../layouts/BaseLayout.astro';
---

<BaseLayout title="À propos" description="Qui sommes-nous et comment ce site est construit.">
  <h1>À propos</h1>
  <p>
    Ce site est construit avec <a href="https://astro.build">Astro</a>. Le contenu vit dans
    <code>src/content/</code> et chaque page respecte les règles d'accessibilité de base :
    hiérarchie de titres, contrastes, navigation au clavier.
  </p>
</BaseLayout>
