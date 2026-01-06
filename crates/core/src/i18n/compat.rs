//! Compatibility Layer for i18n Migration
//!
//! Provides a wrapper that tries the new JSON-based translation system first,
//! falling back to the legacy Dictionary trait if a key is not found.

use crate::i18n::legacy::Dictionary;
use crate::i18n::loader::Translations;

/// Compatibility wrapper that combines new JSON translations with legacy Dictionary
pub struct TranslationCompat<'a> {
    /// New JSON-based translations (primary)
    translations: &'a Translations,
    /// Legacy Dictionary trait implementation (fallback)
    dictionary: &'a dyn Dictionary,
}

impl<'a> TranslationCompat<'a> {
    /// Create a new compatibility wrapper
    pub fn new(translations: &'a Translations, dictionary: &'a dyn Dictionary) -> Self {
        Self {
            translations,
            dictionary,
        }
    }

    /// Get a translation by key
    ///
    /// Tries the new JSON system first, falls back to legacy if not found
    pub fn get(&self, key: &str) -> String {
        // Try new system first
        let value = self.translations.get(key);

        // If we got a placeholder back (key not found), try legacy
        if value.starts_with('[') && value.ends_with(']') {
            self.get_legacy(key)
        } else {
            value
        }
    }

    /// Get a translation with parameters
    pub fn format(&self, key: &str, params: &[(&str, &str)]) -> String {
        // Try new system first
        let value = self.translations.format(key, params);

        // If we got a placeholder back, try legacy
        if value.starts_with('[') && value.ends_with(']') {
            // Legacy doesn't support format, use as-is
            self.get_legacy(key)
        } else {
            value
        }
    }

    /// Direct legacy lookup by method name pattern
    fn get_legacy(&self, key: &str) -> String {
        // Map common keys to Dictionary methods
        match key {
            // Cover & Intro
            "label_tlp" => self.dictionary.label_tlp(),
            "label_tlp_desc" => self.dictionary.label_tlp_desc(),
            "label_company" => self.dictionary.label_company(),
            "label_partner" => self.dictionary.label_partner(),
            "cover_title_dynamic" => self.dictionary.cover_title_dynamic(),
            "cover_title_static" => self.dictionary.cover_title_static(),
            "intro_title" => self.dictionary.intro_title(),
            "intro_text_dynamic" => self.dictionary.intro_text_dynamic(),
            "intro_text_static" => self.dictionary.intro_text_static(),
            "intro_text_closing" => self.dictionary.intro_text_closing(),

            // Solutions
            "solutions_title" => self.dictionary.solutions_title(),
            "solutions_subtitle_1" => self.dictionary.solutions_subtitle_1(),
            "solutions_subtitle_2" => self.dictionary.solutions_subtitle_2(),
            "solutions_subtitle_3" => self.dictionary.solutions_subtitle_3(),
            "solution_takedown" => self.dictionary.solution_takedown(),
            "solution_brand_protection" => self.dictionary.solution_brand_protection(),
            "solution_threat_intel" => self.dictionary.solution_threat_intel(),

            // TOC
            "toc_title" => self.dictionary.toc_title(),

            // Metrics
            "metrics_title" => self.dictionary.metrics_title(),
            "metrics_total_tickets" => self.dictionary.metrics_total_tickets(),
            "metrics_desc_tickets" => self.dictionary.metrics_desc_tickets(),
            "eff_title" => self.dictionary.eff_title(),
            "eff_text_speed" => self.dictionary.eff_text_speed(),

            // Takedowns
            "takedowns_title" => self.dictionary.takedowns_title(),
            "takedowns_requested" => self.dictionary.takedowns_requested(),
            "takedowns_success_rate" => self.dictionary.takedowns_success_rate(),
            "takedowns_median_notify" => self.dictionary.takedowns_median_notify(),
            "takedowns_median_uptime" => self.dictionary.takedowns_median_uptime(),
            "takedowns_status_title" => self.dictionary.takedowns_status_title(),
            "takedowns_solved" => self.dictionary.takedowns_solved(),
            "takedowns_in_progress" => self.dictionary.takedowns_in_progress(),
            "takedowns_interrupted" => self.dictionary.takedowns_interrupted(),
            "takedowns_not_solved" => self.dictionary.takedowns_not_solved(),

            // ROI/Operational
            "op_badge" => self.dictionary.op_badge(),
            "roi_title" => self.dictionary.roi_title(),
            "op_time_saved_title" => self.dictionary.op_time_saved_title(),
            "op_time_saved_desc" => self.dictionary.op_time_saved_desc(),
            "op_capacity_title" => self.dictionary.op_capacity_title(),
            "op_capacity_desc" => self.dictionary.op_capacity_desc(),
            "op_response_title" => self.dictionary.op_response_title(),
            "op_response_desc" => self.dictionary.op_response_desc(),
            "op_unit_person_days" => self.dictionary.op_unit_person_days(),
            "op_unit_hours" => self.dictionary.op_unit_hours(),

            // Footer
            "footer_text" => self.dictionary.footer_text(),

            // Closing
            "closing_title" => self.dictionary.closing_title(),
            "closing_subtitle" => self.dictionary.closing_subtitle(),

            // Default: return the key as placeholder
            _ => format!("[{}]", key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::legacy::English;

    #[test]
    fn test_compat_prefers_json() {
        let translations = Translations::load("en").unwrap();
        let dict = English;
        let compat = TranslationCompat::new(&translations, &dict);

        // Should get from JSON
        let result = compat.get("footer_text");
        assert!(!result.is_empty());
        assert!(!result.starts_with('['));
    }

    #[test]
    fn test_compat_falls_back_to_legacy() {
        let translations = Translations::load("en").unwrap();
        let dict = English;
        let compat = TranslationCompat::new(&translations, &dict);

        // A key that might not be in JSON yet
        let result = compat.get("solutions_title");
        assert!(!result.is_empty());
    }
}
