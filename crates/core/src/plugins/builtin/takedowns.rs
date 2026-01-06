//! Takedowns Slide Plugin
//!
//! Displays takedown statistics and status chart.

use super::helpers::footer_light;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the takedowns results slide
pub struct TakedownsSlidePlugin;

impl SlidePlugin for TakedownsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.takedowns"
    }
    fn name(&self) -> &'static str {
        "Takedowns Results"
    }
    fn priority(&self) -> i32 {
        70
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        // Only show if there are takedowns
        let data = ctx.data;
        data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved
            > 0
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let donut_data = vec![
            data.takedown_resolved,
            data.takedown_pending,
            data.takedown_aborted,
            data.takedown_unresolved,
        ];
        let donut_json = serde_json::to_string(&donut_data).unwrap_or_default();

        let donut_labels = vec![
            t.get("takedowns_solved"),
            t.get("takedowns_in_progress"),
            t.get("takedowns_interrupted"),
            t.get("takedowns_not_solved"),
        ];
        let labels_json = serde_json::to_string(&donut_labels).unwrap_or_default();

        let total_takedowns = donut_data.iter().sum::<u64>();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><div class="mb-4"><h2 class="text-4xl font-bold mb-4">{title}</h2></div><div class="grid grid-cols-12 gap-8 flex-grow"><div class="col-span-4 flex flex-col gap-4"><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{req}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_req}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{rate:.1}%</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_rate}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{notify}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_notify}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{uptime}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_uptime}</p></div></div><div class="col-span-8 bg-white p-8 rounded-lg shadow-md border border-zinc-200 flex flex-col"><h3 class="text-xl font-bold text-zinc-700 mb-6">{status_title}</h3><div class="flex-grow relative"><canvas id="takedownChart"></canvas></div></div></div></div></div>{footer}<script>(function(){{
    function initTakedownChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initTakedownChart, 100); return; }}
        const ctx=document.getElementById('takedownChart').getContext('2d');
        new Chart(ctx,{{
            type:'doughnut',
            data:{{
                labels:{labels},
                datasets:[{{
                    data:{data},
                    backgroundColor:['#10b981','#f59e0b','#ef4444','#64748b'],
                    borderWidth:0
                }}]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{
                    legend:{{position:'right',labels:{{font:{{size:14}}}}}}
                }}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initTakedownChart(); }} else {{ window.addEventListener('load', initTakedownChart); }}
}})();</script></div></div>"#,
            title = t.get("takedowns_title"),
            req = total_takedowns,
            lbl_req = t.get("takedowns_requested"),
            rate = data.takedown_success_rate,
            lbl_rate = t.get("takedowns_success_rate"),
            notify = data.takedown_median_time_to_notify,
            lbl_notify = t.get("takedowns_median_notify"),
            uptime = data.takedown_median_uptime,
            lbl_uptime = t.get("takedowns_median_uptime"),
            status_title = t.get("takedowns_status_title"),
            labels = labels_json,
            data = donut_json,
            footer = footer_light(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "takedowns".into(),
            html,
        }]
    }
}
