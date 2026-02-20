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

    #[allow(dead_code)]
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
    let use_mock_data = create_rw_signal(false); // Mock report mode
    let debug_mode = create_rw_signal(false); // Admin/Debug mode

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
    let state_for_action = state.clone();
    let generate_action = create_action(move |_: &()| {
        let state = state_for_action.clone();
        async move {
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
                        let sp = streaming_progress;
                        let pd = preview_data;
                        let pl = preview_loading;
                        let _spm = show_preview_modal;
                        let _err = error;
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
                        })
                            as Box<dyn FnMut(_)>);

                        event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                        onmessage.forget(); // Prevent closure from being dropped

                        // Error handler
                        let sp_err = streaming_progress;
                        let pl_err = preview_loading;
                        let es_err = event_source.clone();
                        let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
                            sp_err.update(|s| {
                                s.error_message = Some("Connection error".into());
                                s.is_finished = true;
                                s.is_streaming = false;
                            });
                            pl_err.set(false);
                            es_err.close();
                        })
                            as Box<dyn FnMut(_)>);

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
                use_mock_data.get(),
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
                    let report = report_html;
                    let err = error;
                    let gen = generating;
                    let ti = threat_intel;
                    let es_clone = event_source.clone();
                    let app_state = state.clone();

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
                                            crate::onboarding::unlock_achievement(
                                                "threat_hunting",
                                                &app_state,
                                            );
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
                    let err_clone = error;
                    let gen_clone = generating;
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

        <div class="min-h-screen bg-cyber-grid bg-fixed">
            // Header
            <header class="glass-panel sticky top-0 z-50 border-b border-white/5 px-6 py-4 backdrop-blur-xl">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    <div class="flex items-center gap-3 group cursor-default">
                        <div class="relative">
                            <span class="text-brand-primary text-2xl font-black italic relative z-10 group-hover:animate-pulse">"///"</span>
                            <div class="absolute inset-0 bg-brand-primary/20 blur-lg rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                        </div>
                        <span class="text-white text-xl font-bold tracking-[0.2em] font-display">"AXUR"</span>
                        <span class="text-zinc-500 text-xs font-mono uppercase tracking-widest border border-zinc-800 px-2 py-0.5 rounded bg-surface-base">"Web"</span>
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
            <main class="max-w-7xl mx-auto p-6 relative z-10">
                // Cinematic Hero Section
                <div class="relative mb-12 py-10 overflow-hidden rounded-3xl border border-white/5 bg-surface-base/50 backdrop-blur-sm group">
                    <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                    <div class="absolute -top-24 -right-24 w-96 h-96 bg-brand-primary/20 blur-[100px] rounded-full animate-pulse-slow"></div>
                    <div class="absolute -bottom-24 -left-24 w-64 h-64 bg-brand-secondary/10 blur-[80px] rounded-full"></div>

                    <div class="relative z-10 px-8 flex flex-col items-start gap-4">
                        <div class="flex items-center gap-3">
                            <span class="px-3 py-1 bg-brand-primary/10 border border-brand-primary/30 text-brand-primary text-xs font-bold font-mono tracking-widest uppercase rounded">
                                "System Ready"
                            </span>
                            <div class="h-px w-12 bg-gradient-to-r from-brand-primary/50 to-transparent"></div>
                        </div>

                        <h1 class="text-6xl md:text-7xl font-display font-bold text-transparent bg-clip-text bg-gradient-to-r from-white via-white to-zinc-500 uppercase tracking-tighter drop-shadow-lg">
                            "Threat" <br />
                            <span class="text-stroke-current text-transparent bg-clip-text bg-gradient-to-r from-brand-primary to-brand-secondary selection:text-white">"Intelligence"</span>
                        </h1>

                        <p class="max-w-2xl text-zinc-400 text-lg font-light border-l-2 border-brand-primary/50 pl-6 mt-4">
                            "Advanced cyber-surveillance telemetry. Generate exec-ready reports in seconds."
                        </p>
                    </div>

                    // Decorative scanline
                    <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-brand-primary/5 to-transparent h-[20%] w-full animate-scanline opacity-30"></div>
                </div>

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
                    <div class="lg:col-span-1 space-y-6">
                        <div class="glass-panel rounded-2xl p-1 overflow-hidden group hover:border-brand-primary/30 transition-all duration-500">
                            <div class="bg-surface-base/80 p-6 rounded-xl space-y-6 backdrop-blur-md">
                                <div class="flex items-center gap-3 mb-6 pb-4 border-b border-white/5">
                                    <div class="w-8 h-8 rounded-lg bg-brand-primary/10 flex items-center justify-center text-brand-primary">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                                    </div>
                                    <h2 class="text-sm font-bold uppercase tracking-widest text-zinc-400 font-mono">
                                        {move || dict.get().configuration}
                                    </h2>
                                </div>

                                <Show
                                    when=move || !loading_tenants.get()
                                    fallback=|| view! {
                                        <div class="space-y-4">
                                            <div class="h-4 w-20 bg-white/5 rounded animate-pulse"></div>
                                            <div class="w-full h-12 bg-white/5 rounded-lg animate-pulse"></div>
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

                                <div class="space-y-2">
                                    <label class="text-xs font-bold text-zinc-500 uppercase tracking-wider">{move || dict.get().from_date}</label>
                                    <input
                                        type="date"
                                        class="w-full bg-surface-elevated/50 border border-white/10 text-white rounded-lg py-3 px-4 outline-none focus:border-brand-primary/50 focus:shadow-glow-sm transition-all duration-300 placeholder-zinc-600"
                                        prop:value=move || from_date.get()
                                        on:input=move |ev| from_date.set(event_target_value(&ev))
                                    />
                                </div>

                                <div class="space-y-2">
                                    <label class="text-xs font-bold text-zinc-500 uppercase tracking-wider">{move || dict.get().to_date}</label>
                                    <input
                                        type="date"
                                        class="w-full bg-surface-elevated/50 border border-white/10 text-white rounded-lg py-3 px-4 outline-none focus:border-brand-primary/50 focus:shadow-glow-sm transition-all duration-300 placeholder-zinc-600"
                                        prop:value=move || to_date.get()
                                        on:input=move |ev| to_date.set(event_target_value(&ev))
                                    />
                                </div>

                                <div class="space-y-2">
                                    <label class="text-xs font-bold text-zinc-500 uppercase tracking-wider">"Story Tag (Optional)"</label>
                                    <div class="relative group">
                                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-zinc-500 group-focus-within:text-brand-primary transition-colors">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path></svg>
                                        </div>
                                        <input
                                            type="text"
                                            class="w-full bg-surface-elevated/50 border border-white/10 text-white rounded-lg py-3 pl-10 pr-4 outline-none focus:border-brand-primary/50 focus:shadow-glow-sm transition-all duration-300 placeholder-zinc-600 font-mono text-sm"
                                            placeholder="tag:campaign-name"
                                            prop:value=move || story_tag.get()
                                            on:input=move |ev| story_tag.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>

                                <div class="space-y-2">
                                    <label class="text-xs font-bold text-zinc-500 uppercase tracking-wider">{move || dict.get().language_label}</label>
                                    <div class="relative">
                                        <select
                                            class="w-full bg-surface-elevated/50 border border-white/10 text-white rounded-lg py-3 px-4 outline-none cursor-pointer focus:border-brand-primary/50 focus:shadow-glow-sm transition-all appearance-none"
                                            on:change=move |ev| language.set(event_target_value(&ev))
                                        >
                                            <option value="es" selected>"Español"</option>
                                            <option value="en">"English"</option>
                                            <option value="pt">"Português"</option>
                                        </select>
                                        <div class="absolute inset-y-0 right-0 flex items-center px-4 pointer-events-none text-zinc-500">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                                        </div>
                                    </div>
                                </div>

                            // Template selection
                            <div class="space-y-2">
                                <label class="text-xs font-bold text-zinc-500 uppercase tracking-wider">"Report Template"</label>
                                <div class="flex gap-2">
                                    <div class="relative flex-1">
                                        <select
                                            class="w-full bg-surface-elevated/50 border border-white/10 text-white rounded-lg py-3 px-4 outline-none focus:border-brand-primary/50 focus:shadow-glow-sm transition-all appearance-none"
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
                                        <div class="absolute inset-y-0 right-0 flex items-center px-4 pointer-events-none text-zinc-500">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                                        </div>
                                    </div>
                                    <button
                                        on:click=move |_| state.current_page.set(crate::Page::Marketplace)
                                        class="px-4 py-2 bg-surface-elevated hover:bg-white/10 text-zinc-400 hover:text-white border border-white/10 hover:border-brand-primary/50 rounded-lg transition-all"
                                        title="Open Template Marketplace"
                                    >
                                        "🛒"
                                    </button>
                                </div>
                                <Show when=move || selected_template.get().is_some()>
                                    <p class="text-xs text-brand-primary mt-1 font-mono">
                                        "Using custom template: " {move || selected_template.get().unwrap_or_default()}
                                    </p>
                                </Show>
                            </div>

                            // Threat Hunting Toggle
                            <ThreatHuntingToggle
                                include_threat_intel=include_threat_intel
                                debug_mode=debug_mode
                            />

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

                            // Mock Data Toggle (Dev)
                            <div class="mb-8 p-4 rounded-xl bg-indigo-900/10 border border-indigo-500/20 backdrop-blur-sm">
                                <label class="flex items-center gap-3 cursor-pointer">
                                    <div class="relative">
                                        <input
                                            type="checkbox"
                                            class="peer sr-only"
                                            prop:checked=move || use_mock_data.get()
                                            on:change=move |ev| use_mock_data.set(event_target_checked(&ev))
                                        />
                                        <div class="w-9 h-5 bg-zinc-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-indigo-500"></div>
                                    </div>
                                    <span class="text-indigo-200 font-bold text-xs uppercase tracking-wider">"Mock Mode"</span>
                                </label>
                            </div>

                            <button
                                class="w-full bg-gradient-to-r from-brand-primary to-brand-primary-hover hover:to-orange-500 text-white font-bold py-4 px-6 rounded-xl shadow-glow hover:shadow-glow-orange transition-all duration-300 flex items-center justify-center gap-3 group relative overflow-hidden"
                                disabled=move || generating.get()
                                on:click=move |_| generate_action.dispatch(())
                            >
                                <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300"></div>
                                {move || if generating.get() {
                                    view! {
                                        <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                    }.into_view()
                                } else {
                                    view! { <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg> }.into_view()
                                }}
                                <span class="relative z-10">{move || dict.get().generate_btn}</span>
                            </button>
                        </div>
                    </div>
                </div>

                </div> // Close Col 1
                // Preview Column
                <div class="lg:col-span-2">
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
                                    fallback=|| view! {
                                        <div class="flex-1 flex flex-col items-center justify-center text-zinc-600 space-y-6 animate-pulse-slow">
                                            <div class="w-24 h-24 rounded-2xl bg-surface-elevated/50 flex items-center justify-center border border-white/5 shadow-inner">
                                                <svg class="w-10 h-10 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                                </svg>
                                            </div>
                                            <p class="font-mono text-sm uppercase tracking-widest text-zinc-500">"Waiting for Input..."</p>
                                        </div>
                                    }
                                >
                                    <div class="mt-auto pt-6 border-t border-white/5 grid grid-cols-2 gap-4">
                                        <button
                                            class="bg-surface-elevated hover:bg-brand-primary text-zinc-400 hover:text-white font-bold py-3 px-6 rounded-lg transition-all duration-300 flex items-center justify-center gap-2 group"
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
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                                            "Download HTML"
                                        </button>

                                        <button
                                            class="bg-blue-600/20 hover:bg-blue-600 border border-blue-500/30 text-blue-400 hover:text-white font-bold py-3 px-6 rounded-lg transition-all duration-300 flex items-center justify-center gap-2 group disabled:opacity-50 disabled:cursor-not-allowed"
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
                        </div>
                    </div>
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
