//! Slide Editor Page
//!
//! Canvas-based editor for creating and editing presentation templates

use crate::api;
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
}

/// Slide in the editor
#[derive(Clone, Debug)]
pub struct EditorSlide {
    pub id: String,
    pub name: String,
    pub canvas_json: String,
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

                // Check if we need to load a template
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
                <div class="flex-1"></div>
                <button
                    on:click=save_template
                    disabled=move || is_saving.get()
                    class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium disabled:opacity-50"
                >
                    {move || if is_saving.get() { "Saving..." } else { "Save Template" }}
                </button>
            </header>

            <div class="flex-1 flex overflow-hidden">
                // Left Sidebar - Slides Panel
                <aside class="w-48 bg-zinc-900 border-r border-zinc-800 flex flex-col">
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
                    // Toolbar
                    <div class="h-12 bg-zinc-900 border-b border-zinc-800 flex items-center px-4 gap-2">
                        <button
                            on:click=add_text
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white"
                            title="Add Text"
                        >
                            "T Text"
                        </button>
                        <button
                            on:click=add_rectangle
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white"
                            title="Add Rectangle"
                        >
                            "□ Rectangle"
                        </button>
                        <button
                            on:click=add_circle
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white"
                            title="Add Circle"
                        >
                            "○ Circle"
                        </button>
                        <div class="w-px h-6 bg-zinc-700 mx-2"></div>
                        <button
                            on:click=toggle_library
                            class="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-700 rounded text-sm text-white font-medium"
                            title="Open Placeholder Library"
                        >
                            "📦 Placeholders"
                        </button>
                        <div class="w-px h-6 bg-zinc-700 mx-2"></div>
                        // Image upload button - trigger file picker via JS
                        <button
                            on:click=move |_| trigger_image_upload()
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white"
                            title="Upload Image"
                        >
                            "🖼️ Image"
                        </button>
                        <div class="w-px h-6 bg-zinc-700 mx-2"></div>
                        // Undo/Redo buttons
                        <button
                            on:click=do_undo
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white disabled:opacity-40"
                            title="Undo (Ctrl+Z)"
                        >
                            "↶ Undo"
                        </button>
                        <button
                            on:click=do_redo
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white disabled:opacity-40"
                            title="Redo (Ctrl+Y)"
                        >
                            "↷ Redo"
                        </button>
                        <div class="w-px h-6 bg-zinc-700 mx-2"></div>
                        // Duplicate slide button
                        <button
                            on:click=duplicate_slide
                            class="px-3 py-1.5 bg-zinc-800 hover:bg-zinc-700 rounded text-sm text-white"
                            title="Duplicate Current Slide"
                        >
                            "📋 Duplicate"
                        </button>
                    </div>

                    // Canvas Container
                    <div class="flex-1 flex items-center justify-center p-8 overflow-auto">
                        <div class="relative bg-zinc-900 shadow-2xl" style="width: 960px; height: 540px;">
                            <canvas id="editor-canvas" width="960" height="540"></canvas>
                        </div>
                    </div>
                </main>

                // Right Sidebar - Tools Panel
                <aside class="w-64 bg-zinc-900 border-l border-zinc-800 flex flex-col">
                    // Zoom Controls
                    <div class="p-3 border-b border-zinc-800">
                        <h3 class="text-xs text-zinc-500 font-medium uppercase mb-2">"Zoom"</h3>
                        <div class="flex items-center gap-2">
                            <button
                                on:click=move |_| { zoom_out(); }
                                class="w-8 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white flex items-center justify-center"
                                title="Zoom Out"
                            >"−"</button>
                            <span class="flex-1 text-center text-sm text-zinc-300">"100%"</span>
                            <button
                                on:click=move |_| { zoom_in(); }
                                class="w-8 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white flex items-center justify-center"
                                title="Zoom In"
                            >"+"</button>
                            <button
                                on:click=move |_| { zoom_reset(); }
                                class="px-2 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs"
                                title="Reset Zoom"
                            >"⟲"</button>
                        </div>
                    </div>

                    // Alignment Tools
                    <div class="p-3 border-b border-zinc-800">
                        <h3 class="text-xs text-zinc-500 font-medium uppercase mb-2">"Align (select 2+)"</h3>
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
                    </div>

                    // Layer Controls
                    <div class="p-3 border-b border-zinc-800">
                        <h3 class="text-xs text-zinc-500 font-medium uppercase mb-2">"Layers"</h3>
                        <div class="grid grid-cols-2 gap-1">
                            <button on:click=move |_| { bring_to_front(); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Bring to Front"
                            >"↑ Front"</button>
                            <button on:click=move |_| { send_to_back(); }
                                class="h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs" title="Send to Back"
                            >"↓ Back"</button>
                        </div>
                    </div>

                    // Delete Selected
                    <div class="p-3 border-b border-zinc-800">
                        <h3 class="text-xs text-zinc-500 font-medium uppercase mb-2">"Actions"</h3>
                        <div class="flex gap-1">
                            <button on:click=move |_| { duplicate_selected(); }
                                class="flex-1 h-8 bg-zinc-800 hover:bg-zinc-700 rounded text-white text-xs"
                            >"📋 Clone"</button>
                            <button on:click=move |_| { delete_selected(); }
                                class="flex-1 h-8 bg-red-900/50 hover:bg-red-800/50 rounded text-red-300 text-xs"
                            >"🗑 Delete"</button>
                        </div>
                    </div>

                    // Keyboard Shortcuts Reference
                    <div class="p-3 flex-1">
                        <h3 class="text-xs text-zinc-500 font-medium uppercase mb-2">"Shortcuts"</h3>
                        <div class="text-xs text-zinc-500 space-y-1">
                            <div>"Ctrl+Z = Undo"</div>
                            <div>"Ctrl+Y = Redo"</div>
                            <div>"Del = Delete"</div>
                            <div>"Ctrl+D = Duplicate"</div>
                            <div>"Ctrl+A = Select All"</div>
                        </div>
                    </div>
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

/// Placeholder Library Modal
#[component]
fn PlaceholderLibraryModal<C, I>(on_close: C, on_insert: I) -> impl IntoView
where
    C: Fn() + 'static + Clone,
    I: Fn(String, String) + 'static + Clone,
{
    // Categories and placeholders
    let placeholders = vec![
        ("🔤 Dynamic Text", vec![
            ("company_name", "Company Name", "<span style='font-size:24px;font-weight:bold;'>ACME Corp</span>"),
            ("report_date_range", "Report Date Range", "<span>Dec 1 - Dec 31, 2024</span>"),
            ("total_incidents", "Total Incidents", "<span style='font-size:48px;font-weight:bold;color:#6366F1;'>247</span>"),
            ("total_takedowns", "Total Takedowns", "<span style='font-size:48px;font-weight:bold;color:#10B981;'>189</span>"),
        ]),
        ("📊 Data Visualizations", vec![
            ("risk_score_gauge", "Risk Score Gauge", "<div style='text-align:center;'><div style='font-size:64px;font-weight:bold;color:#F59E0B;'>7.2</div><div style='color:#94A3B8;'>RISK SCORE</div></div>"),
            ("incidents_chart", "Incidents Timeline", "<div style='background:#1E293B;padding:20px;border-radius:8px;'>[Bar Chart]</div>"),
            ("threat_distribution_pie", "Threat Distribution", "<div style='background:#1E293B;padding:20px;border-radius:8px;'>[Pie Chart]</div>"),
        ]),
        ("📋 Tables", vec![
            ("top_threats_table", "Top Threats Table", "<div style='background:#1E293B;padding:10px;border-radius:4px;'>Threat Table</div>"),
            ("takedowns_table", "Recent Takedowns", "<div style='background:#1E293B;padding:10px;border-radius:4px;'>Takedowns Table</div>"),
        ]),
        ("🗺️ Geospatial", vec![
            ("geospatial_map", "Geographic Distribution", "<div style='text-align:center;font-size:32px;'>🌎 World Map</div>"),
        ]),
        ("🖼️ Media", vec![
            ("evidence_gallery", "Evidence Gallery", "<div style='display:grid;grid-template-columns:repeat(3,1fr);gap:4px;'><div style='background:#334155;aspect-ratio:16/9;border-radius:4px;'>📷</div><div style='background:#334155;aspect-ratio:16/9;border-radius:4px;'>📷</div><div style='background:#334155;aspect-ratio:16/9;border-radius:4px;'>📷</div></div>"),
        ]),
        ("🤖 AI Analysis", vec![
            ("ai_executive_summary", "AI Executive Summary", "<div style='padding:15px;background:linear-gradient(135deg,#6366F120,#EC489920);border-left:3px solid #6366F1;border-radius:8px;'>AI-generated summary...</div>"),
            ("campaign_summary", "Campaign Detection", "<div style='padding:15px;background:#1E293B;border-radius:8px;'><div style='color:#EC4899;font-weight:bold;'>🎯 Campaign Detected</div></div>"),
        ]),
    ];

    let selected_category = create_rw_signal(0usize);
    let selected_placeholder = create_rw_signal::<Option<(String, String, String)>>(None);

    let close_click = on_close.clone();
    let insert_click = {
        let on_insert = on_insert.clone();
        move |_| {
            if let Some((key, _, html)) = selected_placeholder.get() {
                on_insert(key, html);
            }
        }
    };

    view! {
        <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
            <div class="bg-zinc-900 rounded-xl w-[800px] max-h-[600px] flex flex-col shadow-2xl">
                // Header
                <div class="flex items-center justify-between p-4 border-b border-zinc-800">
                    <h2 class="text-lg font-bold text-white">"📦 Placeholder Library"</h2>
                    <button
                        on:click=move |_| close_click()
                        class="text-zinc-400 hover:text-white text-xl"
                    >
                        "×"
                    </button>
                </div>

                // Content
                <div class="flex-1 flex overflow-hidden">
                    // Categories
                    <div class="w-48 border-r border-zinc-800 py-2">
                        {placeholders.iter().enumerate().map(|(idx, (cat_name, _))| {
                            let is_selected = move || selected_category.get() == idx;
                            let cat_name = cat_name.to_string();
                            view! {
                                <button
                                    on:click=move |_| {
                                        selected_category.set(idx);
                                        selected_placeholder.set(None);
                                    }
                                    class=move || format!(
                                        "w-full px-4 py-2 text-left text-sm {}",
                                        if is_selected() { "bg-indigo-600/20 text-indigo-400" } else { "text-zinc-300 hover:bg-zinc-800" }
                                    )
                                >
                                    {cat_name.clone()}
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    // Placeholders list and preview
                    <div class="flex-1 flex">
                        // List
                        <div class="w-48 border-r border-zinc-800 py-2 overflow-y-auto">
                            {move || {
                                let cat_idx = selected_category.get();
                                if let Some((_, items)) = placeholders.get(cat_idx) {
                                    items.iter().map(|(key, name, html)| {
                                        let key = key.to_string();
                                        let name = name.to_string();
                                        let html = html.to_string();
                                        let key_clone = key.clone();
                                        let name_display = name.clone();
                                        let is_sel = move || selected_placeholder.get().as_ref().map(|p| p.0 == key_clone).unwrap_or(false);
                                        view! {
                                            <button
                                                on:click={
                                                    let key = key.clone();
                                                    let name = name.clone();
                                                    let html = html.clone();
                                                    move |_| {
                                                        selected_placeholder.set(Some((key.clone(), name.clone(), html.clone())));
                                                    }
                                                }
                                                class=move || format!(
                                                    "w-full px-4 py-2 text-left text-sm {}",
                                                    if is_sel() { "bg-zinc-800 text-white" } else { "text-zinc-400 hover:text-white hover:bg-zinc-800/50" }
                                                )
                                            >
                                                {name_display}
                                            </button>
                                        }
                                    }).collect_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }}
                        </div>

                        // Preview
                        <div class="flex-1 p-4 overflow-y-auto">
                            {move || {
                                if let Some((_, name, html)) = selected_placeholder.get() {
                                    view! {
                                        <div class="mb-4">
                                            <h3 class="text-white font-medium mb-2">{name}</h3>
                                            <div class="text-zinc-400 text-sm mb-4">
                                                "Preview of what this placeholder will look like with sample data."
                                            </div>
                                        </div>
                                        <div
                                            class="bg-zinc-800 rounded-lg p-6"
                                            inner_html=html
                                        ></div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="text-zinc-500 text-center mt-8">
                                            "Select a placeholder to see preview"
                                        </div>
                                    }.into_view()
                                }
                            }}
                        </div>
                    </div>
                </div>

                // Footer
                <div class="flex items-center justify-end gap-3 p-4 border-t border-zinc-800">
                    <button
                        on:click=move |_| on_close()
                        class="px-4 py-2 text-zinc-400 hover:text-white"
                    >
                        "Cancel"
                    </button>
                    <button
                        on:click=insert_click.clone()
                        disabled=move || selected_placeholder.get().is_none()
                        class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        "+ Insert Placeholder"
                    </button>
                </div>
            </div>
        </div>
    }
}
