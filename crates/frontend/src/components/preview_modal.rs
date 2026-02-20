//! Threat Hunting Preview Modal Component with SSE Streaming Support

use leptos::*;

/// Props for the preview modal
#[derive(Clone, Default)]
pub struct PreviewData {
    pub signal_lake_count: u64,
    pub credential_count: u64,
    pub chat_message_count: u64,
    pub forum_message_count: u64,
    pub total_count: u64,
    pub estimated_credits: u64,
    pub tickets_count: usize,
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
                <div class="bg-surface-base/95 backdrop-blur-2xl border border-white/10 rounded-2xl shadow-[0_0_50px_rgba(0,0,0,0.8)] max-w-lg w-full overflow-hidden">
                    // Header
                    <div class="bg-surface-elevated/50 border-b border-white/5 px-6 py-5 relative overflow-hidden">
                        <div class="absolute inset-x-0 bottom-0 h-px bg-gradient-to-r from-transparent via-brand-primary/50 to-transparent"></div>
                        <h2 class="text-white font-bold text-lg font-display tracking-wide">"Threat Hunting Preview"</h2>
                        <p class="text-white/60 text-sm mt-1">"Vista previa de resultados disponibles"</p>
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
                                        <div class="w-full bg-surface-base border border-white/5 rounded-full h-2.5 overflow-hidden shadow-inner">
                                            <div
                                                class="bg-brand-primary h-2.5 rounded-full transition-all duration-300 shadow-glow-orange"
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
                                            <div class="bg-surface-elevated/40 rounded-xl p-3 border border-white/5 text-center shadow-inner">
                                                <span class="text-zinc-400 text-xs block font-mono">"Signal-Lake"</span>
                                                <span class="text-lg font-bold text-blue-400">{progress.signal_lake_count}</span>
                                            </div>
                                            <div class="bg-surface-elevated/40 rounded-xl p-3 border border-white/5 text-center shadow-inner">
                                                <span class="text-zinc-400 text-xs block font-mono">"Chatter"</span>
                                                <span class="text-lg font-bold text-green-400">{progress.chatter_count}</span>
                                            </div>
                                            <div class="bg-surface-elevated/40 rounded-xl p-3 border border-white/5 text-center shadow-inner">
                                                <span class="text-zinc-400 text-xs block font-mono">"Credenciales"</span>
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
                                            <div class="bg-red-900/20 backdrop-blur-md border border-red-500/20 shadow-[0_0_15px_rgba(239,68,68,0.15)] rounded-xl p-4 mb-4">
                                                <p class="text-red-400 text-sm">{e}</p>
                                            </div>
                                        })}

                                        <div class="grid grid-cols-2 gap-4 mb-6">
                                            <div class="bg-surface-elevated/40 backdrop-blur-md rounded-xl p-4 border border-white/5 group hover:border-white/10 transition-colors">
                                                <span class="text-zinc-400 text-sm font-mono tracking-wider">"Signal-Lake"</span>
                                                <div class="text-2xl font-bold text-blue-400">{data.signal_lake_count}</div>
                                            </div>
                                            <div class="bg-surface-elevated/40 backdrop-blur-md rounded-xl p-4 border border-white/5 group hover:border-white/10 transition-colors">
                                                <span class="text-zinc-400 text-sm font-mono tracking-wider">"Credenciales"</span>
                                                <div class="text-2xl font-bold text-red-400">{data.credential_count}</div>
                                            </div>
                                            <div class="bg-surface-elevated/40 backdrop-blur-md rounded-xl p-4 border border-white/5 group hover:border-white/10 transition-colors">
                                                <span class="text-zinc-400 text-sm font-mono tracking-wider">"Chat/Telegram"</span>
                                                <div class="text-2xl font-bold text-green-400">{data.chat_message_count}</div>
                                            </div>
                                            <div class="bg-surface-elevated/40 backdrop-blur-md rounded-xl p-4 border border-white/5 group hover:border-white/10 transition-colors">
                                                <span class="text-zinc-400 text-sm font-mono tracking-wider">"Foros Dark Web"</span>
                                                <div class="text-2xl font-bold text-purple-400">{data.forum_message_count}</div>
                                            </div>
                                        </div>

                                        <div class="bg-amber-500/5 backdrop-blur-md border border-amber-500/20 shadow-[0_0_20px_rgba(245,158,11,0.05)] rounded-xl p-5 mb-6">
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
                                                <span class="text-brand-primary font-bold">"~" {data.estimated_credits}</span>
                                            </div>
                                        </div>

                                        <div class="mb-5 p-3 bg-brand-primary/10 border border-brand-primary/20 rounded-xl text-sm text-brand-primary/90 flex items-start gap-2 backdrop-blur-md font-mono">
                                            <span class="text-lg leading-none">"⚡"</span>
                                            <span>"La vista previa consume créditos iniciales de búsqueda (1 pag)."</span>
                                        </div>

                                        <p class="text-zinc-400 mb-6 px-1">
                                            "¿Desea generar el reporte completo e investigar a fondo estos hallazgos?"
                                        </p>

                                        <div class="bg-surface-elevated/50 border border-white/5 rounded-xl p-3 text-xs text-zinc-400/80 mb-2">
                                            "Al continuar, se consumiran creditos de Threat Hunting. Esta operacion puede tomar varios minutos."
                                        </div>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>

                    // Footer
                    <div class="bg-surface-elevated/60 border-t border-white/5 px-6 py-4 flex justify-end gap-3">
                        <button
                            class="px-4 py-2 rounded-xl bg-surface-base/50 border border-white/10 text-zinc-300 hover:text-white hover:bg-surface-elevated/80 transition-all duration-300 focus:outline-none"
                            on:click=move |_| on_cancel.call(())
                        >
                            "Cancelar"
                        </button>
                        <button
                            class="px-6 py-2 rounded-xl bg-brand-primary/20 hover:bg-brand-primary/40 border border-brand-primary/30 text-brand-primary hover:text-white font-bold tracking-wide transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:shadow-glow-orange"
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
