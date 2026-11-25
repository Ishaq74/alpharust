@@meta when=lang_en@@
<?php

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
// Services et articles en « en » (semés par database/seeders/SiteSeeder.php).

return [
    'services' => [
        [
            'slug' => 'creation-site-web',
            'title' => 'Website creation',
            'summary' => 'A custom website, fast and well ranked, delivered with its admin area.',
            'icon' => 'code',
            'price' => 'From €2,400',
            'features' => [
                'Mockups and design system',
                'Editable content',
                'Technical SEO and structured data',
                'Launch and training',
            ],
            'body' => '<p>We design your website around your goals: who needs to find it, what they should do there, how to measure it.</p>
<h2 id="our-method">Our method</h2>
<p>Scoping workshops, mockups, then development in short iterations. Each version is <strong>deployed to a staging environment</strong> so you can test it.</p>
<h2 id="what-is-delivered">What is delivered</h2>
<ul>
<li>a responsive, accessible and fast website;</li>
<li>an admin area to edit your content;</li>
<li>documentation and a half-day training session.</li>
</ul>',
            'position' => 1,
        ],
        [
            'slug' => 'refonte-performance',
            'title' => 'Redesign and performance',
            'summary' => 'An existing website made faster, clearer and better ranked, without losing your SEO.',
            'icon' => 'zap',
            'price' => 'From €1,900',
            'features' => [
                'Lighthouse and Core Web Vitals audit',
                'Redirect plan',
                'Image optimization',
                'Post-launch monitoring',
            ],
            'body' => '<p>A successful redesign starts with an audit: load times, user journeys, content that converts.</p>
<h2 id="keeping-your-rankings">Keeping your rankings</h2>
<p>Every old address is <strong>redirected</strong> and monitored for three months after launch.</p>
<h2 id="measuring-the-gain">Measuring the gain</h2>
<ul>
<li>Lighthouse scores before / after;</li>
<li>Core Web Vitals on key pages;</li>
<li>form conversion rates.</li>
</ul>',
            'position' => 2,
        ],
        [
            'slug' => 'accessibilite',
            'title' => 'Accessibility (RGAA)',
            'summary' => 'RGAA / WCAG 2.2 AA audit and remediation, accessibility statement included.',
            'icon' => 'accessibility',
            'price' => 'From €1,200',
            'features' => [
                'Audit on a sample of pages',
                'Prioritized fixes',
                'Accessibility statement',
                'Editor training',
            ],
            'body' => '<p>Accessibility benefits everyone: keyboard navigation, contrast, screen readers, understandable forms.</p>
<h2 id="audit">Audit</h2>
<p>We assess a representative sample of pages against <strong>RGAA 4.1</strong> and deliver a prioritized report.</p>
<h2 id="remediation">Remediation</h2>
<ul>
<li>fixes in the code and templates;</li>
<li>theme contrast checks;</li>
<li>writing the accessibility statement.</li>
</ul>',
            'position' => 3,
        ],
        [
            'slug' => 'maintenance',
            'title' => 'Maintenance and hosting',
            'summary' => 'Updates, backups, monitoring and small improvements, with no surprises.',
            'icon' => 'shield',
            'price' => 'From €90 / month',
            'features' => [
                'Security updates',
                'Daily backups',
                '24/7 monitoring',
                'One hour of improvements per month',
            ],
            'body' => '<p>A website is software: it must be updated, backed up and monitored.</p>
<h2 id="what-is-included">What is included</h2>
<ul>
<li>framework and dependency updates;</li>
<li>daily backups kept for 30 days;</li>
<li>uptime and certificate monitoring.</li>
</ul>
<h2 id="improvements">Improvements</h2>
<p>Every month, one hour of improvements is included: a new section, form or page.</p>',
            'position' => 4,
        ],
    ],
    'posts' => [
        [
            'slug' => 'choisir-son-framework',
            'title' => 'Astro, Laravel or Wagtail: choosing your framework',
            'excerpt' => 'Three excellent tools, three ways of working. Our criteria for choosing without regret.',
            'category' => 'Technical',
            'author' => 'Camille Martin',
            'published_at' => '2026-10-02',
            'body' => '<p>The right framework is the one your team can keep alive. Here are our criteria.</p>
<h2 id="astro-for-content-sites">Astro for content sites</h2>
<p>Static pages by default, interactive islands on demand: ideal for a very fast showcase site or blog.</p>
<h2 id="laravel-for-applications">Laravel for applications</h2>
<p>Authentication, queues, e-mails, payments: <strong>everything is built in</strong> for a business application.</p>
<h2 id="wagtail-for-editorial-teams">Wagtail for editorial teams</h2>
<p>A real editorial CMS: page tree, StreamField, revisions and previews.</p>',
        ],
        [
            'slug' => 'accessibilite-checklist',
            'title' => 'Accessibility in 10 points',
            'excerpt' => 'A short, concrete list to avoid the most common mistakes.',
            'category' => 'Accessibility',
            'author' => 'Inès Benali',
            'published_at' => '2026-09-15',
            'body' => '<p>These ten points cover most of the issues found in audits.</p>
<h2 id="structure">Structure</h2>
<ul>
<li>a single <code>h1</code> per page and hierarchical headings;</li>
<li>landmarks: <code>header</code>, <code>nav</code>, <code>main</code>, <code>footer</code>;</li>
<li>a skip link to the content.</li>
</ul>
<h2 id="interaction">Interaction</h2>
<ul>
<li>everything is usable <strong>with the keyboard</strong>;</li>
<li>focus is always visible;</li>
<li>form fields have a label.</li>
</ul>',
        ],
        [
            'slug' => 'performance-images',
            'title' => 'Lightweight images without quality loss',
            'excerpt' => 'Modern formats, explicit dimensions, lazy loading: the winning trio.',
            'category' => 'Performance',
            'author' => 'Camille Martin',
            'published_at' => '2026-08-28',
            'body' => '<p>Images often account for more than half of a page’s weight.</p>
<h2 id="the-right-formats">The right formats</h2>
<p>AVIF and WebP cut the weight by two to five compared with JPEG, at equal quality.</p>
<h2 id="the-right-attributes">The right attributes</h2>
<ul>
<li><code>width</code> and <code>height</code> to avoid layout shifts;</li>
<li><code>loading="lazy"</code> below the fold;</li>
<li><code>srcset</code> to serve the right size.</li>
</ul>',
        ],
    ],
];
