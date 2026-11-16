---
name: slack-token
apply: "*"
severity: error
pattern: "\\bxox[abprs]-[0-9A-Za-z-]{10,}"
examples:
  bad:
    - "SLACK = \"xo¦xb-1234567890-abcdefghij\""
  good:
    - "SLACK = os.environ[\"SLACK_BOT_TOKEN\"]"
---

# Jeton Slack en clair

Les jetons Slack (`xoxb-…`, `xoxp-…`) ne doivent pas être versionnés.
