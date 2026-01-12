//! Progress Checklist - Floating achievement tracker
//!
//! Shows a minimizable widget with user's progress through achievements.

use crate::onboarding::{storage, ALL_ACHIEVEMENTS};
use crate::UiLanguage;
use leptos::*;

/// Floating progress checklist widget
#[component]
pub fn ProgressChecklist() -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let ui_language = state.ui_language;

    // Track expanded/collapsed state
    let (is_expanded, set_is_expanded) = create_signal(false);

    // Get unlocked achievements (reactive)
    let progress = create_rw_signal(storage::get_onboarding_progress());

    // Refresh progress periodically and on visibility
    let refresh_progress = move || {
        progress.set(storage::get_onboarding_progress());
    };

    // Count unlocked achievements
    let unlocked_count = move || progress.get().unlocked_achievements.len();

    let total_count = ALL_ACHIEVEMENTS.len();

    // Localized strings
    let title_text = move || match ui_language.get() {
        UiLanguage::Es => "📋 Tu Progreso",
        UiLanguage::En => "📋 Your Progress",
        UiLanguage::Pt => "📋 Seu Progresso",
    };

    let completed_text = move || match ui_language.get() {
        UiLanguage::Es => "completados",
        UiLanguage::En => "completed",
        UiLanguage::Pt => "concluídos",
    };

    view! {
        <div
            class="fixed bottom-4 left-4 z-40 transition-all duration-300"
            class:w-72=is_expanded
            class:w-auto=move || !is_expanded.get()
        >
            <div class="bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl overflow-hidden">
                // Header (always visible)
                <button
                    class="w-full flex items-center justify-between gap-3 px-4 py-3 hover:bg-zinc-800/50 transition-colors"
                    on:click=move |_| {
                        set_is_expanded.update(|v| *v = !*v);
                        refresh_progress();
                    }
                >
                    <div class="flex items-center gap-2">
                        <span class="text-sm font-medium">
                            {title_text}
                        </span>
                        <span class="text-xs bg-orange-600/20 text-orange-400 px-2 py-0.5 rounded-full">
                            {move || format!("{}/{}", unlocked_count(), total_count)}
                        </span>
                    </div>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="2"
                        stroke="currentColor"
                        class="w-4 h-4 text-zinc-400 transition-transform"
                        class:rotate-180=is_expanded
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5" />
                    </svg>
                </button>

                // Expanded content
                <Show when=move || is_expanded.get()>
                    <div class="border-t border-zinc-800 px-4 py-3 space-y-2 max-h-64 overflow-y-auto">
                        {ALL_ACHIEVEMENTS.iter().map(|achievement| {
                            let id = achievement.id;
                            let is_unlocked = move || progress.get().unlocked_achievements.contains(&id.to_string());
                            let lang = ui_language.get();

                            view! {
                                <div
                                    class="flex items-center gap-3 py-1.5"
                                    class:opacity-50=move || !is_unlocked()
                                >
                                    <span class="text-lg" class:grayscale=move || !is_unlocked()>
                                        {achievement.icon}
                                    </span>
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm font-medium truncate" class:text-zinc-400=move || !is_unlocked()>
                                            {achievement.title(lang)}
                                        </p>
                                        <p class="text-xs text-zinc-500 truncate">
                                            {achievement.description(lang)}
                                        </p>
                                    </div>
                                    {move || if is_unlocked() {
                                        view! { <span class="text-green-500 text-sm">{"\u{2713}"}</span> }.into_view()
                                    } else {
                                        view! { <span class="text-zinc-600 text-sm">{"\u{25CB}"}</span> }.into_view()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    // Footer with progress bar
                    <div class="border-t border-zinc-800 px-4 py-2">
                        <div class="flex items-center gap-2 text-xs text-zinc-500">
                            <div class="flex-1 h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                                <div
                                    class="h-full bg-gradient-to-r from-orange-500 to-yellow-500 transition-all duration-500"
                                    style=move || format!("width: {}%", (unlocked_count() * 100) / total_count.max(1))
                                />
                            </div>
                            <span>{move || format!("{} {}", unlocked_count(), completed_text())}</span>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
