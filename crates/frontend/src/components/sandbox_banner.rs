//! Sandbox Banner Component
//!
//! Shows a sticky banner at the top when in practice mode.

use crate::onboarding::sandbox::{exit_sandbox_text, sandbox_banner_text};
use leptos::*;

/// Sandbox mode banner - shows at top of editor when in practice mode
#[component]
pub fn SandboxBanner(
    #[prop(into)] is_active: Signal<bool>,
    #[prop(into)] on_exit: Callback<()>,
) -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let ui_language = state.ui_language;

    view! {
        <Show when=move || is_active.get()>
            <div class="fixed top-0 left-0 right-0 z-50 bg-gradient-to-r from-amber-600 to-orange-600 text-white px-4 py-2 flex items-center justify-between shadow-lg">
                <div class="flex items-center gap-3">
                    <span class="text-2xl animate-bounce">{"\u{1F393}"}</span>
                    <span class="font-medium text-sm">
                        {move || sandbox_banner_text(ui_language.get())}
                    </span>
                </div>
                <button
                    class="bg-white/20 hover:bg-white/30 px-4 py-1.5 rounded-full text-sm font-medium transition-colors flex items-center gap-2"
                    on:click=move |_| on_exit.call(())
                >
                    <span>{"\u{2715}"}</span>
                    <span class="hidden sm:inline">
                        {move || exit_sandbox_text(ui_language.get())}
                    </span>
                </button>
            </div>
            // Spacer to push content down
            <div class="h-12"></div>
        </Show>
    }
}
