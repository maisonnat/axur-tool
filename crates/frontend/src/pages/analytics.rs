use crate::api::{self, DailyStats};
use crate::components::*;
use crate::{get_ui_dict, AppState, Page};
use leptos::*;

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let current_page = state.current_page;
    let ui_language = state.ui_language;
    let _dict = Signal::derive(move || get_ui_dict(ui_language.get()));

    let stats = create_resource(|| (), |_| async move { api::get_log_stats(Some(7)).await });

    view! {
        <div class="min-h-screen">
            // Header
            <header class="bg-zinc-900 border-b border-zinc-800 px-6 py-4">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                     <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| current_page.set(Page::Dashboard)>
                        <span class="text-orange-500 text-2xl font-black italic">"///"</span>
                        <span class="text-white text-xl font-bold tracking-widest">"AXUR"</span>
                        <span class="text-zinc-500 ml-2">"Web"</span>
                    </div>
                    <div class="flex items-center gap-4">
                         <button
                            class="text-zinc-400 hover:text-white transition-colors"
                            on:click=move |_| current_page.set(Page::Dashboard)
                        >
                            "Generar Reporte"
                        </button>
                        <button
                            class="text-emerald-400 font-bold transition-colors cursor-default"
                        >
                            "📊 Analytics"
                        </button>
                         <button
                            class="text-zinc-400 hover:text-emerald-400 transition-colors"
                            on:click=move |_| current_page.set(Page::Logs)
                        >
                            "📋 Logs"
                        </button>
                    </div>
                </div>
            </header>

            <main class="max-w-7xl mx-auto p-6">
                 <h1 class="text-2xl font-bold text-white mb-6">"System Analytics (7 Days)"</h1>

                 <Suspense fallback=move || view! { <div class="flex justify-center p-8"><Spinner /></div> }>
                    {move || match stats.get() {
                        Some(Ok(data)) => view! {
                             <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                                <KpiCard title="Total Reports" value=data.total_reports.to_string() color="text-orange-500" />
                                <KpiCard title="Total Errors" value=data.total_errors.to_string() color="text-red-500" />
                                <KpiCard title="Success Rate"
                                    value=format!("{:.1}%", calculate_success_rate(data.total_reports, data.total_errors))
                                    color="text-emerald-500"
                                />
                             </div>

                             // Chart Section
                             <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-6">
                                <h3 class="text-lg font-bold text-zinc-300 mb-6">"Activity Overview"</h3>
                                <Chart daily_stats=data.daily_stats />
                             </div>
                        }.into_view(),
                        Some(Err(e)) => view! {
                            <ErrorDisplay error_code="API-ERR" error_message=e />
                        }.into_view(),
                        None => view! {}.into_view()
                    }}
                 </Suspense>
            </main>
        </div>
    }
}

fn calculate_success_rate(reports: usize, errors: usize) -> f64 {
    let total = reports + errors;
    if total == 0 {
        return 100.0;
    }
    (reports as f64 / total as f64) * 100.0
}

#[component]
fn KpiCard(title: &'static str, value: String, color: &'static str) -> impl IntoView {
    view! {
        <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-6">
            <h3 class="text-zinc-500 text-sm font-medium uppercase tracking-wider mb-2">{title}</h3>
            <p class={format!("text-4xl font-bold {}", color)}>{value}</p>
        </div>
    }
}

#[component]
fn Chart(daily_stats: Vec<DailyStats>) -> impl IntoView {
    let max_val = daily_stats
        .iter()
        .map(|d| d.reports.max(d.errors))
        .max()
        .unwrap_or(10) as f64;
    let height = 200.0;

    view! {
        <div class="w-full h-64 flex items-end justify-between gap-2 border-b border-zinc-700 pb-2 relative">
             {daily_stats.into_iter().map(|day| {
                 let report_h = (day.reports as f64 / max_val) * height;
                 let error_h = (day.errors as f64 / max_val) * height;

                 view! {
                     <div class="flex-1 flex flex-col items-center gap-1 group relative">
                         <div class="w-full flex items-end justify-center gap-1 h-52">
                            <div style=format!("height: {}px", report_h) class="w-3 bg-orange-600 rounded-t opacity-80 group-hover:opacity-100 transition-all"></div>
                            <div style=format!("height: {}px", error_h) class="w-3 bg-red-600 rounded-t opacity-80 group-hover:opacity-100 transition-all"></div>
                         </div>
                         <span class="text-xs text-zinc-500 truncate w-full text-center">{day.date}</span>

                         // Tooltip
                         <div class="absolute bottom-full mb-2 hidden group-hover:block bg-black text-white text-xs p-2 rounded z-10 whitespace-nowrap border border-zinc-700 shadow-xl">
                            <p class="font-bold text-zinc-300">{format!("Reports: {}", day.reports)}</p>
                            <p class="font-bold text-red-400">{format!("Errors: {}", day.errors)}</p>
                         </div>
                     </div>
                 }
             }).collect_view()}
        </div>
    }
}
