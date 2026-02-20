//! Dashboard page with tenant selector and report generation

use crate::api::{self, GenerateReportResponse, TemplateListItem, Tenant};
use crate::components::*;
use crate::utils::{get_default_from_date, get_today};
use crate::{get_ui_dict, AppState};
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

    // Logout action handled in Header component

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
            <Header />

            // Progress Checklist (onboarding achievements)
            <crate::components::ProgressChecklist />

            // Tutorial Orchestrator (shows welcome modal for new users)
            <crate::components::TutorialOrchestrator />

            // Hint Manager (shows contextual hints after idle)
            <crate::components::HintManager />

            // Main content
            <main class="max-w-7xl mx-auto p-6 relative z-10">
                <HeroSection />

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
                    // NEW: Extracted Dynamic Form Component
                    <div class="lg:col-span-1">
                        <ReportGeneratorForm
                            ui_dict=dict
                            loading_tenants=loading_tenants.read_only()
                            tenant_options=tenant_options
                            selected_tenant=selected_tenant
                            from_date=from_date
                            to_date=to_date
                            story_tag=story_tag
                            language=language
                            user_templates=user_templates.read_only()
                            selected_template=selected_template
                            include_threat_intel=include_threat_intel
                            debug_mode=debug_mode
                            use_plugins=use_plugins
                            plugin_theme=plugin_theme
                            disabled_slides=disabled_slides
                            use_mock_data=use_mock_data
                            generating=generating.read_only()
                            generate_action=generate_action
                        />
                    </div>

                // Preview Column
                <div class="lg:col-span-2">
                    <LivePreviewPanel
                        report_html=report_html
                        selected_tenant=selected_tenant
                        tenants=tenants
                        user_templates=user_templates
                        selected_pptx_template=selected_pptx_template
                        from_date=from_date
                        to_date=to_date
                        selected_template=selected_template
                        exporting_slides=exporting_slides
                        exporting_pptx=exporting_pptx
                        state=state.clone()
                    />
                </div>
            </div>
            </main>
            <FeedbackWidget />
            <QueueStatusPanel job_id=current_queue_job.into() />
        </div>
    }
}
