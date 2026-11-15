use crate::parser::Rule;

/// Applique les priorités : framework > domain > language > core
pub fn compose(rules: Vec<Rule>) -> Vec<Rule> {
    // Pour l’instant on retourne tout ; la logique de priorité fine viendra avec les profils
    rules
}
