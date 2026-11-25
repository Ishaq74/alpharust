//! Catalogues gettext (.po → .mo) du site Wagtail, sans dépendance à GNU gettext.

use super::*;

/// Lit un catalogue gettext (.po simple : msgid/msgstr, chaînes sur plusieurs lignes, échappements C).
pub fn parse_po(text: &str) -> Vec<(String, String)> {
    fn unquote(line: &str) -> String {
        let t = line.trim();
        let inner = t
            .strip_prefix('"')
            .and_then(|t| t.strip_suffix('"'))
            .unwrap_or("");
        let mut out = String::new();
        let mut chars = inner.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some(o) => out.push(o),
                    None => {}
                }
            } else {
                out.push(c);
            }
        }
        out
    }
    let mut entries = Vec::new();
    let (mut id, mut msg, mut field) = (None::<String>, None::<String>, 0u8);
    for line in text.lines().chain(std::iter::once("")) {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("msgid ") {
            if let (Some(i), Some(m)) = (id.take(), msg.take()) {
                entries.push((i, m));
            }
            id = Some(unquote(rest));
            field = 1;
        } else if let Some(rest) = t.strip_prefix("msgstr ") {
            msg = Some(unquote(rest));
            field = 2;
        } else if t.starts_with('"') {
            let target = if field == 1 {
                id.as_mut()
            } else {
                msg.as_mut()
            };
            if let Some(s) = target {
                s.push_str(&unquote(t));
            }
        } else if t.is_empty() || t.starts_with('#') {
            if let (Some(i), Some(m)) = (id.take(), msg.take()) {
                entries.push((i, m));
            }
            field = 0;
        }
    }
    entries
}

/// Catalogue binaire .mo (format GNU gettext, sans table de hachage) : ce que produit `msgfmt` / `compilemessages`.
pub fn compile_mo(entries: &[(String, String)]) -> Vec<u8> {
    let mut e: Vec<&(String, String)> = entries.iter().filter(|(_, m)| !m.is_empty()).collect();
    e.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let n = e.len() as u32;
    let (orig_at, trans_at) = (28u32, 28 + 8 * n);
    let mut data_at = trans_at + 8 * n;
    let (mut origs, mut transs, mut blob) = (Vec::new(), Vec::new(), Vec::new());
    for (id, _) in &e {
        origs.push((id.len() as u32, data_at));
        blob.extend_from_slice(id.as_bytes());
        blob.push(0);
        data_at += id.len() as u32 + 1;
    }
    for (_, m) in &e {
        transs.push((m.len() as u32, data_at));
        blob.extend_from_slice(m.as_bytes());
        blob.push(0);
        data_at += m.len() as u32 + 1;
    }
    let mut out = Vec::new();
    for v in [
        0x9504_12de_u32,
        0,
        n,
        orig_at,
        trans_at,
        0,
        trans_at + 8 * n,
    ] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    for (len, off) in origs.iter().chain(transs.iter()) {
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&off.to_le_bytes());
    }
    out.extend_from_slice(&blob);
    out
}

/// Compile chaque `locale/<langue>/LC_MESSAGES/*.po` du projet en `.mo` (équivalent de `manage.py compilemessages`,
/// sans dépendre de GNU gettext).
pub(super) fn compile_messages(target: &Path) -> Result<String> {
    let mut n = 0;
    for entry in walkdir::WalkDir::new(target)
        .into_iter()
        .filter_entry(|e| {
            !matches!(
                e.file_name().to_str(),
                Some(".venv" | "node_modules" | ".git")
            )
        })
        .filter_map(std::result::Result::ok)
    {
        let p = entry.path();
        if p.extension().and_then(|x| x.to_str()) == Some("po")
            && p.parent()
                .and_then(|d| d.file_name())
                .and_then(|d| d.to_str())
                == Some("LC_MESSAGES")
        {
            let text = fs::read_to_string(p)?;
            fs::write(p.with_extension("mo"), compile_mo(&parse_po(&text)))?;
            n += 1;
        }
    }
    Ok(format!("{n} catalogue(s) gettext compilé(s)"))
}
