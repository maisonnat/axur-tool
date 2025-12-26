//! Threat Hunting Preview Modal Component with SSE Streaming Support

use leptos::*;

/// Props for the preview modal
#[derive(Clone)]
pub struct PreviewData {
    pub signal_lake_count: u64,
    pub credential_count: u64,
    pub chat_message_count: u64,
    pub forum_message_count: u64,
    pub total_count: u64,
    pub estimated_credits: u64,
    pub tickets_count: usize,
}

impl Default for PreviewData {
    fn default() -> Self {
        Self {
            signal_lake_count: 0,
            credential_count: 0,
            chat_message_count: 0,
            forum_message_count: 0,
            total_count: 0,
            estimated_credits: 0,
            tickets_count: 0,
        }
    }
}

/// Streaming progress state
#[derive(Clone, Default)]
pub struct StreamingProgress {
    pub is_streaming: bool,
    pub current_domain: String,
    pub current_index: usize,
    pub total_domains: usize,
    pub current_source: String,
    pub signal_lake_count: u64,
    pub chatter_count: u64,
    pub credential_count: u64,
    pub error_message: Option<String>,
    pub is_finished: bool,
}

impl StreamingProgress {
    pub fn progress_percentage(&self) -> f64 {
        if self.total_domains == 0 {
            0.0
        } else {
            // Each domain has 2 sources, so total steps = total_domains * 2
            let total_steps = self.total_domains * 2;
            let current_step = (self.current_index.saturating_sub(1)) * 2
                + if self.current_source == "credential" {
                    1
                } else {
                    0
                };
            (current_step as f64 / total_steps as f64) * 100.0
        }
    }
}

/// Threat Hunting Preview Modal with Streaming Support
#[component]
pub fn ThreatHuntingPreviewModal(
    is_open: ReadSignal<bool>,
    preview: ReadSignal<PreviewData>,
    is_loading: ReadSignal<bool>,
    streaming_progress: ReadSignal<StreamingProgress>,
    on_confirm: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || is_open.get()>
            <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                <div class="bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl max-w-lg w-full overflow-hidden">
                    // Header
                    <div class="bg-gradient-to-r from-orange-600 to-amber-600 px-6 py-4">
                        <h2 class="text-white font-bold text-lg">"Threat Hunting Preview"</h2>
                        <p class="text-white/70 text-sm">"Vista previa de resultados disponibles"</p>
                    </div>

                    // Content
                    <div class="p-6">
                        {move || {
                            let progress = streaming_progress.get();

                            // Show streaming progress if active
                            if progress.is_streaming && !progress.is_finished {
                                view! {
                                    <div class="space-y-4">
                                        // Progress bar
                                        <div class="w-full bg-zinc-700 rounded-full h-2.5">
                                            <div
                                                class="bg-gradient-to-r from-orange-500 to-amber-500 h-2.5 rounded-full transition-all duration-300"
                                                style=format!("width: {}%", progress.progress_percentage())
                                            ></div>
                                        </div>

                                        // Current operation
                                        <div class="text-center">
                                            <div class="flex items-center justify-center gap-2 mb-2">
                                                <div class="animate-spin h-5 w-5 border-2 border-orange-500 border-t-transparent rounded-full"></div>
                                                <span class="text-orange-400 font-medium">
                                                    "Buscando dominio " {progress.current_index} " / " {progress.total_domains}
                                                </span>
                                            </div>
                                            <p class="text-zinc-400 text-sm truncate max-w-xs mx-auto">
                                                {progress.current_domain.clone()}
                                            </p>
                                            <p class="text-zinc-500 text-xs mt-1">
                                                "Fuente: " {progress.current_source.clone()}
                                            </p>
                                        </div>

                                        // Running totals
                                        <div class="grid grid-cols-3 gap-3 mt-4">
                                            <div class="bg-zinc-800/50 rounded-lg p-2 border border-zinc-700 text-center">
                                                <span class="text-zinc-400 text-xs block">"Signal-Lake"</span>
                                                <span class="text-lg font-bold text-blue-400">{progress.signal_lake_count}</span>
                                            </div>
                                            <div class="bg-zinc-800/50 rounded-lg p-2 border border-zinc-700 text-center">
                                                <span class="text-zinc-400 text-xs block">"Chatter"</span>
                                                <span class="text-lg font-bold text-green-400">{progress.chatter_count}</span>
                                            </div>
                                            <div class="bg-zinc-800/50 rounded-lg p-2 border border-zinc-700 text-center">
                                                <span class="text-zinc-400 text-xs block">"Credenciales"</span>
                                                <span class="text-lg font-bold text-red-400">{progress.credential_count}</span>
                                            </div>
                                        </div>
                                    </div>
                                }.into_view()
                            } else if is_loading.get() {
                                // Legacy loading state
                                view! {
                                    <div class="flex flex-col items-center py-8">
                                        <div class="animate-spin h-10 w-10 border-4 border-orange-500 border-t-transparent rounded-full mb-4"></div>
                                        <p class="text-zinc-400">"Buscando en fuentes..."</p>
                                    </div>
                                }.into_view()
                            } else {
                                // Finished state - show results
                                let data = preview.get();
                                let error = progress.error_message.clone();

                                view! {
                                    <div>
                                        // Show error if present
                                        {error.map(|e| view! {
                                            <div class="bg-red-900/30 border border-red-500/50 rounded-lg p-4 mb-4">
                                                <p class="text-red-400 text-sm">{e}</p>
                                            </div>
                                        })}

                                        <div class="grid grid-cols-2 gap-4 mb-6">
                                            <div class="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
                                                <span class="text-zinc-400 text-sm">"Signal-Lake"</span>
                                                <div class="text-2xl font-bold text-blue-400">{data.signal_lake_count}</div>
                                            </div>
                                            <div class="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
                                                <span class="text-zinc-400 text-sm">"Credenciales"</span>
                                                <div class="text-2xl font-bold text-red-400">{data.credential_count}</div>
                                            </div>
                                            <div class="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
                                                <span class="text-zinc-400 text-sm">"Chat/Telegram"</span>
                                                <div class="text-2xl font-bold text-green-400">{data.chat_message_count}</div>
                                            </div>
                                            <div class="bg-zinc-800/50 rounded-lg p-4 border border-zinc-700">
                                                <span class="text-zinc-400 text-sm">"Foros Dark Web"</span>
                                                <div class="text-2xl font-bold text-purple-400">{data.forum_message_count}</div>
                                            </div>
                                        </div>

                                        <div class="bg-amber-900/20 border border-amber-500/30 rounded-lg p-4 mb-6">
                                            <div class="flex items-center justify-between mb-2">
                                                <span class="text-amber-300 font-medium">"Resumen"</span>
                                                <span class="text-white font-bold text-lg">{data.total_count} " resultados"</span>
                                            </div>
                                            <div class="flex items-center justify-between text-sm mb-1">
                                                <span class="text-amber-200/70">"Tickets Base:"</span>
                                                <span class="text-amber-100">{data.tickets_count}</span>
                                            </div>
                                            <div class="flex items-center justify-between text-sm">
                                                <span class="text-amber-200/70">"Creditos estimados:"</span>
                                                <span class="text-amber-400 font-bold">"~" {data.estimated_credits}</span>
                                            </div>
                                        </div>

                                        <div class="mb-4 p-3 bg-yellow-900/10 border border-yellow-500/20 rounded text-sm text-yellow-200">
                                            <span class="font-bold">Nota:</span> "La vista previa consume créditos iniciales de búsqueda (1 pag)."
                                        </div>

                                        <p class="text-gray-400 mb-6">
                                            "¿Desea generar el reporte completo e investigar a fondo estos hallazgos?"
                                        </p>

                                        <div class="bg-zinc-800 rounded-lg p-3 text-xs text-zinc-400 mb-6">
                                            "Al continuar, se consumiran creditos de Threat Hunting. Esta operacion puede tomar 1-2 minutos."
                                        </div>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>

                    // Footer
                    <div class="bg-zinc-800/50 px-6 py-4 flex justify-end gap-3">
                        <button
                            class="px-4 py-2 rounded-lg border border-zinc-600 text-zinc-300 hover:bg-zinc-700 transition-colors"
                            on:click=move |_| on_cancel.call(())
                        >
                            "Cancelar"
                        </button>
                        <button
                            class="px-6 py-2 rounded-lg bg-orange-600 hover:bg-orange-500 text-white font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                            on:click=move |_| on_confirm.call(())
                            disabled=move || is_loading.get() || (streaming_progress.get().is_streaming && !streaming_progress.get().is_finished)
                        >
                            "Generar Reporte Completo"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
