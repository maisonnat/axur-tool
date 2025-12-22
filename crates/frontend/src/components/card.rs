//! Card component

use leptos::*;

/// Card container
#[component]
pub fn Card(children: Children, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let class = format!(
        "bg-zinc-900 border border-zinc-800 rounded-xl p-6 {}",
        class.unwrap_or_default()
    );

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

/// Card with header - supports reactive titles for i18n
#[component]
pub fn CardWithHeader(#[prop(into)] title: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div class="bg-zinc-900 border border-zinc-800 rounded-xl overflow-hidden">
            <div class="bg-zinc-800 px-6 py-4 border-b border-zinc-700">
                <h3 class="text-lg font-semibold text-white">{move || title.get()}</h3>
            </div>
            <div class="p-6">
                {children()}
            </div>
        </div>
    }
}
