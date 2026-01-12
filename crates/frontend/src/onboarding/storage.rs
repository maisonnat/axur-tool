//! Onboarding Storage - Persist progress in localStorage
//!
//! Stores tutorial completion, unlocked achievements, and sandbox preferences.

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "axur_onboarding";

/// Onboarding progress stored in localStorage
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OnboardingProgress {
    /// Whether the welcome tutorial has been completed
    pub tutorial_completed: bool,
    /// Current step index if tutorial in progress (None = not started)
    pub current_step: Option<usize>,
    /// IDs of unlocked achievements
    pub unlocked_achievements: Vec<String>,
    /// Whether user dismissed the sandbox mode prompt
    pub sandbox_dismissed: bool,
    /// Whether user has seen the welcome modal
    pub welcome_seen: bool,
    /// Number of shortcuts used (for shortcut_master achievement)
    pub shortcuts_used: u32,
}

/// Get the current onboarding progress from localStorage
pub fn get_onboarding_progress() -> OnboardingProgress {
    LocalStorage::get(STORAGE_KEY).unwrap_or_default()
}

/// Save onboarding progress to localStorage
pub fn save_onboarding_progress(progress: &OnboardingProgress) {
    let _ = LocalStorage::set(STORAGE_KEY, progress);
}

/// Mark the tutorial as complete
pub fn mark_tutorial_complete() {
    let mut progress = get_onboarding_progress();
    progress.tutorial_completed = true;
    progress.current_step = None;
    save_onboarding_progress(&progress);
}

/// Update current tutorial step
pub fn set_current_step(step: usize) {
    let mut progress = get_onboarding_progress();
    progress.current_step = Some(step);
    save_onboarding_progress(&progress);
}

/// Mark the welcome modal as seen
pub fn mark_welcome_seen() {
    let mut progress = get_onboarding_progress();
    progress.welcome_seen = true;
    save_onboarding_progress(&progress);
}

/// Unlock an achievement by ID
pub fn mark_achievement_unlocked(id: &str) {
    let mut progress = get_onboarding_progress();
    if !progress.unlocked_achievements.contains(&id.to_string()) {
        progress.unlocked_achievements.push(id.to_string());
        save_onboarding_progress(&progress);

        // Trigger celebration (via JS)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen(
                inline_js = "export function showAchievementToast(id) { if(window.showAchievementToast) window.showAchievementToast(id); }"
            )]
            extern "C" {
                fn showAchievementToast(id: &str);
            }
            showAchievementToast(id);
        }
    }
}

/// Check if an achievement is unlocked
pub fn is_achievement_unlocked(id: &str) -> bool {
    get_onboarding_progress()
        .unlocked_achievements
        .contains(&id.to_string())
}

/// Dismiss sandbox mode prompt
pub fn dismiss_sandbox() {
    let mut progress = get_onboarding_progress();
    progress.sandbox_dismissed = true;
    save_onboarding_progress(&progress);
}

/// Increment shortcut usage counter
pub fn increment_shortcut_usage() {
    let mut progress = get_onboarding_progress();
    progress.shortcuts_used += 1;
    save_onboarding_progress(&progress);

    // Check for shortcut_master achievement (5 shortcuts)
    if progress.shortcuts_used >= 5 {
        mark_achievement_unlocked("shortcut_master");
    }
}

/// Reset all onboarding progress (for testing)
pub fn reset_onboarding() {
    let _ = LocalStorage::delete(STORAGE_KEY);
}
