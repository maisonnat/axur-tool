//! Threats Chart Slide Plugin
//!
//! Displays threats distribution by type with Axur.com dark theme aesthetics.

use super::helpers::{footer_dark, format_number};
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
        let max_count = data
            .threats_by_type
            .iter()
            .map(|t| t.count)
            .max()
            .unwrap_or(1);

        // Sort threats by count descending for the chart/list
        let mut sorted_threats = data.threats_by_type.clone();
        sorted_threats.sort_by(|a, b| b.count.cmp(&a.count));

        // Generate semantic progress bars
        let bars_html: String = sorted_threats
            .iter()
            .take(6) // Limit to top 6 to prevent overcrowding
            .map(|threat| {
                let percentage = (threat.count as f64 / max_count as f64) * 100.0;

                // Determine color based on threat type (heuristic)
                let color = if threat.threat_type.to_lowercase().contains("phishing") {
                    "orange" // High severity -> Orange
                } else if threat.threat_type.to_lowercase().contains("brand") {
                    "blue" // Brand -> Blue
                } else {
                    "orange" // Default
                };

                // Use the theme helper
                crate::plugins::builtin::theme::progress_bar_colored(
                    percentage,
                    Some(&format!(
                        "{} ({})",
                        threat.threat_type,
                        format_number(threat.count)
                    )),
                    color,
                )
            })
            .collect();

        // Top Threat for the "Hero" card
        let top_threat = sorted_threats.first();
        let top_name = top_threat.map(|t| t.threat_type.as_str()).unwrap_or("N/A");
        let top_count = top_threat.map(|t| t.count).unwrap_or(0);

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}
                
                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-12 flex-grow mt-4">
                    <!-- Column 1: Context & Key Stats -->
                    <div class="flex flex-col gap-6 h-full"> 
                        <div class="bg-zinc-900/50 p-8 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col flex-grow justify-between">
                            <div>
                                <h3 class="text-xl font-bold text-white mb-6 flex items-center gap-3">
                                    <span class="text-[#FF671F] bg-[#FF671F]/10 p-2 rounded-lg">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                    </span>
                                    AMENAZAS CONFIRMADAS
                                </h3>
                                <p class="text-zinc-400 text-sm leading-relaxed mb-8">
                                    Las siguientes detecciones fueron <strong class="text-white">validadas por IA</strong> como riesgos reales, descartando falsos positivos.
                                </p>
                            </div>
                            
                            <!-- Hero Stat -->
                            <div class="mt-auto">
                                {card_total}
                            </div>
                        </div>

                        <!-- Top Threat Mention -->
                        <div class="p-6 rounded-xl border border-[#FF671F]/20 bg-[#FF671F]/5 flex-none">
                            <h4 class="text-[#FF671F] uppercase text-xs font-bold tracking-wider mb-2">PRINCIPAL VECTOR</h4>
                            <div class="flex items-end gap-3">
                                <span class="text-3xl font-bold text-white">{top_name}</span>
                                <span class="text-xl text-zinc-500 mb-1">{top_count} incidentes</span>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Column 2: Distribution Chart -->
                    <div class="flex flex-col gap-6">
                         <div class="flex items-center gap-3 border-b border-zinc-800 pb-2">
                            <h3 class="text-xl font-bold text-white uppercase tracking-wider">DISTRIBUCIÓN POR TIPO</h3>
                        </div>
                        
                        <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm space-y-5 flex-grow overflow-y-auto">
                            {bars}
                        </div>
                    </div>
                </div>

                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            // LABELED: Naming the value (Validation) not the metric (Threats)
            header = crate::plugins::builtin::theme::section_header(
                "VALIDACIÓN DE RIESGOS",
                "Detecciones Reales vs Ruido"
            ),
            card_total = crate::plugins::builtin::theme::stat_card_glow(
                &format_number(total_threats),
                "AMENAZAS ACTIVAS",
                true
            ),
            top_name = top_name,
            top_count = format_number(top_count),
            bars = bars_html,
            footer = footer_dark(7, &t.get("footer_text")),
        );

        let html = html
            .replace("#FF671F", "var(--color-primary)")
            .replace("bg-[#FF671F]", "bg-brand-primary")
            .replace("text-[#FF671F]", "text-brand-primary")
            .replace("border-[#FF671F]", "border-brand-primary");

        vec![SlideOutput {
            id: "threats".into(),
            html,
        }]
    }
}
