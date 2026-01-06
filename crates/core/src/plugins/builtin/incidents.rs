//! Incidents Chart Slide Plugin
//!
//! Displays incidents breakdown by type with chart.

use super::helpers::footer_light;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct IncidentsSlidePlugin;

impl SlidePlugin for IncidentsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.incidents"
    }
    fn name(&self) -> &'static str {
        "Incidents by Type"
    }
    fn priority(&self) -> i32 {
        75
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.incidents_by_type.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total_incidents: u64 = data.incidents_by_type.iter().map(|i| i.incidents).sum();

        // Prepare chart data
        let labels: Vec<String> = data
            .incidents_by_type
            .iter()
            .map(|i| i.incident_type.clone())
            .collect();
        let detections: Vec<u64> = data
            .incidents_by_type
            .iter()
            .map(|i| i.detections)
            .collect();
        let incidents: Vec<u64> = data.incidents_by_type.iter().map(|i| i.incidents).collect();

        let labels_json = serde_json::to_string(&labels).unwrap_or_default();
        let det_json = serde_json::to_string(&detections).unwrap_or_default();
        let inc_json = serde_json::to_string(&incidents).unwrap_or_default();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><h2 class="text-4xl font-bold mb-4">{title}</h2><p class="text-lg text-zinc-600 mb-8">{desc}</p><div class="flex-grow relative"><canvas id="incidentsChart"></canvas></div></div></div>{footer}<script>(function(){{
    function initIncidentsChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initIncidentsChart, 100); return; }}
        const ctx=document.getElementById('incidentsChart').getContext('2d');
        new Chart(ctx,{{
            type:'bar',
            data:{{
                labels:{labels},
                datasets:[
                    {{label:'Detecciones',data:{detections},backgroundColor:'#94a3b8',borderWidth:0}},
                    {{label:'Incidentes',data:{incidents},backgroundColor:'#ea580c',borderWidth:0}}
                ]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{legend:{{position:'bottom'}}}},
                scales:{{y:{{beginAtZero:true}}}}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initIncidentsChart(); }} else {{ window.addEventListener('load', initIncidentsChart); }}
}})();</script></div></div>"#,
            title = t.get("incidents_title"),
            desc = t.format("incidents_desc", &[("total", &total_incidents.to_string())]),
            labels = labels_json,
            detections = det_json,
            incidents = inc_json,
            footer = footer_light(10, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "incidents".into(),
            html,
        }]
    }
}
