//! Axur Frontend - Leptos Web UI
//!
//! Client-side rendered Leptos application for Axur Web.

mod api;
mod components;
mod i18n;
mod pages;

use leptos::*;
use pages::{DashboardPage, LoginPage, LogsPage};

pub use i18n::{get_ui_dict, UiDict, UiLanguage};

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
}

#[derive(Clone, Debug, PartialEq)]
pub enum Page {
    Login,
    Dashboard,
    Logs,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_authenticated: create_rw_signal(false),
            current_page: create_rw_signal(Page::Login),
            error_message: create_rw_signal(None),
            ui_language: create_rw_signal(UiLanguage::Es),
            user_email: create_rw_signal(None),
            has_log_access: create_rw_signal(false),
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

    // Check if already authenticated on load
    spawn_local(async move {
        if let Ok(valid) = api::validate_session().await {
            if valid {
                state.is_authenticated.set(true);
                state.current_page.set(Page::Dashboard);
            }
        }
    });

    view! {
        <div class="min-h-screen bg-zinc-950 text-zinc-100 font-sans">
            {move || match state.current_page.get() {
                Page::Login => view! { <LoginPage/> }.into_view(),
                Page::Dashboard => view! { <DashboardPage/> }.into_view(),
                Page::Logs => view! { <LogsPage/> }.into_view(),
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
