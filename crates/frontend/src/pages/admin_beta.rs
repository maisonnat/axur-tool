use crate::api::{self, AllowedUser};
use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct BetaReq {
    id: String, // UUID as string
    email: String,
    company: String,
    status: String,
    requested_at: Option<String>,
}

#[derive(Clone, PartialEq)]
enum AdminTab {
    Requests,
    Users,
}

#[component]
pub fn AdminBetaPage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(AdminTab::Requests);

    // Requests State
    let (requests, set_requests) = create_signal(Vec::<BetaReq>::new());

    // Users State
    let (users, set_users) = create_signal(Vec::<AllowedUser>::new());

    // Shared State
    let (error, set_error) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(false);

    // Add User Form State
    let new_user_email = create_rw_signal(String::new());
    let new_user_role = create_rw_signal("beta_tester".to_string());
    let new_user_desc = create_rw_signal(String::new());

    // --- Actions ---

    let fetch_requests = move || {
        set_loading.set(true);
        spawn_local(async move {
            let resp = Request::get("/api/admin/beta/requests").send().await;
            match resp {
                Ok(r) => {
                    if r.ok() {
                        let data: Vec<BetaReq> = r.json().await.unwrap_or_default();
                        set_requests.set(data);
                    } else {
                        set_error.set(Some("Failed to load requests".into()));
                    }
                }
                Err(e) => set_error.set(Some(e.to_string())),
            }
            set_loading.set(false);
        });
    };

    let fetch_users = move || {
        set_loading.set(true);
        spawn_local(async move {
            match api::list_users().await {
                Ok(data) => set_users.set(data),
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        });
    };

    let approve = move |id: String| {
        spawn_local(async move {
            let _ = Request::post(&format!("/api/admin/beta/requests/{}/action", id))
                .json(&serde_json::json!({ "action": "approve" }))
                .unwrap()
                .send()
                .await;
            fetch_requests();
        });
    };

    let reject = move |id: String| {
        spawn_local(async move {
            let _ = Request::post(&format!("/api/admin/beta/requests/{}/action", id))
                .json(&serde_json::json!({ "action": "reject" }))
                .unwrap()
                .send()
                .await;
            fetch_requests();
        });
    };

    let start_add_user = move || {
        let email = new_user_email.get();
        let role = new_user_role.get();
        let desc = new_user_desc.get();

        if email.is_empty() {
            set_error.set(Some("Email required".into()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        set_success_msg.set(None);

        spawn_local(async move {
            match api::add_user(
                &email,
                &role,
                if desc.is_empty() { None } else { Some(desc) },
            )
            .await
            {
                Ok(_) => {
                    set_success_msg.set(Some(format!("User {} added successfully", email)));
                    new_user_email.set("".into());
                    new_user_desc.set("".into());
                    fetch_users();
                }
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        });
    };

    let remove_user = move |email: String| {
        let confirmed = window()
            .confirm_with_message(&format!("Remove access for {}?", email))
            .unwrap_or(false);

        if !confirmed {
            return;
        }

        set_loading.set(true);
        spawn_local(async move {
            match api::remove_user(&email).await {
                Ok(_) => {
                    set_success_msg.set(Some(format!("User {} removed", email)));
                    fetch_users();
                }
                Err(e) => set_error.set(Some(e)),
            }
            set_loading.set(false);
        });
    };

    // Load initial data based on active tab
    create_effect(move |_| {
        if active_tab.get() == AdminTab::Requests {
            fetch_requests();
        } else {
            fetch_users();
        }
    });

    view! {
        <div class="p-8 max-w-7xl mx-auto min-h-screen">
            <header class="flex justify-between items-center mb-8">
                <div>
                   <h1 class="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-zinc-400">"Admin Console"</h1>
                   <p class="text-zinc-400">"Manage access and beta requests"</p>
                </div>

                // Tabs
                <div class="flex bg-zinc-900 rounded-lg p-1 border border-zinc-800">
                    <button
                        class=move || format!("px-4 py-2 rounded-md text-sm font-medium transition-all {}",
                            if active_tab.get() == AdminTab::Requests { "bg-zinc-800 text-white shadow" } else { "text-zinc-400 hover:text-white" })
                        on:click=move |_| set_active_tab.set(AdminTab::Requests)
                    >
                        "Requests"
                    </button>
                    <button
                        class=move || format!("px-4 py-2 rounded-md text-sm font-medium transition-all {}",
                            if active_tab.get() == AdminTab::Users { "bg-zinc-800 text-white shadow" } else { "text-zinc-400 hover:text-white" })
                        on:click=move |_| set_active_tab.set(AdminTab::Users)
                    >
                        "Allowed Users"
                    </button>
                    <button
                        class="px-3 py-2 text-zinc-400 hover:text-white ml-2 border-l border-zinc-800"
                        on:click=move |_| {
                            if active_tab.get() == AdminTab::Requests { fetch_requests() } else { fetch_users() }
                        }
                        title="Refresh"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 4.992l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                        </svg>
                    </button>
                </div>
            </header>

            {move || if let Some(msg) = success_msg.get() {
                view! { <div class="bg-green-900/20 border border-green-500/30 text-green-400 px-4 py-3 rounded-lg mb-6">{msg}</div> }.into_view()
            } else {
                view! {}.into_view()
            }}

            {move || if let Some(err) = error.get() {
                 view! { <div class="bg-red-900/20 border border-red-500/30 text-red-400 px-4 py-3 rounded-lg mb-6">{err}</div> }.into_view()
            } else {
                view! {}.into_view()
            }}

            {move || match active_tab.get() {
                AdminTab::Requests => view! {
                    <div>
                        {if loading.get() {
                            view! { <div class="text-center py-10 opacity-50">"Loading requests..."</div> }.into_view()
                        } else if requests.get().is_empty() {
                             view! { <div class="text-center py-20 text-zinc-500 bg-zinc-900/30 border border-zinc-800/50 rounded-xl">"No pending requests"</div> }.into_view()
                        } else {
                            view! {
                                <div class="grid gap-4">
                                    {requests.get().into_iter().map(|req| {
                                        let id_clone = req.id.clone();
                                        let id_clone_reject = req.id.clone();
                                        view! {
                                            <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 flex justify-between items-center hover:border-zinc-700 transition-colors">
                                                <div>
                                                    <h3 class="font-bold text-lg text-white">{req.company}</h3>
                                                    <p class="text-zinc-400">{req.email}</p>
                                                    <div class="flex gap-2 mt-2 text-xs">
                                                        <span class={format!("px-2 py-1 rounded-full {}",
                                                            match req.status.as_str() {
                                                                "pending" => "bg-yellow-900/30 text-yellow-500",
                                                                "approved" => "bg-green-900/30 text-green-500",
                                                                "rejected" => "bg-red-900/30 text-red-500",
                                                                _ => "bg-zinc-800 text-zinc-500"
                                                            }
                                                        )}>
                                                            {req.status.clone()}
                                                        </span>
                                                        <span class="text-zinc-600">{req.requested_at.clone().unwrap_or_default()}</span>
                                                    </div>
                                                </div>

                                                <div class="flex gap-3">
                                                    {if req.status == "pending" {
                                                        view! {
                                                            <button
                                                                class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg text-sm text-zinc-300 transition-colors"
                                                                on:click=move |_| reject(id_clone_reject.clone())
                                                            >
                                                                "Reject"
                                                            </button>
                                                            <button
                                                                class="px-4 py-2 bg-white text-black hover:bg-zinc-200 rounded-lg text-sm font-medium transition-colors"
                                                                on:click=move |_| approve(id_clone.clone())
                                                            >
                                                                "Approve Access"
                                                            </button>
                                                        }.into_view()
                                                    } else {
                                                        view! { <span class="text-zinc-600 italic">"Processed"</span> }.into_view()
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }}
                    </div>
                }.into_view(),
                AdminTab::Users => view! {
                    <div class="space-y-8">
                        // Add User Form
                        <div class="bg-zinc-900/80 border border-zinc-800 rounded-xl p-6">
                            <h3 class="text-lg font-semibold text-white mb-4">"Add New User"</h3>
                            <div class="grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
                                <div class="md:col-span-2">
                                    <label class="block text-xs uppercase text-zinc-500 font-bold mb-1">"Email"</label>
                                    <input
                                        type="email"
                                        class="w-full bg-zinc-950 border border-zinc-800 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-orange-500 transition-colors"
                                        placeholder="user@example.com"
                                        prop:value=new_user_email
                                        on:input=move |ev| new_user_email.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label class="block text-xs uppercase text-zinc-500 font-bold mb-1">"Role"</label>
                                    <select
                                        class="w-full bg-zinc-950 border border-zinc-800 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-orange-500 transition-colors"
                                        prop:value=new_user_role
                                        on:change=move |ev| new_user_role.set(event_target_value(&ev))
                                    >
                                        <option value="beta_tester">"Beta Tester"</option>
                                        <option value="admin">"Admin"</option>
                                    </select>
                                </div>
                                <div>
                                    <button
                                        class="w-full bg-orange-600 hover:bg-orange-500 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                                        on:click=move |_| start_add_user()
                                        disabled=move || loading.get()
                                    >
                                        {if loading.get() { "..." } else { "Add User" }}
                                    </button>
                                </div>
                            </div>
                            <div class="mt-4">
                                <label class="block text-xs uppercase text-zinc-500 font-bold mb-1">"Description (Optional)"</label>
                                <input
                                    type="text"
                                    class="w-full bg-zinc-950 border border-zinc-800 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-orange-500 transition-colors"
                                    placeholder="e.g. VIP Partner"
                                    prop:value=new_user_desc
                                    on:input=move |ev| new_user_desc.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        // Users List
                        <div>
                         {if loading.get() && users.get().is_empty() {
                            view! { <div class="text-center py-10 opacity-50">"Loading users..."</div> }.into_view()
                        } else if users.get().is_empty() {
                             view! { <div class="text-center py-20 text-zinc-500 bg-zinc-900/30 border border-zinc-800/50 rounded-xl">"No allowed users found"</div> }.into_view()
                        } else {
                            view! {
                                <div class="overflow-hidden bg-zinc-900/30 border border-zinc-800 rounded-xl">
                                    <table class="w-full text-left">
                                        <thead class="bg-zinc-900/80 border-b border-zinc-800 text-xs text-zinc-400 uppercase">
                                            <tr>
                                                <th class="px-6 py-4 font-semibold">"Email"</th>
                                                <th class="px-6 py-4 font-semibold">"Role"</th>
                                                <th class="px-6 py-4 font-semibold">"Description"</th>
                                                <th class="px-6 py-4 font-semibold">"Added By"</th>
                                                <th class="px-6 py-4 font-semibold text-right">"Actions"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-zinc-800">
                                            {users.get().into_iter().map(|user| {
                                                let email_clone = user.email.clone();
                                                view! {
                                                    <tr class="hover:bg-zinc-800/20 transition-colors">
                                                        <td class="px-6 py-4 text-white">{user.email}</td>
                                                        <td class="px-6 py-4">
                                                            <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}",
                                                                if user.role == "admin" { "bg-purple-900/30 text-purple-400" }
                                                                else { "bg-blue-900/30 text-blue-400" }
                                                            )}>
                                                                {user.role}
                                                            </span>
                                                        </td>
                                                        <td class="px-6 py-4 text-zinc-400 text-sm">{user.description.unwrap_or_default()}</td>
                                                        <td class="px-6 py-4 text-zinc-500 text-sm">{user.added_by.unwrap_or_else(|| "-".to_string())}</td>
                                                        <td class="px-6 py-4 text-right">
                                                            <button
                                                                class="text-red-500 hover:text-red-300 hover:bg-red-900/20 px-3 py-1 rounded-full transition-colors text-sm"
                                                                on:click=move |_| remove_user(email_clone.clone())
                                                            >
                                                                "Remove"
                                                            </button>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_view()
                        }}
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
