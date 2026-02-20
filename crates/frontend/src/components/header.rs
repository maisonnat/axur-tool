use crate::{api, get_ui_dict, AppState, Page};
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn Header() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    let ui_language = state.ui_language;
    let is_authenticated = state.is_authenticated;
    let current_page = state.current_page;
    let has_log_access = state.has_log_access;
    let is_admin = state.is_admin;

    let dict = create_memo(move |_| get_ui_dict(ui_language.get()));

    let pending_beta_count = create_rw_signal(0i64);

    spawn_local(async move {
        if is_admin.get_untracked() {
            if let Ok(resp) = gloo_net::http::Request::get("/api/admin/beta/requests/pending-count")
                .send()
                .await
            {
                if resp.ok() {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(count) = json.get("count").and_then(|v| v.as_i64()) {
                            pending_beta_count.set(count);
                        }
                    }
                }
            }
        }
    });

    let logout_action = create_action(move |_: &()| async move {
        let _ = api::logout().await;
        is_authenticated.set(false);
        current_page.set(Page::Login);
    });

    view! {
        <header class="glass-panel sticky top-0 z-50 border-b border-white/5 px-6 py-4 backdrop-blur-xl">
            <div class="max-w-7xl mx-auto flex items-center justify-between">
                <div class="flex items-center gap-3 group cursor-default">
                    <div class="relative">
                        <span class="text-brand-primary text-2xl font-black italic relative z-10 group-hover:animate-pulse">"///"</span>
                        <div class="absolute inset-0 bg-brand-primary/20 blur-lg rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                    </div>
                    <span class="text-white text-xl font-bold tracking-[0.2em] font-display">"AXUR"</span>
                    <span class="text-zinc-500 text-xs font-mono uppercase tracking-widest border border-zinc-800 px-2 py-0.5 rounded bg-surface-base">"Web"</span>
                </div>
                <div class="flex items-center gap-4">
                    // Main Apps (High Contrast)
                    <div class="flex items-center gap-2 pr-6 border-r border-white/10">
                        // Dashboard (Generator)
                        <button
                            class=move || {
                                let base = "flex items-center gap-2 px-4 py-2 rounded-full font-medium transition-all duration-300 ";
                                if current_page.get() == Page::Dashboard {
                                    format!("{} bg-brand-primary/10 text-brand-primary shadow-[inset_0_0_10px_rgba(255,103,49,0.2)]", base)
                                } else {
                                    format!("{} text-zinc-400 hover:text-white hover:bg-white/5", base)
                                }
                            }
                            on:click=move |_| current_page.set(Page::Dashboard)
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                            "Generator"
                        </button>

                        // Editor
                        <button
                            class=move || {
                                let base = "flex items-center gap-2 px-4 py-2 rounded-full font-medium transition-all duration-300 ";
                                if current_page.get() == Page::Editor {
                                    format!("{} bg-indigo-500/10 text-indigo-400 shadow-[inset_0_0_10px_rgba(99,102,241,0.2)]", base)
                                } else {
                                    format!("{} text-zinc-400 hover:text-white hover:bg-white/5", base)
                                }
                            }
                            on:click=move |_| current_page.set(Page::Editor)
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                            "Editor"
                        </button>

                        // Templates
                        <button
                            class=move || {
                                let base = "flex items-center gap-2 px-4 py-2 rounded-full font-medium transition-all duration-300 ";
                                if current_page.get() == Page::Marketplace {
                                    format!("{} bg-purple-500/10 text-purple-400 shadow-[inset_0_0_10px_rgba(168,85,247,0.2)]", base)
                                } else {
                                    format!("{} text-zinc-400 hover:text-white hover:bg-white/5", base)
                                }
                            }
                            on:click=move |_| current_page.set(Page::Marketplace)
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 002-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
                            "Templates"
                        </button>
                    </div>

                    // Support / Ops Tools (Lower Contrast)
                    <div class="flex items-center gap-1">
                        <button
                            class=move || {
                                let base = "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all duration-300 ";
                                format!("{} text-amber-400/80 hover:text-amber-300 hover:bg-amber-400/10", base)
                            }
                            on:click=move |_| {
                                crate::onboarding::set_sandbox_mode(true);
                                current_page.set(Page::Editor);
                            }
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            "Demo"
                        </button>

                        <button
                            class=move || {
                                let base = "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all duration-300 ";
                                if current_page.get() == Page::Onboarding {
                                    format!("{} bg-blue-500/10 text-blue-400", base)
                                } else {
                                    format!("{} text-zinc-500 hover:text-zinc-300 hover:bg-white/5", base)
                                }
                            }
                            on:click=move |_| current_page.set(Page::Onboarding)
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            "Help"
                        </button>

                        <Show when=move || has_log_access.get()>
                            <button
                                class=move || {
                                    let base = "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all duration-300 ";
                                    if current_page.get() == Page::Analytics {
                                        format!("{} bg-emerald-500/10 text-emerald-400", base)
                                    } else {
                                        format!("{} text-zinc-500 hover:text-zinc-300 hover:bg-white/5", base)
                                    }
                                }
                                on:click=move |_| current_page.set(Page::Analytics)
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
                            </button>
                            <button
                                class=move || {
                                    let base = "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all duration-300 ";
                                    if current_page.get() == Page::Logs {
                                        format!("{} bg-emerald-500/10 text-emerald-400", base)
                                    } else {
                                        format!("{} text-zinc-500 hover:text-zinc-300 hover:bg-white/5", base)
                                    }
                                }
                                on:click=move |_| current_page.set(Page::Logs)
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                            </button>
                        </Show>
                        <Show when=move || state.is_admin.get()>
                            <button
                                class=move || {
                                    let base = "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all duration-300 relative ";
                                    if current_page.get() == Page::BetaAdmin {
                                        format!("{} bg-red-500/10 text-red-400", base)
                                    } else {
                                        format!("{} text-zinc-500 hover:text-red-300 hover:bg-red-500/5", base)
                                    }
                                }
                                on:click=move |_| current_page.set(Page::BetaAdmin)
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                                {move || {
                                    let count = pending_beta_count.get();
                                    if count > 0 {
                                        Some(view! {
                                            <span class="absolute -top-1 -right-1 bg-yellow-500 text-black text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center animate-pulse">
                                                {count}
                                            </span>
                                        })
                                    } else {
                                        None
                                    }
                                }}
                            </button>
                        </Show>
                    </div>
                    <button
                        class="text-zinc-400 hover:text-white transition-colors"
                        on:click=move |_| logout_action.dispatch(())
                    >
                        {move || dict.get().logout}
                    </button>
                </div>
            </div>
        </header>
    }
}
