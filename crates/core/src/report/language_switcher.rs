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

/// Generates the floating language selector UI with inline onclick handlers
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
            // Use inline onclick that calls the global function
            format!(
                r#"<button onclick="if(window._setLang)window._setLang('{code}')" 
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
/// Uses a self-executing pattern that works even with innerHTML insertion
pub fn language_switcher_script() -> String {
    // This script uses setTimeout(0) to defer execution and ensure it runs
    // even when inserted via innerHTML. The logic is all self-contained.
    r#"<script>
// Self-executing language switcher initialization
// Uses setTimeout to ensure execution even when inserted via innerHTML
setTimeout(function() {
    var translations = {};
    
    // Load translations from embedded JSON
    var i18nData = document.getElementById('i18n-data');
    if (i18nData) {
        try {
            translations = JSON.parse(i18nData.textContent);
        } catch (e) {
            console.error('Failed to parse i18n data:', e);
        }
    }
    
    // Define the language switching function globally
    window._setLang = function(lang) {
        if (!translations[lang]) {
            console.warn('Language not found:', lang);
            return;
        }
        
        var dict = translations[lang];
        
        // Update all elements with data-i18n attribute
        var elements = document.querySelectorAll('[data-i18n]');
        for (var i = 0; i < elements.length; i++) {
            var el = elements[i];
            var key = el.getAttribute('data-i18n');
            if (dict[key]) {
                var text = dict[key];
                
                // Handle interpolation: {variable} patterns
                var placeholders = el.getAttribute('data-i18n-params');
                if (placeholders) {
                    try {
                        var params = JSON.parse(placeholders);
                        for (var pkey in params) {
                            text = text.replace(new RegExp('\\{' + pkey + '\\}', 'g'), params[pkey]);
                        }
                    } catch (e) {}
                }
                
                el.innerHTML = text;
            }
        }
        
        // Update button states
        var buttons = document.querySelectorAll('#lang-switcher button');
        for (var j = 0; j < buttons.length; j++) {
            var btn = buttons[j];
            if (btn.getAttribute('data-lang') === lang) {
                btn.className = 'px-3 py-1.5 text-sm font-medium rounded transition-colors bg-orange-500 text-white';
            } else {
                btn.className = 'px-3 py-1.5 text-sm font-medium rounded transition-colors bg-zinc-700 text-zinc-300 hover:bg-zinc-600';
            }
        }
        
        // Store preference
        try { localStorage.setItem('report_lang', lang); } catch(e) {}
    };
    
    // Auto-load saved preference
    try {
        var saved = localStorage.getItem('report_lang');
        if (saved && translations[saved]) {
            window._setLang(saved);
        }
    } catch(e) {}
    
    console.log('Language switcher initialized');
}, 0);
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
