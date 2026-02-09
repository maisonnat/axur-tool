//! Incidents Chart Slide Plugin
//!
//! Displays incidents breakdown by type with chart.

use super::helpers::footer_dark;
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

        // Use detections as the primary count since incidents often come as 0 from API
        let total_detections: u64 = data.incidents_by_type.iter().map(|i| i.detections).sum();

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

        // ... (data prep is same) ...

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 50% 10%, #FF4B00 0%, transparent 40%);"></div><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RESULTADOS</span></div><h2 class="text-4xl font-black mb-4 uppercase tracking-tight">{title}</h2><p class="text-lg text-zinc-400 mb-8 max-w-4xl">{desc}</p><div class="flex-grow relative"><canvas id="incidentsChart"></canvas></div></div></div>{footer}<script>(function(){{
    function initIncidentsChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initIncidentsChart, 100); return; }}
        const ctx=document.getElementById('incidentsChart').getContext('2d');
        Chart.defaults.color = '#a1a1aa';
        Chart.defaults.borderColor = '#27272a';
        new Chart(ctx,{{
            type:'bar',
            data:{{
                labels:{labels},
                datasets:[
                    {{label:'Detecciones',data:{detections},backgroundColor:'#27272a',hoverBackgroundColor:'#3f3f46',borderRadius:4,borderSkipped:false,barPercentage:0.6}},
                    {{label:'Incidentes',data:{incidents},backgroundColor:'#FF5824',hoverBackgroundColor:'#FF7A4D',borderRadius:4,borderSkipped:false,barPercentage:0.6}}
                ]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{
                    legend:{{position:'top',align:'end',labels:{{color:'#d4d4d8',usePointStyle:true,boxWidth:8}}}},
                    tooltip:{{backgroundColor:'#18181b',titleColor:'#fff',bodyColor:'#d4d4d8',borderColor:'#3f3f46',borderWidth:1,padding:10,displayColors:true}}
                }},
                scales:{{
                    y:{{beginAtZero:true,grid:{{color:'#27272a',drawBorder:false}},ticks:{{font:{{family:"'Inter', sans-serif"}} }} }},
                    x:{{grid:{{display:false}},ticks:{{font:{{family:"'Inter', sans-serif"}} }} }}
                }},
                interaction:{{mode:'index',intersect:false}}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initIncidentsChart(); }} else {{ window.addEventListener('load', initIncidentsChart); }}
}})();</script></div></div>"#,
            title = t.get("incidents_title"),
            desc = t.get("incidents_desc"),
            labels = labels_json,
            detections = det_json,
            incidents = inc_json,
            footer = footer_dark(10, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "incidents".into(),
            html,
        }]
    }
}
