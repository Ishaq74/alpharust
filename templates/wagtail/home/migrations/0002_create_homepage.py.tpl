from django.db import migrations


def create_homepage(apps, schema_editor):
    ContentType = apps.get_model("contenttypes.ContentType")
    Page = apps.get_model("wagtailcore.Page")
    Site = apps.get_model("wagtailcore.Site")
    HomePage = apps.get_model("home.HomePage")

    # Supprime la page d'accueil par défaut créée par wagtailcore.0002_initial_data
    page_content_type = ContentType.objects.get(model="page", app_label="wagtailcore")
    Page.objects.filter(content_type=page_content_type, slug="home", depth=2).delete()

    homepage_content_type, __ = ContentType.objects.get_or_create(
        model="homepage", app_label="home"
    )
    homepage = HomePage.objects.create(
        title="Accueil",
        draft_title="Accueil",
        slug="home",
        content_type=homepage_content_type,
        path="00010001",
        depth=2,
        numchild=0,
        url_path="/home/",
        body="<p>Bienvenue sur @@title@@.</p>",
    )

    # Site par défaut dont la racine est la nouvelle page d'accueil
    Site.objects.create(hostname="localhost", root_page=homepage, is_default_site=True)


def remove_homepage(apps, schema_editor):
    HomePage = apps.get_model("home.HomePage")
    # Page et Site sont supprimés en cascade
    HomePage.objects.filter(slug="home", depth=2).delete()


class Migration(migrations.Migration):
    # Doit s'exécuter avant la migration qui introduit `Locale` dans wagtailcore.
    run_before = [
        ("wagtailcore", "0053_locale_model"),
    ]

    dependencies = [
        ("home", "0001_initial"),
    ]

    operations = [
        migrations.RunPython(create_homepage, remove_homepage),
    ]
