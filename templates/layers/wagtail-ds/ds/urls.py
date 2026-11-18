from django.urls import path

from .views import ComponentsView

urlpatterns = [
    path("", ComponentsView.as_view(), name="ds-components"),
]
