//! Slide Editor Page
//!
//! Canvas-based editor for creating and editing presentation templates

use crate::api;
use crate::storage;
use leptos::*;
use wasm_bindgen::prelude::*;

/// JavaScript interop for Fabric.js canvas
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window, js_name = initFabricCanvas)]
    fn init_fabric_canvas(canvas_id: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = window, js_name = addTextToCanvas)]
    fn add_text_to_canvas(text: &str);

    #[wasm_bindgen(js_namespace = window, js_name = addPlaceholderToCanvas)]
    fn add_placeholder_to_canvas(key: &str, html: &str);

    #[wasm_bindgen(js_namespace = window, js_name = addShapeToCanvas)]
    fn add_shape_to_canvas(shape_type: &str);

    #[wasm_bindgen(js_namespace = window, js_name = getCanvasJSON)]
    fn get_canvas_json() -> String;

    #[wasm_bindgen(js_namespace = window, js_name = loadCanvasJSON)]
    fn load_canvas_json(json: &str);

    #[wasm_bindgen(js_namespace = window, js_name = clearCanvas)]
    fn clear_canvas();

    #[wasm_bindgen(js_namespace = window, js_name = addImageToCanvas)]
    fn add_image_to_canvas(data_url: &str);

    #[wasm_bindgen(js_namespace = window, js_name = getCanvasThumbnail)]
    fn get_canvas_thumbnail() -> String;

    #[wasm_bindgen(js_namespace = window, js_name = triggerImageUpload)]
    fn trigger_image_upload();

    #[wasm_bindgen(js_namespace = window, js_name = undoCanvas)]
    fn undo_canvas() -> bool;

    #[wasm_bindgen(js_namespace = window, js_name = redoCanvas)]
    fn redo_canvas() -> bool;

    #[wasm_bindgen(js_namespace = window, js_name = canUndo)]
    fn can_undo() -> bool;

    #[wasm_bindgen(js_namespace = window, js_name = canRedo)]
    fn can_redo() -> bool;

    #[wasm_bindgen(js_namespace = window, js_name = duplicateObject)]
    fn duplicate_object();

    #[wasm_bindgen(js_namespace = window, js_name = showCsat)]
    fn show_csat();

    // Zoom controls
    #[wasm_bindgen(js_namespace = window, js_name = zoomIn)]
    fn zoom_in() -> f64;

    #[wasm_bindgen(js_namespace = window, js_name = zoomOut)]
    fn zoom_out() -> f64;

    #[wasm_bindgen(js_namespace = window, js_name = zoomReset)]
    fn zoom_reset() -> f64;

    #[wasm_bindgen(js_namespace = window, js_name = getZoom)]
    fn get_zoom() -> f64;

    // Alignment
    #[wasm_bindgen(js_namespace = window, js_name = alignSelected)]
    fn align_selected(direction: &str);

    // Layer controls
    #[wasm_bindgen(js_namespace = window, js_name = bringToFront)]
    fn bring_to_front();

    #[wasm_bindgen(js_namespace = window, js_name = sendToBack)]
    fn send_to_back();

    // Delete
    #[wasm_bindgen(js_namespace = window, js_name = deleteSelected)]
    fn delete_selected();

    #[wasm_bindgen(js_namespace = window, js_name = duplicateSelected)]
    fn duplicate_selected();

    // Preview mode
    #[wasm_bindgen(js_namespace = window, js_name = togglePreviewMode)]
    fn toggle_preview_mode(enabled: bool) -> bool;

    #[wasm_bindgen(js_namespace = window, js_name = getPreviewMode)]
    fn get_preview_mode() -> bool;

    // Smart templates analysis (accepts array of slide JSON strings)
    #[wasm_bindgen(js_namespace = window, js_name = analyzeTemplateData)]
    fn analyze_template_data(slides_json: JsValue) -> JsValue;
}

/// Slide in the editor
#[derive(Clone, Debug)]
pub struct EditorSlide {
    pub id: String,
    pub name: String,
    pub canvas_json: String,
}

/// Convert sandbox slide to Fabric.js canvas JSON
fn sandbox_slides_to_canvas_json(slide: &crate::onboarding::SandboxSlide) -> String {
    let title = &slide.name;
    let json = serde_json::json!({ "version": "5.3.0", "objects": [{ "type": "text", "left": 100, "top": 100, "text": title, "fontSize": 32, "fill": "white" }] });
    json.to_string()
}

/// Main Editor Page Component
#[component]
pub fn EditorPage() -> impl IntoView {
    let state = expect_context::<crate::AppState>();

    // Editor state
    let slides = create_rw_signal(vec![EditorSlide {
        id: "slide-1".to_string(),
        name: "Slide 1".to_string(),
        canvas_json: "{}".to_string(),
    }]);
    let current_slide_idx = create_rw_signal(0usize);
    let show_placeholder_library = create_rw_signal(false);
    let template_name = create_rw_signal("Untitled Template".to_string());
    let is_saving = create_rw_signal(false);
    let is_exporting_slides = create_rw_signal(false); // Google Slides export
    let preview_mode = create_rw_signal(false); // Real-time preview toggle

    // Sandbox mode - demo environment for practice
    let is_sandbox = create_rw_signal(crate::onboarding::is_sandbox_mode());

    // New state for CRUD
    let template_id = create_rw_signal::<Option<String>>(None);
    let has_unsaved_changes = create_rw_signal(false);
    let show_template_list = create_rw_signal(false);
    let available_templates = create_rw_signal::<Vec<api::TemplateListItem>>(vec![]);
    let is_loading = create_rw_signal(false);
    let save_message = create_rw_signal::<Option<String>>(None);

    // Suppress unused warnings for future features
    let _ = (show_template_list, available_templates, save_message);

    // Check if we should load a template from Marketplace (Clone or Edit)
    let clone_id = state.editor_clone_from_id.get();
    let edit_id = state.editor_template_id.get();

    // Prioritize clone
    let (target_id, is_clone) = if let Some(cid) = clone_id {
        (Some(cid), true)
    } else if let Some(eid) = edit_id {
        (Some(eid), false)
    } else {
        (None, false)
    };

    let target_id_clone = target_id.clone();

    // Initialize canvas and load template on mount
    create_effect(move |prev_ran: Option<bool>| {
        // Only run once
        if prev_ran.unwrap_or(false) {
            return true;
        }

        let tid_opt = target_id_clone.clone();
        set_timeout(
            move || {
                init_fabric_canvas("editor-canvas");
                // Enable drag-drop for placeholders onto canvas
                let _ =
                    js_sys::eval("if(window.setupCanvasDragDrop) window.setupCanvasDragDrop();");

                // Check if we need to load a template
                // SANDBOX MODE: Load demo slides instead of user templates
                if is_sandbox.get() {
                    let demo_slides = crate::onboarding::get_sandbox_slides();
                    let new_slides: Vec<EditorSlide> = demo_slides
                        .iter()
                        .enumerate()
                        .map(|(i, s)| EditorSlide {
                            id: s.id.to_string(),
                            name: s.name.to_string(),
                            canvas_json: sandbox_slides_to_canvas_json(s),
                        })
                        .collect();

                    if !new_slides.is_empty() {
                        slides.set(new_slides);
                        current_slide_idx.set(0);
                        template_name.set("Demo Template (Sandbox)".to_string());
                        if let Some(first) = slides.get().first() {
                            load_canvas_json(&first.canvas_json);
                        }
                    }
                    log("Sandbox mode: Loaded demo slides");
                    return; // Don't load user templates
                }

                if let Some(tid) = tid_opt {
                    is_loading.set(true);
                    spawn_local(async move {
                        match api::get_template(&tid).await {
                            Ok(resp) => {
                                if let Some(tmpl) = resp.template {
                                    if !is_clone {
                                        template_id.set(Some(tmpl.id.clone()));
                                    } else {
                                        // Clone mode: User starts with template content but no ID (new save)
                                        // Clear the clone signal to avoid re-triggering? (not needed if run once)
                                    }
                                    template_name.set(tmpl.name.clone());

                                    // Load slides from template
                                    let new_slides: Vec<EditorSlide> = tmpl
                                        .slides
                                        .iter()
                                        .enumerate()
                                        .map(|(i, s)| EditorSlide {
                                            id: s
                                                .get("id")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(&format!("slide-{}", i + 1))
                                                .to_string(),
                                            name: s
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(&format!("Slide {}", i + 1))
                                                .to_string(),
                                            canvas_json: s
                                                .get("canvas_json")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("{}")
                                                .to_string(),
                                        })
                                        .collect();

                                    if !new_slides.is_empty() {
                                        slides.set(new_slides);
                                        current_slide_idx.set(0);
                                        // Load first slide into canvas
                                        if let Some(first) = slides.get().first() {
                                            load_canvas_json(&first.canvas_json);
                                        }
                                    }
                                    log(&format!("Loaded template: {}", tmpl.name));
                                }
                            }
                            Err(e) => log(&format!("Failed to load template: {}", e)),
                        }
                        is_loading.set(false);
                    });
                    // Clear the template ID from app state
                    state.editor_template_id.set(None);
                }
            },
            std::time::Duration::from_millis(100),
        );
        true
    });

    // Navigation back to dashboard (with unsaved changes check)
    let go_back = move |_| {
        if has_unsaved_changes.get() {
            // TODO: Show confirmation modal
            // For now, just navigate
            state.current_page.set(crate::Page::Dashboard);
        } else {
            state.current_page.set(crate::Page::Dashboard);
        }
    };

    // Add new slide
    let add_slide = move |_| {
        slides.update(|s| {
            let num = s.len() + 1;
            s.push(EditorSlide {
                id: format!("slide-{}", num),
                name: format!("Slide {}", num),
                canvas_json: "{}".to_string(),
            });
        });
        current_slide_idx.set(slides.get().len() - 1);
        clear_canvas();
    };

    // Select slide
    let select_slide = move |idx: usize| {
        // Save current slide first
        slides.update(|s| {
            if let Some(slide) = s.get_mut(current_slide_idx.get()) {
                slide.canvas_json = get_canvas_json();
            }
        });
        // Load new slide
        current_slide_idx.set(idx);
        if let Some(slide) = slides.get().get(idx) {
            load_canvas_json(&slide.canvas_json);
        }
    };

    // Delete current slide
    let delete_slide = move |_| {
        if slides.get().len() > 1 {
            let idx = current_slide_idx.get();
            slides.update(|s| {
                s.remove(idx);
            });
            if idx > 0 {
                current_slide_idx.set(idx - 1);
            }
            if let Some(slide) = slides.get().get(current_slide_idx.get()) {
                load_canvas_json(&slide.canvas_json);
            }
            has_unsaved_changes.set(true);
        }
    };

    // Duplicate current slide
    let duplicate_slide = move |_: web_sys::MouseEvent| {
        let idx = current_slide_idx.get();
        let current_slides = slides.get();
        if let Some(current) = current_slides.get(idx) {
            // Save current canvas state
            let canvas_json = get_canvas_json();
            let current_name = current.name.clone();
            slides.update(|s| {
                let num = s.len() + 1;
                s.insert(
                    idx + 1,
                    EditorSlide {
                        id: format!("slide-{}", num),
                        name: format!("{} (copy)", current_name),
                        canvas_json,
                    },
                );
            });
            current_slide_idx.set(idx + 1);
            has_unsaved_changes.set(true);
        }
    };

    // Undo action
    let do_undo = move |_: web_sys::MouseEvent| {
        undo_canvas();
        has_unsaved_changes.set(true);
    };

    // Redo action
    let do_redo = move |_: web_sys::MouseEvent| {
        redo_canvas();
        has_unsaved_changes.set(true);
    };

    // Duplicate object action
    let do_duplicate_object = move |_: web_sys::MouseEvent| {
        duplicate_object();
        has_unsaved_changes.set(true);
    };

    // Add text element
    let add_text = move |_| {
        add_text_to_canvas("Edit this text");
    };

    // Add shape
    let add_rectangle = move |_| {
        add_shape_to_canvas("rectangle");
    };

    let add_circle = move |_| {
        add_shape_to_canvas("circle");
    };

    // Toggle placeholder library
    let toggle_library = move |_| {
        show_placeholder_library.update(|v| *v = !*v);
    };

    // Insert placeholder from library
    let insert_placeholder = move |key: String, html: String| {
        add_placeholder_to_canvas(&key, &html);
        show_placeholder_library.set(false);
    };

    // Save template
    let save_template = move |_| {
        is_saving.set(true);
        // Save current slide
        slides.update(|s| {
            if let Some(slide) = s.get_mut(current_slide_idx.get()) {
                slide.canvas_json = get_canvas_json();
            }
        });

        // Auto-save version to local storage (if we have a template_id)
        if let Some(tid) = template_id.get() {
            let slides_json: Vec<String> =
                slides.get().iter().map(|s| s.canvas_json.clone()).collect();
            storage::save_template_version(&tid, slides_json);
            log(&format!("Version saved locally for template {}", tid));
        }

        let name = template_name.get();
        let slides_data: Vec<_> = slides
            .get()
            .iter()
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "canvas_json": s.canvas_json
                })
            })
            .collect();

        // Get thumbnail from canvas
        let thumbnail = get_canvas_thumbnail();
        let thumbnail_opt = if thumbnail.is_empty() {
            None
        } else {
            Some(thumbnail)
        };

        // Get original file from global PPTXImporter state
        let original_file = web_sys::window()
            .and_then(|w| js_sys::Reflect::get(&w, &"PPTXImporter".into()).ok())
            .and_then(|imp| js_sys::Reflect::get(&imp, &"state".into()).ok())
            .and_then(|st| js_sys::Reflect::get(&st, &"originalFile".into()).ok())
            .and_then(|f| {
                use wasm_bindgen::JsCast;
                f.dyn_into::<web_sys::File>().ok()
            });

        // Get existing template ID for update (PUT) vs create (POST)
        let existing_id = template_id.get();
        let is_new = existing_id.is_none();

        spawn_local(async move {
            match api::save_template(
                existing_id.as_deref(),
                &name,
                None, // No description for now
                slides_data,
                thumbnail_opt,
                original_file,
            )
            .await
            {
                Ok(resp) => {
                    if resp.success {
                        log(&format!("Template saved! ID: {:?}", resp.template_id));
                        // Update template_id if this was a new template
                        if is_new {
                            if let Some(new_id) = resp.template_id {
                                template_id.set(Some(new_id));
                            }
                        }
                        has_unsaved_changes.set(false);
                        // Trigger CSAT survey
                        show_csat();
                    } else {
                        log(&format!("Save failed: {}", resp.message));
                    }
                }
                Err(e) => {
                    log(&format!("Save error: {}", e));
                }
            }
            is_saving.set(false);
        });
    };

    view! {
        <div class="min-h-screen bg-zinc-950 flex flex-col">
            // Sandbox Mode Banner
            <crate::components::SandboxBanner
                is_active=Signal::derive(move || is_sandbox.get())
                on_exit=Callback::new(move |_| {
                    crate::onboarding::set_sandbox_mode(false);
                    state.current_page.set(crate::Page::Dashboard);
                })
            />
            
            // Top Bar
            <header class="h-14 bg-zinc-900 border-b border-zinc-800 flex items-center px-4 gap-4">
                <button
                    on:click=go_back
                    class="text-zinc-400 hover:text-white flex items-center gap-2"
                >
                    <span>"← Back"</span>
                </button>
                <div class="h-6 w-px bg-zinc-700"></div>
                <input
                    type="text"
                    class="bg-transparent border-b border-transparent hover:border-zinc-600 focus:border-indigo-500 outline-none text-white font-medium px-1"
                    prop:value=move || template_name.get()
                    on:input=move |ev| template_name.set(event_target_value(&ev))
                />
                // Preview mode toggle
                <div class="h-6 w-px bg-zinc-700"></div>
                <button
                    on:click=move |_| {
                        let new_state = !preview_mode.get();
                        preview_mode.set(new_state);
                        toggle_preview_mode(new_state);
                    }
                    class="flex items-center gap-1 px-2 py-1 rounded text-xs transition-colors"
                    class:bg-emerald-900=move || preview_mode.get()
                    class:text-emerald-400=move || preview_mode.get()
                    class:bg-zinc-800=move || !preview_mode.get()
                    class:text-zinc-400=move || !preview_mode.get()
                    title="Toggle between placeholder text and preview values"
                >
                    <span>{move || if preview_mode.get() { "👁 Preview ON" } else { "👁 Preview" }}</span>
                </button>
                <div class="flex-1"></div>
                // Import PPTX button with hidden file input
                <div class="flex items-center gap-2">
                    // Use pure HTML onchange to avoid WASM closure issues
                    <input
                        type="file"
                        id="pptx-input"
                        accept=".pptx"
                        class="hidden"
                        onchange="window._pendingPptxFile = this.files[0]; setTimeout(function() { window.handlePptxImportFromGlobal(); }, 0);"
                    />
                    <button
                        class="px-4 py-2 bg-zinc-700 hover:bg-zinc-600 text-white rounded-lg font-medium flex items-center gap-2"
                        onclick="document.getElementById('pptx-input').click();"
                        title="Sube tu presentación para agregar placeholders"
                    >
                        <span>"📤"</span>
                        <span>"Importar mi PPTX"</span>
                    </button>
                    // Export PPTX (Hybrid Native)
                    <button
                        class="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium flex items-center gap-2"
                        onclick="if(window.PPTXImporter) window.PPTXImporter.exportPPTX(); else alert('PPTX Engine not ready');"
                        title="Descarga el template con placeholders (sin datos) para usar después"
                    >
                        <span>"💾"</span>
                        <span>"Descargar Template"</span>
                    </button>
                    // Export to Google Slides
                    <button
                        on:click=move |_| {
                            if is_exporting_slides.get() { return; }
                            is_exporting_slides.set(true);
                            let title = template_name.get();

                            // Create slide data from template slides
                            let slide_data: Vec<api::ExportSlideData> = slides.get()
                                .iter()
                                .map(|s| api::ExportSlideData {
                                    title: s.name.clone(),
                                    body: vec![format!("Slide content from template: {}", s.name)],
                                    layout: Some("BLANK".to_string()),
                                })
                                .collect();

                            spawn_local(async move {
                                match api::export_to_slides(&title, slide_data).await {
                                    Ok(resp) => {
                                        log(&format!("Exported to Google Slides: {}", resp.presentation_url));
                                        // Trigger CSAT survey
                                        show_csat();
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
                                        log(&format!("Export failed: {}", e));
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.alert_with_message(&format!("Export failed: {}", e));
                                        }
                                    }
                                }
                                is_exporting_slides.set(false);
                            });
                        }
                        disabled=move || is_exporting_slides.get()
                        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium flex items-center gap-2 disabled:opacity-50"
                        title="Export to Google Slides"
                    >
                        <span>{move || if is_exporting_slides.get() { "⏳" } else { "📊" }}</span>
                        <span>{move || if is_exporting_slides.get() { "Exporting..." } else { "Google Slides" }}</span>
                    </button>
                </div>
                <button
                    on:click=save_template
                    disabled=move || is_saving.get() || is_sandbox.get()
                    class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                    title=move || if is_sandbox.get() { "Guardar deshabilitado en modo práctica" } else { "Guarda el template en tu perfil para usar en Dashboard" }
                >
                    {move || if is_sandbox.get() { "🔒 Modo Práctica" } else if is_saving.get() { "Guardando..." } else { "☁️ Guardar en Perfil" }}
                </button>
            </header>

            <div class="flex-1 flex overflow-hidden">
                // Left Sidebar - Slides Panel (wider for bigger thumbnails)
                <aside class="w-56 bg-zinc-900 border-r border-zinc-800 flex flex-col">
                    <div class="p-3 border-b border-zinc-800 flex items-center justify-between">
                        <span class="text-sm text-zinc-400 font-medium">"Slides"</span>
                        <button
                            on:click=add_slide
                            class="w-6 h-6 flex items-center justify-center bg-zinc-800 hover:bg-zinc-700 rounded text-white"
                            title="Add Slide"
                        >
                            "+"
                        </button>
                    </div>
                    <div class="flex-1 overflow-y-auto p-2 space-y-2">
                        <For
                            each=move || slides.get().into_iter().enumerate()
                            key=|(_, s)| s.id.clone()
                            children=move |(idx, slide)| {
                                let is_current = move || current_slide_idx.get() == idx;
                                view! {
                                    <div
                                        class=move || format!(
                                            "relative aspect-video rounded cursor-pointer border-2 transition {} {}",
                                            if is_current() { "border-indigo-500" } else { "border-zinc-700 hover:border-zinc-600" },
                                            "bg-zinc-800"
                                        )
                                        on:click=move |_| select_slide(idx)
                                    >
                                        <div class="absolute inset-0 flex items-center justify-center text-zinc-500 text-xs">
                                            {slide.name.clone()}
                                        </div>
                                        <div class="absolute bottom-1 right-1 text-zinc-600 text-xs">
                                            {idx + 1}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                    // Imported PPTX Slides Section
                    <div
                        id="pptx-slides-panel"
                        class="border-t border-zinc-800 p-2"
                        style="display: none;"
                    >
                        <div class="text-xs text-orange-400 font-medium mb-2 flex items-center justify-between">
                            <span class="flex items-center gap-1">"📤 Imported PPTX"</span>
                            <div class="flex gap-1">
                                // Duplicate current PPTX slide
                                <button
                                    on:click=move |_| {
                                        let _ = js_sys::eval("window.duplicateSlide();");
                                    }
                                    class="p-1 hover:bg-zinc-700 rounded text-zinc-400 hover:text-white"
                                    title="Duplicate current slide"
                                >"📋"</button>
                                // Delete current PPTX slide
                                <button
                                    on:click=move |_| {
                                        let _ = js_sys::eval("window.deleteSlide();");
                                    }
                                    class="p-1 hover:bg-red-900/30 rounded text-zinc-400 hover:text-red-400"
                                    title="Delete current slide"
                                >"🗑"</button>
                            </div>
                        </div>
                        <div id="pptx-thumbnails" class="space-y-1">
                            // Thumbnails will be inserted here by JavaScript
                        </div>
                        <div class="mt-2 text-xs text-zinc-600 text-center">
                            "Drag to reorder"
                        </div>
                        // Detect Placeholders button
                        <button
                            on:click=move |_| {
                                let _ = js_sys::eval(r#"
                                    const placeholders = window.detectPlaceholders();
                                    if (placeholders.length > 0) {
                                        alert('Found ' + placeholders.length + ' placeholders:\\n' + 
                                            placeholders.map(p => p.fullMatch + ' (slide ' + (p.slideIndex+1) + ')').join('\\n'));
                                    } else {
                                        const suggestions = window.suggestPlaceholders();
                                        if (suggestions.length > 0) {
                                            alert('No placeholders found. Suggestions:\\n' +
                                                suggestions.map(s => s.suggestedPlaceholder).join('\\n'));
                                        } else {
                                            alert('No placeholders or suggestions found.');
                                        }
                                    }
                                "#);
                            }
                            class="w-full mt-2 py-1 text-xs bg-zinc-800 hover:bg-zinc-700 rounded text-orange-400"
                            title="Scan slides for {{placeholders}}"
                        >
                            "🔍 Detect Placeholders"
                        </button>
                    </div>
                    <div class="p-2 border-t border-zinc-800">
                        <button
                            on:click=delete_slide
                            disabled=move || slides.get().len() <= 1
                            class="w-full py-1.5 text-sm text-red-400 hover:text-red-300 disabled:opacity-30 disabled:cursor-not-allowed"
                        >
                            "Delete Slide"
                        </button>
                    </div>
                </aside>

                // Main Canvas Area
                <main class="flex-1 flex flex-col bg-zinc-950">
                    // Toolbar - Grouped by function
                    <div class="h-14 bg-zinc-900 border-b border-zinc-800 flex items-center px-4 gap-3">
                        // GROUP 1: Add Elements
                        <div class="flex items-center gap-1 bg-zinc-800/50 rounded-lg px-2 py-1">
                            <span class="text-xs text-zinc-500 mr-1">"Add:"</span>
                            <button
                                on:click=add_text
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Add Text (T)"
                            >
                                "T"
                            </button>
                            <button
                                on:click=add_rectangle
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Add Rectangle"
                            >
                                "□"
                            </button>
                            <button
                                on:click=add_circle
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Add Circle"
                            >
                                "○"
                            </button>
                            <button
                                on:click=move |_| trigger_image_upload()
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Upload Image"
                            >
                                "🖼️"
                            </button>
                        </div>

                        // GROUP 2: Placeholders (highlighted)
                        <button
                            on:click=toggle_library
                            class="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-700 rounded-lg text-sm text-white font-medium shadow-sm"
                            title="Marcadores que se reemplazan con datos reales del reporte"
                        >
                            "📦 Placeholders"
                        </button>

                        // Separator
                        <div class="w-px h-8 bg-zinc-700"></div>

                        // GROUP 3: Edit Actions
                        <div class="flex items-center gap-1 bg-zinc-800/50 rounded-lg px-2 py-1">
                            <button
                                on:click=do_undo
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white disabled:opacity-40"
                                title="Undo (Ctrl+Z)"
                            >
                                "↶"
                            </button>
                            <button
                                on:click=do_redo
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white disabled:opacity-40"
                                title="Redo (Ctrl+Y)"
                            >
                                "↷"
                            </button>
                            <button
                                on:click=do_duplicate_object
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Duplicate Object (Ctrl+D)"
                            >
                                "❐"
                            </button>
                            <div class="w-px h-4 bg-zinc-600 mx-1"></div>
                            <button
                                on:click=duplicate_slide
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Duplicate Slide"
                            >
                                "📋"
                            </button>
                            <div class="w-px h-4 bg-zinc-600 mx-1"></div>
                            <button
                                onclick="window.toggleCanvasGrid(!window._gridEnabled);"
                                class="px-2 py-1 hover:bg-zinc-700 rounded text-sm text-white"
                                title="Toggle Grid"
                            >
                                "⊞"
                            </button>
                        </div>

                        // Spacer
                        <div class="flex-1"></div>

                        // GROUP 4: PPTX Navigation (right side)
                        <div class="flex items-center gap-2 bg-zinc-800/50 rounded-lg px-2 py-1">
                            <span class="text-xs text-zinc-500">"PPTX:"</span>
                            <button
                                on:click=move |_| {
                                    let _ = js_sys::eval("if(window._importedSlides && window._currentImportedSlideIndex > 0) { window.navigateImportedSlide(window._currentImportedSlideIndex - 1); }");
                                }
                                class="px-2 py-1 bg-zinc-700 hover:bg-zinc-600 rounded text-sm text-white disabled:opacity-40"
                                title="Previous Imported Slide"
                            >
                                "◀"
                            </button>
                            <span
                                class="text-xs text-zinc-400 min-w-[40px] text-center"
                                id="pptx-slide-indicator"
                            >
                                {move || "--"}
                            </span>
                            <button
                                on:click=move |_| {
                                    let _ = js_sys::eval("if(window._importedSlides && window._currentImportedSlideIndex < window._importedSlides.length - 1) { window.navigateImportedSlide(window._currentImportedSlideIndex + 1); }");
                                }
                                class="px-2 py-1 bg-zinc-700 hover:bg-zinc-600 rounded text-sm text-white disabled:opacity-40"
                                title="Next Imported Slide"
                            >
                                "▶"
                            </button>
                        </div>

                        // Auto-save indicator (updated by cloudAutoSave)
                        <span id="autosave-indicator" class="text-xs ml-2"></span>
                    </div>

                    // Canvas Container with Onboarding Banner
                    <div class="flex-1 flex flex-col items-center justify-center p-8 overflow-auto">
                        // Onboarding banner - shown only first time (dismissed via localStorage)
                        {move || {
                            // Check if banner was dismissed
                            let dismissed = web_sys::window()
                                .and_then(|w| w.local_storage().ok().flatten())
                                .and_then(|s| s.get_item("onboarding_dismissed").ok().flatten())
                                .map(|v| v == "true")
                                .unwrap_or(false);

                            if dismissed {
                                view! { <div></div> }.into_view()
                            } else {
                                view! {
                                    <div
                                        id="onboarding-banner"
                                        class="mb-4 p-4 bg-zinc-800/80 rounded-lg border border-zinc-700 max-w-xl text-center relative"
                                    >
                                        // Dismiss button
                                        <button
                                            class="absolute top-2 right-2 text-zinc-500 hover:text-white text-sm"
                                            title="Dismiss"
                                            onclick="localStorage.setItem('onboarding_dismissed', 'true'); this.parentElement.style.display='none';"
                                        >
                                            "✕"
                                        </button>
                                        <h3 class="text-lg font-bold text-white mb-2">"📤 ¿Cómo funciona?"</h3>
                                        <div class="text-sm text-zinc-400 space-y-1">
                                            <p>"1. Importa tu PPTX corporativo"</p>
                                            <p>"2. Agrega placeholders donde irán los datos"</p>
                                            <p>"3. Guarda el template para usarlo en Dashboard"</p>
                                        </div>
                                        <button
                                            class="mt-3 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-medium"
                                            onclick="document.getElementById('pptx-input').click(); localStorage.setItem('onboarding_dismissed', 'true'); document.getElementById('onboarding-banner').style.display='none';"
                                        >
                                            "📤 Importar mi PPTX"
                                        </button>
                                        <button
                                            class="mt-3 ml-2 px-4 py-2 bg-zinc-700 hover:bg-zinc-600 text-zinc-300 rounded-lg text-sm"
                                            onclick="localStorage.setItem('onboarding_dismissed', 'true'); document.getElementById('onboarding-banner').style.display='none';"
                                        >
                                            "Saltar tutorial"
                                        </button>
                                    </div>
                                }.into_view()
                            }
                        }}
                        <div class="relative bg-zinc-900 shadow-2xl canvas-drop-zone" style="width: 960px; height: 540px;">
                            <canvas id="editor-canvas" width="960" height="540"></canvas>
                        </div>
                    </div>
                </main>

                // Right Sidebar - Tools Panel (Collapsible Sections)
                <aside class="w-64 bg-zinc-900 border-l border-zinc-800 flex flex-col overflow-y-auto">
                    // Zoom Controls
                    <CollapsibleSection storage_key="zoom" title="Zoom" icon="🔍" default_expanded=true>
                        <div class="flex items-center gap-2">
                            <button
                                on:click=move |_| {
                                    let _ = js_sys::eval("window.zoomOut(); document.getElementById('zoom-display').textContent = Math.round(window.getZoom() * 100) + '%';");
                                }
                                class="w-8 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white flex items-center justify-center"
                                title="Zoom Out"
                            >"-"</button>
                            <span class="flex-1 text-center text-sm text-zinc-300" id="zoom-display">
                                "100%"
                            </span>
                            <button
                                on:click=move |_| {
                                    let _ = js_sys::eval("window.zoomIn(); document.getElementById('zoom-display').textContent = Math.round(window.getZoom() * 100) + '%';");
                                }
                                class="w-8 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white flex items-center justify-center"
                                title="Zoom In"
                            >"+"</button>
                            <button
                                on:click=move |_| {
                                    let _ = js_sys::eval("window.zoomReset(); document.getElementById('zoom-display').textContent = '100%';");
                                }
                                class="px-2 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs"
                                title="Reset Zoom"
                            >"⟲"</button>
                        </div>
                    </CollapsibleSection>

                    // Alignment Tools
                    <CollapsibleSection storage_key="align" title="Align (select 2+)" icon="⫿" default_expanded=false>
                        <div class="grid grid-cols-3 gap-1">
                            <button on:click=move |_| { align_selected("left"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Left">"⫷"</button>
                            <button on:click=move |_| { align_selected("center"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Center H">"⫿"</button>
                            <button on:click=move |_| { align_selected("right"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Right">"⫸"</button>
                            <button on:click=move |_| { align_selected("top"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Top">"⟙"</button>
                            <button on:click=move |_| { align_selected("middle"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Center V">"⎯"</button>
                            <button on:click=move |_| { align_selected("bottom"); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Bottom">"⟘"</button>
                        </div>
                    </CollapsibleSection>

                    // Layer Controls
                    <CollapsibleSection storage_key="layers" title="Layers" icon="📚" default_expanded=false>
                        <div class="grid grid-cols-2 gap-1">
                            <button on:click=move |_| { bring_to_front(); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Bring to Front"
                            >"↑ Front"</button>
                            <button on:click=move |_| { send_to_back(); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Send to Back"
                            >"↓ Back"</button>
                        </div>
                    </CollapsibleSection>

                    // Actions
                    <CollapsibleSection storage_key="actions" title="Actions" icon="⚡" default_expanded=true>
                        <div class="flex gap-1">
                            <button on:click=move |_| { duplicate_selected(); }
                                class="flex-1 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs"
                            >"📋 Clone"</button>
                            <button on:click=move |_| { delete_selected(); }
                                class="flex-1 h-8 bg-red-900/50 hover:bg-red-800/50 rounded text-red-300 text-xs"
                            >"🗑 Delete"</button>
                        </div>
                    </CollapsibleSection>

                    // Collapsible Placeholder Side Panel
                    <PlaceholderSidePanel on_insert=insert_placeholder.clone() />

                    // Smart Templates Analysis
                    <TemplateAnalysisPanel slides=slides />

                    // Version History
                    <VersionHistoryPanel template_id=template_id slides=slides />
                </aside>
            </div>

            // Placeholder Library Modal
            <Show when=move || show_placeholder_library.get()>
                <PlaceholderLibraryModal
                    on_close=move || show_placeholder_library.set(false)
                    on_insert=insert_placeholder
                />
            </Show>
        </div>
    }
}

/// Collapsible Section with localStorage persistence
#[component]
fn CollapsibleSection(
    /// Unique key for localStorage persistence
    storage_key: &'static str,
    /// Section title
    title: &'static str,
    /// Section icon (emoji)
    #[prop(default = "")]
    icon: &'static str,
    /// Whether default is expanded
    #[prop(default = true)]
    default_expanded: bool,
    /// Children content
    children: Children,
) -> impl IntoView {
    // Read initial state from localStorage
    let key = format!("sidebar_{}_expanded", storage_key);
    let initial_expanded = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(&key).ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(default_expanded);

    let expanded = create_rw_signal(initial_expanded);
    let key_for_toggle = key.clone();

    let toggle = move |_| {
        let new_val = !expanded.get();
        expanded.set(new_val);
        // Persist to localStorage
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item(&key_for_toggle, if new_val { "true" } else { "false" });
        }
    };

    // Render children immediately to avoid FnOnce closure issue
    let children_view = children();

    view! {
        <div class="border-b border-zinc-800">
            // Header (always visible, clickable)
            <button
                on:click=toggle
                class="w-full p-3 flex items-center justify-between text-left hover:bg-zinc-800/50 transition-colors"
            >
                <span class="text-xs text-zinc-500 font-medium uppercase flex items-center gap-1">
                    {if !icon.is_empty() { Some(view! { <span>{icon}</span> }) } else { None }}
                    {title}
                </span>
                <span class="text-zinc-500 text-xs transition-transform"
                    class:rotate-180=move || expanded.get()
                >
                    "▼"
                </span>
            </button>
            // Content (collapsible)
            <div class="px-3 pb-3" style:display=move || if expanded.get() { "block" } else { "none" }>
                {children_view}
            </div>
        </div>
    }
}

/// Placeholder Library Modal with Quick Wins UX (Search, Colors, Tooltips, D&D)
#[component]
fn PlaceholderLibraryModal<C, I>(on_close: C, on_insert: I) -> impl IntoView
where
    C: Fn() + 'static + Clone,
    I: Fn(String, String) + 'static + Clone,
{
    // Category colors for visual distinction
    let category_colors = vec![
        ("📋", "#6366f1", "#6366f120"), // General - Indigo
        ("📊", "#f97316", "#f9731620"), // Métricas - Orange
        ("🎯", "#ef4444", "#ef444420"), // Amenazas - Red
        ("🔐", "#a855f7", "#a855f720"), // Credenciales - Purple
        ("✅", "#22c55e", "#22c55e20"), // Takedowns - Green
        ("💰", "#eab308", "#eab30820"), // ROI - Yellow
        ("⚠️", "#f59e0b", "#f59e0b20"), // Risk Score - Amber
        ("📦", "#8b5cf6", "#8b5cf620"), // Code Leaks - Violet
        ("🕵️", "#3b82f6", "#3b82f620"), // Threat Intel - Blue
    ];

    // Complete placeholder catalog
    let placeholders = vec![
        // General
        ("📋 General", vec![
            ("company_name", "Nombre de Empresa", "<span style='font-size:32px;font-weight:bold;color:white;'>Banco Ejemplo S.A.</span>"),
            ("partner_name", "Nombre de Partner", "<span style='font-size:18px;color:#a1a1aa;'>MSSP Security Partner</span>"),
            ("tlp_level", "Nivel TLP", "<span style='background:#FF4B00;color:white;padding:4px 12px;font-weight:bold;'>AMBER</span>"),
            ("date_range", "Rango de Fechas", "<span style='font-size:18px;color:#a1a1aa;'>Ene 2025 - Mar 2025</span>"),
            ("start_date", "Fecha Inicio", "<span style='color:#a1a1aa;'>01 Ene 2025</span>"),
            ("end_date", "Fecha Fin", "<span style='color:#a1a1aa;'>31 Mar 2025</span>"),
        ]),
        // Métricas
        ("📊 Métricas", vec![
            ("total_tickets", "Total Tickets", "<div style='text-align:center;'><span style='font-size:72px;font-weight:900;color:#FF4B00;text-shadow:0 0 30px rgba(255,75,0,0.5);'>1,247</span><div style='color:#71717a;font-size:14px;text-transform:uppercase;'>Amenazas procesadas</div></div>"),
            ("total_threats", "Total Amenazas", "<span style='font-size:48px;font-weight:bold;color:white;'>892</span>"),
            ("hours_saved", "Horas Ahorradas", "<div><span style='font-size:48px;font-weight:bold;color:white;'>312</span><span style='font-size:24px;color:#a1a1aa;'>h</span></div>"),
            ("analysts_equivalent", "FTE Equivalente", "<span style='font-size:36px;font-weight:bold;color:#22C55E;'>1.9</span>"),
            ("validation_hours", "Horas en Validación", "<span style='font-size:24px;color:white;'>24.5h</span>"),
        ]),
        // Amenazas
        ("🎯 Amenazas", vec![
            ("threats_by_type", "Distribución por Tipo", "<div style='background:#18181b;padding:16px;border-radius:8px;'><div style='display:flex;align-items:center;gap:8px;margin-bottom:8px;'><span style='color:#a1a1aa;width:100px;'>Phishing</span><div style='flex:1;height:24px;background:#27272a;border-radius:4px;overflow:hidden;'><div style='width:78%;height:100%;background:#FF5824;'></div></div><span style='color:#FF5824;font-weight:bold;'>312</span></div></div>"),
            ("top_threat_1_name", "Top Amenaza #1", "<span style='font-size:18px;color:white;'>Phishing</span>"),
            ("top_threat_1_count", "Top #1 Cantidad", "<span style='font-size:36px;font-weight:bold;color:#FF5824;'>312</span>"),
            ("top_threat_2_name", "Top Amenaza #2", "<span style='font-size:18px;color:white;'>Fake Mobile App</span>"),
            ("top_threat_2_count", "Top #2 Cantidad", "<span style='font-size:36px;font-weight:bold;color:#FF5824;'>156</span>"),
        ]),
        // Credenciales
        ("🔐 Credenciales", vec![
            ("credentials_total", "Credenciales Expuestas", "<div style='text-align:center;'><span style='font-size:64px;font-weight:900;color:white;'>3,456</span></div>"),
            ("credentials_critical", "Credenciales Críticas", "<span style='font-size:48px;font-weight:bold;color:#EF4444;'>127</span>"),
            ("stealer_log_count", "Stealer Logs", "<span style='font-size:36px;font-weight:bold;color:#F59E0B;'>31</span>"),
            ("plain_password_count", "Passwords Plano", "<span style='font-size:36px;font-weight:bold;color:#EF4444;'>47</span>"),
            ("high_risk_users", "Usuarios Alto Riesgo", "<span style='font-size:24px;font-weight:bold;color:#EF4444;'>45</span>"),
        ]),
        // Takedowns
        ("✅ Takedowns", vec![
            ("takedown_total", "Total Takedowns", "<div style='text-align:center;'><span style='font-size:64px;font-weight:900;color:#FF5824;'>423</span></div>"),
            ("takedown_resolved", "Resueltos", "<span style='font-size:48px;font-weight:bold;color:#22C55E;'>367</span>"),
            ("takedown_pending", "En Progreso", "<span style='font-size:36px;font-weight:bold;color:#F59E0B;'>38</span>"),
            ("takedown_success_rate", "Tasa de Éxito", "<div style='text-align:center;'><span style='font-size:56px;font-weight:900;color:#22C55E;'>87.5%</span></div>"),
            ("takedown_median_uptime", "Uptime Mediano", "<span style='font-size:28px;font-weight:bold;color:#A855F7;'>7.7 días</span>"),
        ]),
        // ROI
        ("💰 ROI", vec![
            ("roi_hours_total", "Horas Ahorradas Total", "<div><span style='font-size:56px;font-weight:900;color:#FF5824;'>312</span><span style='font-size:24px;color:#a1a1aa;margin-left:8px;'>horas</span></div>"),
            ("roi_person_days", "Person-Days Ahorrados", "<div><span style='font-size:48px;font-weight:bold;color:#FF5824;'>39</span><span style='font-size:18px;color:#a1a1aa;margin-left:8px;'>person-days</span></div>"),
            ("roi_analysts_monthly", "Analistas Equivalente", "<span style='font-size:48px;font-weight:bold;color:#FF5824;'>1.9</span>"),
        ]),
        // Risk Score
        ("⚠️ Risk Score", vec![
            ("risk_score_value", "Valor Risk Score", "<div style='text-align:center;'><span style='font-size:80px;font-weight:900;color:#F59E0B;'>72</span></div>"),
            ("risk_score_label", "Label Risk Score", "<span style='background:#F59E0B;color:black;padding:6px 16px;border-radius:4px;font-weight:bold;font-size:18px;'>⚠️ Alerta</span>"),
            ("risk_score_gauge", "Gauge Risk Score", "<div style='text-align:center;'><div style='font-size:64px;font-weight:900;color:#F59E0B;'>72</div><div style='background:#27272a;height:8px;border-radius:4px;margin-top:8px;overflow:hidden;'><div style='width:72%;height:100%;background:linear-gradient(90deg,#22C55E,#F59E0B,#EF4444);'></div></div></div>"),
        ]),
        // Code Leaks
        ("📦 Code Leaks", vec![
            ("secrets_total", "Total Secretos", "<span style='font-size:48px;font-weight:bold;color:#8B5CF6;'>23</span>"),
            ("unique_repos", "Repositorios Únicos", "<span style='font-size:36px;color:white;'>8</span>"),
            ("production_secrets", "Secretos Producción", "<span style='font-size:36px;font-weight:bold;color:#EF4444;'>5</span>"),
        ]),
        // Threat Intel
        ("🕵️ Threat Intel", vec![
            ("dark_web_mentions", "Menciones Dark Web", "<span style='font-size:36px;font-weight:bold;color:#EF4444;'>12</span>"),
            ("chat_group_shares", "Shares en Chat", "<span style='font-size:36px;font-weight:bold;color:#3B82F6;'>47</span>"),
            ("social_media_mentions", "Menciones Sociales", "<span style='font-size:36px;font-weight:bold;color:#3B82F6;'>23</span>"),
        ]),
    ];

    // State
    let selected_category = create_rw_signal(0usize);
    let selected_placeholder = create_rw_signal::<Option<(String, String, String)>>(None);
    let search_query = create_rw_signal(String::new());
    let hovered_placeholder = create_rw_signal::<Option<(String, String)>>(None); // (name, html) for tooltip

    let close_click = on_close.clone();
    let insert_click = {
        let on_insert = on_insert.clone();
        move |_| {
            if let Some((key, _, html)) = selected_placeholder.get() {
                on_insert(key, html);
            }
        }
    };

    // Filter placeholders based on search
    let get_filtered_items = {
        let placeholders = placeholders.clone();
        move || {
            let query = search_query.get().to_lowercase();
            if query.is_empty() {
                // Return items from selected category
                let cat_idx = selected_category.get();
                placeholders
                    .get(cat_idx)
                    .map(|(_, items)| items.clone())
                    .unwrap_or_default()
            } else {
                // Search across ALL categories
                placeholders
                    .iter()
                    .flat_map(|(_, items)| items.iter())
                    .filter(|(key, name, _)| {
                        key.to_lowercase().contains(&query) || name.to_lowercase().contains(&query)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            }
        }
    };

    view! {
        <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
            <div class="bg-zinc-900 rounded-xl w-[900px] max-h-[650px] flex flex-col shadow-2xl">
                // Header with Search
                <div class="flex items-center justify-between p-4 border-b border-zinc-800 gap-4">
                    <div>
                        <h2 class="text-lg font-bold text-white">"📦 Biblioteca de Placeholders"</h2>
                        <p class="text-xs text-zinc-500">"Arrastra al canvas o haz clic para insertar"</p>
                    </div>
                    // Search Input
                    <div class="flex-1 max-w-xs">
                        <input
                            type="text"
                            placeholder="🔍 Buscar placeholder..."
                            class="w-full px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-white text-sm focus:outline-none focus:border-indigo-500"
                            prop:value=move || search_query.get()
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                        />
                    </div>
                    <button
                        on:click=move |_| close_click()
                        class="text-zinc-400 hover:text-white text-xl"
                    >
                        "×"
                    </button>
                </div>

                // Content
                <div class="flex-1 flex overflow-hidden">
                    // Categories - Color-coded
                    <div class="w-48 border-r border-zinc-800 py-2 overflow-y-auto">
                        {placeholders.iter().enumerate().map(|(idx, (cat_name, _))| {
                            let is_selected = move || selected_category.get() == idx;
                            let cat_name = cat_name.to_string();
                            let (_, color, bg) = category_colors.get(idx).cloned().unwrap_or(("", "#71717a", "#71717a20"));
                            view! {
                                <button
                                    on:click=move |_| {
                                        selected_category.set(idx);
                                        selected_placeholder.set(None);
                                        search_query.set(String::new()); // Clear search when selecting category
                                    }
                                    class=move || format!(
                                        "w-full px-4 py-2.5 text-left text-sm transition-all {}",
                                        if is_selected() { "font-medium" } else { "hover:bg-zinc-800/50" }
                                    )
                                    style=move || if is_selected() {
                                        format!("background: {}; color: {};", bg, color)
                                    } else {
                                        "color: #a1a1aa;".to_string()
                                    }
                                >
                                    {cat_name.clone()}
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    // Placeholders list (filterable, draggable)
                    <div class="w-56 border-r border-zinc-800 py-2 overflow-y-auto">
                        {move || {
                            let items = get_filtered_items();
                            let cat_idx = selected_category.get();
                            let (_, color, _) = category_colors.get(cat_idx).cloned().unwrap_or(("", "#71717a", "#71717a20"));

                            if items.is_empty() {
                                view! {
                                    <div class="p-4 text-center text-zinc-500 text-sm">
                                        "No se encontraron placeholders"
                                    </div>
                                }.into_view()
                            } else {
                                items.iter().map(|(key, name, html)| {
                                    let key = key.to_string();
                                    let name = name.to_string();
                                    let html = html.to_string();
                                    let key_clone = key.clone();
                                    let key_for_title = key.clone(); // Extra clone for title attribute
                                    let name_display = name.clone();
                                    let name_tooltip = name.clone();
                                    let html_tooltip = html.clone();
                                    let is_sel = move || selected_placeholder.get().as_ref().map(|p| p.0 == key_clone).unwrap_or(false);

                                    view! {
                                        <div
                                            draggable="true"
                                            on:dragstart={
                                                let key = key.clone();
                                                let html = html.clone();
                                                move |ev: web_sys::DragEvent| {
                                                    if let Some(dt) = ev.data_transfer() {
                                                        let _ = dt.set_data("text/plain", &format!("PLACEHOLDER:{}:{}", key, html));
                                                        dt.set_effect_allowed("copy");
                                                    }
                                                }
                                            }
                                            on:mouseenter={
                                                let name_tooltip = name_tooltip.clone();
                                                let html_tooltip = html_tooltip.clone();
                                                move |_| {
                                                    hovered_placeholder.set(Some((name_tooltip.clone(), html_tooltip.clone())));
                                                }
                                            }
                                            on:mouseleave=move |_| {
                                                hovered_placeholder.set(None);
                                            }
                                            on:click={
                                                let key = key.clone();
                                                let name = name.clone();
                                                let html = html.clone();
                                                move |_| {
                                                    selected_placeholder.set(Some((key.clone(), name.clone(), html.clone())));
                                                }
                                            }
                                            class=move || format!(
                                                "w-full px-4 py-2.5 text-left text-sm cursor-grab active:cursor-grabbing transition-all border-l-2 {} {}",
                                                if is_sel() { "bg-zinc-800 text-white" } else { "text-zinc-400 hover:text-white hover:bg-zinc-800/50" },
                                                if is_sel() { format!("border-l-[{}]", color) } else { "border-l-transparent".to_string() }
                                            )
                                            title=format!("Arrastra al canvas: {{{{{}}}}}", key_for_title)
                                        >
                                            <div class="flex items-center gap-2">
                                                <span class="text-zinc-600 text-xs">"≡"</span>
                                                <span>{name_display}</span>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()
                            }
                        }}
                    </div>

                    // Preview panel (shows selected OR hovered)
                    <div class="flex-1 p-4 overflow-y-auto">
                        {move || {
                            // Prioritize hover for quick preview, fall back to selected
                            let preview = if let Some((name, html)) = hovered_placeholder.get() {
                                Some((name, html, true)) // true = is hover preview
                            } else if let Some((_, name, html)) = selected_placeholder.get() {
                                Some((name, html, false)) // false = is selected
                            } else {
                                None
                            };

                            if let Some((name, html, is_hover)) = preview {
                                view! {
                                    <div class="mb-4">
                                        <div class="flex items-center gap-2 mb-2">
                                            <h3 class="text-white font-medium">{name}</h3>
                                            {if is_hover {
                                                view! { <span class="text-xs text-zinc-500 bg-zinc-800 px-2 py-0.5 rounded">"Vista rápida"</span> }.into_view()
                                            } else {
                                                view! { <span class="text-xs text-indigo-400 bg-indigo-500/20 px-2 py-0.5 rounded">"Seleccionado"</span> }.into_view()
                                            }}
                                        </div>
                                        <div class="text-zinc-400 text-sm mb-4">
                                            "Datos de ejemplo. Arrastra al canvas o inserta."
                                        </div>
                                    </div>
                                    <div
                                        class="bg-zinc-800 rounded-lg p-6"
                                        inner_html=html
                                    ></div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="text-zinc-500 text-center mt-8 space-y-2">
                                        <div class="text-2xl">"👆"</div>
                                        <div>"Selecciona o pasa el cursor sobre un placeholder"</div>
                                        <div class="text-xs text-zinc-600">"Tip: Arrastra directamente al canvas"</div>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>
                </div>

                // Footer
                <div class="flex items-center justify-between gap-3 p-4 border-t border-zinc-800">
                    <div class="text-xs text-zinc-500">
                        "💡 Tip: Escribe para buscar en todas las categorías"
                    </div>
                    <div class="flex gap-3">
                        <button
                            on:click=move |_| on_close()
                            class="px-4 py-2 text-zinc-400 hover:text-white"
                        >
                            "Cancelar"
                        </button>
                        <button
                            on:click=insert_click.clone()
                            disabled=move || selected_placeholder.get().is_none()
                            class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            "+ Insertar Placeholder"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Side Panel with Placeholders - Favorites & Recents enabled
#[component]
fn PlaceholderSidePanel<I>(on_insert: I) -> impl IntoView
where
    I: Fn(String, String) + 'static + Clone,
{
    // Flat placeholder data: (icon, color, key, label)
    let items: Vec<(&'static str, &'static str, &'static str, &'static str)> = vec![
        ("📋", "#6366f1", "company_name", "Empresa"),
        ("📋", "#6366f1", "date_range", "Fechas"),
        ("📊", "#f97316", "total_tickets", "Tickets"),
        ("📊", "#f97316", "total_threats", "Amenazas"),
        ("📊", "#f97316", "hours_saved", "Horas"),
        ("🎯", "#ef4444", "top_threat_1_name", "Top#1"),
        ("🎯", "#ef4444", "threats_by_type", "Distr."),
        ("🔐", "#a855f7", "credentials_total", "Creds"),
        ("🔐", "#a855f7", "credentials_critical", "Críticas"),
        ("✅", "#22c55e", "takedown_total", "TD Total"),
        ("✅", "#22c55e", "takedown_success_rate", "%Éxito"),
        ("⚠️", "#f59e0b", "risk_score_value", "Risk"),
        // Conditional placeholders
        ("🔀", "#06b6d4", "risk_status", "🔀 Estado Risk"),
        ("🔀", "#06b6d4", "threat_level", "🔀 Nivel Threats"),
        ("🔀", "#06b6d4", "credential_alert", "🔀 Alerta Creds"),
        ("🔀", "#06b6d4", "takedown_efficiency", "🔀 Efic. TD"),
    ];

    let search_query = create_rw_signal(String::new());
    // Favorites signal - loaded from storage on mount, updated on toggle
    let favorites = create_rw_signal(storage::load_favorites());

    view! {
        <div class="flex-1 flex flex-col overflow-hidden border-t border-zinc-800">
            <div class="p-2 border-b border-zinc-800">
                <div class="text-xs text-zinc-500 mb-1">"📦 Placeholders"</div>
                <input
                    type="text"
                    placeholder="🔍 Buscar..."
                    class="w-full px-2 py-1 bg-zinc-800 border border-zinc-700 rounded text-white text-xs"
                    prop:value=move || search_query.get()
                    on:input=move |ev| search_query.set(event_target_value(&ev))
                />
            </div>
            <div class="flex-1 overflow-y-auto py-1">
                // Favorites indicated by ★, click star to toggle
                // All items
                {items.into_iter().map(|(icon, color, key, label)| {
                    let on_insert = on_insert.clone();
                    let key_c = key.to_string();
                    let key_star_1 = key.to_string();
                    let key_star_2 = key.to_string();
                    let key_star_3 = key.to_string();
                    let key_star_4 = key.to_string();
                    let key_star_5 = key.to_string();
                    let label_c = label.to_string();
                    view! {
                        <div
                            class="mx-1 px-2 py-1 text-xs text-zinc-400 hover:text-white hover:bg-zinc-800 rounded cursor-grab flex items-center gap-1 group"
                            class:hidden=move || {
                                let q = search_query.get().to_lowercase();
                                !q.is_empty() && !key.to_lowercase().contains(&q) && !label.to_lowercase().contains(&q)
                            }
                            draggable="true"
                            on:dragstart=move |ev: web_sys::DragEvent| {
                                if let Some(dt) = ev.data_transfer() {
                                    let html = format!("<span>{{{{{}}}}}</span>", key);
                                    let _ = dt.set_data("text/plain", &format!("PLACEHOLDER:{}:{}", key, html));
                                    dt.set_effect_allowed("copy");
                                }
                            }
                            on:click={
                                let on_insert = on_insert.clone();
                                let key_c = key_c.clone();
                                move |_| {
                                    storage::add_to_recents(&key_c);
                                    let html = format!("<span>{{{{{}}}}}</span>", key_c);
                                    on_insert(key_c.clone(), html);
                                }
                            }
                            title=format!("{{{{{}}}}}", key)
                        >
                            // Star toggle (visible on hover or if favorited)
                            <button
                                class="text-xs hover:scale-110 transition-transform"
                                class:text-yellow-500=move || favorites.get().contains(&key_star_1)
                                class:text-zinc-600=move || !favorites.get().contains(&key_star_2)
                                class:opacity-0=move || !favorites.get().contains(&key_star_3)
                                class:group-hover:opacity-100=true
                                on:click={
                                    let key = key_star_4.clone();
                                    move |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                        let is_fav = storage::toggle_favorite(&key);
                                        favorites.set(storage::load_favorites());
                                        let _ = is_fav; // suppress unused warning
                                    }
                                }
                                title="Toggle favorito"
                            >
                                {move || if favorites.get().contains(&key_star_5) { "★" } else { "☆" }}
                            </button>
                            <span class="text-zinc-600">"≡"</span>
                            <span style=format!("color:{}", color)>{icon}</span>
                            <span class="flex-1">{label_c}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="p-1 border-t border-zinc-800 text-center">
                <span class="text-xs text-zinc-600">"💡 Arrastra al canvas"</span>
            </div>
        </div>
    }
}

/// Template Analysis Panel - Shows placeholder category usage
#[component]
fn TemplateAnalysisPanel(slides: RwSignal<Vec<EditorSlide>>) -> impl IntoView {
    // Analysis results: (used_categories, placeholder_count)
    let used_categories = create_rw_signal::<Vec<String>>(vec![]);
    let placeholder_count = create_rw_signal(0usize);
    let slide_count = create_rw_signal(0usize);
    let has_analyzed = create_rw_signal(false);

    let run_analysis = move |_| {
        // Collect all slide canvas_json into a JS array
        let slides_vec = slides.get();
        let js_array = js_sys::Array::new();
        for slide in slides_vec.iter() {
            js_array.push(&wasm_bindgen::JsValue::from_str(&slide.canvas_json));
        }
        slide_count.set(slides_vec.len());

        let result = analyze_template_data(js_array.into());
        // Parse JsValue - it returns { used: [], unused: [], placeholders: [] }
        if let Ok(obj) = js_sys::Reflect::get(&result, &wasm_bindgen::JsValue::from_str("used")) {
            let used_arr = js_sys::Array::from(&obj);
            let used: Vec<String> = used_arr.iter().filter_map(|v| v.as_string()).collect();
            used_categories.set(used);
        }
        if let Ok(placeholders_obj) =
            js_sys::Reflect::get(&result, &wasm_bindgen::JsValue::from_str("placeholders"))
        {
            let placeholders_arr = js_sys::Array::from(&placeholders_obj);
            placeholder_count.set(placeholders_arr.length() as usize);
        }
        has_analyzed.set(true);
    };

    view! {
        <div class="border-t border-zinc-800 p-2">
            <div class="flex items-center justify-between mb-2">
                <span class="text-xs text-zinc-500">"📊 Datos Requeridos"</span>
                <button
                    on:click=run_analysis
                    class="px-2 py-0.5 text-xs bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white rounded"
                >
                    "Analizar"
                </button>
            </div>

            // Results after analysis
            <Show when=move || has_analyzed.get()>
                <div class="text-xs text-zinc-500 mb-2">
                    {move || format!("{} slides, {} placeholders", slide_count.get(), placeholder_count.get())}
                </div>
                <div class="space-y-1">
                    // General
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"General".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"General".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"General".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#6366f1">"📋"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"General".to_string())>"General"</span>
                    </div>
                    // Metrics
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"Metrics".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"Metrics".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"Metrics".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#f97316">"📊"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"Metrics".to_string())>"Metrics"</span>
                    </div>
                    // Threats
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"Threats".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"Threats".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"Threats".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#ef4444">"🎯"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"Threats".to_string())>"Threats"</span>
                    </div>
                    // Credentials
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"Credentials".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"Credentials".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"Credentials".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#a855f7">"🔐"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"Credentials".to_string())>"Credentials"</span>
                    </div>
                    // Takedowns
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"Takedowns".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"Takedowns".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"Takedowns".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#22c55e">"✅"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"Takedowns".to_string())>"Takedowns"</span>
                    </div>
                    // Risk
                    <div class="flex items-center gap-1 text-xs px-1 py-0.5 rounded"
                        class:bg-green-900=move || used_categories.get().contains(&"Risk".to_string())
                        class:bg-zinc-800=move || !used_categories.get().contains(&"Risk".to_string())
                    >
                        <span>{move || if used_categories.get().contains(&"Risk".to_string()) { "✅" } else { "⚪" }}</span>
                        <span style="color:#f59e0b">"⚠️"</span>
                        <span class:text-green-400=move || used_categories.get().contains(&"Risk".to_string())>"Risk"</span>
                    </div>
                </div>
            </Show>

            // Empty state
            <Show when=move || !has_analyzed.get()>
                <div class="text-xs text-zinc-600 text-center py-2">
                    "Haz clic en Analizar"
                </div>
            </Show>
        </div>
    }
}

/// Version History Panel - Shows template version history
#[component]
fn VersionHistoryPanel(
    template_id: RwSignal<Option<String>>,
    slides: RwSignal<Vec<EditorSlide>>,
) -> impl IntoView {
    let versions = create_rw_signal::<Vec<storage::TemplateVersion>>(vec![]);
    let is_expanded = create_rw_signal(false);

    // Load versions when expanded
    let load_versions = move |_| {
        is_expanded.update(|v| *v = !*v);
        if is_expanded.get() {
            if let Some(tid) = template_id.get() {
                versions.set(storage::load_template_versions(&tid));
            }
        }
    };

    // Restore a version
    let restore_version = move |version_num: u32| {
        if let Some(tid) = template_id.get() {
            if let Some(ver) = storage::get_template_version(&tid, version_num) {
                // Restore slides from version
                let restored_slides: Vec<EditorSlide> = ver
                    .slides_json
                    .iter()
                    .enumerate()
                    .map(|(i, json)| EditorSlide {
                        id: format!("slide-{}", i + 1),
                        name: format!("Slide {}", i + 1),
                        canvas_json: json.clone(),
                    })
                    .collect();
                slides.set(restored_slides);
                // Load first slide into canvas
                if let Some(first) = slides.get().first() {
                    load_canvas_json(&first.canvas_json);
                }
                log(&format!("Restored to version {}", version_num));
            }
        }
    };

    view! {
        <div class="border-t border-zinc-800 p-2">
            <button
                on:click=load_versions
                class="w-full flex items-center justify-between text-xs text-zinc-500 hover:text-zinc-300"
            >
                <span>"📜 Historial"</span>
                <span>{move || if is_expanded.get() { "▼" } else { "▶" }}</span>
            </button>

            <Show when=move || is_expanded.get()>
                <div class="mt-2 space-y-1">
                    {move || {
                        let vers = versions.get();
                        if vers.is_empty() {
                            view! { <div class="text-xs text-zinc-600 text-center py-1">"Sin versiones guardadas"</div> }.into_view()
                        } else {
                            vers.iter().take(5).map(|v| {
                                let version_num = v.version;
                                let date_str = v.date.clone();
                                // Format date for display
                                let display_date = date_str.get(..16).unwrap_or(&date_str).replace("T", " ");
                                view! {
                                    <div class="flex items-center justify-between text-xs bg-zinc-800 rounded px-2 py-1">
                                        <div>
                                            <span class="text-zinc-300">{format!("v{}", version_num)}</span>
                                            <span class="text-zinc-500 ml-1">{display_date}</span>
                                        </div>
                                        <button
                                            on:click=move |_| restore_version(version_num)
                                            class="text-blue-400 hover:text-blue-300 text-xs"
                                        >
                                            "Restaurar"
                                        </button>
                                    </div>
                                }
                            }).collect_view()
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
