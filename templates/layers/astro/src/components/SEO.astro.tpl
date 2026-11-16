@@meta when=!official@@
---
import { SITE_DESCRIPTION, SITE_TITLE } from '../consts';

interface Props {
  title: string;
  description?: string;
}

const { title, description = SITE_DESCRIPTION } = Astro.props;
const fullTitle = title === SITE_TITLE ? title : `${title} · ${SITE_TITLE}`;
const canonical = Astro.site ? new URL(Astro.url.pathname, Astro.site).href : undefined;
---

<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>{fullTitle}</title>
<meta name="description" content={description} />
{canonical && <link rel="canonical" href={canonical} />}
<link rel="icon" type="image/svg+xml" href="/favicon.svg" />
<meta property="og:type" content="website" />
<meta property="og:title" content={fullTitle} />
<meta property="og:description" content={description} />
{canonical && <meta property="og:url" content={canonical} />}
<meta name="generator" content={Astro.generator} />
