//! Dashboard page with tenant selector and report generation

use crate::api::{self, GenerateReportResponse, TemplateListItem, Tenant};
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

/// Helper to create a slide toggle checkbox
fn slide_toggle(
    plugin_id: &'static str,
    label: &'static str,
    disabled_slides: RwSignal<Vec<String>>,
) -> impl IntoView {
    let is_enabled = move || !disabled_slides.get().contains(&plugin_id.to_string());

    let toggle = move |_| {
        disabled_slides.update(|list| {
            if list.contains(&plugin_id.to_string()) {
                list.retain(|id| id != plugin_id);
            } else {
                list.push(plugin_id.to_string());
            }
        });
    };

    view! {
        <label class="flex items-center gap-2 text-zinc-400 cursor-pointer hover:text-zinc-300">
            <input
                type="checkbox"
                class="w-3 h-3 rounded"
                prop:checked=is_enabled
                on:change=toggle
            />
            {label}
        </label>
    }
}

/// Dashboard page component
#[component]
#[allow(non_snake_case)]
pub fn DashboardPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    // Extract signals we need
    let ui_language = state.ui_language;
    let is_authenticated = state.is_authenticated;
    let current_page = state.current_page;
    let has_log_access = state.has_log_access;

    // Get dictionary based on current language
    let dict = create_memo(move |_| get_ui_dict(ui_language.get()));

    // Data
    let tenants = create_rw_signal(Vec::<Tenant>::new());
    let selected_tenant = create_rw_signal(String::new());
    let user_templates = create_rw_signal(Vec::<TemplateListItem>::new());
    let from_date = create_rw_signal(get_default_from_date());
    let to_date = create_rw_signal(get_today());
    // Default report language to the UI language
    let language = create_rw_signal(ui_language.get_untracked().code().to_string());
    let story_tag = create_rw_signal(String::new());
    let include_threat_intel = create_rw_signal(false);
    let use_user_credits = create_rw_signal(false);
    let use_plugins = create_rw_signal(false); // New plugin system toggle

    // Load preferences from localStorage
    let plugin_theme = create_rw_signal(crate::load_theme());
    let disabled_slides = create_rw_signal(crate::load_disabled_slides());
    let selected_template = create_rw_signal::<Option<String>>(None);

    // Auto-save theme when it changes
    create_effect(move |_| {
        let theme = plugin_theme.get();
        crate::save_theme(&theme);
    });

    // Auto-save disabled slides when they change
    create_effect(move |_| {
        let slides = disabled_slides.get();
        crate::save_disabled_slides(&slides);
    });

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

    // Queue status
    let current_queue_job = create_rw_signal::<Option<String>>(None);

    // Google Slides export state
    let exporting_slides = create_rw_signal(false);

    // PPTX export state
    let exporting_pptx = create_rw_signal(false);
    let selected_pptx_template = create_rw_signal::<Option<String>>(None);

    // Beta requests pending count (for admin notification badge)
    let pending_beta_count = create_rw_signal(0i64);

    // Load tenants on mount
    spawn_local(async move {
        match api::list_tenants().await {
            Ok(t) => {
                tenants.set(t);
            }
            Err(e) => error.set(Some(AppError::simple(e))),
        }

        // Fetch user templates
        match api::list_templates().await {
            Ok(resp) => {
                if resp.success {
                    user_templates.set(resp.templates);
                }
            }
            Err(e) => leptos::logging::error!("Failed to fetch templates: {}", e),
        }

        loading_tenants.set(false);
    });

    // Fetch pending beta requests count for admin badge
    let is_admin = state.is_admin;
    spawn_local(async move {
        if is_admin.get_untracked() {
            if let Ok(resp) = gloo_net::http::Request::get("/api/admin/beta/requests/pending-count")
                .send()
                .await
            {
                if resp.ok() {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(count) = json.get("count").and_then(|v| v.as_i64()) {
                            pending_beta_count.set(count);
                        }
                    }
                }
            }
        }
    });

    // Tenant options for select
    let tenant_options = create_memo(move |_| {
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

        // Generate full report via SSE streaming (avoids Cloudflare 524 timeout)
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
            Some(tag.as_str())
        };

        let template_id = selected_template.get();

        // Build SSE stream URL
        let stream_url = api::get_report_stream_url(
            &tenant,
            &from,
            &to,
            &lang,
            tag_opt,
            threat_intel,
            template_id.as_deref(),
            use_plugins.get(),
        );

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
                let report = report_html.clone();
                let err = error.clone();
                let gen = generating.clone();
                let ti = threat_intel;
                let es_clone = event_source.clone();

                // Message handler
                let onmessage = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                    if let Some(data) = event.data().as_string() {
                        match serde_json::from_str::<api::ReportStreamEvent>(&data) {
                            Ok(evt) => match evt {
                                api::ReportStreamEvent::StageProgress { message, .. } => {
                                    leptos::logging::log!("Report progress: {}", message);
                                }
                                api::ReportStreamEvent::Finished { html, .. } => {
                                    report.set(Some(html));
                                    gen.set(false);
                                    es_clone.close();
                                    // Unlock achievement if threat intel was used
                                    if ti {
                                        crate::onboarding::unlock_achievement("threat_hunting");
                                    }
                                }
                                api::ReportStreamEvent::Error { code, message } => {
                                    err.set(Some(AppError {
                                        code: Some(code),
                                        message,
                                    }));
                                    gen.set(false);
                                    es_clone.close();
                                }
                                _ => {} // Started, StageComplete - just progress indicators
                            },
                            Err(e) => {
                                leptos::logging::error!("Failed to parse SSE event: {}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                // Error handler
                let err_clone = error.clone();
                let gen_clone = generating.clone();
                let es_err = event_source.clone();
                let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    err_clone.set(Some(AppError::simple(
                        "Connection error during report generation",
                    )));
                    gen_clone.set(false);
                    es_err.close();
                }) as Box<dyn FnMut(_)>);

                event_source.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                onerror.forget();
            }
            Err(_) => {
                error.set(Some(AppError::simple(
                    "Failed to create EventSource for report generation",
                )));
                generating.set(false);
            }
        }
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
                    <div class="flex items-center gap-4">
                        // Editor & Marketplace - visible to all
                        <button
                            class="text-zinc-400 hover:text-indigo-400 transition-colors flex items-center gap-2"
                            on:click=move |_| current_page.set(Page::Editor)
                        >
                            "✏️ Editor"
                        </button>
                        <button
                            class="text-zinc-400 hover:text-purple-400 transition-colors flex items-center gap-2"
                            on:click=move |_| current_page.set(Page::Marketplace)
                        >
                            "📦 Templates"
                        </button>
                        // Sandbox Demo button
                        <button
                            class="text-amber-400 hover:text-amber-300 transition-colors flex items-center gap-2"
                            on:click=move |_| {
                                crate::onboarding::set_sandbox_mode(true);
                                current_page.set(Page::Editor);
                            }
                        >
                            "Probar Demo"
                        </button>
                        <button
                            class="text-zinc-400 hover:text-blue-400 transition-colors flex items-center gap-2"
                            on:click=move |_| current_page.set(Page::Onboarding)
                        >
                            "Help"
                        </button>
                        <Show when=move || has_log_access.get()>
                            <button
                                class="text-zinc-400 hover:text-emerald-400 transition-colors flex items-center gap-2"
                                on:click=move |_| current_page.set(Page::Analytics)
                            >
                                "📊 Analytics"
                            </button>
                            <button
                                class="text-zinc-400 hover:text-emerald-400 transition-colors flex items-center gap-2"
                                on:click=move |_| current_page.set(Page::Logs)
                            >
                                "📋 Logs"
                            </button>
                        </Show>
                        // Admin button in header (for admins)
                        <Show when=move || state.is_admin.get()>
                            <button
                                class="text-red-400 hover:text-red-300 transition-colors flex items-center gap-2 relative"
                                on:click=move |_| current_page.set(Page::BetaAdmin)
                            >
                                "Admin"
                                {move || {
                                    let count = pending_beta_count.get();
                                    if count > 0 {
                                        Some(view! {
                                            <span class="absolute -top-1 -right-3 bg-yellow-500 text-black text-xs font-bold w-5 h-5 rounded-full flex items-center justify-center animate-pulse">
                                                {count}
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }}
                            </button>
                        </Show>
                        <button
                            class="text-zinc-400 hover:text-white transition-colors"
                            on:click=move |_| logout_action.dispatch(())
                        >
                            {move || dict.get().logout}
                        </button>
                    </div>
                </div>
            </header>

            // Progress Checklist (onboarding achievements)
            <crate::components::ProgressChecklist />

            // Tutorial Orchestrator (shows welcome modal for new users)
            <crate::components::TutorialOrchestrator />

            // Hint Manager (shows contextual hints after idle)
            <crate::components::HintManager />

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
                                    <div class="mb-4">
                                        <div class="h-4 w-20 bg-zinc-700 rounded animate-pulse mb-2"></div>
                                        <div class="relative">
                                            <div class="w-full h-12 bg-zinc-800 border border-zinc-700 rounded-lg animate-pulse"></div>
                                            <div class="absolute inset-0 flex items-center justify-center gap-2">
                                                <div class="animate-spin h-4 w-4 border-2 border-orange-500 border-t-transparent rounded-full"></div>
                                                <span class="text-zinc-500 text-sm">"Conectando..."</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            >
                                <Combobox
                                    label=Signal::derive(move || dict.get().tenant_label.to_string())
                                    options=tenant_options.into()
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

                            // Template selection
                            <div class="mb-4">
                                <label class="block text-sm font-medium text-zinc-400 mb-2">"📄 Report Template"</label>
                                <div class="flex gap-2">
                                    <select
                                        class="flex-1 bg-zinc-800 border border-zinc-700 text-white rounded-lg py-3 px-4 outline-none appearance-none"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            if val == "default" {
                                                selected_template.set(None);
                                            } else {
                                                selected_template.set(Some(val));
                                            }
                                        }
                                    >
                                        <optgroup label="System Templates">
                                            <option value="default" selected>"⭐ Axur Default"</option>
                                            <option value="executive">"📊 Executive Summary"</option>
                                            <option value="technical">"🔧 Technical Deep Dive"</option>
                                            <option value="compliance">"📋 Compliance Report"</option>
                                        </optgroup>
                                        <optgroup label="My Templates">
                                            <Show when=move || !user_templates.get().is_empty()
                                                fallback=|| view! { <option disabled>"No templates yet"</option> }
                                            >
                                                <For
                                                    each=move || user_templates.get()
                                                    key=|t| t.id.clone()
                                                    children=move |t| view! {
                                                        <option value={t.id.clone()}>{t.name}</option>
                                                    }
                                                />
                                            </Show>
                                        </optgroup>
                                        <optgroup label="Options">
                                            <option value="custom">"✏️ Browse Custom..."</option>
                                        </optgroup>
                                    </select>
                                    <button
                                        on:click=move |_| state.current_page.set(crate::Page::Marketplace)
                                        class="px-4 py-2 bg-zinc-700 hover:bg-zinc-600 text-white rounded-lg text-sm"
                                        title="Open Template Marketplace"
                                    >
                                        "🛒"
                                    </button>
                                </div>
                                <Show when=move || selected_template.get().is_some()>
                                    <p class="text-xs text-indigo-400 mt-1">
                                        "Using custom template: " {move || selected_template.get().unwrap_or_default()}
                                    </p>
                                </Show>
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

                            // Plugin System Toggle (Experimental)
                            <div class="mb-6 p-4 rounded-lg bg-green-900/20 border border-green-700/50">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 rounded border-green-600 bg-zinc-700 text-green-500 focus:ring-green-500 focus:ring-offset-zinc-900 cursor-pointer"
                                        prop:checked=move || use_plugins.get()
                                        on:change=move |ev| use_plugins.set(event_target_checked(&ev))
                                    />
                                    <span class="text-white font-medium">"🧩 Usar Sistema de Plugins (Beta)"
                                    </span>
                                </label>
                                <p class="text-zinc-500 text-xs mt-1 ml-7">"Activa la nueva arquitectura modular con 20 plugins de slides"
                                </p>

                                // Plugin Options - only show when enabled
                                <Show when=move || use_plugins.get()>
                                    <div class="mt-4 pt-4 border-t border-green-700/30 space-y-4">
                                        // Industry Template Selector
                                        <div>
                                            <label class="block text-sm font-medium text-zinc-400 mb-2">"📊 Plantilla por Industria"</label>
                                            <select
                                                class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-2 px-3 outline-none text-sm"
                                                on:change=move |ev| {
                                                    let template = event_target_value(&ev);
                                                    // Apply template presets
                                                    match template.as_str() {
                                                        "fintech" => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec!["builtin.geospatial".to_string()]);
                                                        },
                                                        "retail" => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec![]);
                                                        },
                                                        "healthcare" => {
                                                            plugin_theme.set("light".to_string());
                                                            disabled_slides.set(vec!["builtin.evidence".to_string()]);
                                                        },
                                                        _ => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec![]);
                                                        }
                                                    }
                                                }
                                            >
                                                <option value="general">"🎯 General"</option>
                                                <option value="fintech">"🏦 Fintech - Credenciales y fraude"</option>
                                                <option value="retail">"🛒 Retail - Phishing y falsificaciones"</option>
                                                <option value="healthcare">"🏥 Healthcare - Exposición de datos"</option>
                                            </select>
                                            <p class="text-zinc-500 text-xs mt-1">"Selecciona una plantilla para aplicar configuraciones predefinidas"</p>
                                        </div>

                                        // Theme Selector
                                        <div>
                                            <label class="block text-sm font-medium text-zinc-400 mb-2">"🎨 Tema del Reporte"</label>
                                            <select
                                                class="w-full bg-zinc-800 border border-zinc-700 text-white rounded-lg py-2 px-3 outline-none text-sm"
                                                on:change=move |ev| plugin_theme.set(event_target_value(&ev))
                                            >
                                                <option value="dark" selected=move || plugin_theme.get() == "dark">"🌙 Oscuro (Axur.com)"</option>
                                                <option value="light" selected=move || plugin_theme.get() == "light">"☀️ Claro"</option>
                                                <option value="auto" selected=move || plugin_theme.get() == "auto">"🔄 Automático"</option>
                                            </select>
                                        </div>

                                        // Slide Toggles (Collapsible)
                                        <details class="group">
                                            <summary class="cursor-pointer text-zinc-400 text-sm hover:text-white flex items-center gap-2">
                                                <svg class="w-4 h-4 transition-transform group-open:rotate-90" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                </svg>
                                                "Personalizar slides incluidos"
                                            </summary>
                                            <div class="mt-3 ml-6 space-y-2 text-xs">
                                                {slide_toggle("builtin.cover", "Cover & Intro", disabled_slides)}
                                                {slide_toggle("builtin.toc", "Table of Contents", disabled_slides)}
                                                {slide_toggle("builtin.metrics", "General Metrics", disabled_slides)}
                                                {slide_toggle("builtin.threats", "Threats Analysis", disabled_slides)}
                                                {slide_toggle("builtin.takedowns", "Takedowns & ROI", disabled_slides)}
                                                {slide_toggle("builtin.exposure", "Data Exposure", disabled_slides)}
                                                {slide_toggle("builtin.geospatial", "Geospatial Map", disabled_slides)}
                                                {slide_toggle("builtin.evidence", "Evidence & Screenshots", disabled_slides)}
                                            </div>
                                        </details>
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
                                <div class="mb-4 flex gap-4">
                                    <button
                                        class="flex-1 bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
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

                                    // Export to Google Slides button
                                    <button
                                        class="flex-1 bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
                                        disabled=move || exporting_slides.get()
                                        on:click=move |_| {
                                            if exporting_slides.get() { return; }
                                            exporting_slides.set(true);

                                            let html = report_html.get().unwrap_or_default();
                                            let tenant_key = selected_tenant.get();
                                            let tenant_name = tenants.get()
                                                .iter()
                                                .find(|t| t.key == tenant_key)
                                                .map(|t| t.name.clone())
                                                .unwrap_or_else(|| "Report".to_string());

                                            spawn_local(async move {
                                                // Parse HTML to extract slides
                                                let slide_data = parse_html_to_slides(&html);
                                                let title = format!("{} - Threat Report", tenant_name);

                                                match api::export_to_slides(&title, slide_data).await {
                                                    Ok(resp) => {
                                                        leptos::logging::log!("Exported to Google Slides: {}", resp.presentation_url);
                                                        // Unlock achievement
                                                        crate::onboarding::unlock_achievement("first_export");
                                                        // Open presentation in new tab
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
                                    >
                                        <span>{move || if exporting_slides.get() { "⏳" } else { "📊" }}</span>
                                        <span>{move || if exporting_slides.get() { "Exporting..." } else { "Google Slides" }}</span>
                                    </button>

                                    // Export to PPTX button (only if user has templates)
                                    <Show when=move || !user_templates.get().is_empty()>
                                        <button
                                            class="flex-1 bg-purple-600 hover:bg-purple-500 disabled:bg-zinc-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
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

                                                    // For now, show info about what would happen
                                                    // Full implementation requires:
                                                    // 1. Load template from API (get PPTX file + edits)
                                                    // 2. Map placeholder values from current report data
                                                    // 3. Call generate_pptx_report API
                                                    // 4. Download resulting PPTX

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
                                <div class="bg-white rounded-lg overflow-hidden aspect-video">
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
                        </CardWithHeader>
                    </div>
                </div>
            </main>
            <FeedbackWidget />
            <QueueStatusPanel job_id=current_queue_job.into() />
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

/// Parse HTML report to extract slides for Google Slides export
fn parse_html_to_slides(html: &str) -> Vec<api::ExportSlideData> {
    let mut slides = Vec::new();

    // Simple regex-like parsing to extract slide sections
    // Look for section patterns in the HTML report

    // Pattern 1: Look for h1/h2 headers and following content
    let mut current_title = String::new();
    let mut current_body: Vec<String> = Vec::new();

    for line in html.lines() {
        let line = line.trim();

        // Check for slide section markers (common patterns in Axur reports)
        if line.contains("<section") || line.contains("class=\"slide\"") {
            // Save previous slide if exists
            if !current_title.is_empty() || !current_body.is_empty() {
                slides.push(api::ExportSlideData {
                    title: if current_title.is_empty() {
                        format!("Slide {}", slides.len() + 1)
                    } else {
                        current_title.clone()
                    },
                    body: current_body.clone(),
                    layout: Some("TITLE_AND_BODY".to_string()),
                });
            }
            current_title = String::new();
            current_body = Vec::new();
        }

        // Extract h1/h2 as titles
        if line.contains("<h1") || line.contains("<h2") {
            // Simple extraction between > and </h
            if let Some(start) = line.find('>') {
                if let Some(end) = line.find("</h") {
                    let title = &line[start + 1..end];
                    // Strip remaining HTML tags
                    current_title = title
                        .replace("<span>", "")
                        .replace("</span>", "")
                        .replace("<strong>", "")
                        .replace("</strong>", "")
                        .trim()
                        .to_string();
                }
            }
        }

        // Extract paragraphs as body text
        if line.contains("<p") && line.contains("</p>") {
            if let Some(start) = line.find('>') {
                if let Some(end) = line.rfind("</p>") {
                    let text = &line[start + 1..end];
                    // Strip HTML tags and get clean text
                    let clean_text: String = text
                        .replace("<span", "")
                        .replace("</span>", "")
                        .replace("<strong>", "")
                        .replace("</strong>", "")
                        .replace("<em>", "")
                        .replace("</em>", "")
                        .chars()
                        .filter(|c| !c.is_control())
                        .collect::<String>()
                        .trim()
                        .to_string();

                    if !clean_text.is_empty() && clean_text.len() > 5 {
                        current_body.push(clean_text);
                    }
                }
            }
        }
    }

    // Don't forget the last slide
    if !current_title.is_empty() || !current_body.is_empty() {
        slides.push(api::ExportSlideData {
            title: if current_title.is_empty() {
                format!("Slide {}", slides.len() + 1)
            } else {
                current_title
            },
            body: current_body,
            layout: Some("TITLE_AND_BODY".to_string()),
        });
    }

    // If no slides were extracted, create a basic one
    if slides.is_empty() {
        slides.push(api::ExportSlideData {
            title: "Threat Intelligence Report".to_string(),
            body: vec!["Report generated by Axur Web".to_string()],
            layout: Some("TITLE".to_string()),
        });
    }

    slides
}
