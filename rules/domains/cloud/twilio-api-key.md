---
name: twilio-api-key
apply: "*"
severity: error
pattern: "\\bSK[0-9a-f]{32}\\b"
examples:
  bad:
    - "sid = \"S¦K0123456789abcdef0123456789abcdef\""
  good:
    - "sid = os.environ[\"TWILIO_API_KEY\"]"
---

# Clé d'API Twilio en clair

Les SID de clés d'API Twilio (`SK…`) et leurs secrets ne doivent pas être versionnés.
