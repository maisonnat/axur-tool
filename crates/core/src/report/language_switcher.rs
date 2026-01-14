//! Client-Side Language Switcher for HTML Reports
//!
//! This module provides functionality to embed all translations into the HTML report,
//! enabling client-side language switching without server round-trips.
//!
//! ## How It Works
//!
//! 1. All 3 translation files (en, es, pt-br) are embedded as JSON in the HTML
//! 2. A floating language selector button is added to the report
//! 3. JavaScript switches text content by looking up `data-i18n` attributes
//!
//! ## Usage in Render Functions
//!
//! When rendering text that should be translatable, use:
//! ```html
//! <h1 data-i18n="metrics_title">{initial_text}</h1>
//! ```
//!
//! The initial_text is the server-rendered value (for the initially selected language).
//! The data-i18n key maps to the JSON translation files.
//!
//! ## Adding New Translations
//!
//! 1. Add the key to `crates/core/translations/*.json` (all 3 files)
//! 2. Use `data-i18n="your_key"` in HTML elements
//! 3. The switcher will automatically pick up the new key

/// Generates embedded translation JSON for client-side switching
pub fn embed_translations_json() -> String {
    let en_json = include_str!("../../translations/en.json");
    let es_json = include_str!("../../translations/es.json");
    let pt_json = include_str!("../../translations/pt-br.json");

    format!(
        r#"<script id="i18n-data" type="application/json">
{{
    "en": {en},
    "es": {es},
    "pt": {pt}
}}
</script>"#,
        en = en_json,
        es = es_json,
        pt = pt_json
    )
}

/// Generates the floating language selector UI
pub fn language_selector_ui(current_lang: &str) -> String {
    let langs = [("es", "ES"), ("en", "EN"), ("pt", "PT")];

    let buttons: Vec<String> = langs
        .iter()
        .map(|(code, label)| {
            let active_class = if *code == current_lang {
                "bg-orange-500 text-white"
            } else {
                "bg-zinc-700 text-zinc-300 hover:bg-zinc-600"
            };
            format!(
                r#"<button onclick="setReportLanguage('{code}')" 
                    class="px-3 py-1.5 text-sm font-medium rounded transition-colors {active_class}"
                    data-lang="{code}">{label}</button>"#,
                code = code,
                label = label,
                active_class = active_class
            )
        })
        .collect();

    format!(
        r#"<div id="lang-switcher" class="fixed top-4 right-4 z-50 flex gap-1 bg-zinc-800/90 backdrop-blur p-1 rounded-lg shadow-lg no-print">
    {buttons}
</div>"#,
        buttons = buttons.join("\n    ")
    )
}

/// Generates the JavaScript code for language switching
pub fn language_switcher_script() -> String {
    r#"<script>
// Client-Side Language Switcher
// Loads translations from embedded JSON and switches text on-the-fly
(function() {
    let translations = {};
    let currentLang = 'es'; // Default

    // Load translations from embedded JSON
    const i18nData = document.getElementById('i18n-data');
    if (i18nData) {
        try {
            translations = JSON.parse(i18nData.textContent);
        } catch (e) {
            console.error('Failed to parse i18n data:', e);
        }
    }

    // Set report language and update all translatable elements
    window.setReportLanguage = function(lang) {
        if (!translations[lang]) {
            console.warn('Language not found:', lang);
            return;
        }
        
        currentLang = lang;
        const dict = translations[lang];
        
        // Update all elements with data-i18n attribute
        document.querySelectorAll('[data-i18n]').forEach(el => {
            const key = el.dataset.i18n;
            if (dict[key]) {
                // Preserve any dynamic content in placeholders
                let text = dict[key];
                
                // Handle interpolation: {variable} patterns
                const placeholders = el.dataset.i18nParams;
                if (placeholders) {
                    try {
                        const params = JSON.parse(placeholders);
                        for (const [pkey, pval] of Object.entries(params)) {
                            text = text.replace(new RegExp(`\\{${pkey}\\}`, 'g'), pval);
                        }
                    } catch (e) {}
                }
                
                el.innerHTML = text;
            }
        });
        
        // Update button states
        document.querySelectorAll('#lang-switcher button').forEach(btn => {
            if (btn.dataset.lang === lang) {
                btn.className = 'px-3 py-1.5 text-sm font-medium rounded transition-colors bg-orange-500 text-white';
            } else {
                btn.className = 'px-3 py-1.5 text-sm font-medium rounded transition-colors bg-zinc-700 text-zinc-300 hover:bg-zinc-600';
            }
        });
        
        // Store preference
        try { localStorage.setItem('report_lang', lang); } catch(e) {}
    };

    // Load saved preference on page load
    document.addEventListener('DOMContentLoaded', function() {
        try {
            const saved = localStorage.getItem('report_lang');
            if (saved && translations[saved]) {
                setReportLanguage(saved);
            }
        } catch(e) {}
    });
})();
</script>"#.to_string()
}

/// Generates the complete language switching components for HTML reports
///
/// Returns a tuple of (head_content, body_start_content, body_end_content)
pub fn generate_language_switching_components(current_lang: &str) -> (String, String, String) {
    let head = "".to_string(); // No head content needed
    let body_start = language_selector_ui(current_lang);
    let body_end = format!(
        "{}\n{}",
        embed_translations_json(),
        language_switcher_script()
    );

    (head, body_start, body_end)
}
