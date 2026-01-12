//! Hint Manager Component
//!
//! Manages contextual hints that appear after user inactivity.

use crate::components::HintBubble;
use crate::onboarding::HINTS;
use leptos::*;

/// Manages hints for key UI elements - shows after idle time
#[component]
pub fn HintManager() -> impl IntoView {
    let state = use_context::<crate::AppState>().expect("AppState");
    let ui_language = state.ui_language;

    // Idle detection (30 seconds)
    let idle_seconds = 30u32;
    let is_idle = create_rw_signal(false);
    let current_hint_idx = create_rw_signal(0usize);
    let hint_dismissed = create_rw_signal(false);

    // Set up idle timer
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        create_effect(move |_| {
            let window = web_sys::window().expect("window");
            let document = window.document().expect("document");

            // Create a timer ID holder
            let timer_id = std::rc::Rc::new(std::cell::RefCell::new(0i32));
            let timer_clone = timer_id.clone();

            let reset_and_restart = Closure::wrap(Box::new(move || {
                is_idle.set(false);
                hint_dismissed.set(false);

                // Clear existing timer
                let id = *timer_clone.borrow();
                if id != 0 {
                    web_sys::window().unwrap().clear_timeout_with_handle(id);
                }

                // Set new timer
                let set_idle = Closure::once(Box::new(move || {
                    is_idle.set(true);
                }) as Box<dyn FnOnce()>);

                let new_id = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        set_idle.as_ref().unchecked_ref(),
                        (idle_seconds * 1000) as i32,
                    )
                    .unwrap_or(0);

                *timer_clone.borrow_mut() = new_id;
                set_idle.forget();
            }) as Box<dyn Fn()>);

            let _ = document.add_event_listener_with_callback(
                "mousemove",
                reset_and_restart.as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "keydown",
                reset_and_restart.as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "click",
                reset_and_restart.as_ref().unchecked_ref(),
            );

            reset_and_restart.forget();
        });
    }

    // Get current hint
    let current_hint = move || {
        if hint_dismissed.get() || !is_idle.get() {
            return None;
        }
        HINTS.get(current_hint_idx.get())
    };

    // Dismiss handler
    let on_dismiss = move |_| {
        hint_dismissed.set(true);
        // Cycle to next hint for next idle period
        let next = (current_hint_idx.get() + 1) % HINTS.len().max(1);
        current_hint_idx.set(next);
    };

    view! {
        {move || {
            if let Some(hint) = current_hint() {
                let lang = ui_language.get();
                let message = hint.message(lang).to_string();
                let pos = hint.position_style();

                view! {
                    <HintBubble
                        message=Signal::derive(move || message.clone())
                        position_style=Signal::derive(move || pos.clone())
                        is_visible=Signal::derive(move || true)
                        on_dismiss=Callback::new(on_dismiss)
                    />
                }.into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}
