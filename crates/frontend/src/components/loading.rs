//! Loading components

use leptos::*;

/// Full-screen loading spinner (basic)
#[component]
pub fn FullScreenLoader() -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-zinc-950/80 backdrop-blur-sm flex items-center justify-center z-50">
            <div class="flex flex-col items-center gap-4">
                <svg class="animate-spin h-12 w-12 text-orange-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span class="text-zinc-400 text-sm">"Cargando..."</span>
            </div>
        </div>
    }
}

/// Enhanced Report Loader with progress steps and rotating messages
#[component]
pub fn ReportLoader(#[prop(optional)] include_threat_intel: bool) -> impl IntoView {
    // Current step index (simulated progress)
    let current_step = create_rw_signal(0u8);
    let message_index = create_rw_signal(0usize);

    // Progress steps
    let steps = if include_threat_intel {
        vec![
            ("📡", "Conectando con APIs..."),
            ("📊", "Obteniendo métricas..."),
            ("🔐", "Analizando credenciales..."),
            ("🕵️", "Buscando en Dark Web..."),
            ("📡", "Rastreando viralidad..."),
            ("💰", "Detectando ads maliciosos..."),
            ("🎨", "Renderizando slides..."),
        ]
    } else {
        vec![
            ("📡", "Conectando con APIs..."),
            ("📊", "Obteniendo métricas..."),
            ("🎭", "Analizando amenazas..."),
            ("📸", "Descargando evidencias..."),
            ("🎨", "Renderizando slides..."),
        ]
    };

    // Rotating fun messages
    let messages = [
        "Preparando tu reporte ejecutivo...",
        "Analizando datos de seguridad...",
        "Procesando incidentes...",
        "Generando visualizaciones...",
        "Casi listo...",
        "Puliendo los detalles finales...",
    ];

    // Advance step every 8 seconds (to better match actual API timing)
    // Does NOT cycle - stays on last step once reached
    // Uses create_effect with set_timeout to respect component lifecycle (auto-cleanup on unmount)
    let steps_len = steps.len();
    create_effect(move |_| {
        fn schedule_step_advance(current_step: RwSignal<u8>, steps_len: usize) {
            set_timeout(
                move || {
                    // Check if signal is still valid before accessing
                    if let Some(current) = current_step.try_get() {
                        if (current as usize) < steps_len - 1 {
                            let _ = current_step.try_set(current + 1);
                            // Schedule next only if we're not at the end
                            schedule_step_advance(current_step, steps_len);
                        }
                    }
                },
                std::time::Duration::from_millis(8000),
            );
        }
        schedule_step_advance(current_step, steps_len);
    });

    // Keep rotating messages to show activity
    // Uses create_effect with set_timeout to respect component lifecycle
    let messages_len = messages.len();
    create_effect(move |_| {
        fn schedule_message_rotate(message_index: RwSignal<usize>, messages_len: usize) {
            set_timeout(
                move || {
                    // Check if signal is still valid before accessing
                    if let Some(current) = message_index.try_get() {
                        let next = (current + 1) % messages_len;
                        let _ = message_index.try_set(next);
                        // Keep rotating indefinitely while component is alive
                        schedule_message_rotate(message_index, messages_len);
                    }
                },
                std::time::Duration::from_millis(4000),
            );
        }
        schedule_message_rotate(message_index, messages_len);
    });

    view! {
        <div class="fixed inset-0 bg-zinc-950/90 backdrop-blur-md flex items-center justify-center z-50">
            <div class="bg-zinc-900 border border-zinc-800 rounded-2xl p-8 shadow-2xl max-w-md w-full mx-4">
                // Header
                <div class="text-center mb-6">
                    <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-gradient-to-br from-orange-500 to-orange-600 mb-4 shadow-lg shadow-orange-500/20">
                        <svg class="animate-spin h-8 w-8 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-2">"Generando Reporte"</h3>
                    <p class="text-zinc-400 text-sm transition-all duration-500">
                        {move || messages[message_index.get()]}
                    </p>
                </div>

                // Progress steps
                <div class="space-y-3 mb-6">
                    {steps.iter().enumerate().map(|(i, (icon, label))| {
                        let icon = icon.to_string();
                        let label = label.to_string();
                        let idx = i as u8;
                        view! {
                            <div class=move || format!(
                                "flex items-center gap-3 px-3 py-2 rounded-lg transition-all duration-300 {}",
                                if current_step.get() == idx {
                                    "bg-orange-500/10 border border-orange-500/30"
                                } else if idx < current_step.get() {
                                    "bg-zinc-800/50 opacity-50"
                                } else {
                                    "bg-zinc-800/30 opacity-30"
                                }
                            )>
                                <span class="text-lg">{icon.clone()}</span>
                                <span class=move || format!(
                                    "text-sm transition-colors {}",
                                    if current_step.get() == idx { "text-orange-400 font-medium" }
                                    else if idx < current_step.get() { "text-zinc-500 line-through" }
                                    else { "text-zinc-600" }
                                )>
                                    {label.clone()}
                                </span>
                                {move || if current_step.get() == idx {
                                    view! {
                                        <div class="ml-auto">
                                            <div class="w-2 h-2 rounded-full bg-orange-500 animate-pulse"></div>
                                        </div>
                                    }.into_view()
                                } else if idx < current_step.get() {
                                    view! {
                                        <div class="ml-auto text-green-400">
                                            "✓"
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Progress bar
                <div class="h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                    <div
                        class="h-full bg-gradient-to-r from-orange-500 to-orange-400 transition-all duration-500 ease-out"
                        style=move || format!("width: {}%", (current_step.get() as f32 / steps_len as f32 * 100.0) as i32)
                    ></div>
                </div>

                // Threat Intel badge if enabled
                {if include_threat_intel {
                    view! {
                        <div class="mt-4 text-center">
                            <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-xs">
                                <span>"🕵️"</span>
                                "Threat Hunting activo"
                            </span>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>
        </div>
    }
}

/// Login loader with specific message
#[component]
pub fn LoginLoader(#[prop(into)] message: Signal<String>) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-zinc-950/90 backdrop-blur-md flex items-center justify-center z-50">
            <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-6 shadow-xl max-w-sm w-full mx-4 text-center">
                <div class="inline-flex items-center justify-center w-14 h-14 rounded-full bg-gradient-to-br from-orange-500 to-orange-600 mb-4">
                    <svg class="animate-spin h-7 w-7 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                </div>
                <p class="text-white font-medium mb-1">{move || message.get()}</p>
                <p class="text-zinc-500 text-sm">"Por favor espera..."</p>
            </div>
        </div>
    }
}

/// Inline loading spinner
#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <svg class="animate-spin h-5 w-5 text-orange-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
    }
}
