//! Logs viewer page

use crate::api::{self, LogEntry};
use leptos::*;

/// Logs page component
#[component]
pub fn LogsPage() -> impl IntoView {
    // State
    let logs = create_rw_signal(Vec::<LogEntry>::new());
    let categories = create_rw_signal(Vec::<String>::new());
    let selected_date = create_rw_signal(get_today_date());
    let selected_category = create_rw_signal(String::new());
    let search_query = create_rw_signal(String::new());
    let loading = create_rw_signal(false);
    let error = create_rw_signal(Option::<String>::None);
    let total_count = create_rw_signal(0usize);

    // Modal state
    let selected_log = create_rw_signal(Option::<LogEntry>::None);
    let log_content = create_rw_signal(String::new());
    let loading_content = create_rw_signal(false);

    // Load categories when date changes
    let load_categories = move || {
        let date = selected_date.get();
        spawn_local(async move {
            match api::list_log_categories(Some(&date)).await {
                Ok(resp) if resp.success => {
                    categories.set(resp.categories);
                }
                _ => {
                    categories.set(vec![]);
                }
            }
        });
    };

    // Load logs
    let load_logs = move || {
        let date = selected_date.get();
        let category = selected_category.get();

        loading.set(true);
        error.set(None);

        spawn_local(async move {
            let cat_ref = if category.is_empty() {
                None
            } else {
                Some(category.as_str())
            };
            match api::list_logs(Some(&date), cat_ref, Some(100), None).await {
                Ok(resp) => {
                    if resp.success {
                        total_count.set(resp.total);
                        logs.set(resp.files);
                    } else {
                        error.set(Some(resp.message));
                    }
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            loading.set(false);
        });
    };

    // View log content
    let view_log = move |log: LogEntry| {
        selected_log.set(Some(log.clone()));
        loading_content.set(true);
        log_content.set(String::new());

        let path = log.path.clone();
        spawn_local(async move {
            match api::get_log_content(&path).await {
                Ok(resp) if resp.success => {
                    log_content.set(resp.content);
                }
                Ok(resp) => {
                    log_content.set(format!("Error: {}", resp.content));
                }
                Err(e) => {
                    log_content.set(format!("Failed to load: {}", e));
                }
            }
            loading_content.set(false);
        });
    };

    // Close modal
    let close_modal = move |_| {
        selected_log.set(None);
        log_content.set(String::new());
    };

    // Initial load
    create_effect(move |_| {
        load_categories();
        load_logs();
    });

    // Reload when date changes
    create_effect(move |prev: Option<String>| {
        let date = selected_date.get();
        if prev.is_some() && prev.as_ref() != Some(&date) {
            load_categories();
            load_logs();
        }
        date
    });

    // Reload when category changes
    create_effect(move |prev: Option<String>| {
        let cat = selected_category.get();
        if prev.is_some() {
            load_logs();
        }
        cat
    });

    // Filter logs by search
    let filtered_logs = move || {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            logs.get()
        } else {
            logs.get()
                .into_iter()
                .filter(|log| log.name.to_lowercase().contains(&query))
                .collect()
        }
    };

    // Get app state for navigation
    let state = use_context::<crate::AppState>().expect("AppState not found");
    let current_page = state.current_page;

    view! {
        <div class="logs-page">
            // Header with back button
            <div class="logs-header">
                <div class="logs-header-row">
                    <button
                        class="back-btn"
                        on:click=move |_| current_page.set(crate::Page::Dashboard)
                    >
                        "← Dashboard"
                    </button>
                </div>
                <h1 class="logs-title">"📋 Log Viewer"</h1>
                <p class="logs-subtitle">"Browse and search application logs"</p>
            </div>

            // Filters bar
            <div class="logs-filters">
                // Date picker
                <div class="filter-group">
                    <label>"Date"</label>
                    <input
                        type="date"
                        class="filter-input"
                        prop:value=move || selected_date.get().replace("/", "-")
                        on:change=move |ev| {
                            let value = event_target_value(&ev).replace("-", "/");
                            selected_date.set(value);
                        }
                    />
                </div>

                // Category dropdown
                <div class="filter-group">
                    <label>"Category"</label>
                    <select
                        class="filter-input"
                        on:change=move |ev| {
                            selected_category.set(event_target_value(&ev));
                        }
                    >
                        <option value="">"All Categories"</option>
                        <For
                            each=move || categories.get()
                            key=|cat| cat.clone()
                            children=move |cat| {
                                view! {
                                    <option value=cat.clone()>{cat}</option>
                                }
                            }
                        />
                    </select>
                </div>

                // Search box
                <div class="filter-group search-group">
                    <label>"Search"</label>
                    <input
                        type="text"
                        class="filter-input"
                        placeholder="Filter by filename..."
                        prop:value=move || search_query.get()
                        on:input=move |ev| {
                            search_query.set(event_target_value(&ev));
                        }
                    />
                </div>

                // Refresh button
                <button
                    class="refresh-btn"
                    on:click=move |_| load_logs()
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "Loading..." } else { "🔄 Refresh" }}
                </button>
            </div>

            // Status bar
            <div class="logs-status">
                <span class="status-count">
                    {move || format!("Showing {} of {} logs", filtered_logs().len(), total_count.get())}
                </span>
            </div>

            // Error message
            {move || error.get().map(|msg| view! {
                <div class="error-banner">
                    <span class="error-icon">"⚠️"</span>
                    <span>{msg}</span>
                </div>
            })}

            // Logs list
            <div class="logs-list">
                {move || if loading.get() {
                    view! { <div class="loading-spinner">"Loading logs..."</div> }.into_view()
                } else if filtered_logs().is_empty() {
                    view! { <div class="empty-state">"No logs found for the selected criteria"</div> }.into_view()
                } else {
                    view! {
                        <table class="logs-table">
                            <thead>
                                <tr>
                                    <th>"Filename"</th>
                                    <th>"Size"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=filtered_logs
                                    key=|log| log.sha.clone()
                                    children=move |log| {
                                        let log_clone = log.clone();
                                        let log_display = log.clone();
                                        view! {
                                            <tr class="log-row">
                                                <td class="log-name">
                                                    <span class="log-badge">{get_category_badge(&log_display.name)}</span>
                                                    {log_display.name.clone()}
                                                </td>
                                                <td class="log-size">{format_size(log_display.size)}</td>
                                                <td class="log-actions">
                                                    <button
                                                        class="view-btn"
                                                        on:click=move |_| view_log(log_clone.clone())
                                                    >
                                                        "👁️ View"
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    }.into_view()
                }}
            </div>

            // Log detail modal
            {move || selected_log.get().map(|log| view! {
                <div class="modal-overlay" on:click=close_modal>
                    <div class="modal-content" on:click=move |e| e.stop_propagation()>
                        <div class="modal-header">
                            <h2>{log.name.clone()}</h2>
                            <button class="modal-close" on:click=close_modal>"✕"</button>
                        </div>
                        <div class="modal-body">
                            {move || if loading_content.get() {
                                view! { <div class="loading-spinner">"Loading content..."</div> }.into_view()
                            } else {
                                view! {
                                    <pre class="log-content">{log_content.get()}</pre>
                                }.into_view()
                            }}
                        </div>
                    </div>
                </div>
            })}

            <style>
                r#"
                .logs-page {
                    padding: 2rem;
                    max-width: 1400px;
                    margin: 0 auto;
                }
                .logs-header {
                    margin-bottom: 1.5rem;
                }
                .logs-header-row {
                    margin-bottom: 1rem;
                }
                .back-btn {
                    padding: 0.5rem 1rem;
                    background: transparent;
                    border: 1px solid #3f3f46;
                    border-radius: 0.375rem;
                    color: #a1a1aa;
                    cursor: pointer;
                    font-size: 0.875rem;
                    transition: all 0.2s;
                }
                .back-btn:hover {
                    background: #27272a;
                    color: #f4f4f5;
                    border-color: #52525b;
                }
                .logs-title {
                    font-size: 1.75rem;
                    font-weight: 600;
                    color: #f4f4f5;
                    margin: 0;
                }
                .logs-subtitle {
                    color: #a1a1aa;
                    margin: 0.25rem 0 0 0;
                }
                .logs-filters {
                    display: flex;
                    gap: 1rem;
                    flex-wrap: wrap;
                    align-items: flex-end;
                    padding: 1rem;
                    background: rgba(39, 39, 42, 0.5);
                    border-radius: 0.5rem;
                    border: 1px solid #3f3f46;
                    margin-bottom: 1rem;
                }
                .filter-group {
                    display: flex;
                    flex-direction: column;
                    gap: 0.25rem;
                }
                .filter-group label {
                    font-size: 0.75rem;
                    color: #a1a1aa;
                    text-transform: uppercase;
                }
                .filter-input {
                    padding: 0.5rem 0.75rem;
                    background: #18181b;
                    border: 1px solid #3f3f46;
                    border-radius: 0.375rem;
                    color: #f4f4f5;
                    font-size: 0.875rem;
                    min-width: 150px;
                }
                .filter-input:focus {
                    outline: none;
                    border-color: #10b981;
                }
                .search-group {
                    flex: 1;
                    min-width: 200px;
                }
                .search-group .filter-input {
                    width: 100%;
                }
                .refresh-btn {
                    padding: 0.5rem 1rem;
                    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
                    border: none;
                    border-radius: 0.375rem;
                    color: white;
                    font-weight: 500;
                    cursor: pointer;
                    transition: transform 0.2s, opacity 0.2s;
                }
                .refresh-btn:hover:not(:disabled) {
                    transform: translateY(-1px);
                }
                .refresh-btn:disabled {
                    opacity: 0.5;
                    cursor: not-allowed;
                }
                .logs-status {
                    padding: 0.5rem 0;
                    color: #71717a;
                    font-size: 0.875rem;
                }
                .error-banner {
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                    padding: 0.75rem 1rem;
                    background: rgba(239, 68, 68, 0.1);
                    border: 1px solid #ef4444;
                    border-radius: 0.375rem;
                    color: #fca5a5;
                    margin-bottom: 1rem;
                }
                .logs-list {
                    background: rgba(24, 24, 27, 0.8);
                    border: 1px solid #3f3f46;
                    border-radius: 0.5rem;
                    overflow: hidden;
                }
                .logs-table {
                    width: 100%;
                    border-collapse: collapse;
                }
                .logs-table th {
                    text-align: left;
                    padding: 0.75rem 1rem;
                    background: #27272a;
                    color: #a1a1aa;
                    font-size: 0.75rem;
                    text-transform: uppercase;
                    font-weight: 600;
                    border-bottom: 1px solid #3f3f46;
                }
                .log-row {
                    border-bottom: 1px solid #27272a;
                    transition: background 0.2s;
                }
                .log-row:hover {
                    background: rgba(39, 39, 42, 0.5);
                }
                .log-row td {
                    padding: 0.75rem 1rem;
                }
                .log-name {
                    font-family: ui-monospace, monospace;
                    font-size: 0.875rem;
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                }
                .log-badge {
                    font-size: 0.75rem;
                    padding: 0.125rem 0.375rem;
                    border-radius: 0.25rem;
                    font-weight: 500;
                }
                .log-size {
                    color: #71717a;
                    font-size: 0.875rem;
                }
                .view-btn {
                    padding: 0.375rem 0.75rem;
                    background: #3f3f46;
                    border: none;
                    border-radius: 0.25rem;
                    color: #f4f4f5;
                    cursor: pointer;
                    font-size: 0.875rem;
                    transition: background 0.2s;
                }
                .view-btn:hover {
                    background: #52525b;
                }
                .loading-spinner, .empty-state {
                    padding: 3rem;
                    text-align: center;
                    color: #71717a;
                }
                .modal-overlay {
                    position: fixed;
                    inset: 0;
                    background: rgba(0, 0, 0, 0.8);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 1000;
                    padding: 2rem;
                }
                .modal-content {
                    background: #18181b;
                    border: 1px solid #3f3f46;
                    border-radius: 0.5rem;
                    width: 100%;
                    max-width: 900px;
                    max-height: 80vh;
                    display: flex;
                    flex-direction: column;
                }
                .modal-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 1rem;
                    border-bottom: 1px solid #3f3f46;
                }
                .modal-header h2 {
                    font-size: 1rem;
                    font-family: ui-monospace, monospace;
                    margin: 0;
                    color: #f4f4f5;
                }
                .modal-close {
                    background: none;
                    border: none;
                    color: #a1a1aa;
                    font-size: 1.25rem;
                    cursor: pointer;
                    padding: 0.25rem;
                }
                .modal-close:hover {
                    color: #f4f4f5;
                }
                .modal-body {
                    padding: 1rem;
                    overflow: auto;
                    flex: 1;
                }
                .log-content {
                    margin: 0;
                    font-family: ui-monospace, monospace;
                    font-size: 0.75rem;
                    line-height: 1.6;
                    white-space: pre-wrap;
                    word-break: break-word;
                    color: #d4d4d8;
                }
                "#
            </style>
        </div>
    }
}

/// Get current date in YYYY/MM/DD format
fn get_today_date() -> String {
    // Use JS Date API via web_sys
    let now = js_sys::Date::new_0();
    let year = now.get_full_year();
    let month = now.get_month() + 1; // 0-indexed
    let day = now.get_date();
    format!("{}/{:02}/{:02}", year, month, day)
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Get category badge based on filename
fn get_category_badge(name: &str) -> &'static str {
    if name.contains("error") {
        "🔴"
    } else if name.contains("request") {
        "📤"
    } else if name.contains("response") {
        "📥"
    } else if name.contains("performance") {
        "⚡"
    } else if name.contains("feature") {
        "✨"
    } else {
        "📄"
    }
}
