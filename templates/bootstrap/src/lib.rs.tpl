//! @@title@@

/// Message d'accueil affiché par le binaire.
pub fn greeting(name: &str) -> String {
    format!("Hello from {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_mentions_name() {
        assert_eq!(greeting("demo"), "Hello from demo");
    }

    #[test]
    fn greeting_is_never_empty() {
        assert!(!greeting("").is_empty());
    }
}
