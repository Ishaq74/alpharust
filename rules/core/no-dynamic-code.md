---
name: no-dynamic-code
apply: "*"
severity: error
pattern: "\\bcreate_function\\s*\\(|\\bpreg_replace\\s*\\(\\s*['\\\"][^'\\\"]*/[a-z]*e[a-z]*['\\\"]"
examples:
  bad:
    - "$f = create_function('$a', 'return $a;');"
    - "preg_replace('/x/e', 'y', $s);"
  good:
    - "preg_replace('/x/i', 'y', $s);"
    - "assert!(x == 1);"
    - "assert(x == 1)"
---

# Éviter le code dynamique dangereux
