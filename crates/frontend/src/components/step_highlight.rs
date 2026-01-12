//! Step Highlight Overlay Component
//!
//! Highlights a specific element during tutorial steps with a spotlight effect.

use leptos::*;

/// Step highlight overlay that creates a spotlight effect on the target element
#[component]
pub fn StepHighlight(
    /// CSS selector for the target element to highlight
    #[prop(into)]
    target_selector: Signal<String>,
    /// Title text for the step
    #[prop(into)]
    title: Signal<String>,
    /// Instruction text for the step
    #[prop(into)]
    instruction: Signal<String>,
    /// Step number (1-indexed)
    #[prop(into)]
    step_number: Signal<usize>,
    /// Total steps
    #[prop(into)]
    total_steps: Signal<usize>,
    /// Callback when user clicks "Next"
    #[prop(into)]
    on_next: Callback<()>,
    /// Callback when user clicks "Skip"
    #[prop(into)]
    on_skip: Callback<()>,
    /// Whether overlay is visible
    #[prop(into)]
    is_visible: Signal<bool>,
) -> impl IntoView {
    // Position state for the tooltip
    let tooltip_style = create_rw_signal(String::from("bottom: 100px; left: 50%;"));

    // Effect to position tooltip near target element
    create_effect(move |_| {
        if is_visible.get() {
            let selector = target_selector.get();
            // JavaScript will position the tooltip
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen(inline_js = r#"
                    export function positionTooltip(selector) {
                        const el = document.querySelector(selector);
                        if (el) {
                            const rect = el.getBoundingClientRect();
                            el.scrollIntoView({ behavior: 'smooth', block: 'center' });
                            el.style.position = 'relative';
                            el.style.zIndex = '10001';
                            el.style.boxShadow = '0 0 0 4px #f97316, 0 0 20px rgba(249, 115, 22, 0.5)';
                            el.style.borderRadius = '8px';
                            return `top: ${rect.bottom + 20}px; left: ${rect.left}px;`;
                        }
                        return 'bottom: 100px; left: 50%; transform: translateX(-50%);';
                    }
                "#)]
                extern "C" {
                    fn positionTooltip(selector: &str) -> String;
                }
                tooltip_style.set(positionTooltip(&selector));
            }
        }
    });

    view! {
        <Show when=move || is_visible.get()>
            // Dark overlay with hole for target element
            <div
                class="fixed inset-0 z-[10000] bg-black/70 backdrop-blur-sm"
                style="pointer-events: auto;"
            >
                // Floating tooltip card
                <div
                    class="fixed z-[10002] bg-zinc-900 border border-orange-500 rounded-xl shadow-2xl p-5 max-w-sm animate-fadeIn"
                    style=move || tooltip_style.get()
                >
                    // Step indicator
                    <div class="flex items-center justify-between mb-3">
                        <span class="text-xs text-orange-400 font-medium">
                            {move || format!("Paso {} de {}", step_number.get(), total_steps.get())}
                        </span>
                        <div class="flex gap-1">
                            {move || (1..=total_steps.get()).map(|i| {
                                let is_current = i == step_number.get();
                                let is_done = i < step_number.get();
                                view! {
                                    <div class={
                                        if is_current { "w-6 h-1.5 bg-orange-500 rounded-full" }
                                        else if is_done { "w-1.5 h-1.5 bg-orange-500 rounded-full" }
                                        else { "w-1.5 h-1.5 bg-zinc-600 rounded-full" }
                                    }></div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // Title
                    <h3 class="text-white font-bold text-lg mb-2">
                        {move || title.get()}
                    </h3>

                    // Instruction
                    <p class="text-zinc-400 text-sm mb-4 leading-relaxed">
                        {move || instruction.get()}
                    </p>

                    // Buttons
                    <div class="flex gap-2">
                        <button
                            class="flex-1 bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-white font-semibold py-2 px-4 rounded-lg transition-all"
                            on:click=move |_| on_next.call(())
                        >
                            {move || if step_number.get() == total_steps.get() {
                                "\u{2713} Finalizar"
                            } else {
                                "Siguiente \u{2192}"
                            }}
                        </button>
                        <button
                            class="px-4 py-2 text-zinc-400 hover:text-white text-sm transition-colors"
                            on:click=move |_| on_skip.call(())
                        >
                            "Saltar"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
