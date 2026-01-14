//! GitHub Sync Overlay Component
//!
//! Displays a non-intrusive indicator when syncing data with GitHub storage.
//! Shows different states: syncing, complete, offline/error.

use leptos::*;

/// Sync state for the overlay
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncState {
    Idle,
    Syncing,
    Complete,
    Offline,
}

/// GitHub sync status overlay - shows in bottom-right corner
#[component]
pub fn GitHubSyncOverlay(
    /// Current sync state
    state: ReadSignal<SyncState>,
) -> impl IntoView {
    // Auto-hide after 2 seconds for Complete state
    let visible = create_memo(move |_| state.get() != SyncState::Idle);

    view! {
        <Show when=move || visible.get()>
            <div class="github-sync-overlay">
                {move || match state.get() {
                    SyncState::Syncing => view! {
                        <div class="sync-icon spinning">"🔄"</div>
                        <span class="pulse-text">"Sincronizando..."</span>
                    }.into_view(),
                    SyncState::Complete => view! {
                        <div class="sync-icon">"✓"</div>
                        <span class="fade-text">"Listo"</span>
                    }.into_view(),
                    SyncState::Offline => view! {
                        <div class="sync-icon warning">"⚠"</div>
                        <span class="warning-text">"Modo offline"</span>
                    }.into_view(),
                    SyncState::Idle => view! { <></> }.into_view(),
                }}
            </div>
        </Show>
    }
}

/// Cold start overlay - freeze to thaw effect
#[component]
pub fn ColdStartOverlay(
    /// Whether the server is warming up
    is_warming: ReadSignal<bool>,
    /// Whether warming is complete
    is_ready: ReadSignal<bool>,
) -> impl IntoView {
    let overlay_class = create_memo(move |_| {
        if is_ready.get() {
            "cold-overlay ready"
        } else if is_warming.get() {
            "cold-overlay warming"
        } else {
            "cold-overlay"
        }
    });

    view! {
        <Show when=move || is_warming.get() || !is_ready.get()>
            <div class=move || overlay_class.get()>
                <div class="cold-content">
                    <div class="cold-icon">"❄️"</div>
                    <span class="cold-text">
                        {move || if is_ready.get() {
                            "¡Listo para usar!"
                        } else {
                            "Calentando servidores..."
                        }}
                    </span>
                </div>
            </div>
        </Show>
    }
}
