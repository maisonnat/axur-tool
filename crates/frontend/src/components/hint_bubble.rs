//! Hint Bubble Component
//!
//! Shows contextual hints after user inactivity.

use leptos::*;

/// Hint bubble that appears after idle time to guide users
#[component]
pub fn HintBubble(
    /// Message to display
    #[prop(into)]
    message: Signal<String>,
    /// Position style (CSS)
    #[prop(into)]
    position_style: Signal<String>,
    /// Whether hint is visible
    #[prop(into)]
    is_visible: Signal<bool>,
    /// Callback when dismissed
    #[prop(into)]
    on_dismiss: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || is_visible.get()>
            <div
                class="fixed z-[9999] animate-fadeIn"
                style=move || position_style.get()
            >
                <div class="relative bg-zinc-800 border border-zinc-600 rounded-xl shadow-xl p-4 max-w-xs">
                    // Arrow pointer
                    <div class="absolute -top-2 left-6 w-4 h-4 bg-zinc-800 border-l border-t border-zinc-600 transform rotate-45"></div>

                    // Hint icon and message
                    <div class="flex items-start gap-3">
                        <span class="text-2xl shrink-0 animate-pulse">{"\u{1F4A1}"}</span>
                        <div class="flex-1">
                            <p class="text-zinc-300 text-sm leading-relaxed">
                                {move || message.get()}
                            </p>
                        </div>
                        <button
                            class="text-zinc-500 hover:text-white text-lg transition-colors"
                            on:click=move |_| on_dismiss.call(())
                        >
                            {"\u{2715}"}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Idle timer hook - returns true if user has been idle for specified duration
pub fn use_idle_timer(idle_seconds: u32) -> RwSignal<bool> {
    let is_idle = create_rw_signal(false);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        create_effect(move |_| {
            let window = web_sys::window().expect("window");
            let document = window.document().expect("document");

            // Reset timer on any interaction
            let reset_timer = Closure::wrap(Box::new(move || {
                is_idle.set(false);
            }) as Box<dyn Fn()>);

            let _ = document.add_event_listener_with_callback(
                "mousemove",
                reset_timer.as_ref().unchecked_ref(),
            );
            let _ = document
                .add_event_listener_with_callback("keydown", reset_timer.as_ref().unchecked_ref());
            let _ = document
                .add_event_listener_with_callback("click", reset_timer.as_ref().unchecked_ref());

            reset_timer.forget();

            // Set idle after duration
            let idle_ms = idle_seconds * 1000;
            let set_idle = Closure::wrap(Box::new(move || {
                is_idle.set(true);
            }) as Box<dyn Fn()>);

            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                set_idle.as_ref().unchecked_ref(),
                idle_ms as i32,
            );

            set_idle.forget();
        });
    }

    is_idle
}
