@@meta when=!official@@
/*
 * VOS SURCHARGES — chargé en dernier par global.css.
 *
 * - Changer l'apparence d'un composant : modifiez d'abord son jeton dans tokens/components.css.
 * - Un composant sans fichier dédié : ajoutez components/<nom>.css, importez-le dans global.css, ou écrivez ici.
@@if starwind@@
 *   Les composants Starwind exposent [data-slot="…"] ; placez les règles dans `@layer overrides { … }`
 *   pour passer après leurs classes Tailwind. Exemple :
 *     @layer overrides { [data-slot='avatar'] { border-radius: 0.5rem; } }
@@else@@
@@if concordia@@
 *   Concordia : les jetons de composants sont dans tokens/components.css (--button-border-radius, --card-padding…).
 *   Exemple : :root { --button-border-radius: 9999px; }
@@else@@
 *   Exemple : :root { --button-border-radius: 9999px; }
@@end@@
@@end@@
 */
