use crate::onboarding::achievements::Achievement;
use crate::styles::design_tokens::colors::{BRAND_PRIMARY, SURFACE_ELEVATED, SURFACE_LAYER};
use leptos::*;

#[component]
pub fn AchievementToast(
    achievement: ReadSignal<Option<Achievement>>,
    on_dismiss: Callback<()>,
) -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let lang = state.ui_language;

    view! {
        <Show when=move || achievement.get().is_some()>
            <div
                class="fixed bottom-24 left-5 z-[10000] animate-[slideUp_0.4s_ease,fadeOut_0.4s_ease_3s_forwards]"
                on:animationend=move |_| on_dismiss.call(())
            >
                <div
                    class="flex items-center gap-3 px-5 py-4 rounded-xl shadow-[0_20px_40px_rgba(0,0,0,0.4)] border border-zinc-700 bg-zinc-900"
                    style=format!("background: linear-gradient(135deg, {} 0%, {} 100%)", SURFACE_LAYER, SURFACE_ELEVATED)
                >
                    <span class="text-3xl animate-[bounce_0.5s_ease]">"🏆"</span>
                    <div>
                        <p class="m-0 mb-0.5 text-xs uppercase tracking-widest font-bold" style=format!("color: {}", BRAND_PRIMARY)>
                            "Logro Desbloqueado"
                        </p>
                        <p class="m-0 text-base font-bold text-white">
                            {move || achievement.get().map(|a| a.title(lang.get())).unwrap_or_default()}
                        </p>
                    </div>
                </div>
            </div>
        </Show>
    }
}
