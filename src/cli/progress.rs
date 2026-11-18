//! Affichage de la progression d'une génération (CLI) : étapes numérotées, commande exacte,
//! journal en direct (ligne glissante sur un terminal, lignes complètes sinon) et résumé final.

use crate::scaffold::{Event, ToolStatus};
use std::io::{IsTerminal, Write};

pub struct Printer {
    tty: bool,
    color: bool,
    verbose: bool,
    rolling: bool,
    width: usize,
}

fn term_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(110)
        .clamp(40, 200)
}

impl Printer {
    pub fn new(verbose: bool) -> Self {
        let tty = std::io::stderr().is_terminal();
        let color = tty && std::env::var_os("NO_COLOR").is_none();
        Self {
            tty,
            color,
            verbose,
            rolling: false,
            width: term_width(),
        }
    }

    fn paint(&self, code: &str, s: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }

    fn out(&self, s: &str) {
        let mut e = std::io::stderr().lock();
        let _ = e.write_all(s.as_bytes());
        let _ = e.flush();
    }

    fn clear_rolling(&mut self) {
        if self.rolling {
            self.out("\r\x1b[2K");
            self.rolling = false;
        }
    }

    fn trunc(&self, s: &str, max: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() <= max {
            s.to_string()
        } else {
            let mut t: String = chars[..max.saturating_sub(1)].iter().collect();
            t.push('…');
            t
        }
    }

    pub fn tools(&self, tools: &[ToolStatus]) -> String {
        let mut s = String::new();
        for t in tools {
            s.push_str(&tool_line(t, self.color));
            s.push('\n');
        }
        s
    }

    pub fn handle(&mut self, ev: &Event) {
        match ev {
            Event::Preflight { tools } => {
                self.out(&format!("{}\n", self.paint("1", "Outils détectés")));
                self.out(&self.tools(tools));
            }
            Event::Plan { target, steps } => {
                self.out(&format!(
                    "\n{} {} ({} étapes)\n",
                    self.paint("1", "Projet :"),
                    target,
                    steps.len()
                ));
            }
            Event::StepStart {
                index,
                total,
                label,
                phase,
                command,
                ..
            } => {
                self.clear_rolling();
                self.out(&format!(
                    "\n{} {} {}\n",
                    self.paint("36;1", &format!("[{}/{}]", index + 1, total)),
                    self.paint("1", label),
                    self.paint("2", &format!("· {}", phase.label()))
                ));
                if let Some(c) = command {
                    self.out(&format!(
                        "      {} {}\n",
                        self.paint("2", "$"),
                        self.trunc(c, self.width - 10)
                    ));
                }
            }
            Event::Log { line, .. } => {
                if self.tty && !self.verbose {
                    self.out(&format!(
                        "\r\x1b[2K      {} {}",
                        self.paint("2", "│"),
                        self.paint("2", &self.trunc(line, self.width - 10))
                    ));
                    self.rolling = true;
                } else {
                    self.out(&format!("      │ {}\n", self.trunc(line, self.width * 2)));
                }
            }
            Event::StepEnd {
                ok,
                fatal,
                millis,
                note,
                ..
            } => {
                self.clear_rolling();
                let secs = *millis as f64 / 1000.0;
                let mark = if *ok {
                    self.paint("32;1", "✔")
                } else if *fatal {
                    self.paint("31;1", "✘")
                } else {
                    self.paint("33;1", "⚠")
                };
                let note = note
                    .as_ref()
                    .map(|n| format!(" — {}", n.lines().next().unwrap_or("")))
                    .unwrap_or_default();
                self.out(&format!(
                    "      {} {}{}\n",
                    mark,
                    self.paint("2", &format!("{secs:.1}s")),
                    self.trunc(&note, self.width - 16)
                ));
            }
            Event::Done { .. } => {
                self.clear_rolling();
            }
            Event::Failed { message, .. } => {
                self.clear_rolling();
                self.out(&format!(
                    "\n{} {}\n",
                    self.paint("31;1", "✘ Échec :"),
                    message
                ));
            }
        }
    }
}

pub fn tool_line(t: &ToolStatus, color: bool) -> String {
    let (mark, code) = if t.ok {
        ("✔", "32")
    } else if t.required {
        ("✘", "31")
    } else {
        ("○", "33")
    };
    let mark = if color {
        format!("\x1b[{code};1m{mark}\x1b[0m")
    } else {
        mark.to_string()
    };
    let ver = t.version.clone().unwrap_or_else(|| "—".into());
    let min = t
        .min
        .as_ref()
        .map(|m| format!(" (≥ {m})"))
        .unwrap_or_default();
    let mut s = format!(
        "  {} {:<30} {}{}{}",
        mark,
        t.label,
        ver,
        min,
        if t.required { "" } else { "  [recommandé]" }
    );
    if !t.ok {
        s.push_str(&format!("\n      → {}", t.hint));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool(ok: bool, required: bool) -> ToolStatus {
        ToolStatus {
            id: "php".into(),
            label: "PHP".into(),
            required,
            found: ok,
            path: None,
            version: ok.then(|| "8.5.8".into()),
            min: Some("8.3.0".into()),
            ok,
            hint: "Installer PHP".into(),
        }
    }

    #[test]
    fn tool_lines_show_state_and_hint() {
        let ok = tool_line(&tool(true, true), false);
        assert!(
            ok.contains("✔") && ok.contains("8.5.8") && ok.contains("≥ 8.3.0") && !ok.contains("→")
        );
        let bad = tool_line(&tool(false, true), false);
        assert!(bad.contains("✘") && bad.contains("→ Installer PHP"));
        let rec = tool_line(&tool(false, false), false);
        assert!(rec.contains("○") && rec.contains("[recommandé]"));
    }

    #[test]
    fn truncation_is_char_safe() {
        let p = Printer::new(false);
        assert_eq!(p.trunc("abcdef", 4), "abc…");
        assert_eq!(p.trunc("éééé", 10), "éééé");
    }
}
