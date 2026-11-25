//! Dépôt Git du projet généré : .gitignore des fichiers locaux, init et commit initial.

use super::*;

pub(super) fn git_cfg(target: &Path, key: &str) -> bool {
    crate::scaffold::tools::which("git")
        .and_then(|g| {
            std::process::Command::new(g)
                .args(["-C", &target.display().to_string(), "config", key])
                .output()
                .ok()
        })
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}

/// Fichiers locaux jamais versionnés, quel que soit le `.gitignore` officiel conservé : bases SQLite de
/// développement (`data/<nom>.db` de la plateforme Astro, journaux WAL/SHM compris) et messages de contact
/// du site Astro sans base (`data/contact-messages.jsonl`).
pub const LOCAL_ONLY: &[&str] = &[
    "data/*.db",
    "data/*.db-journal",
    "data/*.db-wal",
    "data/*.db-shm",
    "data/*.jsonl",
];

/// Ajoute au `.gitignore` du projet les motifs absents (sans toucher aux lignes existantes).
pub fn ensure_gitignore(target: &Path, patterns: &[&str]) -> Result<()> {
    let gi = target.join(".gitignore");
    let mut text = fs::read_to_string(&gi).unwrap_or_default();
    let missing: Vec<&str> = patterns
        .iter()
        .copied()
        .filter(|p| !text.lines().any(|l| l.trim() == *p))
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    for p in missing {
        text.push_str(p);
        text.push('\n');
    }
    fs::write(&gi, text)?;
    Ok(())
}

pub(super) fn git_init(
    target: &Path,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<String> {
    if crate::scaffold::tools::which("git").is_none() {
        bail!("git introuvable (dépôt non initialisé)");
    }
    let t = std::time::Duration::from_secs(120);
    let step = |args: Vec<String>, on_line: &mut dyn FnMut(&str)| -> Result<()> {
        on_line(&format!("$ git {}", args.join(" ")));
        let o = exec::run("git", &args, target, cancel, t, on_line)?;
        if !o.success {
            bail!("`git {}` a échoué\n{}", args.join(" "), short_tail(&o.tail));
        }
        Ok(())
    };
    step(vec!["init".into(), "-q".into()], on_line)?;
    // branche `main` quelle que soit la version de git
    step(
        vec![
            "symbolic-ref".into(),
            "HEAD".into(),
            "refs/heads/main".into(),
        ],
        on_line,
    )?;
    ensure_gitignore(target, LOCAL_ONLY)?;
    step(vec!["add".into(), "-A".into()], on_line)?;
    let mut commit: Vec<String> = Vec::new();
    if !git_cfg(target, "user.name") || !git_cfg(target, "user.email") {
        commit.extend(
            [
                "-c",
                "user.name=AlphaRust",
                "-c",
                "user.email=alpharust@users.noreply.local",
            ]
            .map(String::from),
        );
    }
    commit.extend(
        [
            "commit",
            "-q",
            "-m",
            "chore: projet initial généré par AlphaRust",
        ]
        .map(String::from),
    );
    step(commit, on_line)?;
    Ok("dépôt initialisé (branche main)".into())
}
