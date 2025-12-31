//! Marketplace Page
//!
//! Browse and download community templates

use leptos::*;

/// Template card data
#[derive(Clone, Debug)]
struct MarketplaceCard {
    id: String,
    name: String,
    description: Option<String>,
    author: String,
    downloads: i32,
    rating: f64,
    featured: bool,
    category: String,
}

/// Template categories
const CATEGORIES: &[&str] = &[
    "All",
    "Executive",
    "Technical",
    "Compliance",
    "Risk",
    "Custom",
];

/// Marketplace Page Component
#[component]
pub fn MarketplacePage() -> impl IntoView {
    let state = expect_context::<crate::AppState>();

    // Mock data for now - will be loaded from API
    let templates = create_rw_signal(vec![
        MarketplaceCard {
            id: "1".to_string(),
            name: "Axur Official".to_string(),
            description: Some("The default Axur report template with all sections.".to_string()),
            author: "Axur".to_string(),
            downloads: 1250,
            rating: 4.8,
            featured: true,
            category: "Executive".to_string(),
        },
        MarketplaceCard {
            id: "2".to_string(),
            name: "Executive Summary".to_string(),
            description: Some("A concise template for executive presentations.".to_string()),
            author: "Community".to_string(),
            downloads: 342,
            rating: 4.5,
            featured: false,
            category: "Executive".to_string(),
        },
        MarketplaceCard {
            id: "3".to_string(),
            name: "Risk Focus".to_string(),
            description: Some("Highlights risk scores and critical metrics.".to_string()),
            author: "Community".to_string(),
            downloads: 189,
            rating: 4.2,
            featured: false,
            category: "Risk".to_string(),
        },
        MarketplaceCard {
            id: "4".to_string(),
            name: "Technical Deep Dive".to_string(),
            description: Some("Detailed technical analysis for security teams.".to_string()),
            author: "Community".to_string(),
            downloads: 127,
            rating: 4.6,
            featured: false,
            category: "Technical".to_string(),
        },
        MarketplaceCard {
            id: "5".to_string(),
            name: "Compliance Report".to_string(),
            description: Some("Formatted for regulatory compliance requirements.".to_string()),
            author: "Community".to_string(),
            downloads: 98,
            rating: 4.0,
            featured: false,
            category: "Compliance".to_string(),
        },
    ]);

    let is_loading = create_rw_signal(false);
    let search_query = create_rw_signal(String::new());
    let filter_featured = create_rw_signal(false);
    let selected_category = create_rw_signal("All".to_string());
    let preview_template = create_rw_signal::<Option<MarketplaceCard>>(None);
    let _ = is_loading; // For future API calls

    // Navigation
    let go_back = move |_| {
        state.current_page.set(crate::Page::Dashboard);
    };

    let open_editor = move |_| {
        state.editor_template_id.set(None); // New template
        state.current_page.set(crate::Page::Editor);
    };

    // Filtered templates
    let filtered_templates = move || {
        let query = search_query.get().to_lowercase();
        let featured_only = filter_featured.get();
        let category = selected_category.get();

        templates
            .get()
            .into_iter()
            .filter(|t| {
                let matches_search = query.is_empty()
                    || t.name.to_lowercase().contains(&query)
                    || t.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false);
                let matches_featured = !featured_only || t.featured;
                let matches_category = category == "All" || t.category == category;
                matches_search && matches_featured && matches_category
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="min-h-screen bg-zinc-950 flex flex-col">
            // Header
            <header class="bg-zinc-900 border-b border-zinc-800 px-6 py-4">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <button
                            on:click=go_back
                            class="text-zinc-400 hover:text-white"
                        >
                            "← Back"
                        </button>
                        <h1 class="text-xl font-bold text-white">"📦 Template Marketplace"</h1>
                    </div>
                    <button
                        on:click=open_editor
                        class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium"
                    >
                        "+ Create Template"
                    </button>
                </div>
            </header>

            // Filters
            <div class="bg-zinc-900/50 border-b border-zinc-800 px-6 py-3">
                <div class="max-w-7xl mx-auto flex flex-col gap-3">
                    <div class="flex items-center gap-4">
                        <input
                            type="text"
                            placeholder="Search templates..."
                            class="flex-1 max-w-md bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:border-indigo-500"
                            prop:value=move || search_query.get()
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                        />
                        <label class="flex items-center gap-2 text-zinc-400 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 rounded bg-zinc-700 border-zinc-600"
                                prop:checked=move || filter_featured.get()
                                on:change=move |_| filter_featured.update(|v| *v = !*v)
                            />
                            "Featured only"
                        </label>
                    </div>
                    // Category tabs
                    <div class="flex items-center gap-2">
                        {CATEGORIES.iter().map(|cat| {
                            let cat_str = cat.to_string();
                            let cat_for_click = cat_str.clone();
                            view! {
                                <button
                                    on:click=move |_| selected_category.set(cat_for_click.clone())
                                    class=move || format!(
                                        "px-3 py-1 rounded-full text-sm transition {}",
                                        if selected_category.get() == cat_str {
                                            "bg-indigo-600 text-white"
                                        } else {
                                            "bg-zinc-800 text-zinc-400 hover:bg-zinc-700"
                                        }
                                    )
                                >
                                    {*cat}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>

            // Grid
            <main class="flex-1 px-6 py-8">
                <div class="max-w-7xl mx-auto">
                    <Show
                        when=move || !is_loading.get()
                        fallback=|| view! {
                            <div class="text-center py-12 text-zinc-500">"Loading..."</div>
                        }
                    >
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            <For
                                each=filtered_templates
                                key=|t| t.id.clone()
                                children=move |template| {
                                    let tmpl_for_preview = template.clone();
                                    view! {
                                        <TemplateCard
                                            template=template
                                            on_preview=move |_| preview_template.set(Some(tmpl_for_preview.clone()))
                                        />
                                    }
                                }
                            />
                        </div>
                    </Show>

                    <Show when=move || filtered_templates().is_empty() && !is_loading.get()>
                        <div class="text-center py-12">
                            <div class="text-4xl mb-4">"📭"</div>
                            <div class="text-zinc-400">"No templates found"</div>
                        </div>
                    </Show>
                </div>
            </main>

            // Preview Modal
            <Show when=move || preview_template.get().is_some()>
                {move || {
                    let tmpl = preview_template.get().unwrap();
                    let tmpl_id = tmpl.id.clone();
                    view! {
                        <div class="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-8">
                            <div class="bg-zinc-900 rounded-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col">
                                // Header
                                <div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between">
                                    <div>
                                        <h2 class="text-xl font-bold text-white">{tmpl.name.clone()}</h2>
                                        <p class="text-sm text-zinc-400">"by " {tmpl.author.clone()}</p>
                                    </div>
                                    <button
                                        on:click=move |_| preview_template.set(None)
                                        class="w-8 h-8 flex items-center justify-center text-zinc-400 hover:text-white"
                                    >"✕"</button>
                                </div>
                                // Preview area
                                <div class="flex-1 p-6 overflow-auto">
                                    <div class="aspect-video bg-zinc-800 rounded-xl flex items-center justify-center mb-6">
                                        <span class="text-6xl text-zinc-600">"📄"</span>
                                    </div>
                                    <p class="text-zinc-300 mb-4">{tmpl.description.clone().unwrap_or_default()}</p>
                                    <div class="flex items-center gap-4 text-sm text-zinc-400">
                                        <span class="px-2 py-1 bg-zinc-800 rounded">{tmpl.category.clone()}</span>
                                        <span>"⬇️ " {tmpl.downloads} " downloads"</span>
                                        <span>"⭐ " {format!("{:.1}", tmpl.rating)} " rating"</span>
                                    </div>
                                </div>
                                // Actions
                                <div class="px-6 py-4 border-t border-zinc-800 flex justify-end gap-3">
                                    <button
                                        on:click=move |_| preview_template.set(None)
                                        class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-white rounded-lg"
                                    >"Close"</button>
                                    <button
                                        on:click=move |_| {
                                            state.editor_clone_from_id.set(Some(tmpl_id.clone()));
                                            state.editor_template_id.set(None);
                                            state.current_page.set(crate::Page::Editor);
                                        }
                                        class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium"
                                    >"Use This Template"</button>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}

/// Individual template card
#[component]
fn TemplateCard<P>(template: MarketplaceCard, on_preview: P) -> impl IntoView
where
    P: Fn(web_sys::MouseEvent) + 'static + Clone,
{
    let state = expect_context::<crate::AppState>();
    let template_id = template.id.clone();
    let category = template.category.clone();

    let use_template = move |_| {
        // Set the template ID to load as CLONE in the Editor
        state.editor_clone_from_id.set(Some(template_id.clone()));
        state.editor_template_id.set(None);
        state.current_page.set(crate::Page::Editor);
    };

    view! {
        <div class=format!(
            "bg-zinc-900 rounded-xl overflow-hidden border {} hover:border-zinc-600 transition group",
            if template.featured { "border-indigo-500/50" } else { "border-zinc-800" }
        )>
            // Preview area (clickable)
            <div
                class="aspect-video bg-zinc-800 relative cursor-pointer"
                on:click=on_preview
            >
                <div class="absolute inset-0 flex items-center justify-center text-zinc-600">
                    <span class="text-4xl">"📄"</span>
                </div>
                // Category badge
                <div class="absolute top-2 left-2 px-2 py-1 bg-zinc-700/80 text-zinc-300 text-xs rounded-full">
                    {category}
                </div>
                {template.featured.then(|| view! {
                    <div class="absolute top-2 right-2 px-2 py-1 bg-indigo-600 text-white text-xs rounded-full font-medium">
                        "⭐ Featured"
                    </div>
                })}
                // Hover overlay
                <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 flex items-center justify-center transition">
                    <span class="text-white text-sm font-medium">"👁 Preview"</span>
                </div>
            </div>

            // Info
            <div class="p-4">
                <h3 class="font-semibold text-white mb-1">{template.name}</h3>
                <p class="text-sm text-zinc-400 mb-3 line-clamp-2">
                    {template.description.unwrap_or_default()}
                </p>

                <div class="flex items-center justify-between text-sm">
                    <div class="flex items-center gap-2 text-zinc-500">
                        <span>"by " {template.author}</span>
                    </div>
                    <div class="flex items-center gap-3 text-zinc-400">
                        <span class="flex items-center gap-1">
                            "⬇️ " {template.downloads}
                        </span>
                        <span class="flex items-center gap-1">
                            "⭐ " {format!("{:.1}", template.rating)}
                        </span>
                    </div>
                </div>
            </div>

            // Actions (show on hover)
            <div class="px-4 pb-4 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                    on:click=use_template
                    class="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium text-sm"
                >
                    "Use Template"
                </button>
            </div>
        </div>
    }
}
