//! Takedowns Slide Plugin — ACT 4: The Guide Resolves (Action)
//!
//! Narrative Role: After showing the impact/savings (metrics.rs), this slide
//! proves Axur is ACTIVELY eliminating threats. The hero sees the guide in action.
//!
//! Persuasion: Social Proof (success rate) + Quick Win (Day 1 Impact badge)
//! Design: Dual-column with KPIs + progress bars, green success emphasis

use super::helpers::{footer_dark, format_number};
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

        let total_takedowns = data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved;

        // Calculate percentages
        let calc_pct = |val: u64| {
            if total_takedowns > 0 {
                (val as f64 / total_takedowns as f64) * 100.0
            } else {
                0.0
            }
        };

        let resolved_pct = calc_pct(data.takedown_resolved);
        let pending_pct = calc_pct(data.takedown_pending);
        let aborted_pct = calc_pct(data.takedown_aborted);
        let unresolved_pct = calc_pct(data.takedown_unresolved);

        // Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "ELIMINACIÓN DE RIESGOS",
            "Protección Activa de Marca",
            Some("Axur eliminó amenazas activas en nombre de su organización. Cada takedown representa un riesgo que ya no existe.")
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}
                
                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-12 flex-grow mt-4">
                    <!-- Left Column: KPIs (Time & Efficiency) -->
                    <div class="flex flex-col gap-6">
                        <h3 class="text-xs font-bold text-zinc-500 mb-2 uppercase tracking-widest border-b border-zinc-900 pb-2">Eficiencia Operativa</h3>
                        
                        <!-- Total Volume -->
                        {card_total}
                        
                        <!-- Efficiency Metrics -->
                        <div class="grid grid-cols-2 gap-4">
                            {card_success}
                            {card_uptime}
                        </div>
                        
                        <!-- Notify Time -->
                        {card_notify}
                    </div>
                    
                    <!-- Right Column: Status Breakdown -->
                    <div class="flex flex-col gap-6">
                        <h3 class="text-xs font-bold text-zinc-500 mb-2 uppercase tracking-widest border-b border-zinc-900 pb-2">{status_title}</h3>
                        
                        <div class="bg-zinc-900/50 p-8 rounded-3xl border border-zinc-800 backdrop-blur-sm space-y-8 h-full flex flex-col justify-center">
                            <!-- Resolved (Green) -->
                            {progress_resolved}
                            
                            <!-- Pending (Orange) -->
                            {progress_pending}
                            
                            <!-- Aborted (Blue) -->
                            {progress_aborted}
                            
                            <!-- Unresolved (Gray) -->
                            {progress_unresolved}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            // Left Column
            card_total = crate::plugins::builtin::theme::stat_card_hero(
                &format_number(total_takedowns),
                &t.get("takedowns_requested"),
                None // Added missing sublabel
            ),
            card_success = crate::plugins::builtin::theme::stat_card_success(
                &format!("{:.1}%", data.takedown_success_rate),
                "Tasa de Éxito",
                None // Added missing sublabel
            ),
            card_uptime = crate::plugins::builtin::theme::stat_card_large(
                &data.takedown_median_uptime,
                "Tiempo de Vida",
                None
            ),
            card_notify = crate::plugins::builtin::theme::stat_card_large(
                &data.takedown_median_time_to_notify,
                "Reacción Automática",
                Some("Zero-Touch Response")
            ),
            // Right Column
            status_title = "ESTADO DE GESTIÓN",
            progress_resolved = crate::plugins::builtin::theme::progress_bar_colored(
                resolved_pct,
                Some(&format!("{} ({})", "Eliminado", data.takedown_resolved)),
                "green"
            ),
            progress_pending = crate::plugins::builtin::theme::progress_bar_colored(
                pending_pct,
                Some(&format!(
                    "{} ({})",
                    t.get("takedowns_in_progress"),
                    data.takedown_pending
                )),
                "orange"
            ),
            progress_aborted = crate::plugins::builtin::theme::progress_bar_colored(
                aborted_pct,
                Some(&format!(
                    "{} ({})",
                    t.get("takedowns_interrupted"),
                    data.takedown_aborted
                )),
                "blue"
            ),
            progress_unresolved = crate::plugins::builtin::theme::progress_bar_colored(
                unresolved_pct,
                Some(&format!(
                    "{} ({})",
                    t.get("takedowns_not_solved"),
                    data.takedown_unresolved
                )),
                "gray"
            ),
            footer = footer_dark(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "takedowns".into(),
            html,
        }]
    }
}
