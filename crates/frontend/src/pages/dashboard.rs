//! Dashboard page with tenant selector and report generation

use crate::api::{self, GenerateReportResponse, Tenant};
use crate::components::*;
use crate::{get_ui_dict, AppState, Page};
use leptos::*;

/// Structured error with optional code for display
#[derive(Clone)]
struct AppError {
    /// Error code like "TI-001" (optional for legacy errors)
    code: Option<String>,
    /// User-friendly message
    message: String,
}

impl AppError {
    fn simple(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
        }
    }

    fn from_response(resp: &GenerateReportResponse) -> Self {
        Self {
            code: resp.error_code.clone(),
            message: resp
                .error_message
                .clone()
                .or_else(|| Some(resp.message.clone()))
                .unwrap_or_else(|| "Error desconocido".into()),
        }
    }
}

/// Dashboard page component
#[component]
pub fn DashboardPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    // Extract signals we need
    let ui_language = state.ui_language;
    let is_authenticated = state.is_authenticated;
    let current_page = state.current_page;

    // Get dictionary based on current language
    let dict = Signal::derive(move || get_ui_dict(ui_language.get()));

    // Data
    let tenants = create_rw_signal(Vec::<Tenant>::new());
    let selected_tenant = create_rw_signal(String::new());
    let from_date = create_rw_signal(get_default_from_date());
    let to_date = create_rw_signal(get_today());
    // Default report language to the UI language
    let language = create_rw_signal(ui_language.get_untracked().code().to_string());
    let story_tag = create_rw_signal(String::new());
    let include_threat_intel = create_rw_signal(false);
    let use_user_credits = create_rw_signal(false);

    // UI state
    let loading_tenants = create_rw_signal(true);
    let generating = create_rw_signal(false);
    let error = create_rw_signal(Option::<AppError>::None);
    let report_html = create_rw_signal(Option::<String>::None);

    // Preview modal state
    let show_preview_modal = create_rw_signal(false);
    let preview_loading = create_rw_signal(false);
    let preview_data = create_rw_signal(PreviewData::default());
    let preview_confirmed = create_rw_signal(false);
    let streaming_progress = create_rw_signal(StreamingProgress::default());

    // Load tenants on mount
    spawn_local(async move {
        match api::list_tenants().await {
            Ok(t) => {
                tenants.set(t);
            }
            Err(e) => error.set(Some(AppError::simple(e))),
        }
        loading_tenants.set(false);
    });

    // Tenant options for select
    let tenant_options = Signal::derive(move || {
        tenants
            .get()
            .iter()
            .map(|t| (t.key.clone(), t.name.clone()))
            .collect::<Vec<_>>()
    });

    // Generate report action - shows preview first if Threat Hunting enabled
    let generate_action = create_action(move |_: &()| async move {
        if selected_tenant.get().is_empty() {
            error.set(Some(AppError {
                code: Some("RPT-004".into()),
                message: "Selecciona un tenant para continuar".into(),
            }));
            return;
        }

        let threat_intel = include_threat_intel.get();
        let tag = story_tag.get();

        // If Threat Hunting enabled and tag provided, show preview first
        if threat_intel && !tag.trim().is_empty() && !preview_confirmed.get() {
            show_preview_modal.set(true);

            // Reset streaming state
            streaming_progress.set(StreamingProgress {
                is_streaming: true,
                current_domain: String::new(),
                current_index: 0,
                total_domains: 0,
                current_source: String::new(),
                signal_lake_count: 0,
                chatter_count: 0,
                credential_count: 0,
                error_message: None,
                is_finished: false,
            });

            // Start SSE streaming
            let tenant = selected_tenant.get();
            let stream_url =
                api::get_threat_hunting_stream_url(&tenant, &tag, use_user_credits.get());

            // Use JavaScript EventSource for SSE
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            // Create EventSource with credentials
            let init = web_sys::EventSourceInit::new();
            init.set_with_credentials(true);
            let es = web_sys::EventSource::new_with_event_source_init_dict(&stream_url, &init);

            match es {
                Ok(event_source) => {
                    // Clone signals for closures
                    let sp = streaming_progress.clone();
                    let pd = preview_data.clone();
                    let pl = preview_loading.clone();
                    let _spm = show_preview_modal.clone();
                    let _err = error.clone();
                    let es_clone = event_source.clone();

                    // Message handler
                    let onmessage = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                        if let Some(data) = e.data().as_string() {
                            if let Ok(event) =
                                serde_json::from_str::<api::ThreatHuntingStreamEvent>(&data)
                            {
                                match event {
                                    api::ThreatHuntingStreamEvent::Started {
                                        total_domains,
                                        total_tickets,
                                    } => {
                                        sp.update(|s| {
                                            s.total_domains = total_domains;
                                            s.is_streaming = true;
                                        });
                                        pd.update(|p| p.tickets_count = total_tickets);
                                    }
                                    api::ThreatHuntingStreamEvent::DomainProcessing {
                                        domain,
                                        index,
                                        source,
                                    } => {
                                        sp.update(|s| {
                                            s.current_domain = domain;
                                            s.current_index = index;
                                            s.current_source = source;
                                        });
                                    }
                                    api::ThreatHuntingStreamEvent::DomainComplete {
                                        source,
                                        count,
                                        ..
                                    } => {
                                        sp.update(|s| {
                                            if source == "signal-lake" {
                                                s.signal_lake_count += count;
                                            } else if source == "credential" {
                                                s.credential_count += count;
                                            } else {
                                                s.chatter_count += count;
                                            }
                                        });
                                    }
                                    api::ThreatHuntingStreamEvent::Finished {
                                        total_count,
                                        signal_lake_count,
                                        chatter_count,
                                        credential_count,
                                        estimated_credits,
                                    } => {
                                        pd.set(PreviewData {
                                            signal_lake_count,
                                            credential_count,
                                            chat_message_count: chatter_count,
                                            forum_message_count: 0,
                                            total_count,
                                            estimated_credits: estimated_credits as u64,
                                            tickets_count: pd.get_untracked().tickets_count,
                                        });
                                        sp.update(|s| {
                                            s.is_finished = true;
                                            s.is_streaming = false;
                                        });
                                        pl.set(false);
                                        es_clone.close();
                                    }
                                    api::ThreatHuntingStreamEvent::Error { message } => {
                                        sp.update(|s| {
                                            s.error_message = Some(message.clone());
                                            s.is_finished = true;
                                            s.is_streaming = false;
                                        });
                                        pl.set(false);
                                        es_clone.close();
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget(); // Prevent closure from being dropped

                    // Error handler
                    let sp_err = streaming_progress.clone();
                    let pl_err = preview_loading.clone();
                    let es_err = event_source.clone();
                    let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        sp_err.update(|s| {
                            s.error_message = Some("Connection error".into());
                            s.is_finished = true;
                            s.is_streaming = false;
                        });
                        pl_err.set(false);
                        es_err.close();
                    }) as Box<dyn FnMut(_)>);

                    event_source.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                    onerror.forget();
                }
                Err(_) => {
                    // Fallback to regular API if EventSource fails
                    error.set(Some(AppError::simple("Failed to create EventSource")));
                    show_preview_modal.set(false);
                }
            }

            return;
        }

        // Generate full report
        generating.set(true);
        error.set(None);
        report_html.set(None);
        preview_confirmed.set(false);

        let tenant = selected_tenant.get();
        let from = from_date.get();
        let to = to_date.get();
        let lang = language.get();
        let tag_opt = if tag.trim().is_empty() {
            None
        } else {
            Some(tag)
        };

        match api::generate_report(&tenant, &from, &to, &lang, tag_opt, threat_intel).await {
            Ok(resp) => {
                if resp.success {
                    report_html.set(resp.html);
                } else {
                    error.set(Some(AppError::from_response(&resp)));
                }
            }
            Err(e) => error.set(Some(AppError::simple(e))),
        }
        generating.set(false);
    });

    // Preview callbacks
    let on_preview_confirm = Callback::new(move |_| {
        show_preview_modal.set(false);
        preview_confirmed.set(true);
        generate_action.dispatch(());
    });

    let on_preview_cancel = Callback::new(move |_| {
        show_preview_modal.set(false);
        preview_confirmed.set(false);
    });

    // Logout action
    let logout_action = create_action(move |_: &()| async move {
        let _ = api::logout().await;
        is_authenticated.set(false);
        current_page.set(Page::Login);
    });

    view! {
        // Loading overlay with enhanced progress
        <Show when=move || generating.get()>
            <ReportLoader include_threat_intel=include_threat_intel.get() />
        </Show>

        // Threat Hunting Preview Modal
        <ThreatHuntingPreviewModal
            is_open=show_preview_modal.read_only()
            preview=preview_data.read_only()
            is_loading=preview_loading.read_only()
            streaming_progress=streaming_progress.read_only()
            on_confirm=on_preview_confirm
            on_cancel=on_preview_cancel
        />

        <div class="min-h-screen">
            // Header
            <header class="bg-zinc-900 border-b border-zinc-800 px-6 py-4">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <span class="text-orange-500 text-2xl font-black italic">"///"</span>
                        <span class="text-white text-xl font-bold tracking-widest">"AXUR"</span>
                        <span class="text-zinc-500 ml-2">"Web"</span>
                    </div>
                    <button
                        class="text-zinc-400 hover:text-white transition-colors"
                        on:click=move |_| logout_action.dispatch(())
                    >
                        {move || dict.get().logout}
                    </button>
                </div>
            </header>

            // Main content
            <main class="max-w-7xl mx-auto p-6">
                <h1 class="text-2xl font-bold text-white mb-6">{move || dict.get().generate_report}</h1>

                // Error message with structured error code
                {move || error.get().map(|e| {
                    view! {
                        <div class="mb-6">
                            <ErrorDisplay
                                error_code=e.code.clone().unwrap_or_else(|| "SYS-001".into())
                                error_message=e.message.clone()
                                on_retry=Callback::new(move |_| {
                                    error.set(None);
                                })
                                on_dismiss=Callback::new(move |_| {
                                    error.set(None);
                                })
                            />
                        </div>
                    }
                })}

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    // Form
                    <div class="lg:col-span-1">
                        <CardWithHeader title=Signal::derive(move || dict.get().configuration.to_string())>
                            <Show
                                when=move || !loading_tenants.get()
                                fallback=|| view! {
                                    <div class="text-center py-4">
                                        <div class="animate-spin h-6 w-6 border-2 border-orange-500 border-t-transparent rounded-full mx-auto"></div>
                                        <p class="text-zinc-400 text-sm mt-2">"Cargando tenants..."</p>
                                    </div>
                                }
                            >
                                <Combobox
                                    label=Signal::derive(move || dict.get().tenant_label.to_string())
                                    options=tenant_options
                                    selected=selected_tenant
                                    placeholder=Signal::derive(move || dict.get().tenant_placeholder.to_string())
                                />
                            </Show>

                            <div class="mb-4">
                                <label class="block text-sm font-medium text-zinc-400 mb-2">{move || dict.get().from_date}</label>
                                <input
                                    type="date"
                                    class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-3 px-4 outline-none"
                                    prop:value=move || from_date.get()
                                    on:input=move |ev| from_date.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="mb-4">
                                <label class="block text-sm font-medium text-zinc-400 mb-2">{move || dict.get().to_date}</label>
                                <input
                                    type="date"
                                    class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-3 px-4 outline-none"
                                    prop:value=move || to_date.get()
                                    on:input=move |ev| to_date.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="mb-4">
                                <label class="block text-sm font-medium text-zinc-400 mb-2">"Story Tag (Optional)"</label>
                                <input
                                    type="text"
                                    class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-3 px-4 outline-none placeholder-zinc-600"
                                    placeholder="tag:campaign-name"
                                    prop:value=move || story_tag.get()
                                    on:input=move |ev| story_tag.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="mb-6">
                                <label class="block text-sm font-medium text-zinc-400 mb-2">{move || dict.get().language_label}</label>
                                <select
                                    class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-3 px-4 outline-none cursor-pointer"
                                    on:change=move |ev| language.set(event_target_value(&ev))
                                >
                                    <option value="es" selected>"Español"</option>
                                    <option value="en">"English"</option>
                                    <option value="pt">"Português"</option>
                                </select>
                            </div>

                            // Threat Hunting Toggle
                            <div class="mb-6 p-4 rounded-lg bg-zinc-800/50 border border-zinc-700">
                                <div class="flex items-center justify-between">
                                    <div class="flex-1">
                                        <label class="flex items-center gap-2 cursor-pointer">
                                            <input
                                                type="checkbox"
                                                class="w-5 h-5 rounded border-zinc-600 bg-zinc-700 text-purple-500 focus:ring-purple-500 focus:ring-offset-zinc-900 cursor-pointer"
                                                prop:checked=move || include_threat_intel.get()
                                                on:change=move |ev| include_threat_intel.set(event_target_checked(&ev))
                                            />
                                            <span class="text-white font-medium">"🔍 Threat Hunting"
                                            </span>
                                        </label>
                                        <p class="text-zinc-500 text-xs mt-1 ml-7">"Investigación profunda en Signal-Lake, credenciales y foros"
                                        </p>
                                    </div>
                                </div>
                                // Credits warning - subtle but visible
                                <Show when=move || include_threat_intel.get()>
                                    <div class="mt-3 flex items-center gap-2 text-amber-400/80 text-xs bg-amber-900/20 rounded px-3 py-2 border border-amber-500/20">
                                        <span class="text-sm">"⚡"
                                        </span>
                                        <span>"Consume créditos de Threat Hunting · Añade ~1-2 min al reporte"
                                        </span>
                                    </div>

                                    // Admin credits toggle
                                    <div class="mt-3 pt-3 border-t border-zinc-700/50">
                                        <label class="flex items-center gap-2 cursor-pointer">
                                            <input
                                                type="checkbox"
                                                class="w-4 h-4 rounded border-zinc-600 bg-zinc-700 text-blue-500 focus:ring-blue-500 focus:ring-offset-zinc-900 cursor-pointer"
                                                prop:checked=move || use_user_credits.get()
                                                on:change=move |ev| use_user_credits.set(event_target_checked(&ev))
                                            />
                                            <span class="text-zinc-300 text-sm">"Usar créditos de Admin Global"
                                            </span>
                                        </label>
                                        <p class="text-zinc-500 text-xs mt-1 ml-6">"Seleccionar si el tenant no tiene créditos asignados"
                                        </p>
                                    </div>
                                </Show>
                            </div>

                            <button
                                class="w-full bg-orange-600 hover:bg-orange-500 disabled:bg-zinc-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
                                disabled=move || generating.get()
                                on:click=move |_| generate_action.dispatch(())
                            >
                                {move || if generating.get() {
                                    view! {
                                        <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                    }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                                {move || dict.get().generate_btn}
                            </button>
                        </CardWithHeader>
                    </div>

                    // Preview
                    <div class="lg:col-span-2">
                        <CardWithHeader title=Signal::derive(move || dict.get().preview.to_string())>
                            <Show
                                when=move || report_html.get().is_some()
                                fallback=|| view! {
                                    <div class="text-center py-16 text-zinc-500">
                                        <svg class="w-16 h-16 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                        </svg>
                                        <p>"Configura y genera un reporte para ver la vista previa"</p>
                                    </div>
                                }
                            >
                                <div class="mb-4">
                                    <button
                                        class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
                                        on:click=move |_| {
                                            if let Some(html) = report_html.get() {
                                                // Generate descriptive filename
                                                let tenant_key = selected_tenant.get();
                                                let tenant_name = tenants.get()
                                                    .iter()
                                                    .find(|t| t.key == tenant_key)
                                                    .map(|t| t.name.clone())
                                                    .unwrap_or_else(|| "report".to_string());

                                                // Clean name for filename (remove special chars)
                                                let clean_name: String = tenant_name
                                                    .chars()
                                                    .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
                                                    .collect::<String>()
                                                    .replace(' ', "_");

                                                let today = get_today(); // YYYY-MM-DD
                                                let filename = format!("{}_{}_report.html", clean_name, today);
                                                download_html(&html, &filename);
                                            }
                                        }
                                    >
                                        {move || dict.get().download_html}
                                    </button>
                                </div>
                                <div class="bg-white rounded-lg overflow-hidden aspect-video">
                                    <iframe
                                        class="w-full h-full"
                                        srcdoc=move || report_html.get().unwrap_or_default()
                                    ></iframe>
                                </div>
                            </Show>
                        </CardWithHeader>
                    </div>
                </div>
            </main>
            <FeedbackWidget />
        </div>
    }
}

// Helper functions
fn get_today() -> String {
    let date = js_sys::Date::new_0();
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

fn get_default_from_date() -> String {
    let date = js_sys::Date::new_0();
    // Subtract 30 days in milliseconds (30 * 24 * 60 * 60 * 1000)
    let thirty_days_ms = 30.0 * 24.0 * 60.0 * 60.0 * 1000.0;
    date.set_time(date.get_time() - thirty_days_ms);
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

fn download_html(content: &str, filename: &str) {
    use wasm_bindgen::JsCast;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create blob with proper options
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/html");

    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&content.into()),
        &options,
    )
    .unwrap();

    // Create URL
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    // Create and click download link
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();

    // Cleanup
    web_sys::Url::revoke_object_url(&url).unwrap();
}
