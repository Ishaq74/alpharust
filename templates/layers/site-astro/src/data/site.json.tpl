@@meta mode=replace when=vitrine@@
{
  "default": "fr",
  "auth": {
    "routes": {
      "sign_in": "/auth/sign-in",
      "sign_up": "/auth/sign-up",
      "account": "/auth/dashboard"
    },
    "fields": {
      "sign_in": [
        {
          "name": "email",
          "type": "email",
          "autocomplete": "email"
        },
        {
          "name": "password",
          "type": "password",
          "autocomplete": "current-password"
        }
      ],
      "sign_up": [
        {
          "name": "name",
          "type": "text",
          "autocomplete": "name"
        },
        {
          "name": "email",
          "type": "email",
          "autocomplete": "email"
        },
        {
          "name": "password",
          "type": "password",
          "autocomplete": "new-password"
        }
      ]
    },
    "passwordMin": 8
  }
}
