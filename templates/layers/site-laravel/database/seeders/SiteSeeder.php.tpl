<?php

namespace Database\Seeders;

use App\Models\Post;
use App\Models\Service;
use Illuminate\Database\Seeder;

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
class SiteSeeder extends Seeder
{
    public function run(): void
    {
        foreach ([
            [
                'locale' => 'fr',
                'slug' => 'creation-site-web',
                'title' => 'Création de site web',
                'summary' => 'Un site sur mesure, rapide et référencé, livré avec son espace d’administration.',
                'icon' => 'code',
                'price' => 'À partir de 2 400 €',
                'features' => [
                    'Maquettes et design system',
                    'Contenus éditables',
                    'SEO technique et données structurées',
                    'Mise en ligne et formation',
                ],
                'body' => '<p>Nous concevons votre site à partir de vos objectifs : qui doit le trouver, que doit-il y faire, comment le mesurer.</p>
<h2 id="notre-méthode">Notre méthode</h2>
<p>Ateliers de cadrage, maquettes, puis développement par itérations courtes. Chaque version est <strong>mise en ligne sur un environnement de recette</strong> pour que vous puissiez la tester.</p>
<h2 id="ce-qui-est-livré">Ce qui est livré</h2>
<ul>
<li>un site responsive, accessible et performant ;</li>
<li>un espace d’administration pour modifier vos contenus ;</li>
<li>la documentation et une formation d’une demi-journée.</li>
</ul>',
                'position' => 1,
            ],
            [
                'locale' => 'fr',
                'slug' => 'refonte-performance',
                'title' => 'Refonte et performance',
                'summary' => 'Un site existant plus rapide, plus clair et mieux classé, sans perdre votre référencement.',
                'icon' => 'zap',
                'price' => 'À partir de 1 900 €',
                'features' => [
                    'Audit Lighthouse et Core Web Vitals',
                    'Plan de redirections',
                    'Optimisation des images',
                    'Suivi après mise en ligne',
                ],
                'body' => '<p>Une refonte réussie commence par un audit : temps de chargement, parcours, contenus qui convertissent.</p>
<h2 id="garder-le-référencement">Garder le référencement</h2>
<p>Toutes les anciennes adresses sont <strong>redirigées</strong> et suivies pendant trois mois après la mise en ligne.</p>
<h2 id="mesurer-le-gain">Mesurer le gain</h2>
<ul>
<li>scores Lighthouse avant / après ;</li>
<li>Core Web Vitals sur les pages clés ;</li>
<li>taux de conversion des formulaires.</li>
</ul>',
                'position' => 2,
            ],
            [
                'locale' => 'fr',
                'slug' => 'accessibilite',
                'title' => 'Accessibilité (RGAA)',
                'summary' => 'Audit et mise en conformité RGAA / WCAG 2.2 AA, déclaration d’accessibilité comprise.',
                'icon' => 'accessibility',
                'price' => 'À partir de 1 200 €',
                'features' => [
                    'Audit sur échantillon de pages',
                    'Corrections priorisées',
                    'Déclaration d’accessibilité',
                    'Formation des rédacteurs',
                ],
                'body' => '<p>L’accessibilité profite à tous : navigation au clavier, contrastes, lecteurs d’écran, formulaires compréhensibles.</p>
<h2 id="audit">Audit</h2>
<p>Nous évaluons un échantillon représentatif de pages selon le <strong>RGAA 4.1</strong> et livrons un rapport priorisé.</p>
<h2 id="mise-en-conformité">Mise en conformité</h2>
<ul>
<li>corrections dans le code et les gabarits ;</li>
<li>contrôle des contrastes du thème ;</li>
<li>rédaction de la déclaration d’accessibilité.</li>
</ul>',
                'position' => 3,
            ],
            [
                'locale' => 'fr',
                'slug' => 'maintenance',
                'title' => 'Maintenance et hébergement',
                'summary' => 'Mises à jour, sauvegardes, surveillance et petites évolutions, sans surprise.',
                'icon' => 'shield',
                'price' => 'À partir de 90 € / mois',
                'features' => [
                    'Mises à jour de sécurité',
                    'Sauvegardes quotidiennes',
                    'Surveillance 24 h / 24',
                    'Une heure d’évolution par mois',
                ],
                'body' => '<p>Un site est un logiciel : il doit être mis à jour, sauvegardé et surveillé.</p>
<h2 id="ce-qui-est-compris">Ce qui est compris</h2>
<ul>
<li>mises à jour du framework et des dépendances ;</li>
<li>sauvegardes quotidiennes conservées 30 jours ;</li>
<li>surveillance de la disponibilité et des certificats.</li>
</ul>
<h2 id="évolutions">Évolutions</h2>
<p>Chaque mois, une heure d’évolution est incluse : nouvelle section, formulaire, page.</p>',
                'position' => 4,
            ],
            [
                'locale' => 'en',
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
                'locale' => 'en',
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
                'locale' => 'en',
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
                'locale' => 'en',
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
        ] as $s) {
            Service::updateOrCreate(['locale' => $s['locale'], 'slug' => $s['slug']], $s);
        }
        foreach ([
            [
                'locale' => 'fr',
                'slug' => 'choisir-son-framework',
                'title' => 'Astro, Laravel ou Wagtail : choisir son framework',
                'excerpt' => 'Trois excellents outils, trois façons de travailler. Nos critères pour choisir sans regret.',
                'category' => 'Technique',
                'author' => 'Camille Martin',
                'published_at' => '2026-10-02',
                'body' => '<p>Le bon framework est celui que votre équipe saura faire vivre. Voici nos critères.</p>
<h2 id="astro-pour-les-sites-de-contenu">Astro pour les sites de contenu</h2>
<p>Pages statiques par défaut, îlots interactifs à la demande : idéal pour un site vitrine ou un blog très rapide.</p>
<h2 id="laravel-pour-les-applications">Laravel pour les applications</h2>
<p>Authentification, files d’attente, e-mails, paiements : <strong>tout est prévu</strong> pour une application métier.</p>
<h2 id="wagtail-pour-les-rédactions">Wagtail pour les rédactions</h2>
<p>Un vrai CMS éditorial : arborescence de pages, StreamField, révisions et prévisualisation.</p>',
            ],
            [
                'locale' => 'fr',
                'slug' => 'accessibilite-checklist',
                'title' => 'L’accessibilité en 10 points',
                'excerpt' => 'Une liste courte et concrète pour éviter les erreurs les plus fréquentes.',
                'category' => 'Accessibilité',
                'author' => 'Inès Benali',
                'published_at' => '2026-09-15',
                'body' => '<p>Ces dix points couvrent la majorité des problèmes rencontrés en audit.</p>
<h2 id="structure">Structure</h2>
<ul>
<li>un seul <code>h1</code> par page et des titres hiérarchisés ;</li>
<li>des repères : <code>header</code>, <code>nav</code>, <code>main</code>, <code>footer</code> ;</li>
<li>un lien d’évitement vers le contenu.</li>
</ul>
<h2 id="interaction">Interaction</h2>
<ul>
<li>tout est utilisable <strong>au clavier</strong> ;</li>
<li>le focus est toujours visible ;</li>
<li>les champs de formulaire ont un libellé.</li>
</ul>',
            ],
            [
                'locale' => 'fr',
                'slug' => 'performance-images',
                'title' => 'Des images légères sans perte de qualité',
                'excerpt' => 'Formats modernes, dimensions explicites, chargement différé : le trio gagnant.',
                'category' => 'Performance',
                'author' => 'Camille Martin',
                'published_at' => '2026-08-28',
                'body' => '<p>Les images représentent souvent plus de la moitié du poids d’une page.</p>
<h2 id="les-bons-formats">Les bons formats</h2>
<p>AVIF et WebP divisent le poids par deux à cinq par rapport au JPEG, à qualité égale.</p>
<h2 id="les-bons-attributs">Les bons attributs</h2>
<ul>
<li><code>width</code> et <code>height</code> pour éviter les décalages de mise en page ;</li>
<li><code>loading="lazy"</code> sous la ligne de flottaison ;</li>
<li><code>srcset</code> pour servir la bonne taille.</li>
</ul>',
            ],
            [
                'locale' => 'en',
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
                'locale' => 'en',
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
                'locale' => 'en',
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
        ] as $p) {
            Post::updateOrCreate(['locale' => $p['locale'], 'slug' => $p['slug']], $p);
        }
    }
}
