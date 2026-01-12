//! Tutorial Orchestrator Component
//!
//! Manages the complete tutorial flow: welcome modal → step highlights → completion

use crate::onboarding::{
    get_onboarding_progress, storage::mark_tutorial_complete, storage::set_current_step,
    TUTORIAL_STEPS,
};
use leptos::*;

/// Main tutorial orchestrator - manages the full tutorial experience
#[component]
pub fn TutorialOrchestrator() -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let ui_language = state.ui_language;

    // Tutorial state
    let show_welcome = create_rw_signal(false);
    let show_tutorial = create_rw_signal(false);
    let current_step = create_rw_signal(0usize);

    // Check if tutorial should auto-show
    create_effect(move |_| {
        let progress = get_onboarding_progress();
        if !progress.tutorial_completed && !progress.welcome_seen {
            // Show welcome after a short delay
            #[cfg(target_arch = "wasm32")]
            {
                use gloo_timers::callback::Timeout;
                let timeout = Timeout::new(1500, move || {
                    show_welcome.set(true);
                });
                timeout.forget();
            }
        }
    });

    // Get current step data
    let step_data = move || {
        let idx = current_step.get();
        TUTORIAL_STEPS.get(idx)
    };

    // Callbacks
    let on_start_tutorial = move |_| {
        crate::onboarding::storage::mark_welcome_seen();
        show_welcome.set(false);
        show_tutorial.set(true);
        current_step.set(0);
        set_current_step(0);
    };

    let on_skip_tutorial = move |_| {
        crate::onboarding::storage::mark_welcome_seen();
        show_welcome.set(false);
        show_tutorial.set(false);
    };

    let on_next_step = move |_| {
        let next = current_step.get() + 1;
        if next >= TUTORIAL_STEPS.len() {
            // Tutorial complete
            mark_tutorial_complete();
            show_tutorial.set(false);
            crate::onboarding::unlock_achievement("tutorial");
        } else {
            current_step.set(next);
            set_current_step(next);
        }
    };

    let on_skip_step = move |_| {
        show_tutorial.set(false);
        crate::onboarding::storage::mark_welcome_seen();
    };

    // Convert signals for props
    let welcome_signal: Signal<bool> = Signal::derive(move || show_welcome.get());
    let tutorial_signal: Signal<bool> = Signal::derive(move || show_tutorial.get());

    view! {
        // Welcome Modal
        <crate::components::TutorialWelcomeModal
            is_open=welcome_signal
            on_start_tutorial=Callback::new(on_start_tutorial)
            on_skip=Callback::new(on_skip_tutorial)
        />

        // Step Highlight
        <Show when=move || show_tutorial.get() && step_data().is_some()>
            {move || {
                if let Some(step) = step_data() {
                    let lang = ui_language.get();
                    view! {
                        <crate::components::StepHighlight
                            target_selector=Signal::derive(move || step.target_selector.to_string())
                            title=Signal::derive(move || step.title(lang).to_string())
                            instruction=Signal::derive(move || step.instruction(lang).to_string())
                            step_number=Signal::derive(move || current_step.get() + 1)
                            total_steps=Signal::derive(move || TUTORIAL_STEPS.len())
                            on_next=Callback::new(on_next_step)
                            on_skip=Callback::new(on_skip_step)
                            is_visible=tutorial_signal
                        />
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </Show>
    }
}
