//! Lexer Rust minimal : permet de chercher des mots-clés sans faux positifs
//! dans les commentaires, chaînes, chaînes brutes et littéraux de caractères.

use super::{FileCtx, Finding};

/// Renvoie les lignes (1-indexées) où l'identifiant `ident` apparaît comme *code*.
pub fn find_ident(src: &str, ident: &str) -> Vec<usize> {
    let b = src.as_bytes();
    let mut hits = Vec::new();
    let (mut i, mut line) = (0usize, 1usize);
    while i < b.len() {
        let c = b[i];
        match c {
            b'\n' => {
                line += 1;
                i += 1;
            }
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let mut depth = 1;
                i += 2;
                while i < b.len() && depth > 0 {
                    if b[i] == b'\n' {
                        line += 1;
                    }
                    if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    if b[i] == b'\\' {
                        i += 1;
                    }
                    if b.get(i) == Some(&b'\n') {
                        line += 1;
                    }
                    i += 1;
                }
                i += 1;
            }
            b'r' if matches!(b.get(i + 1), Some(b'"') | Some(b'#')) && is_raw_string(b, i) => {
                let mut j = i + 1;
                let mut hashes = 0;
                while b.get(j) == Some(&b'#') {
                    hashes += 1;
                    j += 1;
                }
                j += 1; // guillemet ouvrant
                loop {
                    if j >= b.len() {
                        break;
                    }
                    if b[j] == b'\n' {
                        line += 1;
                    }
                    if b[j] == b'"' && (0..hashes).all(|k| b.get(j + 1 + k) == Some(&b'#')) {
                        j += 1 + hashes;
                        break;
                    }
                    j += 1;
                }
                i = j;
            }
            b'\'' => {
                // littéral de caractère ('a', '\n') vs lifetime ('a)
                if b.get(i + 1) == Some(&b'\\') {
                    i += 2;
                    while i < b.len() && b[i] != b'\'' {
                        i += 1;
                    }
                    i += 1;
                } else if b.get(i + 2) == Some(&b'\'') {
                    i += 3;
                } else if let Some(ch) = src[i + 1..].chars().next() {
                    // caractère multi-octets 'é'
                    let len = ch.len_utf8();
                    if b.get(i + 1 + len) == Some(&b'\'') {
                        i += 2 + len;
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                if &src[start..i] == ident {
                    hits.push(line);
                }
            }
            _ => i += 1,
        }
    }
    hits
}

fn is_raw_string(b: &[u8], i: usize) -> bool {
    // doit être précédé d'un non-identifiant et suivi de #* puis "
    if i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        return false;
    }
    let mut j = i + 1;
    while b.get(j) == Some(&b'#') {
        j += 1;
    }
    b.get(j) == Some(&b'"')
}

pub fn unsafe_usage(ctx: &FileCtx) -> Vec<Finding> {
    find_ident(ctx.content, "unsafe")
        .into_iter()
        .map(|line| Finding::new(line, "bloc ou fonction `unsafe` : à justifier ou à éviter"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_real_unsafe() {
        let src = "fn a() {\n    unsafe { x() }\n}\nunsafe fn b() {}\nunsafe impl Send for X {}";
        assert_eq!(find_ident(src, "unsafe"), vec![2, 4, 5]);
    }

    #[test]
    fn ignores_comments_strings_and_similar_idents() {
        let src = r##"
// unsafe in line comment
/* unsafe /* nested unsafe */ still comment unsafe */
let s = "unsafe in string \" unsafe";
let r = r#"unsafe "quoted" unsafe"#;
#![forbid(unsafe_code)]
let c = 'u';
let lifetime: &'a str = "";
"##;
        assert!(find_ident(src, "unsafe").is_empty());
    }

    #[test]
    fn lines_are_correct_after_multiline_constructs() {
        let src = "/* a\nb\nc */\nlet s = \"x\ny\";\nunsafe {}";
        assert_eq!(find_ident(src, "unsafe"), vec![6]);
    }
}
