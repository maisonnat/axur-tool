//! Error display components with structured error codes
//!
//! Provides user-friendly error displays with copy-to-clipboard functionality.

use leptos::*;
use wasm_bindgen::prelude::*;

/// Copies text to the clipboard using the Navigator Clipboard API
pub async fn copy_to_clipboard(text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window");
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(())
}

/// Error display with structured error code and copy button
#[component]
pub fn ErrorDisplay(
    /// The error code (e.g., "TI-001")
    #[prop(into)]
    error_code: String,
    /// User-friendly error message
    #[prop(into)]
    error_message: String,
    /// Optional: callback when user clicks retry
    #[prop(optional)]
    on_retry: Option<Callback<()>>,
    /// Optional: callback to dismiss the error
    #[prop(optional)]
    on_dismiss: Option<Callback<()>>,
) -> impl IntoView {
    let copied = create_rw_signal(false);
    let code = error_code.clone();

    let copy_action = create_action(move |_: &()| {
        let code = code.clone();
        async move {
            match copy_to_clipboard(&code).await {
                Ok(_) => {
                    copied.set(true);
                    // Reset after 2 seconds
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    copied.set(false);
                }
                Err(e) => {
                    leptos::logging::error!("Failed to copy: {:?}", e);
                }
            }
        }
    });

    view! {
        <div class="bg-red-900/20 backdrop-blur-md border border-red-500/20 shadow-[0_0_15px_rgba(239,68,68,0.15)] rounded-xl p-4 mb-4">
            // Header with error code
            <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2">
                    <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                    <span class="text-red-200 font-semibold">"Error"</span>
                </div>

                // Error code badge with copy button
                <div class="flex items-center gap-2">
                    <code class="bg-red-950/60 border border-red-500/20 text-red-300 px-2 py-1 rounded-lg text-sm font-mono">
                        {error_code.clone()}
                    </code>
                    <button
                        on:click=move |_| copy_action.dispatch(())
                        class="text-zinc-400 hover:text-white transition-colors p-1"
                        title="Copiar código de error"
                    >
                        {move || if copied.get() {
                            view! {
                                <svg class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                </svg>
                            }.into_view()
                        } else {
                            view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                          d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                </svg>
                            }.into_view()
                        }}
                    </button>
                </div>
            </div>

            // Error message
            <p class="text-red-200 text-sm mb-3">
                {error_message}
            </p>

            // Action buttons
            <div class="flex gap-2">
                {on_retry.map(|callback| view! {
                    <button
                        on:click=move |_| callback.call(())
                        class="px-4 py-2 bg-red-500/20 hover:bg-red-500/40 border border-red-500/30 text-red-400 hover:text-white text-sm font-bold tracking-wide rounded-lg transition-all duration-300 focus:outline-none focus:shadow-[0_0_10px_rgba(239,68,68,0.3)]"
                    >
                        "Reintentar"
                    </button>
                })}
                {on_dismiss.map(|callback| view! {
                    <button
                        on:click=move |_| callback.call(())
                        class="px-4 py-2 bg-surface-base/50 hover:bg-surface-elevated/80 border border-white/10 text-zinc-300 hover:text-white text-sm font-medium rounded-lg transition-all duration-300 focus:outline-none"
                    >
                        "Cerrar"
                    </button>
                })}
            </div>
        </div>
    }
}

/// Inline error badge (for smaller displays)
#[component]
pub fn ErrorBadge(
    /// The error code
    #[prop(into)]
    code: String,
    /// Tooltip message
    #[prop(optional, into)]
    tooltip: Option<String>,
) -> impl IntoView {
    view! {
        <span
            class="inline-flex items-center gap-1 bg-red-900/40 border border-red-500/20 shadow-[0_0_10px_rgba(239,68,68,0.1)] text-red-300 px-2 py-0.5 rounded-lg text-xs font-mono backdrop-blur-sm"
            title=tooltip.unwrap_or_default()
        >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M12 9v2m0 4h.01"/>
            </svg>
            {code}
        </span>
    }
}
