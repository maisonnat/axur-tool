use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
enum Step {
    Company,
    Email,
    CheckStatus,
    Success,
}

#[derive(Serialize)]
struct BetaRequestPayload {
    email: String,
    company: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct BetaRequestResponse {
    success: bool,
    message: String,
}

#[component]
pub fn BetaApplyPage() -> impl IntoView {
    let (step, set_step) = create_signal(Step::Company);
    let (company, set_company) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(String::new());

    let submit = move |_| {
        set_loading.set(true);
        set_error_msg.set(None);

        spawn_local(async move {
            let payload = BetaRequestPayload {
                email: email.get(),
                company: company.get(),
            };

            let resp = Request::post("/api/public/beta-request")
                .json(&payload)
                .unwrap()
                .send()
                .await;

            set_loading.set(false);

            match resp {
                Ok(r) => {
                    if r.ok() {
                        let data: BetaRequestResponse =
                            r.json().await.unwrap_or(BetaRequestResponse {
                                success: true,
                                message: "Request received!".into(),
                            });
                        set_success_msg.set(data.message);
                        set_step.set(Step::Success);
                    } else {
                        let text = r.text().await.unwrap_or_default();
                        set_error_msg.set(Some(format!("Error: {}", text)));
                    }
                }
                Err(e) => set_error_msg.set(Some(e.to_string())),
            }
        });
    };

    let check_status_action = move |_| {
        set_loading.set(true);
        set_error_msg.set(None);

        spawn_local(async move {
            let email_val = email.get();
            if email_val.trim().is_empty() {
                set_error_msg.set(Some("Please enter your email.".into()));
                set_loading.set(false);
                return;
            }

            // We reuse the same endpoint or create a new one?
            // Let's create a specific check endpoint
            let resp = Request::get(&format!("/api/public/beta-status?email={}", email_val))
                .send()
                .await;

            set_loading.set(false);

            match resp {
                Ok(r) => {
                    if r.ok() {
                        let status_text = r.text().await.unwrap_or_default();
                        // Simple text response for now: "pending", "approved", "not_found"
                        if status_text.contains("approved") {
                            set_success_msg
                                .set("Your access is APPROVED! You can log in now.".into());
                            set_step.set(Step::Success);
                        } else if status_text.contains("pending") {
                            set_success_msg
                                .set("Your request is still PENDING. We'll verify it soon.".into());
                            set_step.set(Step::Success);
                        } else {
                            set_error_msg.set(Some("No request found for this email.".into()));
                        }
                    } else {
                        set_error_msg
                            .set(Some("No request found or error checking status.".into()));
                    }
                }
                Err(e) => set_error_msg.set(Some(e.to_string())),
            }
        });
    };

    let next_step = move || match step.get() {
        Step::Company => {
            if !company.get().trim().is_empty() {
                set_step.set(Step::Email);
            } else {
                set_error_msg.set(Some("Please enter your company name.".into()));
            }
        }
        Step::Email => {
            if !email.get().trim().is_empty() && email.get().contains('@') {
                submit(());
            } else {
                set_error_msg.set(Some("Please enter a valid email.".into()));
            }
        }
        _ => {}
    };

    let handle_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            next_step();
        }
    };

    view! {
        <div class="min-h-screen bg-zinc-50 text-zinc-900 flex flex-col items-center justify-center relative overflow-hidden font-sans">
            // Background Elements (Fluid)
            <div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-100 rounded-full blur-3xl opacity-50 animate-pulse"></div>
            <div class="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-purple-100 rounded-full blur-3xl opacity-50 animate-pulse" style="animation-delay: 2s;"></div>

            <div class="z-10 w-full max-w-lg p-8">
                // Header (Minimal)
                <div class="mb-12 text-center transition-all duration-500 ease-in-out">
                    <h1 class="text-4xl font-bold mb-2 tracking-tight">Axur</h1>
                    <p class="text-zinc-500 text-sm uppercase tracking-widest">Beta Access</p>
                </div>

                // Content Card
                <div class="relative bg-white/50 backdrop-blur-sm rounded-2xl p-10 shadow-xl border border-white/80 min-h-[300px] flex flex-col justify-center transition-all duration-500">

                    {move || match step.get() {
                        Step::Company => view! {
                             <div class="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
                             <div class="space-y-4">
                                <label class="block text-sm font-bold text-zinc-700 uppercase tracking-wider">"Company"</label>
                                <input
                                    type="text"
                                    class="w-full text-2xl border-b-2 border-zinc-200 focus:border-blue-600 outline-none py-2 bg-transparent transition-colors placeholder-zinc-300"
                                    placeholder="Acme Corp"
                                    autofocus
                                    on:input=move |ev| {
                                        set_company.set(event_target_value(&ev));
                                        set_error_msg.set(None); // Clear error on input
                                    }
                                    prop:value=company
                                    on:keypress=handle_keypress
                                />
                                <div class="flex justify-between items-center pt-4">
                                     <button
                                        class="text-zinc-400 hover:text-zinc-600 text-sm font-medium"
                                        on:click=move |_| {
                                            set_step.set(Step::CheckStatus);
                                            set_error_msg.set(None); // Clear error on step change
                                        }
                                    >
                                        "Already applied?"
                                    </button>

                                    <button
                                        class="bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded-full font-bold shadow-lg shadow-blue-600/30 transition-all hover:scale-105 active:scale-95"
                                        on:click=move |_| next_step()
                                    >
                                        "Next"
                                    </button>
                                </div>
                             </div>
                             </div>
                        }.into_view(),
                        Step::Email => view! {
                            <div class="space-y-4">
                                <label class="block text-sm font-bold text-zinc-700 uppercase tracking-wider">"Work Email"</label>
                                <input
                                    type="email"
                                    class="w-full text-2xl border-b-2 border-zinc-200 focus:border-blue-600 outline-none py-2 bg-transparent transition-colors placeholder-zinc-300"
                                    placeholder="you@company.com"
                                    autofocus
                                    on:input=move |ev| {
                                        set_email.set(event_target_value(&ev));
                                        set_error_msg.set(None); // Clear error on input
                                    }
                                    prop:value=email
                                    on:keypress=handle_keypress
                                />
                                <div class="flex justify-end pt-4">
                                    <button
                                        class="bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded-full font-bold shadow-lg shadow-blue-600/30 transition-all hover:scale-105 active:scale-95 flex items-center gap-2"
                                        on:click=move |_| next_step()
                                        disabled=move || loading.get()
                                    >
                                        {move || if loading.get() { "Sending..." } else { "Request Access" }}
                                    </button>
                                </div>
                            </div>
                        }.into_view(),
                        Step::CheckStatus => view! {
                             <div class="space-y-4">
                                <label class="block text-sm font-bold text-zinc-700 uppercase tracking-wider">"Email"</label>
                                <input
                                    type="email"
                                    class="w-full text-2xl border-b-2 border-zinc-200 focus:border-blue-600 outline-none py-2 bg-transparent transition-colors placeholder-zinc-300"
                                    placeholder="you@company.com"
                                    autofocus
                                    on:input=move |ev| {
                                        set_email.set(event_target_value(&ev));
                                        set_error_msg.set(None); // Clear error on input
                                    }
                                    prop:value=email
                                    on:keypress=move |ev| if ev.key() == "Enter" { check_status_action(()); }
                                />
                                <div class="flex justify-between items-center pt-4">
                                    <button
                                        class="text-zinc-400 hover:text-zinc-600 text-sm font-medium"
                                        on:click=move |_| {
                                            set_step.set(Step::Company);
                                            set_error_msg.set(None); // Clear error on step change
                                        }
                                    >
                                        "New Request"
                                    </button>

                                    <button
                                        class="bg-purple-600 hover:bg-purple-700 text-white px-8 py-3 rounded-full font-bold shadow-lg shadow-purple-600/30 transition-all hover:scale-105 active:scale-95"
                                        on:click=move |_| check_status_action(())
                                        disabled=move || loading.get()
                                    >
                                        {move || if loading.get() { "Checking..." } else { "Check" }}
                                    </button>
                                </div>
                            </div>
                        }.into_view(),
                        Step::Success => view! {
                            <div class="text-center space-y-6">
                                <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto text-green-600">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                                    </svg>
                                </div>
                                <p class="text-lg text-zinc-600">
                                    {move || success_msg.get()}
                                </p>
                                <button
                                     class="text-blue-600 hover:text-blue-700 font-bold hover:underline"
                                     on:click=move |_| {
                                         let state = use_context::<crate::AppState>().expect("AppState");
                                         state.current_page.set(crate::Page::Login);
                                     }
                                >
                                    "Back to Home"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                    {move || error_msg.get().map(|err| view! {
                        <div class="absolute -bottom-12 left-0 w-full text-center text-red-500 text-sm animate-in fade-in slide-in-from-top-2">
                            {err}
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}
