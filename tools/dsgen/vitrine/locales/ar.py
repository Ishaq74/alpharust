"""Arabe (servi sous /ar/…, écrit de droite à gauche : dir="rtl")."""
META = {
    'code': 'ar',
    'name': 'العربية',
    'label': 'AR',
    'dir': 'rtl',
    'flag': 'lang-ar',
    'order': 4,
    'date_intl': 'ar-u-nu-latn',
    'date_php': 'j F Y',
    'date_django': 'j F Y',
}

UI = {
    'skip': 'الانتقال إلى المحتوى',
    'main_nav': 'التنقل الرئيسي',
    'mobile_nav': 'التنقل على الجوال',
    'theme': 'تبديل المظهر',
    'open_menu': 'فتح القائمة',
    'social': 'شبكات التواصل الاجتماعي',
    'language': 'اللغة',
    'contact': 'اتصل بنا',
    'more': 'اقرأ المزيد',
    'more_label': 'اقرأ المزيد: ',
    'details': 'بيانات الاتصال',
    'included': 'ما تتضمنه الخدمة',
    'all_posts': 'جميع المقالات',
    'services': 'الخدمات',
    'blog': 'المدونة',
    'home': 'الرئيسية',
    'not_found': 'الصفحة غير موجودة',
    'not_found_text': 'الصفحة المطلوبة غير موجودة أو تم نقلها.',
    'back_home': 'العودة إلى الرئيسية',
    'required': 'هذا الحقل إلزامي.',
    'invalid_email': 'عنوان البريد الإلكتروني غير صالح.',
    'sign_in': 'تسجيل الدخول',
    'sign_in_lead': 'ادخلوا إلى مساحة العملاء الخاصة بكم.',
    'sign_in_submit': 'تسجيل الدخول',
    'sign_up': 'إنشاء حساب',
    'sign_up_lead': 'لن يستغرق الأمر سوى بضع ثوانٍ.',
    'sign_up_submit': 'إنشاء حسابي',
    'account': 'حسابي',
    'account_lead': 'لقد سجّلتم الدخول.',
    'sign_out': 'تسجيل الخروج',
    'field_name': 'الاسم',
    'field_email': 'البريد الإلكتروني',
    'field_password': 'كلمة المرور',
    'no_account': 'ليس لديكم حساب بعد؟',
    'have_account': 'لديكم حساب بالفعل؟',
    'err_credentials': 'البريد الإلكتروني أو كلمة المرور غير صحيحة.',
    'err_exists': 'يوجد حساب مسجّل بهذا العنوان بالفعل.',
    'err_password': 'يجب أن تتكوّن كلمة المرور من 8 أحرف على الأقل.',
    'err_required': 'جميع الحقول إلزامية.',
    'err_generic': 'حدث خطأ، يُرجى المحاولة مرة أخرى.',
    # Contenu semé côté Wagtail : sujet des messages du formulaire et titre du menu principal.
    'mail_subject': 'رسالة جديدة من الموقع',
    'menu_main': 'القائمة الرئيسية',
}

# Textes de la spec (clé : texte français de spec.py) -> traduction.
TEXT = {
    'Des sites web rapides, accessibles et faciles à faire évoluer.':
        'مواقع ويب سريعة وسهلة الوصول ويسهل تطويرها.',
    'Agence web indépendante : création de sites, refonte, accessibilité et maintenance.':
        'وكالة ويب مستقلة: إنشاء المواقع، وإعادة التصميم، وإمكانية الوصول، والصيانة.',
    'Demander un devis':
        'اطلبوا عرض سعر',
    '12 rue de la Paix, 75002 Paris':
        '12 rue de la Paix, 75002 Paris',
    'Du lundi au vendredi, 9 h – 18 h':
        'من الاثنين إلى الجمعة، من 9:00 إلى 18:00',
    'GitHub':
        'GitHub',
    'LinkedIn':
        'LinkedIn',
    'Mastodon':
        'Mastodon',
    'Accueil':
        'الرئيسية',
    'Services':
        'الخدمات',
    'Blog':
        'المدونة',
    'À propos':
        'من نحن',
    'Contact':
        'اتصل بنا',
    'Navigation':
        'التنقل',
    'Informations':
        'معلومات',
    'Mentions légales':
        'الإشعار القانوني',
    'Politique de confidentialité':
        'سياسة الخصوصية',
    'Tous droits réservés.':
        'جميع الحقوق محفوظة.',
    'Création de site web':
        'إنشاء المواقع الإلكترونية',
    'À partir de 2 400 €':
        'ابتداءً من 2400 €',
    "Un site sur mesure, rapide et référencé, livré avec son espace d'administration.":
        'موقع مصمم حسب الطلب، سريع ومهيأ لمحركات البحث، يُسلَّم مع لوحة الإدارة الخاصة به.',
    'Maquettes et design system':
        'نماذج التصميم ونظام التصميم',
    'Contenus éditables':
        'محتوى قابل للتعديل',
    'SEO technique et données structurées':
        'تحسين تقني لمحركات البحث وبيانات منظمة',
    'Mise en ligne et formation':
        'الإطلاق والتدريب',
    "Nous concevons votre site à partir de vos objectifs : qui doit le trouver, que doit-il y faire, comment le mesurer.\n\n## Notre méthode\n\nAteliers de cadrage, maquettes, puis développement par itérations courtes. Chaque version est **mise en ligne sur un\nenvironnement de recette** pour que vous puissiez la tester.\n\n## Ce qui est livré\n\n- un site responsive, accessible et performant ;\n- un espace d'administration pour modifier vos contenus ;\n- la documentation et une formation d'une demi-journée.":
        'نصمم موقعكم انطلاقًا من أهدافكم: من يجب أن يجده، وماذا ينبغي أن يفعل فيه، وكيف يُقاس نجاحه.\n\n## منهجيتنا\n\nورش عمل لتحديد النطاق، ثم نماذج التصميم، ثم تطوير على مراحل قصيرة. تُنشر كل نسخة **على بيئة اختبار**\nلتتمكنوا من تجربتها.\n\n## ما نسلمه\n\n- موقع متجاوب وسهل الوصول وعالي الأداء؛\n- لوحة إدارة لتعديل محتواكم؛\n- التوثيق ودورة تدريبية لمدة نصف يوم.',
    'Refonte et performance':
        'إعادة التصميم والأداء',
    'À partir de 1 900 €':
        'ابتداءً من 1900 €',
    'Un site existant plus rapide, plus clair et mieux classé, sans perdre votre référencement.':
        'موقعكم الحالي أسرع وأوضح وفي ترتيب أفضل، دون أن يفقد ظهوره في محركات البحث.',
    'Audit Lighthouse et Core Web Vitals':
        'تدقيق Lighthouse وCore Web Vitals',
    'Plan de redirections':
        'خطة إعادة التوجيه',
    'Optimisation des images':
        'تحسين الصور',
    'Suivi après mise en ligne':
        'متابعة بعد الإطلاق',
    'Une refonte réussie commence par un audit : temps de chargement, parcours, contenus qui convertissent.\n\n## Garder le référencement\n\nToutes les anciennes adresses sont **redirigées** et suivies pendant trois mois après la mise en ligne.\n\n## Mesurer le gain\n\n- scores Lighthouse avant / après ;\n- Core Web Vitals sur les pages clés ;\n- taux de conversion des formulaires.':
        'تبدأ إعادة التصميم الناجحة بالتدقيق: أزمنة التحميل، ومسارات المستخدمين، والمحتوى الذي يحقق التحويل.\n\n## الحفاظ على ترتيبكم في محركات البحث\n\nيُعاد **توجيه** جميع العناوين القديمة وتُتابَع لمدة ثلاثة أشهر بعد الإطلاق.\n\n## قياس التحسن\n\n- نتائج Lighthouse قبل التحسين وبعده؛\n- مؤشرات Core Web Vitals على الصفحات الرئيسية؛\n- معدلات التحويل في النماذج.',
    'Accessibilité (RGAA)':
        'إمكانية الوصول (RGAA)',
    'À partir de 1 200 €':
        'ابتداءً من 1200 €',
    "Audit et mise en conformité RGAA / WCAG 2.2 AA, déclaration d'accessibilité comprise.":
        'تدقيق ومطابقة لمعايير RGAA / WCAG 2.2 AA، مع بيان إمكانية الوصول.',
    'Audit sur échantillon de pages':
        'تدقيق على عينة من الصفحات',
    'Corrections priorisées':
        'تصحيحات حسب الأولوية',
    "Déclaration d'accessibilité":
        'بيان إمكانية الوصول',
    'Formation des rédacteurs':
        'تدريب المحررين',
    "L'accessibilité profite à tous : navigation au clavier, contrastes, lecteurs d'écran, formulaires compréhensibles.\n\n## Audit\n\nNous évaluons un échantillon représentatif de pages selon le **RGAA 4.1** et livrons un rapport priorisé.\n\n## Mise en conformité\n\n- corrections dans le code et les gabarits ;\n- contrôle des contrastes du thème ;\n- rédaction de la déclaration d'accessibilité.":
        'إمكانية الوصول تفيد الجميع: التنقل بلوحة المفاتيح، والتباين، وقارئات الشاشة، ونماذج مفهومة.\n\n## التدقيق\n\nنقيّم عينة تمثيلية من الصفحات وفق **RGAA 4.1** ونسلّم تقريرًا مرتبًا حسب الأولوية.\n\n## المطابقة\n\n- تصحيحات في الشيفرة والقوالب؛\n- التحقق من تباين ألوان المظهر؛\n- صياغة بيان إمكانية الوصول.',
    'Maintenance et hébergement':
        'الصيانة والاستضافة',
    'À partir de 90 € / mois':
        'ابتداءً من 90 € شهريًا',
    'Mises à jour, sauvegardes, surveillance et petites évolutions, sans surprise.':
        'تحديثات ونسخ احتياطية ومراقبة وتحسينات صغيرة، دون مفاجآت.',
    'Mises à jour de sécurité':
        'تحديثات الأمان',
    'Sauvegardes quotidiennes':
        'نسخ احتياطية يومية',
    'Surveillance 24 h / 24':
        'مراقبة على مدار الساعة',
    "Une heure d'évolution par mois":
        'ساعة تطوير واحدة شهريًا',
    "Un site est un logiciel : il doit être mis à jour, sauvegardé et surveillé.\n\n## Ce qui est compris\n\n- mises à jour du framework et des dépendances ;\n- sauvegardes quotidiennes conservées 30 jours ;\n- surveillance de la disponibilité et des certificats.\n\n## Évolutions\n\nChaque mois, une heure d'évolution est incluse : nouvelle section, formulaire, page.":
        'الموقع برنامج: يجب تحديثه ونسخه احتياطيًا ومراقبته.\n\n## ما تشمله الخدمة\n\n- تحديثات إطار العمل والمكتبات التي يعتمد عليها؛\n- نسخ احتياطية يومية تُحفظ لمدة 30 يومًا؛\n- مراقبة توفر الموقع وصلاحية الشهادات.\n\n## التطوير\n\nكل شهر، تُخصَّص ساعة للتطوير: قسم جديد أو نموذج أو صفحة.',
    'Astro, Laravel ou Wagtail : choisir son framework':
        'Astro أو Laravel أو Wagtail: كيف تختار إطار العمل',
    'Technique':
        'تقنية',
    'Camille Martin':
        'كاميل مارتان',
    'Trois excellents outils, trois façons de travailler. Nos critères pour choisir sans regret.':
        'ثلاث أدوات ممتازة، وثلاث طرق للعمل. معاييرنا لاختيار لا ندم بعده.',
    "Le bon framework est celui que votre équipe saura faire vivre. Voici nos critères.\n\n## Astro pour les sites de contenu\n\nPages statiques par défaut, îlots interactifs à la demande : idéal pour un site vitrine ou un blog très rapide.\n\n## Laravel pour les applications\n\nAuthentification, files d'attente, e-mails, paiements : **tout est prévu** pour une application métier.\n\n## Wagtail pour les rédactions\n\nUn vrai CMS éditorial : arborescence de pages, StreamField, révisions et prévisualisation.":
        'إطار العمل المناسب هو الذي يستطيع فريقكم المحافظة عليه وتطويره. إليكم معاييرنا.\n\n## Astro لمواقع المحتوى\n\nصفحات ثابتة افتراضيًا، وجزر تفاعلية عند الحاجة: مثالي لموقع تعريفي أو مدونة فائقة السرعة.\n\n## Laravel للتطبيقات\n\nالمصادقة، وقوائم الانتظار، والبريد الإلكتروني، والمدفوعات: **كل شيء جاهز** لتطبيقات الأعمال.\n\n## Wagtail لفرق التحرير\n\nنظام إدارة محتوى تحريري حقيقي: شجرة صفحات، وStreamField، ومراجعات، ومعاينة.',
    "L'accessibilité en 10 points":
        'إمكانية الوصول في 10 نقاط',
    'Accessibilité':
        'إمكانية الوصول',
    'Inès Benali':
        'إيناس بن علي',
    'Une liste courte et concrète pour éviter les erreurs les plus fréquentes.':
        'قائمة قصيرة وعملية لتجنب الأخطاء الأكثر شيوعًا.',
    "Ces dix points couvrent la majorité des problèmes rencontrés en audit.\n\n## Structure\n\n- un seul `h1` par page et des titres hiérarchisés ;\n- des repères : `header`, `nav`, `main`, `footer` ;\n- un lien d'évitement vers le contenu.\n\n## Interaction\n\n- tout est utilisable **au clavier** ;\n- le focus est toujours visible ;\n- les champs de formulaire ont un libellé.":
        'تغطي هذه النقاط العشر معظم المشكلات التي نصادفها في عمليات التدقيق.\n\n## البنية\n\n- عنوان `h1` واحد في كل صفحة وعناوين متدرجة؛\n- معالم واضحة: `header` و`nav` و`main` و`footer`؛\n- رابط تخطٍّ إلى المحتوى.\n\n## التفاعل\n\n- كل شيء قابل للاستخدام **بلوحة المفاتيح**؛\n- مؤشر التركيز ظاهر دائمًا؛\n- لكل حقل في النماذج تسمية.',
    'Des images légères sans perte de qualité':
        'صور خفيفة دون فقدان الجودة',
    'Performance':
        'الأداء',
    'Formats modernes, dimensions explicites, chargement différé : le trio gagnant.':
        'صيغ حديثة، وأبعاد صريحة، وتحميل مؤجل: الثلاثي الرابح.',
    'Les images représentent souvent plus de la moitié du poids d\'une page.\n\n## Les bons formats\n\nAVIF et WebP divisent le poids par deux à cinq par rapport au JPEG, à qualité égale.\n\n## Les bons attributs\n\n- `width` et `height` pour éviter les décalages de mise en page ;\n- `loading="lazy"` sous la ligne de flottaison ;\n- `srcset` pour servir la bonne taille.':
        'تمثل الصور غالبًا أكثر من نصف حجم الصفحة.\n\n## الصيغ المناسبة\n\nتقلّص صيغتا AVIF وWebP الحجم بمقدار مرتين إلى خمس مرات مقارنةً بصيغة JPEG، بالجودة نفسها.\n\n## السمات المناسبة\n\n- `width` و`height` لتجنب إزاحة التخطيط؛\n- `loading="lazy"` للصور الواقعة أسفل الجزء المرئي من الصفحة؛\n- `srcset` لتقديم الحجم المناسب.',
    'Agence web indépendante':
        'وكالة ويب مستقلة',
    'Des sites rapides, accessibles et faciles à faire évoluer':
        'مواقع سريعة وسهلة الوصول ويسهل تطويرها',
    "Nous concevons et maintenons des sites qui se chargent en moins d'une seconde, respectent le RGAA et restent simples à modifier.":
        'نصمم مواقع تُحمَّل في أقل من ثانية، وتحترم معايير RGAA، وتبقى سهلة التعديل، ونتولى صيانتها.',
    'Voir nos services':
        'اكتشفوا خدماتنا',
    'Score Lighthouse moyen':
        'متوسط نتيجة Lighthouse',
    'Niveau RGAA visé':
        'مستوى RGAA المستهدف',
    'Affichage du contenu':
        'عرض المحتوى',
    "D'expérience":
        'من الخبرة',
    'Pourquoi nous choisir':
        'لماذا تختاروننا',
    'Quatre engagements, sur chaque projet.':
        'أربعة التزامات في كل مشروع.',
    'Pages légères, images optimisées, scores mesurés à chaque livraison.':
        'صفحات خفيفة، وصور محسّنة، ونتائج تُقاس عند كل تسليم.',
    'Navigation au clavier, contrastes contrôlés, sémantique soignée.':
        'تنقل بلوحة المفاتيح، وتباين مضبوط، وبنية دلالية متقنة.',
    'Autonomie':
        'الاستقلالية',
    'Vous modifiez vos contenus sans nous appeler.':
        'تعدّلون محتواكم دون الحاجة إلى الاتصال بنا.',
    'Sérénité':
        'راحة البال',
    'Mises à jour, sauvegardes et surveillance incluses.':
        'التحديثات والنسخ الاحتياطية والمراقبة مشمولة.',
    'Nos services':
        'خدماتنا',
    'De la création à la maintenance.':
        'من الإنشاء إلى الصيانة.',
    'Tous les services':
        'جميع الخدمات',
    'Ils nous font confiance':
        'يثقون بنا',
    'Notre nouveau site est deux fois plus rapide et nos demandes de devis ont augmenté de 40 %.':
        'موقعنا الجديد أسرع بمرتين، وارتفعت طلبات عروض الأسعار لدينا بنسبة 40%.',
    'Directrice, Atelier Laurent':
        'مديرة، Atelier Laurent',
    'SL':
        'SL',
    "Un audit d'accessibilité clair, des corrections livrées en trois semaines.":
        'تدقيق واضح لإمكانية الوصول، وتصحيحات سُلّمت خلال ثلاثة أسابيع.',
    'Responsable digital, Ville de Montreuil':
        'مسؤول التحول الرقمي، بلدية مونتروي',
    'MP':
        'MP',
    'Nous publions nos actualités en cinq minutes, sans aide.':
        'ننشر أخبارنا في خمس دقائق، دون مساعدة.',
    'Chargée de communication, Asso Horizon':
        'مسؤولة الاتصال، جمعية Horizon',
    'LM':
        'LM',
    'Derniers articles':
        'أحدث المقالات',
    "Conseils et retours d'expérience.":
        'نصائح وخبرات من الميدان.',
    'Tout le blog':
        'المدونة كاملة',
    'Questions fréquentes':
        'الأسئلة الشائعة',
    'Combien de temps faut-il pour créer un site ?':
        'كم يستغرق إنشاء موقع؟',
    'Entre quatre et huit semaines selon le nombre de pages et de fonctionnalités.':
        'من أربعة إلى ثمانية أسابيع حسب عدد الصفحات والوظائف.',
    'Pourrai-je modifier mon site moi-même ?':
        'هل سأتمكن من تعديل موقعي بنفسي؟',
    "Oui : chaque site est livré avec un espace d'administration et une formation.":
        'نعم: يُسلَّم كل موقع مع لوحة إدارة ودورة تدريبية.',
    'Mon site sera-t-il accessible ?':
        'هل سيكون موقعي سهل الوصول؟',
    "Nous visons le niveau AA du RGAA et livrons une déclaration d'accessibilité.":
        'نستهدف المستوى AA من معايير RGAA ونسلّم بيانًا لإمكانية الوصول.',
    'Proposez-vous la maintenance ?':
        'هل تقدمون خدمة الصيانة؟',
    'Oui, avec un forfait mensuel qui inclut mises à jour, sauvegardes et petites évolutions.':
        'نعم، باشتراك شهري يشمل التحديثات والنسخ الاحتياطية والتحسينات الصغيرة.',
    'Un projet en tête ?':
        'لديكم مشروع؟',
    'Parlons-en : réponse sous 24 heures ouvrées.':
        'لنتحدث: نرد خلال 24 ساعة عمل.',
    'Qui nous sommes, notre mission et nos valeurs.':
        'من نحن، ومهمتنا، وقيمنا.',
    'Une petite équipe, de grandes exigences':
        'فريق صغير بمعايير عالية',
    'Développeurs, designers et experts accessibilité réunis autour du web durable.':
        'مطورون ومصممون وخبراء في إمكانية الوصول، يجمعهم الويب المستدام.',
    'Notre mission':
        'مهمتنا',
    "Nous croyons qu'un bon site est **rapide, accessible et sobre**. Rapide, parce que chaque seconde\ncompte. Accessible, parce que le web est pour tout le monde. Sobre, parce que le meilleur code est celui qu'on n'écrit pas.\n\nDepuis 2014, nous accompagnons artisans, associations, collectivités et PME, du premier atelier à la maintenance.":
        'نؤمن بأن الموقع الجيد **سريع وسهل الوصول ومقتصد**. سريع، لأن كل ثانية\nمهمة. سهل الوصول، لأن الويب للجميع. مقتصد، لأن أفضل شيفرة هي تلك التي لا نكتبها.\n\nمنذ عام 2014 نرافق الحرفيين والجمعيات والجماعات المحلية والشركات الصغيرة والمتوسطة، من ورشة العمل الأولى حتى الصيانة.',
    'Nos valeurs':
        'قيمنا',
    'Transparence':
        'الشفافية',
    'Devis détaillés, code ouvert, aucun verrouillage.':
        'عروض أسعار مفصلة، وشيفرة مفتوحة، ودون أي ارتباط إلزامي بمزوّد.',
    'Sobriété':
        'الاعتدال',
    "Moins de dépendances, moins de données, moins d'énergie.":
        'تبعيات أقل، وبيانات أقل، وطاقة أقل.',
    'Proximité':
        'القرب',
    'Un interlocuteur unique, joignable directement.':
        'محاور واحد يمكن التواصل معه مباشرة.',
    "L'équipe":
        'الفريق',
    'Quatre personnes, toutes basées à Paris.':
        'أربعة أشخاص، جميعهم في باريس.',
    'Fondatrice, développeuse':
        'مؤسِّسة ومطوِّرة',
    'CM':
        'CM',
    'Experte accessibilité':
        'خبيرة في إمكانية الوصول',
    'IB':
        'IB',
    'Designer UI':
        'مصمم واجهات المستخدم',
    'TR':
        'TR',
    'Développeur back-end':
        'مطوّر الواجهة الخلفية',
    'HL':
        'HL',
    'Travaillons ensemble':
        'لنعمل معًا',
    'Présentez-nous votre projet, nous revenons vers vous rapidement.':
        'عرّفونا بمشروعكم، وسنعود إليكم سريعًا.',
    'Nous contacter':
        'تواصلوا معنا',
    'Création, refonte, accessibilité et maintenance de sites web.':
        'إنشاء المواقع الإلكترونية، وإعادة تصميمها، وإمكانية الوصول، والصيانة.',
    'Ce que nous faisons':
        'ما نقدمه',
    'Quatre offres claires, combinables selon votre projet.':
        'أربعة عروض واضحة يمكن الجمع بينها حسب مشروعكم.',
    "Besoin d'un conseil ?":
        'تحتاجون إلى نصيحة؟',
    'Nous vous aidons à choisir la bonne formule.':
        'نساعدكم على اختيار الصيغة المناسبة.',
    "Conseils et retours d'expérience sur le web performant et accessible.":
        'نصائح وخبرات حول الويب السريع وسهل الوصول.',
    'Articles et conseils':
        'مقالات ونصائح',
    'Performance, accessibilité, choix techniques : ce que nous apprenons sur le terrain.':
        'الأداء، وإمكانية الوصول، والخيارات التقنية: ما نتعلمه في الميدان.',
    'Écrivez-nous : réponse sous 24 heures ouvrées.':
        'راسلونا: نرد خلال 24 ساعة عمل.',
    'Parlons de votre projet':
        'لنتحدث عن مشروعكم',
    'Remplissez le formulaire ou écrivez-nous directement, nous répondons sous 24 heures ouvrées.':
        'املؤوا النموذج أو راسلونا مباشرة، ونرد خلال 24 ساعة عمل.',
    'Écrivez-nous':
        'راسلونا',
    'Merci ! Votre message a bien été envoyé.':
        'شكرًا! تم إرسال رسالتكم بنجاح.',
    'Envoyer le message':
        'إرسال الرسالة',
    'Mentions légales du site.':
        'الإشعار القانوني للموقع.',
    "## Éditeur\n\nCe site est édité par **@@title@@**, 12 rue de la Paix, 75002 Paris. Contact : contact@exemple.fr.\n\n## Hébergement\n\nLe site est hébergé par votre prestataire d'hébergement ; ses coordonnées figurent dans le contrat de service.\n\n## Propriété intellectuelle\n\nLes textes, images et éléments graphiques de ce site sont protégés. Toute reproduction sans autorisation est interdite.":
        '## الناشر\n\nينشر هذا الموقع **@@title@@**، 12 rue de la Paix, 75002 Paris. للتواصل: contact@exemple.fr.\n\n## الاستضافة\n\nيستضيف الموقع مزود الاستضافة الخاص بكم، وترد بياناته في عقد الخدمة.\n\n## الملكية الفكرية\n\nالنصوص والصور والعناصر الرسومية في هذا الموقع محمية. يُمنع أي نسخ دون إذن.',
    'Comment nous traitons vos données personnelles.':
        'كيف نعالج بياناتكم الشخصية.',
    "## Données collectées\n\nLe formulaire de contact recueille votre nom, votre adresse e-mail et votre message, uniquement pour vous répondre.\n\n## Durée de conservation\n\nLes messages sont conservés douze mois, puis supprimés.\n\n## Vos droits\n\nVous pouvez demander l'accès, la rectification ou la suppression de vos données en écrivant à contact@exemple.fr.":
        '## البيانات المجمعة\n\nيجمع نموذج الاتصال اسمكم وبريدكم الإلكتروني ورسالتكم، وذلك فقط للرد عليكم.\n\n## مدة الاحتفاظ\n\nتُحفظ الرسائل اثني عشر شهرًا، ثم تُحذف.\n\n## حقوقكم\n\nيمكنكم طلب الاطلاع على بياناتكم أو تصحيحها أو حذفها بمراسلتنا على contact@exemple.fr.',
    'Page introuvable':
        'الصفحة غير موجودة',
    "La page demandée n'existe pas ou a été déplacée.":
        'الصفحة المطلوبة غير موجودة أو تم نقلها.',
    "Retour à l'accueil":
        'العودة إلى الرئيسية',
    'Nom':
        'الاسم',
    'Adresse e-mail':
        'البريد الإلكتروني',
    'Sujet':
        'الموضوع',
    'Message':
        'الرسالة',
    '98/100':
        '98/100',
    'AA':
        'AA',
    '< 1 s':
        '< 1 ث',
    '12 ans':
        '12 عامًا',
}
