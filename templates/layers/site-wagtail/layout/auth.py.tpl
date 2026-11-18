@@meta when=auth@@
# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Comptes du site avec l'authentification de Django (django.contrib.auth).

Pages servies par HomePage (RoutablePageMixin : /auth/sign-in/, /auth/sign-up/, /auth/dashboard/ dans chaque langue) ;
mêmes données, contrôles et messages qu'Astro (Better Auth) et Laravel (Fortify).
"""
import re

from django.contrib.auth import authenticate, get_user_model, login, logout
from django.http import HttpResponseRedirect
from django.template.response import TemplateResponse
from django.utils.translation import gettext

from layout.context_processors import AUTH_ROUTES, UI, localize

AUTH_FIELDS = {'sign_in': [('email', 'email', 'email'), ('password', 'password', 'current-password')], 'sign_up': [('name', 'text', 'name'), ('email', 'email', 'email'), ('password', 'password', 'new-password')]}
PASSWORD_MIN = 8
EMAIL = re.compile(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")


def ui(key):
    return gettext(UI[key])


def auth_page(kind, error=None, values=None):
    """Connexion / inscription : le mot de passe n'est jamais ré-affiché."""
    values = values or {}
    other = "sign_up" if kind == "sign_in" else "sign_in"
    return {
        "title": ui(kind),
        "lead": ui(kind + "_lead"),
        "submit": ui(kind + "_submit"),
        "error": error,
        "action": localize(AUTH_ROUTES[kind]),
        "fields": [
            {"name": n, "type": t, "autocomplete": a, "id": f"auth-{n}", "label": ui(f"field_{n}"),
             "value": "" if t == "password" else values.get(n, "")}
            for n, t, a in AUTH_FIELDS[kind]
        ],
        "alt_text": ui("no_account" if kind == "sign_in" else "have_account"),
        "alt_label": ui(other),
        "alt_url": localize(AUTH_ROUTES[other]),
    }


def validate(kind, values):
    """Mêmes règles, dans le même ordre, qu'Astro et Laravel : libellé d'erreur ou None."""
    def v(k):
        return (values.get(k) or "").strip()

    if any(not v(n) for n, _, _ in AUTH_FIELDS[kind]):
        return ui("err_required")
    if not EMAIL.match(v("email")):
        return ui("invalid_email")
    if kind == "sign_up" and len(values.get("password") or "") < PASSWORD_MIN:
        return ui("err_password")
    return None


def render(request, template, page, error=None):
    return TemplateResponse(request, template, {"page": page}, status=422 if error else 200)


def done(request):
    return HttpResponseRedirect(localize(AUTH_ROUTES["account"]))


def sign_in(request):
    if request.user.is_authenticated:
        return done(request)
    error, values = None, {}
    if request.method == "POST":
        values = request.POST
        error = validate("sign_in", values)
        if not error:
            user = authenticate(request, username=values["email"].strip().lower(), password=values["password"])
            if user is not None:
                login(request, user)
                return done(request)
            error = ui("err_credentials")
    return render(request, "layout/auth.html", auth_page("sign_in", error, values), error)


def sign_up(request):
    if request.user.is_authenticated:
        return done(request)
    error, values = None, {}
    if request.method == "POST":
        values = request.POST
        error = validate("sign_up", values)
        if not error:
            email = values["email"].strip().lower()
            users = get_user_model().objects
            if users.filter(username=email).exists() or users.filter(email=email).exists():
                error = ui("err_exists")
            else:
                user = users.create_user(
                    username=email, email=email, password=values["password"], first_name=values["name"].strip()[:150]
                )
                login(request, user, backend="django.contrib.auth.backends.ModelBackend")
                return done(request)
    return render(request, "layout/auth.html", auth_page("sign_up", error, values), error)


def account(request):
    """« Mon compte » ; son formulaire POST déconnecte."""
    if not request.user.is_authenticated:
        return HttpResponseRedirect(localize(AUTH_ROUTES["sign_in"]))
    if request.method == "POST":
        logout(request)
        return HttpResponseRedirect(localize("/"))
    user = request.user
    page = {
        "title": ui("account"),
        "lead": ui("account_lead"),
        "name": user.first_name or user.get_username(),
        "email": user.email,
        "logout_action": localize(AUTH_ROUTES["account"]),
    }
    return render(request, "layout/account.html", page)
