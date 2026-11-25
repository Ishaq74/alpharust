---
name: logical-properties
apply: "*.css,*.scss,*.sass,*.less"
severity: warning
pattern: "(?i)\\b(?:margin|padding)-(?:left|right)\\s*:|\\btext-align\\s*:\\s*(?:left|right)\\b|\\bfloat\\s*:\\s*(?:left|right)\\b"
examples:
  bad:
    - ".a { margin-left: 1rem; }"
    - "th { text-align: left; }"
  good:
    - ".a { margin-inline-start: 1rem; }"
    - "th { text-align: start; }"
---

# Propriétés logiques (RTL)

Les marges, retraits, alignements et flottants horizontaux s'écrivent avec les propriétés logiques
(`margin-inline-start`, `padding-inline-end`, `text-align: start`, `float: inline-start`), qui suivent
le sens d'écriture (`dir="rtl"` pour l'arabe). Les positions `left` / `right` restent permises quand le côté
est physique par construction (infobulle `data-side="left"`, centrage `left: 50%` + `translateX(-50%)`).
