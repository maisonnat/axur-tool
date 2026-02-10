use leptos::*;

/// [ComponentName]
/// 
/// [Description]
#[component]
pub fn [ComponentName](
    /// [Prop Description]
    #[prop(into)]
    prop_name: String,
) -> impl IntoView {
    view! {
        <div class="p-4 border rounded shadow-sm bg-white dark:bg-gray-800">
            <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100">{prop_name}</h3>
            // Content
        </div>
    }
}
