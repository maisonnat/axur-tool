use leptos::*;
use web_sys::window;
// Use leptos::task::spawn_local in 0.6 or leptos::spawn_local in older?
// Assuming recent leptos: spawn_local is usually exported in prelude or via leptos::spawn_local
use leptos::event_target_value;
use leptos::spawn_local; // Explicit import just in case // For event access
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn captureScreenshot() -> Result<JsValue, JsValue>;
}

#[component]
pub fn FeedbackWidget() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let (message, set_message) = create_signal(String::new());
    let (is_sending, set_is_sending) = create_signal(false);
    let (sent_success, set_sent_success) = create_signal(false);

    let toggle_modal = move |_| set_is_open.update(|open| *open = !*open);

    let on_submit = move |_| {
        set_is_sending.set(true);
        let msg = message.get();

        spawn_local(async move {
            let _window = window().unwrap();
            let url = _window.location().href().unwrap_or_default();
            let ua = _window.navigator().user_agent().unwrap_or_default();

            // Capture screenshot
            let screenshot = match captureScreenshot().await {
                Ok(val) => val.as_string(),
                Err(e) => {
                    leptos::logging::error!("Screenshot failed: {:?}", e);
                    None
                }
            };

            // TODO: Capture screenshot via html2canvas if possible/needed

            match crate::api::submit_feedback(
                msg, screenshot, // Screenshot
                url, ua, None, // Tenant ID
                None, // Email
            )
            .await
            {
                Ok(_) => {
                    set_is_sending.set(false);
                    set_sent_success.set(true);
                    set_message.set(String::new());
                    // Close after showing success
                    set_timeout(
                        move || {
                            set_sent_success.set(false);
                            set_is_open.set(false);
                        },
                        Duration::from_millis(2000),
                    );
                }
                Err(e) => {
                    set_is_sending.set(false);
                    leptos::logging::error!("Feedback failed: {}", e);
                }
            }
        });
    };

    view! {
        // Floating Button
        <div class="fixed bottom-6 right-6 z-50">
            <button
                class="bg-zinc-800 hover:bg-zinc-700 text-white rounded-full p-3 shadow-lg border border-zinc-600 transition-all hover:scale-105 flex items-center gap-2 group"
                on:click=toggle_modal
                title="Enviar Feedback"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 text-orange-500 group-hover:text-orange-400">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9 7.5 13.5-3.938c.47-.137.88-.582.88-1.063V2.836c0-1.229-1.309-2.022-2.315-1.403L.664 6.812A6.667 6.667 0 0 0 6.082 2.375a6.668 6.668 0 0 0-4.417-6.096A11.75 11.75 0 0 0 2.5 13.5v.75a6.75 6.75 0 0 1-.75-.75h.008c.55 0 1 .45 1 1v.75Z" /> // Bug icon placeholder
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 20.25c4.97 0 9-3.694 9-8.25s-4.03-8.25-9-8.25S3 7.444 3 12c0 2.104.859 4.023 2.273 5.48.432.447.74 1.04.586 1.641a4.483 4.483 0 0 1-.923 1.785A5.969 5.969 0 0 0 6 21c1.282 0 2.47-.402 3.445-1.087.81.22 1.668.337 2.555.337Z" />
                </svg>
                <span class="max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 ease-in-out whitespace-nowrap text-sm font-medium">
                    "Feedback / Bug"
                </span>
            </button>
        </div>

        // Modal
        <Show when=move || is_open.get()>
            <div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-end sm:items-center justify-center p-4 sm:p-0">
                // Click outside to close (overlay)
                <div class="absolute inset-0" on:click=toggle_modal></div>

                <div class="no-screenshot bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl w-full max-w-md relative overflow-hidden animate-in fade-in slide-in-from-bottom-10 duration-300 sm:slide-in-from-bottom-0 sm:zoom-in-95">
                    // Header
                    <div class="bg-zinc-800/50 px-4 py-3 border-b border-zinc-700 flex justify-between items-center">
                        <h3 class="text-white font-bold text-sm flex items-center gap-2">
                            <span class="text-orange-500">"Beta Feedback"</span>
                        </h3>
                        <button class="text-zinc-400 hover:text-white" on:click=toggle_modal>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    // Content
                    <div class="p-4">
                        <Show when=move || !sent_success.get() fallback=|| view! {
                            <div class="flex flex-col items-center justify-center py-8 text-center animate-in fade-in zoom-in">
                                <div class="bg-green-500/10 text-green-500 p-3 rounded-full mb-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                                    </svg>
                                </div>
                                <h4 class="text-white font-bold mb-1">"¡Gracias!"</h4>
                                <p class="text-zinc-400 text-sm">"Tu feedback ha sido enviado."</p>
                            </div>
                        }>
                            <p class="text-zinc-400 text-sm mb-3">
                                "¿Encontraste un error o tienes una idea? Cuéntanos."
                            </p>
                            <textarea
                                class="w-full bg-zinc-800 border border-zinc-700 rounded-lg p-3 text-white placeholder-zinc-500 focus:outline-none focus:border-orange-500 focus:ring-1 focus:ring-orange-500 min-h-[120px] resize-none text-sm"
                                placeholder="Describe el problema o sugerencia..."
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                disabled=move || is_sending.get()
                            ></textarea>

                            <div class="flex justify-end mt-4">
                                <button
                                    class="bg-orange-600 hover:bg-orange-500 text-white font-medium py-2 px-4 rounded-lg text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                    on:click=on_submit
                                    disabled=move || is_sending.get() || message.get().trim().is_empty()
                                >
                                    <Show when=move || is_sending.get() fallback=|| "Enviar">
                                        <div class="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full"></div>
                                        "Enviando..."
                                    </Show>
                                </button>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}
