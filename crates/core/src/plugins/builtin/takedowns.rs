//! Takedowns Slide Plugin
//!
//! Displays takedown statistics and status with Axur.com dark theme aesthetics.

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

        // Translations
        let title = t.get("takedowns_title");

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}
                
                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-12 flex-grow mt-4">
                    <!-- Left Column: KPIs (Time & Efficiency) -->
                    <div class="flex flex-col gap-6">
                        <h3 class="text-xl font-bold text-white mb-2 border-b border-zinc-800 pb-2">EFICIENCIA OPERATIVA</h3>
                        
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
                        <h3 class="text-xl font-bold text-white mb-2 border-b border-zinc-800 pb-2">{status_title}</h3>
                        
                        <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm space-y-6">
                            <!-- Resolved (Green) -->
                            {progress_resolved}
                            
                            <!-- Pending (Blue/Orange -> We use Orange for 'In Progress') -->
                            {progress_pending}
                            
                            <!-- Aborted (Gray/Red -> We use generic or specific color if avail? Using Blue/Generic for now or Orange) -->
                            <!-- Note: theme::progress_bar_colored supports orange, blue, green. -->
                            <!-- Pending -> Orange (Attention) -->
                            <!-- Aborted -> Blue (Neutral/Info) -->
                            {progress_aborted}
                            
                            <!-- Unresolved (Blue) -->
                            {progress_unresolved}
                        </div>

                        <!-- Summary Note -->
                        <div class="p-4 rounded-lg bg-emerald-900/10 border border-emerald-500/20 text-emerald-400 text-sm font-medium text-center">
                            {rate:.1}% Tasa de Éxito Global
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            // FRICTION MINIMIZATION: Emphasize "Elimination" over "Takedowns"
            header = crate::plugins::builtin::theme::section_header(
                "ELIMINACIÓN DE RIESGOS",
                "Protección Activa de Marca"
            ),
            // Left Column
            card_total = crate::plugins::builtin::theme::stat_card_glow(
                &format_number(total_takedowns),
                &t.get("takedowns_requested"),
                true
            ),
            card_success = crate::plugins::builtin::theme::stat_card_large(
                &format!("{:.1}%", data.takedown_success_rate),
                "Tasa de Éxito",
                // QUICK WIN: Green badge for immediate impact
                Some("<span class='text-green-400 font-bold'>Day 1 Impact</span>")
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
                "blue"
            ),
            rate = data.takedown_success_rate,
            footer = footer_dark(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "takedowns".into(),
            html,
        }]
    }
}
