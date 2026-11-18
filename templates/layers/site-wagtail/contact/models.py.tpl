# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django.db import models
from django.template.response import TemplateResponse
from modelcluster.fields import ParentalKey
from wagtail.admin.panels import FieldPanel, FieldRowPanel, InlinePanel, MultiFieldPanel
from wagtail.contrib.forms.models import AbstractEmailForm, AbstractFormField
from wagtail.fields import StreamField

from streams.blocks import SECTION_BLOCKS, ContactBlock

TYPES = {"singleline": "text", "email": "email", "multiline": "textarea"}


class FormField(AbstractFormField):
    page = ParentalKey("ContactPage", on_delete=models.CASCADE, related_name="form_fields")
    autocomplete = models.CharField(max_length=40, blank=True, default="off")

    panels = AbstractFormField.panels + [FieldPanel("autocomplete")]


class ContactPage(AbstractEmailForm):
    """Formulaire de contact (wagtail.contrib.forms) : les envois sont enregistrés (Formulaires dans l'admin)."""

    body = StreamField(SECTION_BLOCKS + [("contact", ContactBlock())], blank=True, use_json_field=True, verbose_name="sections")

    content_panels = AbstractEmailForm.content_panels + [
        FieldPanel("body"),
        InlinePanel("form_fields", label="Champs du formulaire"),
        MultiFieldPanel(
            [FieldRowPanel([FieldPanel("from_address"), FieldPanel("to_address")]), FieldPanel("subject")],
            "E-mail de notification",
        ),
    ]
    max_count = 1

    class Meta:
        verbose_name = "page de contact"

    def get_form(self, *args, **kwargs):
        form = super().get_form(*args, **kwargs)
        for field in form.fields.values():
            field.error_messages["required"] = "Ce champ est obligatoire."
            field.error_messages["invalid"] = "Adresse e-mail invalide."
        return form

    def contact_fields(self, form):
        """Champs normalisés pour le gabarit commun (mêmes clés qu'Astro et Laravel)."""
        out = []
        for f in self.get_form_fields():
            bound = form[f.clean_name]
            errors = form.errors.get(f.clean_name) if form.is_bound else None
            error = errors[0] if errors else None
            field_id = f"contact-{f.clean_name}"
            out.append(
                {
                    "id": field_id,
                    "name": f.clean_name,
                    "label": f.label,
                    "type": TYPES.get(f.field_type, "text"),
                    "autocomplete": f.autocomplete or "off",
                    "required": f.required,
                    "textarea": f.field_type == "multiline",
                    "value": (bound.value() or "") if form.is_bound else "",
                    "error": error,
                    "invalid": "true" if error else None,
                    "error_id": f"{field_id}-error" if error else None,
                }
            )
        return out

    def render_landing_page(self, request, form_submission=None, *args, **kwargs):
        # Même page, avec le message de confirmation et un formulaire vide (comme Astro et Laravel).
        context = self.get_context(request)
        context["form"] = self.get_form(page=self, user=request.user)
        context["sent"] = True
        return TemplateResponse(request, self.get_template(request), context)
