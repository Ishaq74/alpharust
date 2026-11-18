//! Analyse de balises HTML « légère » (sans DOM complet) : suffisante pour HTML,
//! Astro, Vue, Svelte, JSX/TSX, Blade, templates Django…

use super::{line_of, FileCtx, Finding};

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub attrs: Vec<(String, Option<String>)>,
    pub line: usize,
    pub closing: bool,
    /// Contient un spread dynamique (`{...props}`) : les attributs sont inconnus.
    pub has_spread: bool,
}

impl Tag {
    pub fn attr(&self, name: &str) -> Option<&Option<String>> {
        self.attrs
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v)
    }
}

/// Extrait les balises ouvrantes/fermantes en ignorant commentaires `<!-- -->`
/// et contenus de `<script>` / `<style>`.
pub fn tags(content: &str) -> Vec<Tag> {
    let b = content.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'<' {
            i += 1;
            continue;
        }
        if content[i..].starts_with("<!--") {
            i = content[i..].find("-->").map_or(b.len(), |e| i + e + 3);
            continue;
        }
        let closing = b.get(i + 1) == Some(&b'/');
        let name_start = i + 1 + usize::from(closing);
        if !b.get(name_start).is_some_and(|c| c.is_ascii_alphabetic()) {
            i += 1;
            continue;
        }
        let mut j = name_start;
        while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'-' || b[j] == b':') {
            j += 1;
        }
        let name = content[name_start..j].to_ascii_lowercase();
        let line = line_of(content, i);
        // Fin de balise : `>` hors guillemets et hors accolades.
        let (mut quote, mut depth) = (0u8, 0i32);
        let mut k = j;
        while k < b.len() {
            let c = b[k];
            if quote != 0 {
                if c == quote {
                    quote = 0;
                }
            } else if c == b'"' || c == b'\'' {
                quote = c;
            } else if c == b'{' {
                depth += 1;
            } else if c == b'}' {
                depth -= 1;
            } else if c == b'>' && depth <= 0 {
                break;
            }
            k += 1;
        }
        let raw = &content[j..k.min(content.len())];
        let (attrs, has_spread) = parse_attrs(raw);
        out.push(Tag {
            name: name.clone(),
            attrs,
            line,
            closing,
            has_spread,
        });
        i = (k + 1).min(b.len());
        if !closing && (name == "script" || name == "style") {
            let end_tag = format!("</{}", name);
            let lower = content[i..].to_ascii_lowercase();
            i = lower.find(&end_tag).map_or(b.len(), |e| i + e);
        }
    }
    out
}

fn parse_attrs(raw: &str) -> (Vec<(String, Option<String>)>, bool) {
    let b = raw.as_bytes();
    let mut attrs = Vec::new();
    let mut spread = false;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() || c == b'/' {
            i += 1;
            continue;
        }
        if c == b'{' {
            // `{...props}` ou expression nue
            let mut depth = 0;
            let start = i;
            while i < b.len() {
                if b[i] == b'{' {
                    depth += 1;
                } else if b[i] == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            if raw[start..i.min(raw.len())].contains("...") {
                spread = true;
            }
            continue;
        }
        let start = i;
        while i < b.len() && !b[i].is_ascii_whitespace() && b[i] != b'=' && b[i] != b'/' {
            i += 1;
        }
        let name = raw[start..i].to_string();
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        let mut value = None;
        if i < b.len() && b[i] == b'=' {
            i += 1;
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < b.len() && (b[i] == b'"' || b[i] == b'\'') {
                let q = b[i];
                let vs = i + 1;
                i = vs;
                while i < b.len() && b[i] != q {
                    i += 1;
                }
                value = Some(raw[vs..i.min(raw.len())].to_string());
                i += 1;
            } else if i < b.len() && b[i] == b'{' {
                let vs = i;
                let mut depth = 0;
                while i < b.len() {
                    if b[i] == b'{' {
                        depth += 1;
                    } else if b[i] == b'}' {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
                value = Some(raw[vs..i.min(raw.len())].to_string());
            } else {
                let vs = i;
                while i < b.len() && !b[i].is_ascii_whitespace() {
                    i += 1;
                }
                value = Some(raw[vs..i].to_string());
            }
        }
        if !name.is_empty() {
            attrs.push((name, value));
        }
    }
    (attrs, spread)
}

pub fn img_alt(ctx: &FileCtx) -> Vec<Finding> {
    tags(ctx.content)
        .into_iter()
        .filter(|t| !t.closing && t.name == "img" && !t.has_spread && t.attr("alt").is_none())
        .map(|t| {
            Finding::new(
                t.line,
                "image sans attribut `alt` (utiliser alt=\"\" si décorative)",
            )
        })
        .collect()
}

pub fn html_lang(ctx: &FileCtx) -> Vec<Finding> {
    tags(ctx.content)
        .into_iter()
        .filter(|t| !t.closing && t.name == "html" && !t.has_spread && t.attr("lang").is_none())
        .map(|t| Finding::new(t.line, "<html> sans attribut `lang`"))
        .collect()
}

pub fn heading_order(ctx: &FileCtx) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut prev = 0u8;
    let mut h1_count = 0;
    for t in tags(ctx.content).into_iter().filter(|t| !t.closing) {
        let bytes = t.name.as_bytes();
        if bytes.len() == 2 && bytes[0] == b'h' && (b'1'..=b'6').contains(&bytes[1]) {
            let level = bytes[1] - b'0';
            if level == 1 {
                h1_count += 1;
                if h1_count > 1 {
                    out.push(Finding::new(t.line, "plusieurs <h1> dans le même document"));
                }
            }
            if prev != 0 && level > prev + 1 {
                out.push(Finding::new(
                    t.line,
                    format!("niveau de titre sauté : <h{}> après <h{}>", level, prev),
                ));
            }
            prev = level;
        }
    }
    out
}

pub fn semantic(ctx: &FileCtx) -> Vec<Finding> {
    const LANDMARKS: [&str; 7] = [
        "main", "header", "nav", "footer", "article", "section", "aside",
    ];
    let all = tags(ctx.content);
    let Some(body) = all.iter().find(|t| !t.closing && t.name == "body") else {
        return Vec::new();
    };
    if all.iter().any(|t| LANDMARKS.contains(&t.name.as_str())) {
        return Vec::new();
    }
    vec![Finding::new(
        body.line,
        "aucune balise sémantique (<main>, <header>, <nav>, <footer>…) dans la page",
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn run(f: fn(&FileCtx) -> Vec<Finding>, content: &str) -> Vec<Finding> {
        let map = BTreeMap::new();
        f(&FileCtx {
            path: Path::new("a.html"),
            content,
            args: Args(&map),
        })
    }

    #[test]
    fn img_alt_detects_missing_alt_even_multiline() {
        let html = "<img src=\"a.png\">\n<img\n  src=\"b.png\"\n  class=\"x\"\n>\n<img src=c.png alt=\"ok\">\n<img src=d.png alt=\"\">";
        let f = run(img_alt, html);
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].line, 1);
        assert_eq!(f[1].line, 2);
    }

    #[test]
    fn img_alt_ignores_comments_scripts_and_spreads() {
        let html = "<!-- <img src=x> -->\n<script>const s = '<img src=y>';</script>\n<img {...props}>\n<img src={a > b ? x : y} alt={t}>";
        assert!(run(img_alt, html).is_empty());
    }

    #[test]
    fn lang_attribute() {
        assert_eq!(run(html_lang, "<html><body></body></html>").len(), 1);
        assert!(run(html_lang, "<html lang=\"fr\"></html>").is_empty());
    }

    #[test]
    fn heading_order_flags_skips_and_multiple_h1() {
        let f = run(heading_order, "<h1>a</h1>\n<h3>b</h3>\n<h1>c</h1>");
        assert_eq!(f.len(), 2);
        assert!(f[0].message.contains("h3"));
        assert!(run(heading_order, "<h1>a</h1><h2>b</h2><h3>c</h3><h2>d</h2>").is_empty());
    }

    #[test]
    fn semantic_requires_landmarks() {
        assert_eq!(run(semantic, "<body><div>x</div></body>").len(), 1);
        assert!(run(semantic, "<body><main>x</main></body>").is_empty());
        assert!(run(semantic, "<div>fragment</div>").is_empty());
    }
}
