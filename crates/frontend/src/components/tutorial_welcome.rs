//! Tutorial Welcome Modal Component
//!
//! Shows a welcome screen when user enters sandbox/tutorial for the first time.

use crate::onboarding::sandbox::{
    sandbox_explore_freely, sandbox_start_tutorial, sandbox_welcome_desc, sandbox_welcome_title,
};
use leptos::*;

/// Welcome modal for onboarding/sandbox
#[component]
pub fn TutorialWelcomeModal(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_start_tutorial: Callback<()>,
    #[prop(into)] on_skip: Callback<()>,
) -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let ui_language = state.ui_language;

    view! {
        <Show when=move || is_open.get()>
            <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-sm">
                <div class="bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl max-w-lg w-[90%] overflow-hidden animate-fadeIn">
                    // Header with gradient
                    <div class="bg-gradient-to-r from-orange-600 to-amber-500 p-6 text-center">
                        <h2 class="text-2xl font-bold text-white mb-1">
                            {move || sandbox_welcome_title(ui_language.get())}
                        </h2>
                    </div>

                    // Content
                    <div class="p-6 space-y-6">
                        <p class="text-zinc-300 text-center leading-relaxed">
                            {move || sandbox_welcome_desc(ui_language.get())}
                        </p>

                        // Features preview
                        <div class="grid grid-cols-2 gap-3 text-sm">
                            <div class="bg-zinc-800/50 rounded-lg p-3 flex items-center gap-3">
                                <span class="text-2xl">{"\u{1F4DD}"}</span>
                                <span class="text-zinc-400">"Editar textos"</span>
                            </div>
                            <div class="bg-zinc-800/50 rounded-lg p-3 flex items-center gap-3">
                                <span class="text-2xl">{"\u{1F4CA}"}</span>
                                <span class="text-zinc-400">"Ver métricas"</span>
                            </div>
                            <div class="bg-zinc-800/50 rounded-lg p-3 flex items-center gap-3">
                                <span class="text-2xl">{"\u{2328}"}</span>
                                <span class="text-zinc-400">"Usar atajos"</span>
                            </div>
                            <div class="bg-zinc-800/50 rounded-lg p-3 flex items-center gap-3">
                                <span class="text-2xl">{"\u{1F4E4}"}</span>
                                <span class="text-zinc-400">"Exportar"</span>
                            </div>
                        </div>

                        // Buttons
                        <div class="flex flex-col sm:flex-row gap-3">
                            <button
                                class="flex-1 bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-white font-semibold py-3 px-6 rounded-xl transition-all hover:scale-[1.02] flex items-center justify-center gap-2"
                                on:click=move |_| on_start_tutorial.call(())
                            >
                                {move || sandbox_start_tutorial(ui_language.get())}
                            </button>
                            <button
                                class="flex-1 bg-zinc-700 hover:bg-zinc-600 text-white font-medium py-3 px-6 rounded-xl transition-colors flex items-center justify-center gap-2"
                                on:click=move |_| on_skip.call(())
                            >
                                {move || sandbox_explore_freely(ui_language.get())}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
