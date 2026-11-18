# Écrire une règle

1. Copier `rules/_template.md` dans le bon dossier (`core/`, `languages/<l>/`, `frameworks/<f>/`, `domains/<d>/[<service>/]`).
2. Remplir le frontmatter (voir `docs/rule-format.md`) : `pattern` **ou** `checker`.
3. Écrire un titre `# …` (c'est le message affiché) et une description.
4. Ajouter des `examples` (`bad` / `good`) — ils sont testés par `alpharust check-rules` et `cargo test`.
5. Ajouter la règle à un profil si besoin (`profiles/*.toml`).
6. Vérifier : `alpharust check-rules && alpharust lint <chemin> --profile <p>`.

Pour un service tiers, utiliser `domains/<domaine>/<service>/<règle>.md` : les profils `third-party` et
`secrets` les prennent automatiquement (`domains/*/*/*`).
