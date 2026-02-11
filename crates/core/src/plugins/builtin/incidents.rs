//! Incidents Chart Slide Plugin
//!
//! Displays detections breakdown by type with a detection → incident → resolved funnel.
//! Explains the difference: detections are potential threats, incidents are validated.

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

        // Total detections from the breakdown
        let total_detections: u64 = data.incidents_by_type.iter().map(|i| i.detections).sum();

        // Funnel data: Detections → Incidents → Resolved
        let incident_count = data.total_incidents;
        let resolved_count = data.takedown_resolved;

        // Prepare chart data — single series (detections only, no always-0 incidents)
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

        let labels_json = serde_json::to_string(&labels).unwrap_or_default();
        let det_json = serde_json::to_string(&detections).unwrap_or_default();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 50% 10%, #FF4B00 0%, transparent 40%);"></div><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RESULTADOS</span></div><h2 class="text-4xl font-black mb-2 uppercase tracking-tight">{title}</h2><p class="text-sm text-zinc-400 mb-4 max-w-4xl">{desc}</p><div class="flex-grow relative" style="min-height: 200px;"><canvas id="incidentsChart"></canvas></div>
<!-- Detection Funnel -->
<div class="mt-4 bg-zinc-900/50 p-4 rounded-lg border border-zinc-800">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <span class="text-xs text-zinc-500 uppercase tracking-wider">¿Qué pasa con cada detección?</span>
    </div>
  </div>
  <div class="flex items-center gap-2 mt-3">
    <div class="flex items-center gap-2 bg-zinc-800 px-4 py-2 rounded-lg">
      <span class="text-lg font-bold text-[#FF5824]">🔍 {total_det}</span>
      <span class="text-xs text-zinc-400">Detecciones</span>
    </div>
    <svg class="w-5 h-5 text-zinc-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path></svg>
    <div class="flex items-center gap-2 bg-zinc-800 px-4 py-2 rounded-lg">
      <span class="text-lg font-bold text-amber-400">⚠️ {incidents}</span>
      <span class="text-xs text-zinc-400">Incidentes</span>
    </div>
    <svg class="w-5 h-5 text-zinc-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path></svg>
    <div class="flex items-center gap-2 bg-zinc-800 px-4 py-2 rounded-lg">
      <span class="text-lg font-bold text-green-400">✅ {resolved}</span>
      <span class="text-xs text-zinc-400">Resueltos</span>
    </div>
    <div class="ml-auto text-xs text-zinc-500">
      <span class="text-zinc-300 font-medium">Detección</span> = amenaza potencial · <span class="text-zinc-300 font-medium">Incidente</span> = amenaza validada
    </div>
  </div>
</div>
</div></div>{footer}<script>(function(){{
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
                    {{label:'Detecciones',data:{detections},backgroundColor:'#FF5824',hoverBackgroundColor:'#FF7A4D',borderRadius:4,borderSkipped:false,barPercentage:0.5}}
                ]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{
                    legend:{{display:false}},
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
            total_det = total_detections,
            incidents = incident_count,
            resolved = resolved_count,
            footer = footer_dark(10, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "incidents".into(),
            html,
        }]
    }
}
