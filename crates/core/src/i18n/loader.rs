//! New Config-Driven i18n Module
//!
//! Loads translations from JSON files instead of hardcoded trait implementations.
//! Provides compatibility layer to coexist with legacy system during migration.

use serde::Deserialize;
use std::collections::HashMap;

/// Translations loader from JSON files
#[derive(Debug, Deserialize)]
pub struct Translations {
    #[serde(flatten)]
    data: HashMap<String, serde_json::Value>,
}

impl Translations {
    /// Load translations for a given language code
    pub fn load(lang: &str) -> Result<Self, TranslationError> {
        let json = match lang {
            "en" => include_str!("../../translations/en.json"),
            "es" => include_str!("../../translations/es.json"),
            "pt-br" => include_str!("../../translations/pt-br.json"),
            _ => return Err(TranslationError::UnknownLanguage(lang.to_string())),
        };
        serde_json::from_str(json).map_err(TranslationError::ParseError)
    }

    /// Get a translation by key
    pub fn get(&self, key: &str) -> String {
        self.data
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("[MISSING: {}]", key))
    }

    /// Get a translation, returning None if not found (for fallback logic)
    pub fn get_optional(&self, key: &str) -> Option<String> {
        self.data
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Format a translation with interpolation
    /// Example: "Hello {name}" + [("name", "John")] = "Hello John"
    pub fn format(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.get(key);
        for (k, v) in args {
            result = result.replace(&format!("{{{}}}", k), v);
        }
        result
    }

    /// Check if a key exists
    pub fn has_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

/// Errors that can occur when loading translations
#[derive(Debug)]
pub enum TranslationError {
    UnknownLanguage(String),
    ParseError(serde_json::Error),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLanguage(lang) => write!(f, "Unknown language: {}", lang),
            Self::ParseError(e) => write!(f, "Failed to parse translations: {}", e),
        }
    }
}

impl std::error::Error for TranslationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_english() {
        let trans = Translations::load("en").expect("Failed to load English translations");
        assert!(!trans.get("welcome_message").starts_with("[MISSING"));
    }

    #[test]
    fn test_get_existing_key() {
        let trans = Translations::load("en").unwrap();
        let msg = trans.get("welcome_message");
        assert_eq!(msg, "Welcome to Axur CLI");
    }

    #[test]
    fn test_get_missing_key() {
        let trans = Translations::load("en").unwrap();
        let msg = trans.get("nonexistent_key");
        assert!(msg.starts_with("[MISSING:"));
    }

    #[test]
    fn test_get_optional() {
        let trans = Translations::load("en").unwrap();
        assert!(trans.get_optional("welcome_message").is_some());
        assert!(trans.get_optional("nonexistent_key").is_none());
    }

    #[test]
    fn test_unknown_language() {
        let result = Translations::load("xyz");
        assert!(result.is_err());
    }
}
