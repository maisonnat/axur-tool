//! Metrics Slide Plugin — ACT 4: The Guide Resolves
//!
//! Narrative Role: This is the RELIEF moment. After the crisis peak (Acts 2-3),
//! Axur enters as the guide that gives the hero (client) their power back.
//!
//! Persuasion: Anchor Contrast (Before/After) + Social Proof + Need-Payoff (SPIN)
//! Design: Contrast Grid layout, single Von Restorff glow on hero metric
//!
//! Focuses on TIME and FTE, never money (see content_guidelines.md).

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the General Metrics slide
pub struct MetricsSlidePlugin;

impl SlidePlugin for MetricsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.metrics"
    }

    fn name(&self) -> &'static str {
        "General Metrics"
    }

    fn priority(&self) -> i32 {
        90 // High priority, appears early in report
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;
        let roi = &data.roi_metrics;

        // Use ROI metrics for consistent calculations
        let hours_saved = roi.hours_saved_total;
        let analysts_saved = roi.analysts_equivalent_monthly;

        // Format analysts value
        let analysts_display = if analysts_saved >= 1.0 {
            format!("{:.1}", analysts_saved)
        } else {
            format!("{:.0}%", analysts_saved * 100.0)
        };

        // Premium Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "IMPACTO OPERATIVO",
            "Las Horas que Su Equipo Recuperó",
            Some("Cada incidente procesado manualmente consume tiempo que su equipo podría dedicar a decisiones estratégicas. Esta es la diferencia medida.")
        );

        // Teaser
        let next_teaser = crate::plugins::builtin::theme::next_chapter_teaser(
            "Siguiente Capítulo",
            "Eliminación Activa de Amenazas",
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg_pattern}

                <!-- COGNITIVE EMPTYING: Label the category + provocative benefit-framed title -->
                {header}
                
                <div class="grid grid-cols-2 gap-16 flex-grow mb-6 text-center items-center mt-4">
                    <!-- ANCHOR CONTRAST: The "Before" State (Muted, de-emphasized) -->
                    <!-- SPIN IMPLICATION: Quantify the cost of inaction -->
                    <div class="p-10 rounded-3xl border border-zinc-800 bg-zinc-900/30 flex flex-col justify-center items-center relative group/chaos backdrop-blur-sm grayscale opacity-60 hover:grayscale-0 hover:opacity-100 transition-all duration-700">
                        <div class="absolute inset-0 bg-red-500/5 opacity-0 group-hover/chaos:opacity-100 transition-opacity rounded-3xl"></div>
                        <h3 class="text-zinc-500 font-bold tracking-widest text-sm mb-8 uppercase flex items-center gap-2">
                             <svg class="w-4 h-4 text-zinc-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                             Proceso Manual
                        </h3>
                        
                        <div class="mb-10 relative">
                            <p class="text-7xl font-thin text-zinc-700 group-hover/chaos:text-zinc-400 transition-colors duration-500">{total_tickets}</p>
                            <p class="text-sm text-zinc-600 mt-4 uppercase tracking-wider">Incidentes</p>
                            
                            <!-- Strike-through effect on hover -->
                            <div class="absolute top-1/2 left-0 w-full h-px bg-red-500/50 transform scale-x-0 group-hover/chaos:scale-x-100 transition-transform duration-500 origin-left"></div>
                        </div>
                        
                        <!-- Gap Selling: Cost of inaction -->
                        <div class="flex items-center gap-2 text-red-900/40 fill-current border border-red-900/20 px-4 py-2 rounded-full">
                             <span class="text-xs font-mono uppercase">Fricción Operativa Alta</span>
                        </div>
                    </div>

                    <!-- ANCHOR CONTRAST: The "After" State (Dominant, glowing) -->
                    <!-- VON RESTORFF: This is the ONLY glowing element on the slide -->
                    <div class="glass-panel-premium p-10 flex flex-col justify-center items-center relative overflow-hidden ring-1 ring-orange-500/30 shadow-[0_0_50px_rgba(255,103,31,0.15)] transform scale-105">
                        <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-transparent"></div>
                        <div class="bg-orb-orange w-64 h-64 -top-20 -right-20 opacity-40"></div>
                        
                        <h3 class="text-orange-500 font-bold tracking-widest text-sm mb-8 uppercase flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-orange-500 animate-pulse"></span>
                            Con Automatización Axur
                        </h3>

                        <!-- HERO METRIC: Single glow element (Von Restorff) -->
                        <div class="mb-10 relative z-10">
                            <p class="hero-number shimmer-text">{hours_saved:.0}h</p>
                            <p class="text-sm text-zinc-300 mt-4 uppercase tracking-widest">Productividad Recuperada</p>
                        </div>

                        <!-- NEED-PAYOFF: FTE equivalent (let the data sell itself) -->
                        <div class="bg-zinc-950/80 rounded-2xl p-6 border border-zinc-700/50 backdrop-blur-md relative z-10 flex items-center gap-5 shadow-xl">
                            <div class="bg-green-500/20 p-3 rounded-full text-green-400 border border-green-500/30">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                            </div>
                            <div class="text-left">
                                <p class="text-2xl font-bold text-white leading-none mb-1">{analysts_display} FTEs</p>
                                <p class="text-[10px] text-zinc-400 uppercase tracking-widest">Analistas Liberados</p>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- BREAKDOWN: Supporting detail (de-emphasized, bottom of F-pattern) -->
                <div class="flex justify-center gap-12 text-xs text-zinc-500 border-t border-zinc-800/50 pt-6">
                    <span class="flex items-center gap-2"><span class="w-1.5 h-1.5 rounded-full bg-orange-500"></span> Validación: <b class="text-zinc-300">{val_hours:.0}h</b></span>
                    <span class="flex items-center gap-2"><span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span> Credenciales: <b class="text-zinc-300">{cred_hours:.0}h</b></span>
                    <span class="flex items-center gap-2"><span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span> Takedowns: <b class="text-zinc-300">{td_hours:.0}h</b></span>
                </div>

                <!-- ZEIGARNIK EFFECT: Open loop teaser to next section -->
                {next_teaser}

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            total_tickets = format_number(data.total_tickets),
            hours_saved = hours_saved,
            analysts_display = analysts_display,
            val_hours = roi.hours_saved_validation,
            cred_hours = roi.hours_saved_credentials,
            td_hours = roi.hours_saved_takedowns,
            next_teaser = next_teaser,
            footer = footer_dark(6, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "metrics".into(),
            html,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(100), "100");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = MetricsSlidePlugin;
        assert_eq!(plugin.id(), "builtin.metrics");
        assert_eq!(plugin.name(), "General Metrics");
        assert_eq!(plugin.priority(), 90);
    }
}
