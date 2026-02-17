# Client-Side Report Language Switching

## Overview

This feature allows users to switch the language of a generated HTML report without regenerating it from the server. The language selector appears in the top-right corner of the report view.

## Architecture

### Files

| File | Purpose |
|------|---------|
| `crates/core/src/report/language_switcher.rs` | Core module with UI and JS generation |
| `crates/core/translations/*.json` | Translation files (en, es, pt-br) |
| `crates/core/src/report/html.rs` | Integration into HTML template |

### How It Works

1. **Embedded Translations**: All 3 JSON translation files (~48KB total, ~12KB gzipped) are embedded directly in the HTML as a `<script type="application/json">` block.

2. **Language Selector UI**: A floating button group in the top-right corner:
   ```html
   <div id="lang-switcher" class="fixed top-4 right-4 z-50 ...">
       <button onclick="setReportLanguage('es')">ES</button>
       <button onclick="setReportLanguage('en')">EN</button>
       <button onclick="setReportLanguage('pt')">PT</button>
   </div>
   ```

3. **JavaScript Switcher**: The `setReportLanguage(lang)` function updates all elements with `data-i18n` attributes:
   ```html
   <h1 data-i18n="metrics_title">Métricas Generales</h1>
   ```
   becomes:
   ```html
   <h1 data-i18n="metrics_title">General Metrics</h1>
   ```

4. **Preference Persistence**: User's language choice is saved to `localStorage` and restored on page reload.

## Adding Translatable Text

### 1. Add HTML with `data-i18n` attribute

```rust
format!(r#"<h2 data-i18n="my_new_key">{}</h2>"#, translations.get("my_new_key"))
```

### 2. Add key to all translation files

**`translations/en.json`:**
```json
"my_new_key": "My New Feature"
```

**`translations/es.json`:**
```json
"my_new_key": "Mi Nueva Funcionalidad"
```

**`translations/pt-br.json`:**
```json
"my_new_key": "Minha Nova Funcionalidade"
```

### 3. For dynamic values (interpolation)

```rust
format!(
    r#"<p data-i18n="count_message" data-i18n-params='{{"count":"{}"}}'>{}</p>"#,
    count,
    translations.format("count_message", &[("count", &count.to_string())])
)
```

JSON:
```json
"count_message": "Found {count} items"
```

## API Reference

### `embed_translations_json() -> String`
Generates `<script>` block with all translations as nested JSON object.

### `language_selector_ui(current_lang: &str) -> String`
Generates the floating language selector HTML.

### `language_switcher_script() -> String`
Generates the JavaScript code for `setReportLanguage()` function.

### `generate_language_switching_components(current_lang: &str) -> (String, String, String)`
Convenience function returning (head_content, body_start, body_end) tuple.

## Notes

- The selector is hidden when printing (`no-print` class)
- Does not work with legacy `generate_full_report_html()` - only with `generate_report_with_plugins()`
- For future migration of legacy slides, add `data-i18n` attributes during the migration process
