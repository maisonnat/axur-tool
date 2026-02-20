use crate::api::{self, TemplateListItem, Tenant};
use crate::utils::{download_html, get_today, parse_html_to_slides};
use crate::AppState;
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn LivePreviewPanel(
    #[prop(into)] report_html: Signal<Option<String>>,
    #[prop(into)] selected_tenant: Signal<String>,
    #[prop(into)] tenants: Signal<Vec<Tenant>>,
    #[prop(into)] user_templates: Signal<Vec<TemplateListItem>>,
    #[prop(into)] selected_pptx_template: Signal<Option<String>>,
    #[prop(into)] from_date: Signal<String>,
    #[prop(into)] to_date: Signal<String>,
    #[prop(into)] selected_template: Signal<Option<String>>,
    exporting_slides: RwSignal<bool>,
    exporting_pptx: RwSignal<bool>,
    state: AppState,
) -> impl IntoView {
    view! {
        <div class="glass-panel rounded-2xl p-1 overflow-hidden h-full">
            <div class="bg-surface-base/50 h-full flex flex-col backdrop-blur-md">
                // Preview Header
                <div class="p-6 border-b border-white/5 flex items-center justify-between bg-surface-base/80">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                        </div>
                        <h2 class="text-sm font-bold uppercase tracking-widest text-zinc-400 font-mono">
                            "Live Preview"
                        </h2>
                    </div>

                    <Show
                        when=move || report_html.get().is_some()
                        fallback=move || view! {
                            <div class="flex-1 flex flex-col items-center justify-center p-8 w-full h-full min-h-[500px]">
                                <div class="w-full max-w-2xl bg-zinc-900/40 border border-white/5 rounded-2xl p-8 shadow-[0_0_50px_rgba(0,0,0,0.5)] flex flex-col gap-6 relative overflow-hidden backdrop-blur-xl h-full">
                                    // Shimmer effect
                                    <div class="absolute inset-0 -translate-x-full animate-[shimmer_2s_infinite] bg-gradient-to-r from-transparent via-white/[0.03] to-transparent z-0"></div>

                                    <div class="relative z-10 flex justify-between items-start border-b border-white/5 pb-5">
                                        <div class="space-y-4 w-1/2">
                                            <div class="h-6 bg-zinc-800 rounded-md w-3/4 animate-pulse"></div>
                                            <div class="flex items-center gap-2">
                                                <span class="text-[10px] uppercase tracking-widest text-brand-primary font-mono opacity-80">"TARGET:"</span>
                                                <span class="text-xs font-bold text-white tracking-wider">
                                                    {move || {
                                                        let tenant_key = selected_tenant.get();
                                                        if tenant_key.is_empty() {
                                                            "AWAITING DATA".to_string()
                                                        } else {
                                                            tenants.get().iter().find(|t| t.key == tenant_key).map(|t| t.name.clone()).unwrap_or_else(|| tenant_key)
                                                        }
                                                    }}
                                                </span>
                                            </div>
                                        </div>
                                        <div class="text-right space-y-2">
                                            <div class="text-[9px] text-zinc-500 font-mono uppercase tracking-widest opacity-80">"SCAN PERIOD"</div>
                                            <div class="text-[10px] text-zinc-400 bg-black/40 px-2 py-1 rounded-md inline-block font-mono border border-white/5 shadow-inner">
                                                {move || format!("{}  ➡️  {}",
                                                    if from_date.get().is_empty() { "YYYY-MM-DD".into() } else { from_date.get() },
                                                    if to_date.get().is_empty() { "TODAY".into() } else { to_date.get() }
                                                )}
                                            </div>
                                        </div>
                                    </div>

                                    <div class="relative z-10 grid grid-cols-3 gap-4 mt-2">
                                        <div class="h-24 bg-zinc-800/50 rounded-xl border border-white/5 animate-pulse shadow-inner"></div>
                                        <div class="h-24 bg-zinc-800/50 rounded-xl border border-white/5 animate-pulse shadow-inner" style="animation-delay: 150ms;"></div>
                                        <div class="h-24 bg-zinc-800/50 rounded-xl border border-white/5 animate-pulse shadow-inner" style="animation-delay: 300ms;"></div>
                                    </div>

                                    <div class="relative z-10 space-y-4 mt-6 flex-1">
                                        <div class="h-3 bg-zinc-800/70 rounded w-full animate-pulse"></div>
                                        <div class="h-3 bg-zinc-800/70 rounded w-11/12 animate-pulse" style="animation-delay: 100ms;"></div>
                                        <div class="h-3 bg-zinc-800/70 rounded w-4/5 animate-pulse" style="animation-delay: 200ms;"></div>
                                        <div class="h-3 bg-zinc-800/70 rounded w-full mt-8 animate-pulse" style="animation-delay: 300ms;"></div>
                                        <div class="h-3 bg-zinc-800/70 rounded w-3/4 animate-pulse" style="animation-delay: 400ms;"></div>
                                    </div>

                                    <div class="absolute bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-3 text-zinc-500/80 bg-black/40 px-4 py-2 rounded-full backdrop-blur-md border border-white/5 shadow-xl">
                                        <svg class="w-4 h-4 animate-spin text-brand-primary/80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path></svg>
                                        <span class="text-[10px] font-mono uppercase tracking-widest text-[#FFF1E5]">
                                            {move || {
                                                if let Some(t) = selected_template.get() {
                                                    format!("PREPARED LAYER: {}", t)
                                                } else {
                                                    "AWAITING ENGINE DEPLOYMENT".to_string()
                                                }
                                            }}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        }
                    >
                        <div class="mt-auto pt-6 border-t border-white/5 flex flex-wrap gap-4">
                            <button
                                class="flex-1 bg-surface-base/40 backdrop-blur-md border border-white/10 hover:border-white/20 hover:bg-surface-elevated/60 text-white font-bold py-3 px-6 rounded-xl transition-all duration-300 flex items-center justify-center gap-2 group focus:outline-none focus:border-brand-primary/70 focus:shadow-glow-orange"
                                on:click=move |_| {
                                    if let Some(html) = report_html.get() {
                                        let tenant_key = selected_tenant.get();
                                        let tenant_name = tenants.get()
                                            .iter()
                                            .find(|t| t.key == tenant_key)
                                            .map(|t| t.name.clone())
                                            .unwrap_or_else(|| "report".to_string());

                                        let clean_name: String = tenant_name
                                            .chars()
                                            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
                                            .collect::<String>()
                                            .replace(' ', "_");

                                        let today = get_today();
                                        let filename = format!("{}_{}_report.html", clean_name, today);
                                        download_html(&html, &filename);
                                    }
                                }
                            >
                                <svg class="w-5 h-5 group-hover:text-brand-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                                "Download HTML"
                            </button>

                            <button
                                class="flex-1 bg-blue-500/10 backdrop-blur-md border border-blue-500/20 hover:border-blue-500/40 hover:bg-blue-500/20 text-blue-400 hover:text-white font-bold py-3 px-6 rounded-xl transition-all duration-300 flex items-center justify-center gap-2 group disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:border-blue-400/70 focus:shadow-glow-sm"
                                disabled=move || exporting_slides.get()
                                on:click={
                                    let state = state.clone();
                                    move |_| {
                                        if exporting_slides.get() { return; }
                                        exporting_slides.set(true);

                                        let html = report_html.get().unwrap_or_default();
                                        let tenant_key = selected_tenant.get();
                                        let tenant_name = tenants.get()
                                            .iter()
                                            .find(|t| t.key == tenant_key)
                                            .map(|t| t.name.clone())
                                            .unwrap_or_else(|| "Report".to_string());

                                        let state = state.clone();
                                        spawn_local(async move {
                                            let slide_data = parse_html_to_slides(&html);
                                            let title = format!("{} - Threat Report", tenant_name);

                                            match api::export_to_slides(&title, slide_data).await {
                                                Ok(resp) => {
                                                    leptos::logging::log!("Exported to Google Slides: {}", resp.presentation_url);
                                                    crate::onboarding::unlock_achievement("first_export", &state);
                                                    if let Some(window) = web_sys::window() {
                                                        let _ = window.open_with_url_and_target(
                                                            &resp.presentation_url,
                                                            "_blank"
                                                        );
                                                    }
                                                }
                                                Err(e) => {
                                                    leptos::logging::error!("Google Slides export failed: {}", e);
                                                    if let Some(window) = web_sys::window() {
                                                        let _ = window.alert_with_message(&format!("Export failed: {}", e));
                                                    }
                                                }
                                            }
                                            exporting_slides.set(false);
                                        });
                                    }
                                }
                            >
                                <span>{move || if exporting_slides.get() { "⏳" } else { "📊" }}</span>
                                <span>{move || if exporting_slides.get() { "Exporting..." } else { "Google Slides" }}</span>
                            </button>

                            // Export to PPTX button (only if user has templates)
                            <Show when=move || !user_templates.get().is_empty()>
                                <button
                                    class="flex-1 bg-purple-500/10 backdrop-blur-md border border-purple-500/20 hover:border-purple-500/40 hover:bg-purple-500/20 text-purple-400 hover:text-white font-bold py-3 px-6 rounded-xl transition-all duration-300 flex items-center justify-center gap-2 group disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:border-purple-400/70 focus:shadow-glow-sm"
                                    disabled=move || exporting_pptx.get()
                                    on:click=move |_| {
                                        if exporting_pptx.get() { return; }

                                        // Check if template is selected
                                        let templates = user_templates.get();
                                        if templates.is_empty() {
                                            if let Some(window) = web_sys::window() {
                                                let _ = window.alert_with_message("No tienes templates PPTX. Créalos en el Editor.");
                                            }
                                            return;
                                        }

                                        // Get first template (or selected)
                                        let template_id = selected_pptx_template.get()
                                            .or_else(|| templates.first().map(|t| t.id.clone()));

                                        let Some(tid) = template_id else { return; };

                                        exporting_pptx.set(true);
                                        let tenant_key = selected_tenant.get();
                                        let tenant_name = tenants.get()
                                            .iter()
                                            .find(|t| t.key == tenant_key)
                                            .map(|t| t.name.clone())
                                            .unwrap_or_else(|| "Report".to_string());

                                        spawn_local(async move {
                                            leptos::logging::log!("PPTX export initiated: template={}, tenant={}", tid, tenant_key);

                                            if let Some(window) = web_sys::window() {
                                                let msg = format!(
                                                    "📑 PPTX Export\n\n\
                                                    Template: {}\n\
                                                    Empresa: {}\n\n\
                                                    La funcionalidad completa requiere:\n\
                                                    1. Cargar template con archivo PPTX original\n\
                                                    2. El template debe tener placeholders definidos en el Editor\n\n\
                                                    Por ahora, usa el Editor para:\n\
                                                    - Importar tu PPTX\n\
                                                    - Agregar placeholders\n\
                                                    - Exportar con datos de ejemplo",
                                                    tid, tenant_name
                                                );
                                                let _ = window.alert_with_message(&msg);
                                            }

                                            exporting_pptx.set(false);
                                        });
                                    }
                                >
                                    <span>{move || if exporting_pptx.get() { "⏳" } else { "📑" }}</span>
                                    <span>{move || if exporting_pptx.get() { "Exportando..." } else { "PPTX Template" }}</span>
                                </button>
                            </Show>
                        </div>
                        <div class="bg-white rounded-lg overflow-hidden mt-6 aspect-[4/3] w-full">
                            <iframe
                                class="w-full h-full"
                                on:load=move |_| {
                                    // Optional: cleanup or additional logic
                                }
                                prop:src=move || {
                                    if let Some(html) = report_html.get() {
                                        // Create Blob URL for reliable rendering (fixes srcdoc issues)
                                        let options = web_sys::BlobPropertyBag::new();
                                        options.set_type("text/html");
                                        let blob = web_sys::Blob::new_with_str_sequence_and_options(
                                            &js_sys::Array::of1(&html.into()),
                                            &options,
                                        ).unwrap();
                                        web_sys::Url::create_object_url_with_blob(&blob).unwrap_or_default()
                                    } else {
                                        String::new()
                                    }
                                }
                            ></iframe>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}
