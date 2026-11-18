//! Mini moteur de glob (sans dépendance externe).
//!
//! * `?`  : un caractère (hors `/`)
//! * `*`  : n'importe quelle suite de caractères hors `/`
//! * `**` : n'importe quelle suite, `/` compris ; `**/` peut aussi correspondre à « rien »
//!
//! Le mode « lâche » ([`glob_match_loose`]) fait traverser les `/` à `*`
//! (utilisé par les profils : `domains/database/*` couvre aussi les sous-dossiers).

pub fn glob_match(pattern: &str, text: &str) -> bool {
    matches(pattern.as_bytes(), text.as_bytes(), false)
}

pub fn glob_match_loose(pattern: &str, text: &str) -> bool {
    matches(pattern.as_bytes(), text.as_bytes(), true)
}

fn matches(p: &[u8], t: &[u8], loose: bool) -> bool {
    match p.first() {
        None => t.is_empty(),
        Some(b'*') => {
            let double = p.get(1) == Some(&b'*');
            if double {
                let mut rest = &p[2..];
                // `**/` : zéro ou plusieurs dossiers
                if rest.first() == Some(&b'/') {
                    rest = &rest[1..];
                    if matches(rest, t, loose) {
                        return true;
                    }
                }
                (0..=t.len()).any(|i| matches(rest, &t[i..], loose))
            } else {
                let rest = &p[1..];
                for i in 0..=t.len() {
                    if matches(rest, &t[i..], loose) {
                        return true;
                    }
                    if i < t.len() && t[i] == b'/' && !loose {
                        break;
                    }
                }
                false
            }
        }
        Some(b'?') => match t.first() {
            Some(c) if *c != b'/' => matches(&p[1..], &t[1..], loose),
            _ => false,
        },
        Some(c) => t.first() == Some(c) && matches(&p[1..], &t[1..], loose),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_does_not_cross_slash() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(!glob_match("*.rs", "src/main.rs"));
        assert!(glob_match("src/*.rs", "src/main.rs"));
    }

    #[test]
    fn double_star_crosses_slash() {
        assert!(glob_match("**/*.rs", "src/a/b.rs"));
        assert!(glob_match("**/*.rs", "b.rs"));
        assert!(glob_match("rules/**", "rules/core/no-eval.md"));
        assert!(!glob_match("rules/**", "src/rules.rs"));
    }

    #[test]
    fn question_mark() {
        assert!(glob_match("a?c", "abc"));
        assert!(!glob_match("a?c", "a/c"));
    }

    #[test]
    fn loose_mode_for_profiles() {
        assert!(glob_match_loose(
            "domains/database/*",
            "domains/database/mysql/ssl-required"
        ));
        assert!(glob_match_loose("core/*", "core/no-eval"));
        assert!(!glob_match_loose("core/*", "languages/core/x"));
        assert!(glob_match_loose("core/no-eval", "core/no-eval"));
    }
}
