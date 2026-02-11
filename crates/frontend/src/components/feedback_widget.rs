use leptos::event_target_value;
use leptos::spawn_local;
use leptos::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn captureScreenshot() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = window, js_name = openAnnotationEditor)]
    fn open_annotation_editor(screenshot_data: &str);

    #[wasm_bindgen(js_namespace = window, js_name = getAnnotatedScreenshot)]
    fn get_annotated_screenshot() -> Option<String>;
}

#[component]
pub fn FeedbackWidget() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let (message, set_message) = create_signal(String::new());
    let (is_sending, set_is_sending) = create_signal(false);
    let (sent_success, set_sent_success) = create_signal(false);
    let (screenshot_data, set_screenshot_data) = create_signal::<Option<String>>(None);
    let (_is_annotating, set_is_annotating) = create_signal(false);

    // Open modal and capture screenshot
    let open_feedback = move |_| {
        set_is_open.set(true);
        // Capture screenshot immediately when opening
        spawn_local(async move {
            match captureScreenshot().await {
                Ok(val) => {
                    if let Some(data) = val.as_string() {
                        set_screenshot_data.set(Some(data));
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Screenshot failed: {:?}", e);
                }
            }
        });
    };

    let close_modal = move |_| {
        set_is_open.set(false);
        set_is_annotating.set(false);
        set_screenshot_data.set(None);
        set_message.set(String::new());
    };

    let toggle_annotate = move |_| {
        if let Some(data) = screenshot_data.get() {
            open_annotation_editor(&data);
            set_is_annotating.set(true);
        }
    };

    let on_submit = move |_| {
        set_is_sending.set(true);
        let msg = message.get();

        spawn_local(async move {
            let _window = window().unwrap();
            let url = _window.location().href().unwrap_or_default();
            let ua = _window.navigator().user_agent().unwrap_or_default();

            // Get annotated screenshot if available, otherwise use original
            let screenshot = get_annotated_screenshot().or_else(|| screenshot_data.get());

            match crate::api::submit_feedback(msg, screenshot, url, ua, None, None).await {
                Ok(_) => {
                    set_is_sending.set(false);
                    set_sent_success.set(true);
                    set_message.set(String::new());
                    set_timeout(
                        move || {
                            set_sent_success.set(false);
                            set_is_open.set(false);
                            set_screenshot_data.set(None);
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
        // Floating Button - MUCH MORE VISIBLE
        <div class="fixed bottom-6 right-6 z-50">
            <button
                class="bg-gradient-to-r from-orange-500 to-red-500 hover:from-orange-400 hover:to-red-400 text-white rounded-full px-4 py-3 shadow-xl border-2 border-orange-400/50 transition-all hover:scale-105 flex items-center gap-2 animate-pulse hover:animate-none"
                on:click=open_feedback
                title="Reportar problema o sugerencia"
            >
                // Bug/Chat icon
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
                </svg>
                <span class="text-sm font-bold tracking-wide">
                    "Ayuda"
                </span>
            </button>
        </div>

        // Modal
        <Show when=move || is_open.get()>
            <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                // Click outside to close
                <div class="absolute inset-0" on:click=close_modal></div>

                <div class="feedback-widget-ignore bg-zinc-900 border border-zinc-600 rounded-2xl shadow-2xl w-full max-w-lg relative overflow-hidden animate-in fade-in zoom-in-95 duration-200">
                    // Header
                    <div class="bg-gradient-to-r from-orange-600/20 to-red-600/20 px-5 py-4 border-b border-zinc-700 flex justify-between items-center">
                        <h3 class="text-white font-bold text-base flex items-center gap-2">
                            <span class="text-2xl">"🐛"</span>
                            <span>"Reportar Problema"</span>
                        </h3>
                        <button class="text-zinc-400 hover:text-white p-1 hover:bg-zinc-700 rounded-lg transition" on:click=close_modal>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    // Content
                    <div class="p-5">
                        <Show when=move || !sent_success.get() fallback=|| view! {
                            <div class="flex flex-col items-center justify-center py-10 text-center animate-in fade-in zoom-in">
                                <div class="bg-green-500/20 text-green-400 p-4 rounded-full mb-4">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-10 h-10">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                                    </svg>
                                </div>
                                <h4 class="text-white font-bold text-lg mb-1">"¡Gracias!"</h4>
                                <p class="text-zinc-400">"Tu reporte ha sido enviado."</p>
                            </div>
                        }>
                            // Screenshot Preview with Annotate Button
                            <Show when=move || screenshot_data.get().is_some()>
                                <div class="mb-4">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="text-xs text-zinc-500 uppercase tracking-wide font-medium">"📷 Captura de pantalla"</span>
                                        <button
                                            class="text-xs bg-zinc-700 hover:bg-zinc-600 text-orange-400 px-3 py-1.5 rounded-lg flex items-center gap-1.5 transition font-medium"
                                            on:click=toggle_annotate
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" />
                                            </svg>
                                            "Señalar problema"
                                        </button>
                                    </div>
                                    <div class="rounded-lg overflow-hidden border border-zinc-700 bg-zinc-800">
                                        <img
                                            src=move || screenshot_data.get().unwrap_or_default()
                                            alt="Screenshot"
                                            class="w-full h-32 object-cover object-top opacity-80"
                                        />
                                    </div>
                                </div>
                            </Show>

                            <label class="block text-xs text-zinc-500 uppercase tracking-wide font-medium mb-2">"Describe el problema"</label>
                            <textarea
                                class="w-full bg-zinc-800 border border-zinc-600 rounded-xl p-4 text-white placeholder-zinc-500 focus:outline-none focus:border-orange-500 focus:ring-2 focus:ring-orange-500/30 min-h-[100px] resize-none text-sm"
                                placeholder="¿Qué ocurrió? ¿Qué esperabas que pasara?"
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                disabled=move || is_sending.get()
                            ></textarea>

                            <div class="flex justify-between items-center mt-4">
                                <span class="text-xs text-zinc-500">"La captura se adjuntará automáticamente"</span>
                                <button
                                    class="bg-gradient-to-r from-orange-500 to-red-500 hover:from-orange-400 hover:to-red-400 text-white font-bold py-2.5 px-6 rounded-xl text-sm transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 shadow-lg"
                                    on:click=on_submit
                                    disabled=move || is_sending.get() || message.get().trim().is_empty()
                                >
                                    <Show when=move || is_sending.get() fallback=|| view! {
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                                        </svg>
                                        "Enviar"
                                    }>
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

        // Annotation Canvas Overlay (controlled by JS)
        <div id="annotation-overlay" class="fixed inset-0 z-[60] hidden">
            <div class="absolute inset-0 bg-black/90"></div>
            <div class="absolute inset-4 flex flex-col">
                <div class="flex justify-between items-center mb-2">
                    <div class="flex gap-2">
                        <button id="annotation-color-red" class="w-8 h-8 bg-red-500 rounded-full border-2 border-white hover:scale-110 transition"></button>
                        <button id="annotation-color-yellow" class="w-8 h-8 bg-yellow-400 rounded-full border-2 border-transparent hover:border-white hover:scale-110 transition"></button>
                        <button id="annotation-color-blue" class="w-8 h-8 bg-blue-500 rounded-full border-2 border-transparent hover:border-white hover:scale-110 transition"></button>
                    </div>
                    <div class="flex gap-2">
                        <button id="annotation-clear" class="bg-zinc-700 hover:bg-zinc-600 text-white px-4 py-2 rounded-lg text-sm font-medium">
                            "Borrar todo"
                        </button>
                        <button id="annotation-done" class="bg-orange-500 hover:bg-orange-400 text-white px-4 py-2 rounded-lg text-sm font-medium">
                            "Listo ✓"
                        </button>
                    </div>
                </div>
                <canvas id="annotation-canvas" class="flex-1 rounded-lg cursor-crosshair"></canvas>
            </div>
        </div>
    }
}
