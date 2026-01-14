//! Axur Frontend - Leptos Web UI
//!
//! Client-side rendered Leptos application for Axur Web.

mod api;
mod components;
mod i18n;
pub mod onboarding;
mod pages;
mod storage;

use leptos::*;
use pages::{
    AdminBetaPage, AnalyticsPage, BetaApplyPage, DashboardPage, EditorPage, LoginPage, LogsPage,
    MarketplacePage, OnboardingPage,
};

pub use i18n::{get_ui_dict, UiDict, UiLanguage};
pub use storage::{
    load_disabled_slides, load_theme, load_ui_language, save_disabled_slides, save_theme,
    save_ui_language,
};

/// Application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub is_authenticated: RwSignal<bool>,
    pub current_page: RwSignal<Page>,
    pub error_message: RwSignal<Option<String>>,
    pub ui_language: RwSignal<UiLanguage>,
    /// User email (set during login)
    pub user_email: RwSignal<Option<String>>,
    /// Whether user has access to logs
    pub has_log_access: RwSignal<bool>,
    /// Template ID to load when opening Editor (set by Marketplace "Use Template")
    pub editor_template_id: RwSignal<Option<String>>,
    /// Template ID to fetch content from but treat as new (fork)
    pub editor_clone_from_id: RwSignal<Option<String>>,
    pub is_admin: RwSignal<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Page {
    Login,
    Dashboard,
    Logs,
    Analytics,
    Editor,
    Marketplace,
    BetaApply,
    BetaAdmin,
    Onboarding,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_authenticated: create_rw_signal(false),
            current_page: create_rw_signal(Page::Login),
            error_message: create_rw_signal(None),
            ui_language: create_rw_signal(UiLanguage::from_code(&storage::load_ui_language())),
            user_email: create_rw_signal(None),
            has_log_access: create_rw_signal(false),
            editor_template_id: create_rw_signal(None),
            editor_clone_from_id: create_rw_signal(None),
            is_admin: create_rw_signal(false),
        }
    }
}

/// Main application component
#[component]
pub fn App() -> impl IntoView {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    // Create global state
    let state = AppState::default();
    provide_context(state.clone());

    // Cold start detection signals
    let is_warming = create_rw_signal(true);
    let is_ready = create_rw_signal(false);

    // Auto-save UI language when it changes
    {
        let ui_lang = state.ui_language;
        create_effect(move |_| {
            let lang = ui_lang.get();
            storage::save_ui_language(lang.code());
        });
    }

    // Check server health and detect cold starts
    {
        let is_warming = is_warming.clone();
        let is_ready = is_ready.clone();
        spawn_local(async move {
            match api::health_check().await {
                Ok(elapsed_ms) => {
                    if elapsed_ms > 2000.0 {
                        leptos::logging::log!("Cold start detected: {}ms", elapsed_ms);
                    }
                    // Transition: warming -> ready
                    is_warming.set(false);
                    // Small delay for visual transition
                    gloo_timers::callback::Timeout::new(100, move || {
                        is_ready.set(true);
                    })
                    .forget();
                }
                Err(e) => {
                    leptos::logging::error!("Health check failed: {}", e);
                    // Still show as ready to not block UI
                    is_warming.set(false);
                    is_ready.set(true);
                }
            }
        });
    }

    // Check if already authenticated on load
    spawn_local(async move {
        if let Ok(res) = api::validate_session().await {
            if res.valid {
                state.is_authenticated.set(true);
                state.is_admin.set(res.is_admin);
                state.has_log_access.set(res.has_log_access);
                state.current_page.set(Page::Dashboard);
            }
        }
    });

    view! {
        // Cold Start Overlay (Freeze → Thaw effect)
        <components::ColdStartOverlay
            is_warming=is_warming.read_only()
            is_ready=is_ready.read_only()
        />

        <div class="min-h-screen bg-zinc-950 text-zinc-100 font-sans">
            {move || match state.current_page.get() {
                Page::Login => view! { <LoginPage/> }.into_view(),
                Page::Dashboard => view! { <DashboardPage/> }.into_view(),
                Page::Logs => view! { <LogsPage/> }.into_view(),
                Page::Analytics => view! { <AnalyticsPage/> }.into_view(),
                Page::Editor => view! { <EditorPage/> }.into_view(),
                Page::Marketplace => view! { <MarketplacePage/> }.into_view(),
                Page::BetaApply => view! { <BetaApplyPage/> }.into_view(),
                Page::BetaAdmin => view! { <AdminBetaPage/> }.into_view(),
                Page::Onboarding => view! { <OnboardingPage/> }.into_view(),
            }}

        </div>
    }
}

/// Entry point for WASM
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    // Remove loading indicator
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(loader) = document.get_element_by_id("loading-indicator") {
        loader.remove();
    }

    mount_to_body(|| view! { <App/> });
}
