//! Input components

use leptos::*;

/// Text input field with label - supports reactive labels for i18n
#[component]
pub fn TextInput(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] placeholder: Signal<String>,
    value: RwSignal<String>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] autocomplete: Option<String>,
    #[prop(optional)] autofocus: Option<bool>,
) -> impl IntoView {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let autocomplete = autocomplete.unwrap_or_else(|| "off".to_string());
    let should_autofocus = autofocus.unwrap_or(false);

    // Node ref for autofocus
    let input_ref = create_node_ref::<html::Input>();

    // Autofocus effect
    if should_autofocus {
        create_effect(move |_| {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        });
    }

    view! {
        <div class="mb-4">
            <label class="block text-sm font-medium text-zinc-400 mb-2">
                {move || label.get()}
            </label>
            <input
                node_ref=input_ref
                type=input_type
                placeholder=move || placeholder.get()
                autocomplete=autocomplete
                class="w-full bg-zinc-800 border border-zinc-700 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-white placeholder-zinc-500 rounded-lg py-3 px-4 outline-none transition-colors"
                prop:value=move || value.try_get().unwrap_or_default()
                on:input=move |ev| {
                    let _ = value.try_set(event_target_value(&ev));
                }
                disabled=move || disabled.and_then(|d| d.try_get()).unwrap_or(false)
            />
        </div>
    }
}

/// Select dropdown
#[component]
pub fn Select(
    #[prop(into)] label: String,
    options: Signal<Vec<(String, String)>>, // (value, display)
    selected: RwSignal<String>,
    #[prop(optional)] disabled: Option<Signal<bool>>,
) -> impl IntoView {
    view! {
        <div class="mb-4">
            <label class="block text-sm font-medium text-zinc-400 mb-2">
                {label}
            </label>
            <select
                class="w-full bg-zinc-800 border border-zinc-700 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 text-white rounded-lg py-3 px-4 outline-none transition-colors cursor-pointer"
                on:change=move |ev| {
                    selected.set(event_target_value(&ev));
                }
                disabled=move || disabled.map(|d| d.get()).unwrap_or(false)
            >
                <option value="" disabled selected=move || selected.get().is_empty()>
                    "Seleccionar..."
                </option>
                <For
                    each=move || options.get()
                    key=|(value, _)| value.clone()
                    children=move |(value, display)| {
                        let val = value.clone();
                        view! {
                            <option
                                value=value.clone()
                                selected=move || selected.get() == val
                            >
                                {display}
                            </option>
                        }
                    }
                />
            </select>
        </div>
    }
}
