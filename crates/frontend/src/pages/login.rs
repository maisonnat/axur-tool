//! Login page with 3-step authentication flow and language selection

use crate::api;
use crate::components::{Card, TextInput};
use crate::{get_ui_dict, AppState, Page, UiLanguage};
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
enum LoginStep {
    Credentials,
    TwoFactor,
    Finalizing,
}

/// Login page component
#[component]
pub fn LoginPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    // Extract signals we need (signals are Copy, so this is fine)
    let ui_language = state.ui_language;
    let is_authenticated = state.is_authenticated;
    let current_page = state.current_page;

    // Form fields
    let email = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let tfa_code = create_rw_signal(String::new());

    // Login state
    let step = create_rw_signal(LoginStep::Credentials);
    let loading = create_rw_signal(false);
    let error = create_rw_signal(Option::<String>::None);

    // Temporary tokens from auth flow
    let temp_token = create_rw_signal(String::new());
    let correlation = create_rw_signal(Option::<String>::None);
    let device_id = create_rw_signal(String::new());

    // Get dictionary based on current language
    let dict = Signal::derive(move || get_ui_dict(ui_language.get()));

    // Step 1: Submit credentials action
    let credentials_action = create_action(move |_: &()| async move {
        let email_val = email.try_get().unwrap_or_default();
        let password_val = password.try_get().unwrap_or_default();
        let d = dict.get();

        if email_val.is_empty() || password_val.is_empty() {
            let _ = error.try_set(Some(d.email_password_required.to_string()));
            return;
        }

        let _ = loading.try_set(true);
        let _ = error.try_set(None);

        match api::login(&email_val, &password_val).await {
            Ok(resp) => {
                if resp.success {
                    if let Some(token) = resp.token {
                        temp_token.set(token);
                    }
                    correlation.set(resp.correlation);
                    loading.set(false);
                    step.set(LoginStep::TwoFactor);
                } else {
                    error.set(Some(resp.message));
                    loading.set(false);
                }
            }
            Err(e) => {
                error.set(Some(e));
                loading.set(false);
            }
        }
    });

    // Step 2: Submit 2FA action
    let tfa_action = create_action(move |_: &()| {
        async move {
            let d = dict.get();
            if tfa_code.get().is_empty() {
                error.set(Some(d.tfa_required.to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            let code = tfa_code.get();
            let token = temp_token.get();
            let corr = correlation.get();
            let email_val = email.get();
            let password_val = password.get();

            match api::verify_2fa(&code, &token, corr).await {
                Ok(resp) => {
                    if resp.success {
                        if let Some(t) = resp.token {
                            temp_token.set(t);
                        }
                        correlation.set(resp.correlation);
                        if let Some(did) = resp.device_id {
                            device_id.set(did);
                        }
                        step.set(LoginStep::Finalizing);

                        // Finalize login
                        let token_val = temp_token.get();
                        let corr_val = correlation.get();
                        let device_val = device_id.get();

                        leptos::logging::log!("Starting finalization for: {}", email_val);

                        match api::finalize(
                            &email_val,
                            &password_val,
                            &token_val,
                            corr_val,
                            &device_val,
                        )
                        .await
                        {
                            Ok(resp) => {
                                leptos::logging::log!("Finalize response: {:?}", resp);
                                if resp.success {
                                    is_authenticated.set(true);
                                    current_page.set(Page::Dashboard);
                                } else {
                                    leptos::logging::error!("Finalize failed: {}", resp.message);
                                    error.set(Some(resp.message));
                                    step.set(LoginStep::TwoFactor);
                                }
                            }
                            Err(e) => {
                                leptos::logging::error!("Finalize error: {}", e);
                                error.set(Some(e));
                                step.set(LoginStep::TwoFactor);
                            }
                        }
                    } else {
                        error.set(Some(resp.message));
                    }
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        }
    });

    view! {
        <div class="min-h-screen flex items-center justify-center p-4">
            <div class="w-full max-w-md">
                // Logo
                <div class="text-center mb-8">
                    <div class="inline-flex items-center gap-2 mb-4">
                        <span class="text-orange-500 text-4xl font-black italic">"///"</span>
                        <span class="text-white text-3xl font-bold tracking-widest">"AXUR"</span>
                    </div>
                    <p class="text-zinc-400">{move || dict.get().external_threat_intel}</p>
                </div>

                <Card>
                    // Error message
                    {move || error.get().map(|e| view! {
                        <div class="bg-red-900/30 border border-red-500/50 text-red-200 px-4 py-3 rounded-lg mb-4">
                            {e}
                        </div>
                    })}

                    // Step 1: Credentials
                    <Show when=move || step.get() == LoginStep::Credentials>
                        <h2 class="text-xl font-bold text-white mb-6">{move || dict.get().login_title}</h2>

                        <TextInput
                            label=Signal::derive(move || dict.get().email_label.to_string())
                            placeholder=Signal::derive(move || dict.get().email_placeholder.to_string())
                            value=email
                            input_type="email".to_string()
                        />

                        <TextInput
                            label=Signal::derive(move || dict.get().password_label.to_string())
                            placeholder=Signal::derive(move || dict.get().password_placeholder.to_string())
                            value=password
                            input_type="password".to_string()
                        />

                        <div class="mt-6">
                            <button
                                class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
                                on:click=move |_| credentials_action.dispatch(())
                            >
                                {move || dict.get().continue_btn}
                            </button>
                        </div>
                    </Show>

                    // Step 2: 2FA
                    <Show when=move || step.get() == LoginStep::TwoFactor>
                        <h2 class="text-xl font-bold text-white mb-2">{move || dict.get().tfa_title}</h2>
                        <p class="text-zinc-400 text-sm mb-6">{move || dict.get().tfa_subtitle}</p>

                        <TextInput
                            label=Signal::derive(move || dict.get().tfa_label.to_string())
                            placeholder=Signal::derive(move || dict.get().tfa_placeholder.to_string())
                            value=tfa_code
                        />

                        <div class="mt-6">
                            <button
                                class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
                                on:click=move |_| tfa_action.dispatch(())
                            >
                                {move || dict.get().verify_btn}
                            </button>
                        </div>
                    </Show>


                    // Step 3: Finalizing
                    <Show when=move || step.get() == LoginStep::Finalizing>
                        <div class="text-center py-8">
                            <svg class="animate-spin h-12 w-12 text-orange-500 mx-auto mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            <p class="text-zinc-400">{move || dict.get().finalizing}</p>
                        </div>
                    </Show>
                </Card>

                // Footer with copyright and language selector
                <div class="mt-6 text-center">
                    <p class="text-zinc-500 text-sm mb-3">
                        {move || dict.get().copyright}
                    </p>
                    // Compact language selector
                    <div class="flex justify-center gap-2 text-xs">
                        {UiLanguage::all().into_iter().map(|lang| {
                            let is_active = move || ui_language.get() == lang;
                            view! {
                                <button
                                    class=move || format!(
                                        "transition-colors {}",
                                        if is_active() { "text-orange-500" } else { "text-zinc-500 hover:text-zinc-300" }
                                    )
                                    on:click=move |_| ui_language.set(lang)
                                >
                                    {lang.display_name()}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}
