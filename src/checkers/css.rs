//! Contraste WCAG : compare `color` et `background(-color)` déclarés dans un même bloc `{ }`.

use super::{line_of, FileCtx, Finding};

fn named(name: &str) -> Option<(f64, f64, f64)> {
    let c = match name {
        "white" => (255, 255, 255),
        "black" => (0, 0, 0),
        "red" => (255, 0, 0),
        "yellow" => (255, 255, 0),
        "gray" | "grey" => (128, 128, 128),
        "silver" => (192, 192, 192),
        "lightgray" | "lightgrey" => (211, 211, 211),
        "navy" => (0, 0, 128),
        "blue" => (0, 0, 255),
        "green" => (0, 128, 0),
        "orange" => (255, 165, 0),
        _ => return None,
    };
    Some((f64::from(c.0), f64::from(c.1), f64::from(c.2)))
}

/// Analyse `#rgb`, `#rrggbb`, `rgb(r,g,b)` ou un nom de couleur courant.
pub fn parse_color(value: &str) -> Option<(f64, f64, f64)> {
    let v = value
        .trim()
        .trim_end_matches("!important")
        .trim()
        .to_lowercase();
    if let Some(hex) = v.strip_prefix('#') {
        let hex: String = hex.chars().take_while(char::is_ascii_hexdigit).collect();
        let expanded = match hex.len() {
            3 => hex.chars().flat_map(|c| [c, c]).collect::<String>(),
            6 | 8 => hex[..6].to_string(),
            _ => return None,
        };
        let n = u32::from_str_radix(&expanded, 16).ok()?;
        return Some((
            f64::from(n >> 16 & 255),
            f64::from(n >> 8 & 255),
            f64::from(n & 255),
        ));
    }
    if let Some(inner) = v.strip_prefix("rgb(").or_else(|| v.strip_prefix("rgba(")) {
        let inner = inner.split(')').next()?;
        let nums: Vec<f64> = inner
            .split([',', ' ', '/'])
            .filter(|s| !s.is_empty())
            .take(3)
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() == 3 {
            return Some((nums[0], nums[1], nums[2]));
        }
        return None;
    }
    named(v.split_whitespace().next()?)
}

fn luminance((r, g, b): (f64, f64, f64)) -> f64 {
    let f = |c: f64| {
        let c = c / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b)
}

/// Rapport de contraste WCAG (1.0 à 21.0).
pub fn contrast_ratio(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn contrast(ctx: &FileCtx) -> Vec<Finding> {
    let min: f64 = ctx
        .args
        .string("min")
        .and_then(|s| s.parse().ok())
        .unwrap_or(4.5);
    let mut out = Vec::new();
    let content = ctx.content;
    let mut search = 0;
    while let Some(open) = content[search..].find('{').map(|i| i + search) {
        let Some(close) = content[open..].find('}').map(|i| i + open) else {
            break;
        };
        let block = &content[open + 1..close];
        let mut fg = None;
        let mut bg = None;
        let mut offset = open + 1;
        for decl in block.split(';') {
            if let Some((prop, value)) = decl.split_once(':') {
                let prop = prop.trim().to_lowercase();
                // `color: #fff` (pas `background-color`), `background(-color)`
                if prop == "color" {
                    if let Some(c) = parse_color(value) {
                        fg = Some((c, offset + decl.find(':').unwrap_or(0)));
                    }
                } else if prop == "background" || prop == "background-color" {
                    if let Some(c) = value.split_whitespace().find_map(parse_color) {
                        bg = Some(c);
                    }
                }
            }
            offset += decl.len() + 1;
        }
        if let (Some((fg, pos)), Some(bg)) = (fg, bg) {
            let ratio = contrast_ratio(fg, bg);
            if ratio < min {
                out.push(Finding::new(
                    line_of(content, pos),
                    format!("contraste insuffisant : {ratio:.2}:1 (minimum {min:.1}:1)"),
                ));
            }
        }
        search = close + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn run(css: &str) -> Vec<Finding> {
        let args = BTreeMap::new();
        contrast(&FileCtx {
            path: Path::new("a.css"),
            content: css,
            args: Args(&args),
        })
    }

    #[test]
    fn ratio_black_white_is_21() {
        let r = contrast_ratio((0.0, 0.0, 0.0), (255.0, 255.0, 255.0));
        assert!((r - 21.0).abs() < 0.01);
    }

    #[test]
    fn flags_low_contrast_only() {
        let css = ".a {\n  color: #777;\n  background: #888;\n}\n.b { color: #000; background-color: #fff; }";
        let f = run(css);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].line, 2);
    }

    #[test]
    fn parses_color_formats() {
        assert_eq!(parse_color("#fff"), Some((255.0, 255.0, 255.0)));
        assert_eq!(parse_color("rgb(1, 2, 3)"), Some((1.0, 2.0, 3.0)));
        assert_eq!(parse_color("white"), Some((255.0, 255.0, 255.0)));
        assert_eq!(parse_color("var(--x)"), None);
    }

    #[test]
    fn background_shorthand_with_url_is_handled() {
        let css = ".a { color: #fff; background: url(x.png) #fff no-repeat; }";
        assert_eq!(run(css).len(), 1);
    }
}
