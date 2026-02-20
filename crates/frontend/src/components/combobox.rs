//! Combobox component with autocomplete/search functionality

use leptos::*;

/// Combobox with autocomplete - allows typing to filter options
#[component]
pub fn Combobox(
    #[prop(into)] label: Signal<String>,
    options: Signal<Vec<(String, String)>>, // (value, display)
    selected: RwSignal<String>,
    #[prop(into)] placeholder: Signal<String>,
) -> impl IntoView {
    let search_text = create_rw_signal(String::new());
    let is_open = create_rw_signal(false);
    let focused_index = create_rw_signal(0usize);

    // Filter options based on search text
    let filtered_options = Signal::derive(move || {
        let search = search_text.get().to_lowercase();
        let opts = options.get();

        if search.is_empty() {
            opts
        } else {
            opts.into_iter()
                .filter(|(_, display)| display.to_lowercase().contains(&search))
                .collect()
        }
    });

    // Get display name for selected value
    let selected_display = Signal::derive(move || {
        let sel = selected.get();
        if sel.is_empty() {
            return String::new();
        }
        options
            .get()
            .iter()
            .find(|(key, _)| *key == sel)
            .map(|(_, display)| display.clone())
            .unwrap_or_default()
    });

    // Sync search text with selected display when selection changes
    create_effect(move |_| {
        let display = selected_display.get();
        if !display.is_empty() && !is_open.get() {
            search_text.set(display);
        }
    });

    // Handle option selection
    let select_option = move |key: String, display: String| {
        selected.set(key);
        search_text.set(display);
        is_open.set(false);
    };

    // Handle keyboard navigation
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let filtered = filtered_options.get();
        let len = filtered.len();

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                is_open.set(true);
                if len > 0 {
                    focused_index.set((focused_index.get() + 1) % len);
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                is_open.set(true);
                if len > 0 {
                    let current = focused_index.get();
                    focused_index.set(if current == 0 { len - 1 } else { current - 1 });
                }
            }
            "Enter" => {
                ev.prevent_default();
                if is_open.get() && len > 0 {
                    let idx = focused_index.get().min(len - 1);
                    let (key, display) = filtered[idx].clone();
                    select_option(key, display);
                }
            }
            "Escape" => {
                is_open.set(false);
            }
            _ => {}
        }
    };

    view! {
        <div class="mb-4 relative">
            <label class="block text-xs font-bold text-zinc-400 uppercase tracking-widest mb-1.5 ml-1">
                {move || label.get()}
            </label>
            <div class="relative">
                <input
                    type="text"
                    placeholder=move || placeholder.get()
                    class="w-full bg-zinc-900/50 border border-white/10 hover:border-brand-primary/30 text-zinc-300 placeholder-zinc-600 rounded-xl py-3 px-4 pr-10 outline-none focus:border-brand-primary/50 focus:shadow-glow-orange focus:bg-zinc-800 transition-all duration-300 shadow-inner"
                    prop:value=move || search_text.get()
                    on:input=move |ev| {
                        search_text.set(event_target_value(&ev));
                        is_open.set(true);
                        focused_index.set(0);
                        // Clear selection if user is typing something different
                        if !selected.get().is_empty() {
                            let current_display = selected_display.get();
                            if event_target_value(&ev) != current_display {
                                selected.set(String::new());
                            }
                        }
                    }
                    on:focus=move |_| is_open.set(true)
                    on:blur=move |_| {
                        // Delay closing to allow click on options
                        set_timeout(move || is_open.set(false), std::time::Duration::from_millis(200));
                    }
                    on:keydown=on_keydown
                />
                // Dropdown arrow icon
                <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none">
                    <svg
                        class="w-5 h-5 text-zinc-400"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </div>
            </div>

            // Dropdown list
            <Show when=move || is_open.get() && !filtered_options.get().is_empty()>
                <div class="absolute z-50 w-full mt-2 bg-zinc-900/95 backdrop-blur-xl border border-white/10 rounded-xl shadow-[0_0_30px_rgba(0,0,0,0.8)] max-h-60 overflow-auto">
                    <For
                        each=move || filtered_options.get().into_iter().enumerate()
                        key=|(_, (key, _))| key.clone()
                        children=move |(idx, (key, display))| {
                            let key_clone = key.clone();
                            let display_clone = display.clone();
                            let is_focused = move || focused_index.get() == idx;
                            let is_selected = {
                                let key = key.clone();
                                move || selected.get() == key
                            };

                            view! {
                                <div
                                    class=move || format!(
                                        "px-4 py-2 cursor-pointer transition-colors {} {}",
                                        if is_focused() { "bg-zinc-700" } else { "hover:bg-zinc-700/50" },
                                        if is_selected() { "text-orange-500" } else { "text-white" }
                                    )
                                    on:mousedown=move |_| {
                                        let k = key_clone.clone();
                                        let d = display_clone.clone();
                                        selected.set(k);
                                        search_text.set(d);
                                        is_open.set(false);
                                    }
                                    on:mouseenter=move |_| focused_index.set(idx)
                                >
                                    {display}
                                </div>
                            }
                        }
                    />
                </div>
            </Show>

            // No results message
            <Show when=move || is_open.get() && filtered_options.get().is_empty() && !search_text.get().is_empty()>
                <div class="absolute z-50 w-full mt-2 bg-zinc-900/95 backdrop-blur-xl border border-white/10 rounded-xl shadow-[0_0_30px_rgba(0,0,0,0.8)]">
                    <div class="px-4 py-3 text-zinc-400 text-sm">
                        "No se encontraron resultados"
                    </div>
                </div>
            </Show>
        </div>
    }
}
