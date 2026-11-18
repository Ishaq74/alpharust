# Composition des règles

Le composer (`engine::composer`) ordonne les règles par couche, de la plus spécifique à la plus générale :

1. **frameworks** (le plus spécifique)
2. **domains**
3. **languages** et **markdown**
4. **core** (le plus général)

Si deux règles portent le **même nom** dans des couches différentes (ex. `no-eval` dans `core/` et dans
`languages/javascript/`), seule la plus spécifique est conservée. Les profils choisissent quelles règles
entrent dans la composition.

Ensuite le runner dédoublonne les violations identiques (même fichier, même ligne, même règle) et les trie
par fichier puis par ligne.
