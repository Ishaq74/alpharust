from django.urls import path

from .views import AdvancedComponentsView, ComponentsView

urlpatterns = [
    path("", ComponentsView.as_view(), name="ds-components"),
    path("advanced/", AdvancedComponentsView.as_view(), name="ds-components-advanced"),
]
