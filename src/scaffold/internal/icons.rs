//! Icônes du site vitrine : jeux copiés (MDI, Lucide, Circle Flags) ou API Iconify (`--icons`).

use super::*;

/// Jeux d'icônes copiés (MDI par défaut, Lucide, drapeaux Circle Flags) et correspondance des noms logiques :
/// écrit par `tools/dsgen/vitrine/fetch_icons.py`, la même source que les couches du site.
const ICONSETS: &str = include_str!("../../../tools/dsgen/vitrine/iconsets.json");
/// API publique d'Iconify (jeux non copiés : tabler, ph, heroicons, openmoji…).
// alpharust-ignore: no-hardcoded-urls
const ICONIFY_API: &str = "https://api.iconify.design";
/// Emplacement de `icons.json` selon le framework (Astro, Laravel, Wagtail).
const ICON_FILES: [&str; 3] = [
    "src/data/icons.json",
    "resources/data/icons.json",
    "layout/icons.json",
];

/// Un nom de jeu Iconify : minuscules, chiffres et tirets (`mdi`, `lucide`, `material-symbols`).
pub fn valid_icon_set(set: &str) -> bool {
    !set.is_empty()
        && set.len() <= 40
        && set
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Icônes logiques d'un jeu Iconify (`{nom: {body, width, height}}`) à partir de sa réponse JSON
/// (format Iconify : `icons`, `aliases`, `width`/`height` par défaut) ; les noms introuvables sont absents.
pub(super) fn pick_icons(
    set: &str,
    data: &Value,
    catalog: &Value,
) -> serde_json::Map<String, Value> {
    let (w, h) = (
        data.get("width").and_then(Value::as_u64).unwrap_or(16),
        data.get("height").and_then(Value::as_u64).unwrap_or(16),
    );
    let lookup = |name: &str| -> Option<Value> {
        let icons = data.get("icons")?;
        let src = icons.get(name).or_else(|| {
            let parent = data.get("aliases")?.get(name)?.get("parent")?.as_str()?;
            icons.get(parent)
        })?;
        Some(json!({
            "body": src.get("body")?.as_str()?,
            "width": src.get("width").and_then(Value::as_u64).unwrap_or(w),
            "height": src.get("height").and_then(Value::as_u64).unwrap_or(h),
        }))
    };
    let mut out = serde_json::Map::new();
    let empty = serde_json::Map::new();
    let map = catalog["map"].as_object().unwrap_or(&empty);
    for (logical, names) in map {
        let mut tries: Vec<String> = names
            .get(set)
            .and_then(Value::as_str)
            .map(|s| vec![s.to_string()])
            .unwrap_or_default();
        if let Some(c) = catalog["candidates"][logical.as_str()].as_array() {
            tries.extend(c.iter().filter_map(Value::as_str).map(String::from));
        }
        if let Some(icon) = tries.iter().find_map(|n| lookup(n)) {
            out.insert(logical.clone(), icon);
        }
    }
    out
}

/// `--icons <jeu>` : réécrit `icons.json` du site avec un autre jeu Iconify (copié ou téléchargé depuis l'API
/// Iconify). Les icônes absentes du jeu gardent leur tracé MDI ; les drapeaux (Circle Flags) ne changent pas.
pub(super) fn site_icons(
    target: &Path,
    set: &str,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<String> {
    if !valid_icon_set(set) {
        bail!("--icons : nom de jeu Iconify invalide `{set}` (ex. mdi, lucide, tabler, ph)");
    }
    let Some(rel) = ICON_FILES.iter().find(|f| target.join(f).is_file()) else {
        bail!("icons.json introuvable : --icons s'applique au site vitrine (--ui)");
    };
    let path = target.join(rel);
    let mut current: Value = serde_json::from_str(&fs::read_to_string(&path)?)
        .with_context(|| format!("lecture de {rel}"))?;
    let catalog: Value = serde_json::from_str(ICONSETS)?;
    let data = if let Some(bundled) = catalog["sets"].get(set) {
        bundled.clone()
    } else {
        let mut names: Vec<String> = Vec::new();
        for (logical, per_set) in catalog["map"].as_object().into_iter().flatten() {
            names.extend(
                per_set
                    .as_object()
                    .into_iter()
                    .flatten()
                    .filter_map(|(_, v)| v.as_str().map(String::from)),
            );
            names.extend(
                catalog["candidates"][logical.as_str()]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|v| v.as_str().map(String::from)),
            );
        }
        names.sort();
        names.dedup();
        let url = format!("{ICONIFY_API}/{set}.json?icons={}", names.join(","));
        let target = &fs::canonicalize(target)?;
        let tmp = target.join(".icons.json.tmp");
        on_line(&format!("téléchargement du jeu {set} ({ICONIFY_API})"));
        let out = exec::run(
            "curl",
            &["-fsSL".into(), "-o".into(), tmp.display().to_string(), url],
            target,
            cancel,
            std::time::Duration::from_secs(120),
            on_line,
        )?;
        let text = fs::read_to_string(&tmp).unwrap_or_default();
        let _ = fs::remove_file(&tmp);
        if !out.success {
            bail!("jeu d'icônes `{set}` introuvable sur Iconify (réseau ou nom du jeu)");
        }
        serde_json::from_str(&text).context("réponse Iconify illisible")?
    };
    let picked = pick_icons(set, &data, &catalog);
    if picked.is_empty() {
        bail!("le jeu `{set}` n'a aucune des icônes du site : nom de jeu erroné ?");
    }
    let total = catalog["map"].as_object().map_or(0, serde_json::Map::len);
    let n = picked.len();
    let icons = current["icons"]
        .as_object_mut()
        .ok_or_else(|| anyhow!("{rel} : clé `icons` absente"))?;
    for (k, v) in picked {
        icons.insert(k, v);
    }
    current["set"] = json!(set);
    fs::write(&path, serde_json::to_string_pretty(&current)? + "\n")?;
    Ok(format!(
        "{rel} : jeu {set}, {n}/{total} icône(s) ; les autres gardent MDI, drapeaux Circle Flags"
    ))
}

#[cfg(test)]
mod icon_tests {
    use super::*;

    #[test]
    fn bundled_icon_sets_map_every_logical_name() {
        let catalog: Value = serde_json::from_str(ICONSETS).unwrap();
        let total = catalog["map"].as_object().unwrap().len();
        assert_eq!(
            pick_icons("mdi", &catalog["sets"]["mdi"], &catalog).len(),
            total
        );
        // Lucide n'a pas de logo Mastodon : repli MDI
        assert_eq!(
            pick_icons("lucide", &catalog["sets"]["lucide"], &catalog).len(),
            total - 1
        );
        assert!(valid_icon_set("material-symbols"));
        assert!(!valid_icon_set("../x") && !valid_icon_set(""));
    }

    #[test]
    fn site_icons_rewrites_icons_json_and_keeps_flags() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path();
        fs::create_dir_all(p.join("layout")).unwrap();
        fs::write(
            p.join("layout/icons.json"),
            r#"{"set": "mdi", "icons": {"flag-fr": {"body": "F", "width": 512, "height": 512}, "mastodon": {"body": "M", "width": 24, "height": 24}}}"#,
        )
        .unwrap();
        let msg = site_icons(p, "lucide", &AtomicBool::new(false), &mut |_| {}).unwrap();
        assert!(msg.contains("lucide"), "{msg}");
        let v: Value =
            serde_json::from_str(&fs::read_to_string(p.join("layout/icons.json")).unwrap())
                .unwrap();
        assert_eq!(v["set"], "lucide");
        assert_eq!(v["icons"]["flag-fr"]["body"], "F");
        assert_eq!(v["icons"]["mastodon"]["body"], "M");
        assert!(v["icons"]["menu"]["body"]
            .as_str()
            .unwrap()
            .contains("stroke"));
        assert!(site_icons(p, "Bad Set", &AtomicBool::new(false), &mut |_| {}).is_err());
    }
}
