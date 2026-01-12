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

#[component]
pub fn AdminBetaPage() -> impl IntoView {
    let (requests, set_requests) = create_signal(Vec::<BetaReq>::new());
    let (error, set_error) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(true);

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

    let approve = move |id: String| {
        spawn_local(async move {
            let _ = Request::post(&format!("/api/admin/beta/requests/{}/approve", id))
                .send()
                .await;
            // Optimistic update or refresh
            fetch_requests();
        });
    };

    let reject = move |id: String| {
        spawn_local(async move {
            let _ = Request::post(&format!("/api/admin/beta/requests/{}/reject", id))
                .send()
                .await;
            fetch_requests();
        });
    };

    // Load on mount
    create_effect(move |_| {
        fetch_requests();
    });

    view! {
        <div class="p-8 max-w-7xl mx-auto">
            <header class="flex justify-between items-center mb-8">
                <div>
                   <h1 class="text-2xl font-bold">"Beta Requests"</h1>
                   <p class="text-zinc-400">"Manage access to the platform"</p>
                </div>
                <button
                    class="p-2 hover:bg-zinc-800 rounded-lg transition-colors"
                    on:click=move |_| fetch_requests()
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 4.992l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                    </svg>
                </button>
            </header>

            {move || if loading.get() {
                view! { <div class="text-center py-10 opacity-50">"Loading..."</div> }.into_view()
            } else if let Some(err) = error.get() {
                 view! { <div class="text-red-500 bg-red-950/20 p-4 rounded-lg">{err}</div> }.into_view()
            } else if requests.get().is_empty() {
                 view! { <div class="text-center py-10 text-zinc-500 bg-zinc-900/50 rounded-lg">"No pending requests"</div> }.into_view()
            } else {
                view! {
                    <div class="grid gap-4">
                        {requests.get().into_iter().map(|req| {
                            let id_clone = req.id.clone();
                            let id_clone_reject = req.id.clone();
                            view! {
                                <div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 flex justify-between items-center">
                                    <div>
                                        <h3 class="font-bold text-lg">{req.company}</h3>
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
    }
}
