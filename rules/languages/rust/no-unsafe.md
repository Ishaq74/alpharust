---
name: rust-no-unsafe
apply: "rust"
severity: warning
checker: rust-unsafe
examples:
  bad:
    - "fn f() {\n    unsafe { g() }\n}\n"
  good:
    - "// unsafe is forbidden here\nlet s = \"unsafe\";\n#![forbid(unsafe_code)]\n"
---

# Usage d’unsafe à justifier
