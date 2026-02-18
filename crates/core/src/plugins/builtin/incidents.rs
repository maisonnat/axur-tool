//! Incidents Chart Slide Plugin — ACT 2: The Call to Adventure
//!
//! Narrative Role: Show the THREAT LANDSCAPE. The hero (client) sees the scale of
//! their exposure for the first time. This creates the Situation → Problem transition.
//!
//! Persuasion: Cognitive Emptying (label category) + Reciprocity (free funnel insight)
//! Design: 12-col grid, funnel visualization, single Von Restorff glow on incidents count

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
            .take(7) // Limit to top 7 to prevent overflow
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

        // Premium Cards
        let card_detections = crate::plugins::builtin::theme::stat_card_large(
            &format_number(total_detections),
            "Detecciones Brutas",
            Some("Ruido Filtrado"),
        );

        // Hero XL for the main metric
        // Hero Large (Custom for this slide to fit)
        let card_incidents = format!(
            r#"<div class="glass-panel-premium p-8 flex flex-col justify-center items-center text-center relative overflow-hidden">
                <div class="bg-orb-orange w-40 h-40 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" style="position:absolute;"></div>
                <p class="text-6xl font-light text-white display-text relative z-10 text-glow">{value}</p>
                <div class="accent-line w-24 mx-auto relative z-10"></div>
                <p class="label-text text-[#FF671F] mt-3 relative z-10 text-center">{label}</p>
            </div>"#,
            value = format_number(incident_count),
            label = "Incidentes Validados",
        );

        // Success card for resolution
        let card_resolved = crate::plugins::builtin::theme::stat_card_success(
            &format_number(resolved_count),
            "Amenazas Resueltas",
            Some("Protección Activa"),
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}

                <!-- COGNITIVE EMPTYING: Label the fear category -->
                {header}
                
                <div class="grid grid-cols-12 gap-8 flex-grow mt-0">
                    <!-- Column 1: Incidents by Type (6 cols) -->
                    <div class="col-span-6 flex flex-col">
                        <div class="glass-panel-premium p-6 h-full flex flex-col">
                            <h3 class="text-xs font-bold text-orange-500 mb-4 uppercase tracking-[0.2em] flex items-center gap-2">
                                <span class="w-1.5 h-1.5 bg-orange-500 rounded-full shadow-[0_0_5px_#FF671F]"></span>
                                Detecciones por Tipo
                            </h3>
                            <div class="overflow-y-auto pr-2 max-h-[full] space-y-4 flex-grow custom-scrollbar">
                                {bars}
                            </div>
                        </div>
                    </div>
                    
                    <!-- Column 2: Funnel (6 cols) -->
                    <div class="col-span-6 flex flex-col h-full">
                        <div class="glass-panel p-0 h-full flex flex-col relative overflow-hidden bg-zinc-900/40 border border-zinc-800/50 backdrop-blur-md rounded-2xl">
                            <!-- Header -->
                            <div class="p-4 border-b border-white/5 bg-white/5">
                                 <h3 class="text-xs font-bold text-white uppercase tracking-[0.2em]">Funnel de Gestión</h3>
                                 <p class="text-zinc-500 text-[10px] mt-0.5 font-light tracking-wide">De detección bruta a resolución efectiva</p>
                            </div>
                            
                            <div class="flex-grow flex flex-col justify-center px-6 py-1 space-y-0 relative z-10">
                                <!-- Step 1: Detections (Top of funnel) -->
                                <div class="flex items-center gap-3 opacity-60 scale-90 origin-left">
                                    <div class="w-10 h-10 rounded-full bg-zinc-800 flex items-center justify-center text-zinc-400">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                                    </div>
                                    <div class="flex-grow">
                                        {card_detections}
                                    </div>
                                </div>
                                
                                <!-- Connector -->
                                <div class="h-2 pl-5 border-l border-dashed border-zinc-700 ml-5 my-0"></div>
                                
                                <!-- Step 2: Incidents (Hero) -->
                                <div class="flex items-center gap-3 transform translate-x-4">
                                    <div class="w-12 h-12 rounded-full bg-[#FF671F] flex items-center justify-center text-white shadow-[0_0_15px_rgba(255,103,31,0.5)] animate-pulse">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                    </div>
                                    <div class="flex-grow">
                                        {card_incidents}
                                    </div>
                                </div>
                                
                                <!-- Connector -->
                                <div class="h-2 pl-5 border-l border-dashed border-zinc-700 ml-9 my-0"></div>
                                
                                <!-- Step 3: Resolved (Bottom) -->
                                <div class="flex items-center gap-3 opacity-90 scale-95 origin-left ml-4">
                                    <div class="w-12 h-12 rounded-full bg-green-500/20 border border-green-500/50 flex items-center justify-center text-green-500">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                    </div>
                                    <div class="flex-grow">
                                        {card_resolved}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- ZEIGARNIK EFFECT: Open loop to next section -->
                {next_teaser}

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            // COGNITIVE EMPTYING: Label the threat category
            header = crate::plugins::builtin::theme::section_header_premium(
                "PANORAMA DE AMENAZAS",
                &t.get("incidents_title"),
                Some("De cada detección bruta, solo una fracción confirma ser un riesgo real. Su equipo debe enfocarse en los incidentes validados, no en el ruido.")
            ),
            bars = bars_html,
            card_detections = card_detections,
            card_incidents = card_incidents,
            card_resolved = card_resolved,
            next_teaser = crate::plugins::builtin::theme::next_chapter_teaser(
                "Siguiente Capítulo",
                "Datos Expuestos en la Red"
            ),
            footer = footer_dark(10, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "incidents".into(),
            html,
        }]
    }
}
