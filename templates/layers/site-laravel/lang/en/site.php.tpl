<?php

// Généré par tools/dsgen/vitrine (spec.py) : même contenu qu’Astro et Wagtail.
// Textes du site en « en » : lus par App\Support\SiteData via trans('site').

return [
    'name' => '@@title@@',
    'tagline' => 'Fast, accessible websites that are easy to grow.',
    'description' => 'Independent web agency: website creation, redesign, accessibility and maintenance.',
    'cta' => [
        'label' => 'Request a quote',
        'url' => '/contact',
    ],
    'contact' => [
        'email' => 'contact@exemple.fr',
        'phone' => '+33 1 23 45 67 89',
        'address' => '12 rue de la Paix, 75002 Paris',
        'hours' => 'Monday to Friday, 9 am – 6 pm',
    ],
    'social' => [
        [
            'label' => 'GitHub',
            'url' => 'https://github.com/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'github',
        ],
        [
            'label' => 'LinkedIn',
            'url' => 'https://www.linkedin.com/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'linkedin',
        ],
        [
            'label' => 'Mastodon',
            'url' => 'https://mastodon.social/', // alpharust-ignore: no-hardcoded-urls
            'icon' => 'mastodon',
        ],
    ],
    'nav' => [
        [
            'label' => 'Home',
            'url' => '/',
        ],
        [
            'label' => 'Services',
            'url' => '/services',
            'children' => 'services',
        ],
        [
            'label' => 'Blog',
            'url' => '/blog',
        ],
        [
            'label' => 'About',
            'url' => '/a-propos',
        ],
        [
            'label' => 'Contact',
            'url' => '/contact',
        ],
    ],
    'footer' => [
        [
            'title' => 'Navigation',
            'links' => [
                [
                    'label' => 'Home',
                    'url' => '/',
                ],
                [
                    'label' => 'Blog',
                    'url' => '/blog',
                ],
                [
                    'label' => 'About',
                    'url' => '/a-propos',
                ],
                [
                    'label' => 'Contact',
                    'url' => '/contact',
                ],
            ],
        ],
        [
            'title' => 'Services',
            'links' => 'services',
        ],
        [
            'title' => 'Information',
            'links' => [
                [
                    'label' => 'Legal notice',
                    'url' => '/mentions-legales',
                ],
                [
                    'label' => 'Privacy policy',
                    'url' => '/confidentialite',
                ],
            ],
        ],
    ],
    'copyright' => 'All rights reserved.',
    'pages' => [
        'home' => [
            'url' => '/',
            'title' => 'Home',
            'seo_title' => null,
            'description' => 'Independent web agency: website creation, redesign, accessibility and maintenance.',
            'sections' => [
                [
                    'type' => 'hero',
                    'badge' => 'Independent web agency',
                    'title' => 'Fast, accessible websites that are easy to grow',
                    'lead' => 'We design and maintain websites that load in under a second, meet the RGAA and stay easy to edit.',
                    'primary' => [
                        'label' => 'Request a quote',
                        'url' => '/contact',
                    ],
                    'secondary' => [
                        'label' => 'See our services',
                        'url' => '/services',
                    ],
                ],
                [
                    'type' => 'stats',
                    'items' => [
                        [
                            'value' => '98/100',
                            'label' => 'Average Lighthouse score',
                        ],
                        [
                            'value' => 'AA',
                            'label' => 'Target RGAA level',
                        ],
                        [
                            'value' => '< 1 s',
                            'label' => 'Content display',
                        ],
                        [
                            'value' => '12 years',
                            'label' => 'Of experience',
                        ],
                    ],
                ],
                [
                    'type' => 'features',
                    'title' => 'Why choose us',
                    'lead' => 'Four commitments, on every project.',
                    'items' => [
                        [
                            'icon' => 'zap',
                            'title' => 'Performance',
                            'text' => 'Light pages, optimized images, scores measured at every delivery.',
                        ],
                        [
                            'icon' => 'accessibility',
                            'title' => 'Accessibility',
                            'text' => 'Keyboard navigation, checked contrast, careful semantics.',
                        ],
                        [
                            'icon' => 'pencil',
                            'title' => 'Autonomy',
                            'text' => 'You edit your content without calling us.',
                        ],
                        [
                            'icon' => 'shield',
                            'title' => 'Peace of mind',
                            'text' => 'Updates, backups and monitoring included.',
                        ],
                    ],
                ],
                [
                    'type' => 'services',
                    'title' => 'Our services',
                    'lead' => 'From creation to maintenance.',
                    'limit' => 3,
                    'link' => [
                        'label' => 'All services',
                        'url' => '/services',
                    ],
                ],
                [
                    'type' => 'testimonials',
                    'title' => 'They trust us',
                    'items' => [
                        [
                            'quote' => 'Our new website is twice as fast and our quote requests have grown by 40%.',
                            'name' => 'Sophie Laurent',
                            'role' => 'Director, Atelier Laurent',
                            'initials' => 'SL',
                        ],
                        [
                            'quote' => 'A clear accessibility audit, fixes delivered in three weeks.',
                            'name' => 'Marc Petit',
                            'role' => 'Digital manager, City of Montreuil',
                            'initials' => 'MP',
                        ],
                        [
                            'quote' => 'We publish our news in five minutes, without help.',
                            'name' => 'Léa Moreau',
                            'role' => 'Communications officer, Asso Horizon',
                            'initials' => 'LM',
                        ],
                    ],
                ],
                [
                    'type' => 'posts',
                    'title' => 'Latest articles',
                    'lead' => 'Advice and lessons learned.',
                    'limit' => 3,
                    'link' => [
                        'label' => 'The whole blog',
                        'url' => '/blog',
                    ],
                ],
                [
                    'type' => 'faq',
                    'title' => 'Frequently asked questions',
                    'items' => [
                        [
                            'question' => 'How long does it take to build a website?',
                            'answer' => 'Between four and eight weeks depending on the number of pages and features.',
                        ],
                        [
                            'question' => 'Will I be able to edit my website myself?',
                            'answer' => 'Yes: every website comes with an admin area and training.',
                        ],
                        [
                            'question' => 'Will my website be accessible?',
                            'answer' => 'We target RGAA level AA and deliver an accessibility statement.',
                        ],
                        [
                            'question' => 'Do you offer maintenance?',
                            'answer' => 'Yes, with a monthly plan that includes updates, backups and small improvements.',
                        ],
                    ],
                ],
                [
                    'type' => 'cta',
                    'title' => 'Have a project in mind?',
                    'text' => 'Let’s talk: reply within 24 business hours.',
                    'action' => [
                        'label' => 'Request a quote',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'about' => [
            'url' => '/a-propos',
            'title' => 'About',
            'description' => 'Who we are, our mission and our values.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'About',
                    'title' => 'A small team with high standards',
                    'lead' => 'Developers, designers and accessibility experts united around a sustainable web.',
                ],
                [
                    'type' => 'text',
                    'title' => 'Our mission',
                    'body' => '<p>We believe a good website is <strong>fast, accessible and frugal</strong>. Fast, because every second counts. Accessible, because the web is for everyone. Frugal, because the best code is the code you don’t write.</p>
<p>Since 2014, we have been supporting craftspeople, non-profits, local authorities and SMEs, from the first workshop to maintenance.</p>',
                ],
                [
                    'type' => 'features',
                    'title' => 'Our values',
                    'lead' => null,
                    'items' => [
                        [
                            'icon' => 'heart',
                            'title' => 'Transparency',
                            'text' => 'Detailed quotes, open code, no lock-in.',
                        ],
                        [
                            'icon' => 'leaf',
                            'title' => 'Frugality',
                            'text' => 'Fewer dependencies, less data, less energy.',
                        ],
                        [
                            'icon' => 'users',
                            'title' => 'Closeness',
                            'text' => 'A single point of contact, reachable directly.',
                        ],
                    ],
                ],
                [
                    'type' => 'team',
                    'title' => 'The team',
                    'lead' => 'Four people, all based in Paris.',
                    'items' => [
                        [
                            'name' => 'Camille Martin',
                            'role' => 'Founder, developer',
                            'initials' => 'CM',
                        ],
                        [
                            'name' => 'Inès Benali',
                            'role' => 'Accessibility expert',
                            'initials' => 'IB',
                        ],
                        [
                            'name' => 'Thomas Roux',
                            'role' => 'UI designer',
                            'initials' => 'TR',
                        ],
                        [
                            'name' => 'Hugo Lefèvre',
                            'role' => 'Back-end developer',
                            'initials' => 'HL',
                        ],
                    ],
                ],
                [
                    'type' => 'cta',
                    'title' => 'Let’s work together',
                    'text' => 'Tell us about your project, we will get back to you quickly.',
                    'action' => [
                        'label' => 'Contact us',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'services' => [
            'url' => '/services',
            'title' => 'Services',
            'description' => 'Website creation, redesign, accessibility and maintenance.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Services',
                    'title' => 'What we do',
                    'lead' => 'Four clear offers, to combine as your project requires.',
                ],
                [
                    'type' => 'services',
                    'title' => null,
                    'lead' => null,
                    'limit' => 0,
                    'link' => null,
                ],
                [
                    'type' => 'cta',
                    'title' => 'Need advice?',
                    'text' => 'We help you choose the right plan.',
                    'action' => [
                        'label' => 'Request a quote',
                        'url' => '/contact',
                    ],
                ],
            ],
        ],
        'blog' => [
            'url' => '/blog',
            'title' => 'Blog',
            'description' => 'Advice and lessons learned on fast, accessible websites.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Blog',
                    'title' => 'Articles and advice',
                    'lead' => 'Performance, accessibility, technical choices: what we learn in the field.',
                ],
                [
                    'type' => 'posts',
                    'title' => null,
                    'lead' => null,
                    'limit' => 0,
                    'link' => null,
                ],
            ],
        ],
        'contact' => [
            'url' => '/contact',
            'title' => 'Contact',
            'description' => 'Write to us: reply within 24 business hours.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => 'Contact',
                    'title' => 'Let’s talk about your project',
                    'lead' => 'Fill in the form or write to us directly, we reply within 24 business hours.',
                ],
                [
                    'type' => 'contact',
                    'title' => 'Write to us',
                    'success' => 'Thank you! Your message has been sent.',
                    'submit' => 'Send message',
                ],
            ],
        ],
        'legal' => [
            'url' => '/mentions-legales',
            'title' => 'Legal notice',
            'description' => 'Site legal notice.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => null,
                    'title' => 'Legal notice',
                    'lead' => null,
                ],
                [
                    'type' => 'text',
                    'title' => null,
                    'body' => '<h2 id="publisher">Publisher</h2>
<p>This website is published by <strong>@@title@@</strong>, 12 rue de la Paix, 75002 Paris. Contact: contact@exemple.fr.</p>
<h2 id="hosting">Hosting</h2>
<p>The website is hosted by your hosting provider; their details are in the service contract.</p>
<h2 id="intellectual-property">Intellectual property</h2>
<p>The texts, images and graphic elements of this website are protected. Any reproduction without permission is prohibited.</p>',
                ],
            ],
        ],
        'privacy' => [
            'url' => '/confidentialite',
            'title' => 'Privacy policy',
            'description' => 'How we process your personal data.',
            'sections' => [
                [
                    'type' => 'page_header',
                    'eyebrow' => null,
                    'title' => 'Privacy policy',
                    'lead' => null,
                ],
                [
                    'type' => 'text',
                    'title' => null,
                    'body' => '<h2 id="data-collected">Data collected</h2>
<p>The contact form collects your name, e-mail address and message, solely to reply to you.</p>
<h2 id="retention-period">Retention period</h2>
<p>Messages are kept for twelve months, then deleted.</p>
<h2 id="your-rights">Your rights</h2>
<p>You can request access to, correction or deletion of your data by writing to contact@exemple.fr.</p>',
                ],
            ],
        ],
    ],
    'contact_fields' => [
        [
            'name' => 'nom',
            'label' => 'Name',
            'type' => 'text',
            'autocomplete' => 'name',
            'required' => true,
        ],
        [
            'name' => 'email',
            'label' => 'E-mail address',
            'type' => 'email',
            'autocomplete' => 'email',
            'required' => true,
        ],
        [
            'name' => 'sujet',
            'label' => 'Subject',
            'type' => 'text',
            'autocomplete' => 'off',
            'required' => false,
        ],
        [
            'name' => 'message',
            'label' => 'Message',
            'type' => 'textarea',
            'autocomplete' => 'off',
            'required' => true,
        ],
    ],
    'ui' => [
        'skip' => 'Skip to content',
        'main_nav' => 'Main navigation',
        'mobile_nav' => 'Mobile navigation',
        'theme' => 'Toggle theme',
        'open_menu' => 'Open menu',
        'social' => 'Social networks',
        'language' => 'Language',
        'contact' => 'Contact',
        'more' => 'Learn more',
        'more_label' => 'Learn more: ',
        'details' => 'Contact details',
        'included' => 'What’s included',
        'all_posts' => 'All articles',
        'home' => 'Home',
        'not_found' => 'Page not found',
        'not_found_text' => 'The requested page does not exist or has been moved.',
        'back_home' => 'Back to home',
        'required' => 'This field is required.',
        'invalid_email' => 'Invalid e-mail address.',
        'sign_in' => 'Sign in',
        'sign_in_lead' => 'Access your client area.',
        'sign_in_submit' => 'Sign in',
        'sign_up' => 'Create an account',
        'sign_up_lead' => 'It only takes a few seconds.',
        'sign_up_submit' => 'Create my account',
        'account' => 'My account',
        'account_lead' => 'You are signed in.',
        'sign_out' => 'Sign out',
        'field_name' => 'Name',
        'field_email' => 'E-mail',
        'field_password' => 'Password',
        'no_account' => 'No account yet?',
        'have_account' => 'Already registered?',
        'err_credentials' => 'Incorrect e-mail or password.',
        'err_exists' => 'An account already exists with this e-mail.',
        'err_password' => 'The password must be at least 8 characters.',
        'err_required' => 'All fields are required.',
        'err_generic' => 'Something went wrong, please try again.',
    ],
];
