//! LocalStorage utilities for persisting user preferences

use web_sys::window;

const THEME_KEY: &str = "axur_plugin_theme";
const DISABLED_SLIDES_KEY: &str = "axur_disabled_slides";

/// Save a string value to localStorage
fn set_item(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(key, value);
    }
}

/// Get a string value from localStorage
fn get_item(key: &str) -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item(key).ok())
        .flatten()
}

/// Save theme preference
pub fn save_theme(theme: &str) {
    set_item(THEME_KEY, theme);
}

/// Load theme preference (defaults to "dark")
pub fn load_theme() -> String {
    get_item(THEME_KEY).unwrap_or_else(|| "dark".to_string())
}

/// Save disabled slides list
pub fn save_disabled_slides(slides: &[String]) {
    if slides.is_empty() {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
            let _ = storage.remove_item(DISABLED_SLIDES_KEY);
        }
    } else {
        let value = slides.join(",");
        set_item(DISABLED_SLIDES_KEY, &value);
    }
}

/// Load disabled slides list
pub fn load_disabled_slides() -> Vec<String> {
    get_item(DISABLED_SLIDES_KEY)
        .map(|s| s.split(',').map(|x| x.to_string()).collect())
        .unwrap_or_default()
}

// === Placeholder Favorites & Recents ===

const FAVORITES_KEY: &str = "axur_placeholder_favorites";
const RECENTS_KEY: &str = "axur_placeholder_recents";
const MAX_RECENTS: usize = 5;

/// Save favorite placeholder keys
pub fn save_favorites(favorites: &[String]) {
    if favorites.is_empty() {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
            let _ = storage.remove_item(FAVORITES_KEY);
        }
    } else {
        let value = favorites.join(",");
        set_item(FAVORITES_KEY, &value);
    }
}

/// Load favorite placeholder keys
pub fn load_favorites() -> Vec<String> {
    get_item(FAVORITES_KEY)
        .map(|s| {
            s.split(',')
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Toggle a placeholder as favorite (add if not present, remove if present)
pub fn toggle_favorite(key: &str) -> bool {
    let mut favorites = load_favorites();
    if favorites.contains(&key.to_string()) {
        favorites.retain(|k| k != key);
        save_favorites(&favorites);
        false // Not a favorite anymore
    } else {
        favorites.push(key.to_string());
        save_favorites(&favorites);
        true // Now a favorite
    }
}

/// Check if a placeholder is a favorite
pub fn is_favorite(key: &str) -> bool {
    load_favorites().contains(&key.to_string())
}

/// Add a placeholder to recents (most recent first, max 5)
pub fn add_to_recents(key: &str) {
    let mut recents = load_recents();
    // Remove if already exists (to move to front)
    recents.retain(|k| k != key);
    // Add at front
    recents.insert(0, key.to_string());
    // Limit to MAX_RECENTS
    recents.truncate(MAX_RECENTS);
    let value = recents.join(",");
    set_item(RECENTS_KEY, &value);
}

/// Load recent placeholder keys
pub fn load_recents() -> Vec<String> {
    get_item(RECENTS_KEY)
        .map(|s| {
            s.split(',')
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .unwrap_or_default()
}

// === Template Versions ===

const VERSIONS_KEY_PREFIX: &str = "axur_template_versions_";
const MAX_VERSIONS: usize = 10;

/// A single template version entry
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateVersion {
    pub version: u32,
    pub date: String,
    pub slides_json: Vec<String>, // Array of canvas_json for each slide
}

/// Save a new version of a template (keeps last MAX_VERSIONS)
pub fn save_template_version(template_id: &str, slides_json: Vec<String>) {
    let key = format!("{}{}", VERSIONS_KEY_PREFIX, template_id);
    let mut versions = load_template_versions(template_id);

    // Determine next version number
    let next_version = versions.iter().map(|v| v.version).max().unwrap_or(0) + 1;

    // Get current timestamp
    let date = js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_default();

    // Add new version at front
    versions.insert(
        0,
        TemplateVersion {
            version: next_version,
            date,
            slides_json,
        },
    );

    // Keep only MAX_VERSIONS
    versions.truncate(MAX_VERSIONS);

    // Serialize and save
    if let Ok(json) = serde_json::to_string(&versions) {
        set_item(&key, &json);
    }
}

/// Load all versions of a template
pub fn load_template_versions(template_id: &str) -> Vec<TemplateVersion> {
    let key = format!("{}{}", VERSIONS_KEY_PREFIX, template_id);
    get_item(&key)
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

/// Get a specific version of a template
pub fn get_template_version(template_id: &str, version: u32) -> Option<TemplateVersion> {
    load_template_versions(template_id)
        .into_iter()
        .find(|v| v.version == version)
}

/// Get the latest version number for a template
pub fn get_latest_version_number(template_id: &str) -> u32 {
    load_template_versions(template_id)
        .first()
        .map(|v| v.version)
        .unwrap_or(0)
}
