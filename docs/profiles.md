# Profils

Les profils permettent de sélectionner des ensembles de règles selon le type de projet.

| Profil     | Usage                          |
|------------|--------------------------------|
| core       | Règles universelles minimales    |
| strict     | Core + domaines critiques      |
| web        | Frontend + styles + design     |
| ecommerce  | Paiements + auth + database    |
| cms        | Wagtail / Django               |
| starter    | Minimal pour démarrer           |

Utilisation :

```bash
alpharust lint --profile strict
```
