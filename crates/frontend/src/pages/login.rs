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

#[derive(Clone, Debug, PartialEq)]
enum LoginView {
    Landing,
    Login,
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

    // Get dictionary based on current language (Memoized to prevent re-cloning)
    let dict = create_memo(move |_| get_ui_dict(ui_language.get()));

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
                                    // Store email in state
                                    state.user_email.set(Some(email_val.clone()));

                                    // Check log access asynchronously
                                    let email_for_check = email_val.clone();
                                    spawn_local(async move {
                                        if let Ok(has_access) =
                                            api::check_log_access(&email_for_check).await
                                        {
                                            state.has_log_access.set(has_access);
                                        }
                                    });

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

    let (view_state, set_view_state) = create_signal(LoginView::Landing);

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-4 relative overflow-hidden bg-zinc-950 font-sans text-zinc-100">
            // Background Elements
             <div class="absolute top-[-20%] left-[-10%] w-[60%] h-[60%] bg-orange-900/10 rounded-full blur-[120px] pointer-events-none"></div>
             <div class="absolute bottom-[-20%] right-[-10%] w-[60%] h-[60%] bg-blue-900/10 rounded-full blur-[120px] pointer-events-none"></div>


            <div class="z-10 w-full max-w-md animate-in fade-in zoom-in duration-700">
                {move || match view_state.get() {
                    LoginView::Landing => view! {
                         <div class="text-center space-y-8">
                            // Logo
                            <div class="inline-flex items-center gap-2 mb-4 hover:scale-105 transition-transform cursor-default">
                                <span class="text-orange-500 text-5xl font-black italic">"///"</span>
                                <span class="text-white text-4xl font-bold tracking-widest">"AXUR"</span>
                            </div>

                            <h1 class="text-2xl font-light text-zinc-300">
                                <span class="text-white font-semibold">{move || dict.get().landing_tagline_highlight}</span> {move || dict.get().landing_tagline}
                            </h1>
                            <p class="text-zinc-500 text-sm -mt-4">{move || dict.get().landing_subtitle}</p>

                            <div class="space-y-4 pt-8">
                                <button
                                    class="w-full bg-white text-black hover:bg-zinc-200 font-bold text-lg py-4 px-8 rounded-full transition-all hover:scale-105 shadow-xl flex items-center justify-center gap-2"
                                    on:click=move |_| set_view_state.set(LoginView::Login)
                                >
                                    {move || dict.get().landing_login_btn}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75" />
                                    </svg>
                                </button>

                                <button
                                    class="w-full bg-transparent border-2 border-zinc-800 hover:border-zinc-600 text-zinc-400 hover:text-white font-semibold text-lg py-4 px-8 rounded-full transition-all hover:scale-105 flex items-center justify-center gap-2"
                                    on:click=move |_| current_page.set(Page::BetaApply)
                                >
                                    {move || dict.get().landing_beta_btn}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z" />
                                    </svg>
                                </button>
                            </div>

                            // Language selector
                            <div class="flex justify-center gap-3 text-sm pt-4">
                                {UiLanguage::all().into_iter().map(|lang| {
                                    let is_active = move || ui_language.get() == lang;
                                    view! {
                                        <button
                                            class=move || format!(
                                                "px-3 py-1 rounded-full transition-all {}",
                                                if is_active() { "bg-orange-600/20 text-orange-400 font-medium" } else { "text-zinc-500 hover:text-zinc-300" }
                                            )
                                            on:click=move |_| ui_language.set(lang)
                                        >
                                            {lang.display_name()}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>

                            <p class="text-zinc-600 text-sm mt-8">{move || dict.get().landing_rights}</p>
                         </div>
                    }.into_view(),
                    LoginView::Login => view! {
                        <div>
                             <button
                                class="mb-6 text-zinc-500 hover:text-white transition-colors flex items-center gap-2"
                                on:click=move |_| set_view_state.set(LoginView::Landing)
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                                </svg>
                                "Back"
                            </button>

                            <Card>
                                // Error message
                                {move || error.get().map(|e| view! {
                                    <div class="bg-red-900/30 border border-red-500/50 text-red-200 px-4 py-3 rounded-lg mb-4">
                                        {e}
                                    </div>
                                })}

                                // Step 1: Credentials
                                <Show when=move || step.get() == LoginStep::Credentials>
                                    <form on:submit=move |ev| {
                                        ev.prevent_default();
                                        credentials_action.dispatch(());
                                    }>
                                        <h2 class="text-xl font-bold text-white mb-6">{move || dict.get().login_title}</h2>

                                        <TextInput
                                            label=Signal::derive(move || dict.get().email_label.to_string())
                                            placeholder=Signal::derive(move || dict.get().email_placeholder.to_string())
                                            value=email
                                            input_type="email".to_string()
                                            autocomplete="username".to_string()
                                        />

                                        <TextInput
                                            label=Signal::derive(move || dict.get().password_label.to_string())
                                            placeholder=Signal::derive(move || dict.get().password_placeholder.to_string())
                                            value=password
                                            input_type="password".to_string()
                                            autocomplete="current-password".to_string()
                                        />

                                        <div class="mt-6">
                                            <button
                                                type="submit"
                                                class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
                                            >
                                                {move || dict.get().continue_btn}
                                            </button>
                                        </div>
                                    </form>
                                </Show>

                                // Step 2: 2FA
                                <Show when=move || step.get() == LoginStep::TwoFactor>
                                    <form on:submit=move |ev| {
                                        ev.prevent_default();
                                        tfa_action.dispatch(());
                                    }>
                                        <h2 class="text-xl font-bold text-white mb-2">{move || dict.get().tfa_title}</h2>
                                        <p class="text-zinc-400 text-sm mb-6">{move || dict.get().tfa_subtitle}</p>

                                        <TextInput
                                            label=Signal::derive(move || dict.get().tfa_label.to_string())
                                            placeholder=Signal::derive(move || dict.get().tfa_placeholder.to_string())
                                            value=tfa_code
                                            autofocus=true
                                        />

                                        <div class="mt-6">
                                            <button
                                                type="submit"
                                                class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
                                            >
                                                {move || dict.get().verify_btn}
                                            </button>
                                        </div>
                                    </form>
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
                    }.into_view()
                }}
            </div>
        </div>
    }
}
