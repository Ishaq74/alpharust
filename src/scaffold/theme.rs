//! Génération du thème CSS (kit `concordia`) avec contrôle WCAG, d'après `Ishaq74/themecsutomizer` :
//! luminance relative, ratio de contraste, choix automatique du texte lisible (clair ou sombre),
//! variantes de composants activables (initial, retro, modern, futuristic) comme dans son export CSS.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Variantes de composants du système CSS (la variante `initial` est toujours présente).
pub const VARIANTS: &[&str] = &["initial", "retro", "modern", "futuristic"];

pub const DEFAULT_PRIMARY: &str = "#4f46e5";
pub const DEFAULT_SECONDARY: &str = "#8b5cf6";
pub const DEFAULT_ACCENT: &str = "#ec4899";

/// Couleurs de marque choisies (hexadécimal `#rrggbb`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: DEFAULT_PRIMARY.into(),
            secondary: DEFAULT_SECONDARY.into(),
            accent: DEFAULT_ACCENT.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Rating {
    #[serde(rename = "AAA")]
    Aaa,
    #[serde(rename = "AA")]
    Aa,
    #[serde(rename = "FAIL")]
    Fail,
}

impl Rating {
    pub fn label(self) -> &'static str {
        match self {
            Rating::Aaa => "AAA",
            Rating::Aa => "AA",
            Rating::Fail => "insuffisant",
        }
    }
}

/// `#rgb` ou `#rrggbb` → (r, g, b).
pub fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let h = s.strip_prefix('#')?;
    let ok = |c: char| c.is_ascii_hexdigit();
    if !h.chars().all(ok) {
        return None;
    }
    match h.len() {
        3 => {
            let v: Vec<u8> = h
                .chars()
                .map(|c| u8::from_str_radix(&format!("{c}{c}"), 16).unwrap_or(0))
                .collect();
            Some((v[0], v[1], v[2]))
        }
        6 => Some((
            u8::from_str_radix(&h[0..2], 16).ok()?,
            u8::from_str_radix(&h[2..4], 16).ok()?,
            u8::from_str_radix(&h[4..6], 16).ok()?,
        )),
        _ => None,
    }
}

pub fn to_hex(c: (u8, u8, u8)) -> String {
    format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
}

/// Luminance relative WCAG.
pub fn luminance(c: (u8, u8, u8)) -> f64 {
    let f = |v: u8| {
        let v = v as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(c.0) + 0.7152 * f(c.1) + 0.0722 * f(c.2)
}

/// Ratio de contraste WCAG (1 à 21).
pub fn contrast(a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn rating(ratio: f64) -> Rating {
    if ratio >= 7.0 {
        Rating::Aaa
    } else if ratio >= 4.5 {
        Rating::Aa
    } else {
        Rating::Fail
    }
}

const LIGHT_TEXT: (u8, u8, u8) = (255, 255, 255);
const DARK_TEXT: (u8, u8, u8) = (17, 24, 39); // #111827 comme themecsutomizer

/// Texte (blanc ou #111827) le plus lisible sur ce fond.
pub fn accessible_text(bg: (u8, u8, u8)) -> (u8, u8, u8) {
    if contrast(bg, LIGHT_TEXT) > contrast(bg, DARK_TEXT) {
        LIGHT_TEXT
    } else {
        DARK_TEXT
    }
}

/// Éclaircit/assombrit `c` vers `target` jusqu'à atteindre `min` de contraste contre `against`.
pub fn ensure_contrast(
    c: (u8, u8, u8),
    against: (u8, u8, u8),
    min: f64,
    toward: (u8, u8, u8),
) -> (u8, u8, u8) {
    let mut cur = c;
    for step in 1..=20 {
        if contrast(cur, against) >= min {
            break;
        }
        let t = step as f64 * 0.05;
        let mix = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * t).round() as u8;
        cur = (mix(c.0, toward.0), mix(c.1, toward.1), mix(c.2, toward.2));
    }
    cur
}

/// Détail d'une couleur de marque (pour l'API et le rapport).
#[derive(Debug, Clone, Serialize)]
pub struct ColorReport {
    pub name: &'static str,
    pub hex: String,
    /// Texte conseillé sur ce fond.
    pub text: String,
    pub ratio: f64,
    pub rating: Rating,
    /// Couleur adaptée au mode sombre (lisible sur #0f172a).
    pub dark_hex: String,
    pub dark_ratio: f64,
}

pub fn report(name: &'static str, hex: &str) -> Result<ColorReport> {
    let Some(rgb) = parse_hex(hex) else {
        bail!("couleur `{hex}` invalide (attendu : #rrggbb)");
    };
    let text = accessible_text(rgb);
    let ratio = contrast(rgb, text);
    let dark_bg = (15, 23, 42); // #0f172a
    let dark = ensure_contrast(rgb, dark_bg, 4.5, (255, 255, 255));
    Ok(ColorReport {
        name,
        hex: to_hex(rgb),
        text: to_hex(text),
        ratio: (ratio * 100.0).round() / 100.0,
        rating: rating(ratio),
        dark_hex: to_hex(dark),
        dark_ratio: (contrast(dark, dark_bg) * 100.0).round() / 100.0,
    })
}

impl ThemeColors {
    pub fn validate(&self) -> Result<()> {
        for (n, v) in [
            ("primary", &self.primary),
            ("secondary", &self.secondary),
            ("accent", &self.accent),
        ] {
            if parse_hex(v).is_none() {
                bail!("couleur `{n}` invalide : `{v}` (attendu : #rrggbb)");
            }
        }
        Ok(())
    }

    pub fn reports(&self) -> Result<Vec<ColorReport>> {
        Ok(vec![
            report("primary", &self.primary)?,
            report("secondary", &self.secondary)?,
            report("accent", &self.accent)?,
        ])
    }

    /// Feuille `theme.css` : couleurs de marque, textes lisibles et versions sombres.
    pub fn css(&self) -> Result<String> {
        self.validate()?;
        let r = self.reports()?;
        let mut out = String::new();
        out.push_str("/* Thème généré par AlphaRust (contrôle WCAG façon themecsutomizer).\n");
        out.push_str(
            "   Les textes sur fond de marque sont choisis pour le meilleur contraste. */\n",
        );
        out.push_str(":root {\n");
        for c in &r {
            out.push_str(&format!("  --color-{}: {};\n", c.name, c.hex));
        }
        for c in &r {
            out.push_str(&format!("  --button-{}-color: {};\n", c.name, c.text));
            out.push_str(&format!(
                "  --on-{}: {}; /* contraste {:.2}:1 ({}) */\n",
                c.name,
                c.text,
                c.ratio,
                c.rating.label()
            ));
        }
        out.push_str("}\n\n");
        out.push_str(":root[data-theme='dark'] {\n");
        for c in &r {
            out.push_str(&format!("  --color-{}: {};\n", c.name, c.dark_hex));
        }
        for c in &r {
            let t = accessible_text(parse_hex(&c.dark_hex).unwrap_or((0, 0, 0)));
            out.push_str(&format!("  --button-{}-color: {};\n", c.name, to_hex(t)));
            out.push_str(&format!("  --on-{}: {};\n", c.name, to_hex(t)));
        }
        out.push_str("}\n");
        Ok(out)
    }

    /// Résumé lisible pour le journal.
    pub fn summary(&self) -> Result<String> {
        Ok(self
            .reports()?
            .iter()
            .map(|c| {
                format!(
                    "{} {} → texte {} {:.1}:1 ({})",
                    c.name,
                    c.hex,
                    c.text,
                    c.ratio,
                    c.rating.label()
                )
            })
            .collect::<Vec<_>>()
            .join(" · "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_matches_wcag_reference_values() {
        let black = (0, 0, 0);
        let white = (255, 255, 255);
        assert!((contrast(black, white) - 21.0).abs() < 0.01);
        assert!((contrast(white, white) - 1.0).abs() < 1e-9);
        // gris #767676 sur blanc = 4.54 (seuil AA classique)
        let g = parse_hex("#767676").unwrap();
        assert!((contrast(g, white) - 4.54).abs() < 0.02);
        assert_eq!(rating(4.54), Rating::Aa);
        assert_eq!(rating(7.2), Rating::Aaa);
        assert_eq!(rating(3.0), Rating::Fail);
    }

    #[test]
    fn hex_parsing() {
        assert_eq!(parse_hex("#fff"), Some((255, 255, 255)));
        assert_eq!(parse_hex("#4f46e5"), Some((79, 70, 229)));
        assert!(parse_hex("4f46e5").is_none());
        assert!(parse_hex("#12345").is_none());
        assert!(parse_hex("#gggggg").is_none());
        assert_eq!(to_hex((79, 70, 229)), "#4f46e5");
    }

    #[test]
    fn accessible_text_picks_the_readable_side() {
        assert_eq!(accessible_text((255, 255, 0)), DARK_TEXT);
        assert_eq!(accessible_text((10, 10, 80)), LIGHT_TEXT);
    }

    #[test]
    fn dark_variant_reaches_aa_on_the_dark_background() {
        let r = report("primary", "#1e1b4b").unwrap();
        assert!(r.dark_ratio >= 4.5, "{r:?}");
        let r = report("primary", DEFAULT_PRIMARY).unwrap();
        assert!(r.ratio >= 4.5 && r.rating != Rating::Fail);
    }

    #[test]
    fn css_contains_tokens_and_rejects_bad_colors() {
        let css = ThemeColors::default().css().unwrap();
        assert!(css.contains("--color-primary: #4f46e5;"));
        assert!(css.contains("[data-theme='dark']"));
        assert!(css.contains("--button-primary-color: #ffffff;"));
        let bad = ThemeColors {
            primary: "red".into(),
            ..Default::default()
        };
        assert!(bad.css().is_err());
    }
}
