---
name: laravel-no-raw-sql
apply: "*.php"
severity: warning
pattern: "\\bDB::(raw|statement|unprepared)\\b|\\b(whereRaw|selectRaw|havingRaw|orderByRaw)\\s*\\("
examples:
  bad:
    - "DB::raw('count(*)')"
    - "$q->whereRaw('a = '.$x)"
  good:
    - "$q->where('a', $x)"
---

# Éviter le SQL brut en Laravel
