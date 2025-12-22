//! Button component

use leptos::*;

/// Primary button with Axur styling
#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] loading: Option<Signal<bool>>,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    let is_disabled = move || {
        disabled.map(|d| d.get()).unwrap_or(false) || loading.map(|l| l.get()).unwrap_or(false)
    };

    view! {
        <button
            class="w-full bg-orange-600 hover:bg-orange-500 disabled:bg-zinc-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
            disabled=is_disabled
            on:click=move |_| on_click.call(())
        >
            {move || if loading.map(|l| l.get()).unwrap_or(false) {
                view! {
                    <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
            {text}
        </button>
    }
}

/// Secondary/outline button
#[component]
pub fn SecondaryButton(
    #[prop(into)] text: String,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class="w-full border border-zinc-600 hover:border-zinc-400 text-zinc-300 hover:text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
            on:click=move |_| on_click.call(())
        >
            {text}
        </button>
    }
}
