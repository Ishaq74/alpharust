//! CLI autonome de Tailwind (Laravel et Wagtail + Starwind) : téléchargement épinglé et vérifié (SHA-256).

use super::*;

/// Version de la CLI autonome de Tailwind (même version majeure que `tailwindcss` côté Laravel et Astro).
pub const TAILWIND_VERSION: &str = "v4.3.5";
// alpharust-ignore: no-hardcoded-urls (assets officiels des releases Tailwind)
const TAILWIND_RELEASES: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download";

/// Empreintes SHA-256 publiées avec la release (`sha256sums.txt` de tailwindlabs/tailwindcss v4.3.5).
/// Épinglées dans le binaire : un fichier téléchargé qui ne correspond pas est supprimé et refusé.
const TAILWIND_SHA256: &[(&str, &str)] = &[
    (
        "tailwindcss-linux-arm64",
        "ffcd0d21dbd7c605dc12743ff0c0b47debc963c86b399674084c1c8f2c96ecad",
    ),
    (
        "tailwindcss-linux-x64",
        "e47695cee3f0d90623a91f3939b08bdee610eb5ad99340adff9d4145b284d6cc",
    ),
    (
        "tailwindcss-macos-arm64",
        "f3f4f5d1727e2b693b8f1c9a1492b5dd87008c86ea286bee1589cb458e6235fc",
    ),
    (
        "tailwindcss-macos-x64",
        "a63702686a3ed048b3135d84a73be49ea9d47f880c9b20e54b3ef60934ad8cdf",
    ),
    (
        "tailwindcss-windows-x64.exe",
        "2803583268c53f4713e89943e211081fddb186c0a4a7d07b5c342c6ae853ec95",
    ),
];

/// Empreinte attendue d'un asset de la CLI Tailwind.
pub fn tailwind_sha256(asset: &str) -> Option<&'static str> {
    TAILWIND_SHA256
        .iter()
        .find(|(a, _)| *a == asset)
        .map(|(_, h)| *h)
}

/// SHA-256 (hexadécimal) d'un fichier.
pub fn sha256_file(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut f = fs::File::open(path)?;
    let mut h = Sha256::new();
    std::io::copy(&mut f, &mut h)?;
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// Nom du binaire de la CLI autonome pour la plateforme courante (assets des releases tailwindlabs/tailwindcss).
pub fn tailwind_asset() -> Result<&'static str> {
    Ok(match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "tailwindcss-linux-x64",
        ("linux", "aarch64") => "tailwindcss-linux-arm64",
        ("macos", "x86_64") => "tailwindcss-macos-x64",
        ("macos", "aarch64") => "tailwindcss-macos-arm64",
        ("windows", "x86_64") => "tailwindcss-windows-x64.exe",
        (os, arch) => bail!("pas de CLI autonome Tailwind pour {os}/{arch}"),
    })
}

/// Wagtail + Starwind : télécharge la CLI autonome de Tailwind dans `.tailwind/` (une fois) puis compile
/// `tailwind/input.css` vers `<pkg>/static/css/tailwind.css` (doc : https://tailwindcss.com/blog/standalone-cli). alpharust-ignore: no-hardcoded-urls
pub(super) fn tailwind_standalone(
    target: &Path,
    name: &str,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<String> {
    let pkg = Vars::new(name)?.pkg;
    let asset = tailwind_asset()?;
    let target = &fs::canonicalize(target)?; // chemins absolus : curl et la CLI tournent dans le projet
    let dir = target.join(".tailwind");
    fs::create_dir_all(&dir)?;
    let bin = dir.join(if cfg!(windows) {
        "tailwindcss.exe"
    } else {
        "tailwindcss"
    });
    let expected = tailwind_sha256(asset)
        .ok_or_else(|| anyhow::anyhow!("empreinte SHA-256 inconnue pour {asset}"))?;
    if bin.is_file() && sha256_file(&bin)? != expected {
        on_line("CLI Tailwind en cache : empreinte différente, nouveau téléchargement");
        fs::remove_file(&bin)?;
    }
    if !bin.is_file() {
        let url = format!("{TAILWIND_RELEASES}/{TAILWIND_VERSION}/{asset}");
        on_line(&format!("téléchargement de {url}"));
        let out = exec::run(
            "curl",
            &[
                "-fsSL".into(),
                "-o".into(),
                bin.display().to_string(),
                url.clone(),
            ],
            target,
            cancel,
            std::time::Duration::from_secs(300),
            on_line,
        )?;
        if !out.success {
            let _ = fs::remove_file(&bin);
            bail!("téléchargement de la CLI Tailwind impossible ({url})");
        }
        let got = sha256_file(&bin)?;
        if got != expected {
            let _ = fs::remove_file(&bin);
            bail!("CLI Tailwind refusée : SHA-256 {got} au lieu de {expected} ({url})");
        }
        on_line(&format!("SHA-256 vérifié : {expected}"));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&bin, fs::Permissions::from_mode(0o755))?;
        }
    }
    let output = format!("{pkg}/static/css/tailwind.css");
    let out = exec::run(
        &bin.display().to_string(),
        &[
            "-i".into(),
            "tailwind/input.css".into(),
            "-o".into(),
            output.clone(),
            "--minify".into(),
        ],
        target,
        cancel,
        std::time::Duration::from_secs(300),
        on_line,
    )?;
    if !out.success {
        bail!("la CLI Tailwind a échoué sur tailwind/input.css");
    }
    ensure_gitignore(target, &[".tailwind/"])?;
    Ok(format!(
        "CLI Tailwind {TAILWIND_VERSION} ({asset}) : {output}"
    ))
}
