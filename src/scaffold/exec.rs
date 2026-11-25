//! Exécution de commandes externes avec capture ligne à ligne (journal en direct), annulation et délai maximal.

use serde::Serialize;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Dossier de travail d'une commande.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Cwd {
    /// Dossier parent du projet (ex. `sites/`) : le projet n'existe pas encore.
    Parent,
    /// Racine du projet.
    Project,
}

/// Une commande externe à lancer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Cmd {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Cwd,
}

impl Cmd {
    pub fn new(program: &str, args: &[&str], cwd: Cwd) -> Self {
        Self {
            program: program.into(),
            args: args.iter().map(std::string::ToString::to_string).collect(),
            cwd,
        }
    }

    pub fn owned(program: &str, args: Vec<String>, cwd: Cwd) -> Self {
        Self {
            program: program.into(),
            args,
            cwd,
        }
    }

    /// Représentation lisible (quotée) de la commande, pour l'aperçu et le journal.
    pub fn display(&self) -> String {
        let mut parts = vec![quote(&self.program)];
        parts.extend(self.args.iter().map(|a| quote(a)));
        parts.join(" ")
    }
}

fn quote(s: &str) -> String {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./=:@+,%^".contains(c))
    {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}

/// Supprime les séquences ANSI (couleurs, déplacements de curseur).
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    for n in chars.by_ref() {
                        if ('@'..='~').contains(&n) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    for n in chars.by_ref() {
                        if n == '\u{7}' {
                            break;
                        }
                    }
                }
                _ => {
                    chars.next();
                }
            }
        } else if c != '\r' {
            out.push(c);
        }
    }
    out
}

/// Découpe un flux en lignes sur `\n` **et** `\r` (barres de progression) puis nettoie chaque ligne.
fn read_lines<R: Read>(reader: R, tx: mpsc::Sender<String>) {
    let mut buf = BufReader::new(reader);
    let mut chunk = Vec::new();
    loop {
        chunk.clear();
        match buf.read_until(b'\n', &mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let text = String::from_utf8_lossy(&chunk).into_owned();
                for piece in text.split('\r') {
                    let line = strip_ansi(piece.trim_end_matches('\n'));
                    if !line.trim().is_empty() && tx.send(line.trim_end().to_string()).is_err() {
                        return;
                    }
                }
            }
        }
    }
}

/// Une ligne est-elle du « bruit » d'animation (spinner, cadre) à ne pas journaliser ?
fn is_noise(line: &str) -> bool {
    let t = line.trim();
    t.chars().all(|c| "█▓▒░▀▄■□▶▷◼◻●○│─╭╮╰╯┌┐└┘ .·".contains(c))
}

/// Résultat d'une commande terminée.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub success: bool,
    pub code: Option<i32>,
    pub millis: u128,
    pub cancelled: bool,
    pub timed_out: bool,
    /// Dernières lignes de sortie (pour les messages d'erreur).
    pub tail: Vec<String>,
}

/// Variables d'environnement imposées aux sous-processus : pas d'interaction, pas de couleurs, pas de télémétrie.
pub fn child_env() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CI", "1"),
        ("NO_COLOR", "1"),
        ("FORCE_COLOR", "0"),
        ("TERM", "dumb"),
        ("npm_config_yes", "true"),
        ("npm_config_fund", "false"),
        ("npm_config_audit", "false"),
        ("npm_config_update_notifier", "false"),
        ("ASTRO_TELEMETRY_DISABLED", "1"),
        ("DO_NOT_TRACK", "1"),
        ("COMPOSER_NO_INTERACTION", "1"),
        ("COMPOSER_ALLOW_SUPERUSER", "1"),
        ("PIP_DISABLE_PIP_VERSION_CHECK", "1"),
        ("PYTHONUNBUFFERED", "1"),
        ("CARGO_TERM_COLOR", "never"),
        ("GIT_TERMINAL_PROMPT", "0"),
    ]
}

/// Délai maximal par commande (variable `ALPHARUST_STEP_TIMEOUT`, en secondes ; défaut 20 minutes).
pub fn step_timeout() -> Duration {
    let secs = std::env::var("ALPHARUST_STEP_TIMEOUT")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1200);
    Duration::from_secs(secs.max(1))
}

fn kill(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// Lance `program args…` dans `cwd`, appelle `on_line` pour chaque ligne et attend la fin.
pub fn run(
    program: &str,
    args: &[String],
    cwd: &Path,
    cancel: &AtomicBool,
    timeout: Duration,
    on_line: &mut dyn FnMut(&str),
) -> std::io::Result<Outcome> {
    let resolved: PathBuf = if program.starts_with("./") {
        // chemin absolu : avec un `--path` relatif, Command résoudrait le programme une seconde fois depuis current_dir
        let p = cwd.join(program);
        if p.is_relative() {
            std::env::current_dir()?.join(p)
        } else {
            p
        }
    } else {
        super::tools::which(program).unwrap_or_else(|| PathBuf::from(program))
    };
    let start = Instant::now();
    let mut command = Command::new(&resolved);
    command
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (k, v) in child_env() {
        command.env(k, v);
    }
    let mut child = command.spawn()?;
    let (tx, rx) = mpsc::channel::<String>();
    let mut handles = Vec::new();
    if let Some(out) = child.stdout.take() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || read_lines(out, tx)));
    }
    if let Some(err) = child.stderr.take() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || read_lines(err, tx)));
    }
    drop(tx);

    let mut tail: std::collections::VecDeque<String> = Default::default();
    let mut last = String::new();
    let mut cancelled = false;
    let mut timed_out = false;
    let mut handle_line =
        |line: String,
         on_line: &mut dyn FnMut(&str),
         tail: &mut std::collections::VecDeque<String>| {
            if is_noise(&line) || line == last {
                return;
            }
            last = line.clone();
            on_line(&line);
            tail.push_back(line);
            if tail.len() > 25 {
                tail.pop_front();
            }
        };
    let status = loop {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(line) => handle_line(line, on_line, &mut tail),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // tous les flux sont fermés : le processus se termine
                break child.wait()?;
            }
        }
        if cancel.load(Ordering::Relaxed) {
            cancelled = true;
            kill(&mut child);
            break child.wait()?;
        }
        if start.elapsed() > timeout {
            timed_out = true;
            kill(&mut child);
            break child.wait()?;
        }
    };
    for h in handles {
        let _ = h.join();
    }
    while let Ok(line) = rx.try_recv() {
        handle_line(line, on_line, &mut tail);
    }
    Ok(Outcome {
        success: status.success() && !cancelled && !timed_out,
        code: status.code(),
        millis: start.elapsed().as_millis(),
        cancelled,
        timed_out,
        tail: tail.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(script: &str) -> (Outcome, Vec<String>) {
        let mut lines = Vec::new();
        let cancel = AtomicBool::new(false);
        let out = run(
            "sh",
            &["-c".into(), script.into()],
            Path::new("."),
            &cancel,
            Duration::from_secs(10),
            &mut |l| lines.push(l.to_string()),
        )
        .unwrap();
        (out, lines)
    }

    #[test]
    fn strips_ansi_and_carriage_returns() {
        assert_eq!(strip_ansi("\u{1b}[31mrouge\u{1b}[0m"), "rouge");
        assert_eq!(strip_ansi("a\u{1b}[2K\u{1b}[1Ab"), "ab");
        assert_eq!(strip_ansi("x\r"), "x");
    }

    #[test]
    fn quoting() {
        let c = Cmd::new("npm", &["create", "astro@latest", "mon site"], Cwd::Parent);
        assert_eq!(c.display(), "npm create astro@latest 'mon site'");
    }

    #[cfg(unix)]
    #[test]
    fn captures_stdout_stderr_and_exit_code() {
        let (out, lines) = sh("echo un; echo deux 1>&2; exit 3");
        assert_eq!(out.code, Some(3));
        assert!(!out.success);
        assert!(lines.contains(&"un".to_string()) && lines.contains(&"deux".to_string()));
        assert_eq!(out.tail.len(), 2);
    }

    #[cfg(unix)]
    #[test]
    fn splits_carriage_return_progress_and_drops_noise() {
        let (out, lines) = sh("printf 'a\\rb\\rb\\n███ \\n  \\nfin\\n'");
        assert!(out.success);
        assert_eq!(lines, vec!["a", "b", "fin"]);
    }

    #[cfg(unix)]
    #[test]
    fn times_out_and_cancels() {
        let cancel = AtomicBool::new(false);
        let out = run(
            "sh",
            &["-c".into(), "sleep 5".into()],
            Path::new("."),
            &cancel,
            Duration::from_millis(200),
            &mut |_| {},
        )
        .unwrap();
        assert!(out.timed_out && !out.success);

        let cancel = AtomicBool::new(true);
        let out = run(
            "sh",
            &["-c".into(), "sleep 5".into()],
            Path::new("."),
            &cancel,
            Duration::from_secs(10),
            &mut |_| {},
        )
        .unwrap();
        assert!(out.cancelled && !out.success);
    }

    #[test]
    fn missing_program_is_an_io_error() {
        let cancel = AtomicBool::new(false);
        let r = run(
            "alpharust-does-not-exist-xyz",
            &[],
            Path::new("."),
            &cancel,
            Duration::from_secs(1),
            &mut |_| {},
        );
        assert!(r.is_err());
    }
}
