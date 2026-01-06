//! Threats Chart Slide Plugin
//!
//! Displays threats distribution by type.

use super::helpers::footer_light;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct ThreatsSlidePlugin;

impl SlidePlugin for ThreatsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.threats"
    }
    fn name(&self) -> &'static str {
        "Threats by Type"
    }
    fn priority(&self) -> i32 {
        88
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.threats_by_type.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total_threats: u64 = data.threats_by_type.iter().map(|t| t.count).sum();

        // Prepare chart data
        let labels: Vec<String> = data
            .threats_by_type
            .iter()
            .map(|t| t.threat_type.clone())
            .collect();
        let counts: Vec<u64> = data.threats_by_type.iter().map(|t| t.count).collect();

        let labels_json = serde_json::to_string(&labels).unwrap_or_default();
        let counts_json = serde_json::to_string(&counts).unwrap_or_default();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><h2 class="text-4xl font-bold mb-4">{title}</h2><p class="text-lg text-zinc-600 mb-8">{desc}</p><div class="flex-grow relative"><canvas id="threatsChart"></canvas></div></div></div>{footer}<script>(function(){{
    function initThreatsChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initThreatsChart, 100); return; }}
        const ctx=document.getElementById('threatsChart').getContext('2d');
        new Chart(ctx,{{
            type:'doughnut',
            data:{{
                labels:{labels},
                datasets:[{{
                    data:{counts},
                    backgroundColor:['#ea580c','#3b82f6','#10b981','#8b5cf6','#f59e0b','#ef4444','#06b6d4','#84cc16'],
                    borderWidth:0
                }}]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{legend:{{position:'right',labels:{{font:{{size:12}}}}}}}}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initThreatsChart(); }} else {{ window.addEventListener('load', initThreatsChart); }}
}})();</script></div></div>"#,
            title = t.get("threats_title"),
            desc = t.format("threats_desc", &[("total", &total_threats.to_string())]),
            labels = labels_json,
            counts = counts_json,
            footer = footer_light(7, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "threats".into(),
            html,
        }]
    }
}
