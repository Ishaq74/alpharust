//! Cache sur disque des règles parsées (`.alpharust-cache/rules.json`).
//!
//! Le parsing YAML de ~100 fichiers est évité tant que l'empreinte du dossier
//! `rules/` (chemins, tailles, dates de modification, version du binaire)
//! ne change pas. Les regex sont ensuite compilées une seule fois par exécution
//! (voir [`crate::engine::runner::CompiledRule`]).

use super::rule::Rule;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct CacheFile {
    fingerprint: u64,
    version: String,
    rules: Vec<Rule>,
}

pub fn cache_file() -> PathBuf {
    crate::paths::cache_dir().join("rules.json")
}

fn fnv(hash: &mut u64, bytes: &[u8]) {
    for b in bytes {
        *hash ^= u64::from(*b);
        *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

/// Empreinte stable (FNV-1a) du dossier de règles.
pub fn fingerprint(root: &Path) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    fnv(&mut hash, env!("CARGO_PKG_VERSION").as_bytes());
    fnv(&mut hash, root.to_string_lossy().as_bytes());
    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(meta) = entry.metadata() {
            fnv(&mut hash, entry.path().to_string_lossy().as_bytes());
            fnv(&mut hash, &meta.len().to_le_bytes());
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            fnv(&mut hash, &mtime.to_le_bytes());
        }
    }
    hash
}

pub fn load(fingerprint: u64) -> Option<Vec<Rule>> {
    let data = std::fs::read(cache_file()).ok()?;
    let file: CacheFile = serde_json::from_slice(&data).ok()?;
    (file.fingerprint == fingerprint && file.version == env!("CARGO_PKG_VERSION"))
        .then_some(file.rules)
}

/// Écrit le cache ; toute erreur d'écriture est ignorée (le cache est optionnel).
pub fn store(fingerprint: u64, rules: &[Rule]) {
    let file = CacheFile {
        fingerprint,
        version: env!("CARGO_PKG_VERSION").to_string(),
        rules: rules.to_vec(),
    };
    let path = cache_file();
    if let Some(dir) = path.parent() {
        if std::fs::create_dir_all(dir).is_err() {
            return;
        }
    }
    if let Ok(json) = serde_json::to_vec(&file) {
        let _ = std::fs::write(path, json);
    }
}

/// Supprime le cache ; renvoie `true` si un fichier a été supprimé.
pub fn clear() -> bool {
    std::fs::remove_file(cache_file()).is_ok()
}

/// (existe, taille en octets)
pub fn status() -> (bool, u64) {
    match std::fs::metadata(cache_file()) {
        Ok(m) => (true, m.len()),
        Err(_) => (false, 0),
    }
}
