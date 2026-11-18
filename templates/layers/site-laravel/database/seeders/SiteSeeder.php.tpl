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
        ] as $s) {
            Service::updateOrCreate(['slug' => $s['slug']], $s);
        }
        foreach ([
            [
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
        ] as $p) {
            Post::updateOrCreate(['slug' => $p['slug']], $p);
        }
    }
}
