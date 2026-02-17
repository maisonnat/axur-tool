//! Incidents Chart Slide Plugin
//!
//! Displays detections breakdown by type with a detection → incident → resolved funnel.
//! Explains the difference: detections are potential threats, incidents are validated.

use super::helpers::{footer_dark, format_number};
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
        let incident_count = data.total_incidents;
        let resolved_count = data.takedown_resolved;

        // Sort incidents by detections count descending
        let mut sorted_incidents = data.incidents_by_type.clone();
        sorted_incidents.sort_by(|a, b| b.detections.cmp(&a.detections));

        // Calculate max for bar scaling
        let max_detections = sorted_incidents
            .first()
            .map(|i| i.detections)
            .unwrap_or(1)
            .max(1);

        // Generate progress bars for incident types
        let bars_html: String = sorted_incidents
            .iter()
            .map(|item| {
                let percentage = (item.detections as f64 / max_detections as f64) * 100.0;
                crate::plugins::builtin::theme::progress_bar_colored(
                    percentage,
                    Some(&format!(
                        "{} ({})",
                        item.incident_type,
                        format_number(item.detections)
                    )),
                    "orange",
                )
            })
            .collect::<Vec<_>>()
            .join("\n<div class='h-4'></div>\n");

        // Generate funnel cards
        let card_detections = crate::plugins::builtin::theme::stat_card_glow(
            &format_number(total_detections),
            "Detecciones",
            false,
        );
        let card_incidents = crate::plugins::builtin::theme::stat_card_glow(
            &format_number(incident_count),
            "Incidentes",
            true, // Glow for incidents as it's critical
        );
        let card_resolved = crate::plugins::builtin::theme::stat_card_glow(
            &format_number(resolved_count),
            "Resueltos",
            false,
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <!-- Background is handled by Global + Helper {bg} -->
                {bg}

                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-12 gap-8 flex-grow mt-8">
                    <!-- Column 1: Incidents by Type (7 cols) -->
                    <div class="col-span-7 flex flex-col">
                        <div class="glass-panel p-8 h-full backdrop-blur-sm bg-zinc-900/30 border-zinc-800/50 flex flex-col">
                            <h3 class="text-sm font-bold text-orange-500 mb-6 uppercase tracking-[0.2em] flex items-center gap-2">
                                <span class="w-1.5 h-1.5 bg-orange-500 rounded-full shadow-[0_0_5px_#FF5824]"></span>
                                Detecciones por Tipo
                            </h3>
                            <div class="overflow-y-auto pr-2 max-h-[full] space-y-5 flex-grow">
                                {bars}
                            </div>
                        </div>
                    </div>
                    
                    <!-- Column 2: Funnel (5 cols) -->
                    <div class="col-span-5 flex flex-col h-full">
                        <div class="glass-panel p-8 h-full flex flex-col relative overflow-hidden">
                            <!-- Background accent -->
                            <div class="absolute top-0 right-0 w-64 h-64 bg-orange-500/5 rounded-full blur-3xl -mr-16 -mt-16 pointer-events-none"></div>
                            
                            <div class="mb-6">
                                 <h3 class="text-sm font-bold text-white uppercase tracking-[0.2em]">Funnel de Gestión</h3>
                                 <p class="text-zinc-500 text-xs mt-1 font-light tracking-wide">De detección bruta a resolución efectiva</p>
                            </div>
                            
                            <div class="flex-grow flex flex-col justify-center space-y-2 relative z-10">
                                {card_detections}
                                
                                <div class="flex justify-center -my-1 relative z-0">
                                    <div class="h-6 w-px bg-gradient-to-b from-transparent via-zinc-700 to-transparent"></div>
                                </div>
                                
                                {card_incidents}
                                
                                <div class="flex justify-center -my-1 relative z-0">
                                    <div class="h-6 w-px bg-gradient-to-b from-transparent via-zinc-700 to-transparent"></div>
                                </div>
                                
                                {card_resolved}
                            </div>

                            <!-- CO-CREATION FRAME: Draft Actions -->
                            <div class="mt-8 border-t border-zinc-800 pt-6">
                                <h4 class="text-[10px] text-zinc-500 uppercase tracking-widest mb-4 flex items-center justify-between">
                                    Sugerencias de Acción
                                    <span class="bg-zinc-800 text-zinc-400 px-2 py-0.5 rounded text-[9px]">DRAFT</span>
                                </h4>
                                <div class="flex flex-col gap-3">
                                    <div class="flex items-center gap-3 group/action cursor-pointer opacity-80 hover:opacity-100 transition-opacity">
                                        <div class="w-4 h-4 rounded border border-orange-500/50 flex items-center justify-center text-orange-500">
                                            <div class="w-2 h-2 bg-orange-500 rounded-sm opacity-0 group-hover/action:opacity-100 transition-opacity"></div>
                                        </div>
                                        <span class="text-sm text-zinc-300">Aprobar bloqueo automático de Phishing</span>
                                    </div>
                                    <div class="flex items-center gap-3 group/action cursor-pointer opacity-60 hover:opacity-100 transition-opacity">
                                        <div class="w-4 h-4 rounded border border-zinc-700 flex items-center justify-center"></div>
                                        <span class="text-sm text-zinc-400">Revisar política de VIPs</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header(
                "RESULTADOS",
                &t.get("incidents_title")
            ),
            bars = bars_html,
            card_detections = card_detections,
            card_incidents = card_incidents,
            card_resolved = card_resolved,
            footer = footer_dark(10, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "incidents".into(),
            html,
        }]
    }
}
