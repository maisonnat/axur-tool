//! Metrics Slide Plugin
//!
//! Displays general metrics with operational time/people impact.
//! Designed for LATAM audience: focuses on TIME and FTE, not money.
//! Uses constants from report.rs (MINUTES_PER_TICKET_VALIDATION = 5 min).

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
        let person_days = roi.person_days_saved;

        // Breakdown percentages for progress bars
        let max_hours = hours_saved.max(1.0);
        let val_pct = (roi.hours_saved_validation / max_hours * 100.0).min(100.0);
        let cred_pct = (roi.hours_saved_credentials / max_hours * 100.0).min(100.0);
        let td_pct = (roi.hours_saved_takedowns / max_hours * 100.0).min(100.0);

        // Get translations
        let title_metrics = t.get("metrics_title");
        let title_tickets = t.get("metrics_total_tickets");

        // Format analysts value
        let analysts_display = if analysts_saved >= 1.0 {
            format!("{:.1}", analysts_saved)
        } else {
            format!("{:.0}%", analysts_saved * 100.0)
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <!-- Background is Global -->
                {bg_pattern}

                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-12 flex-grow mb-8 text-center">
                    <!-- ANCHOR: The "Before" State (Chaos/Inerta) -->
                    <div class="p-8 rounded-2xl border border-zinc-800 bg-zinc-900/30 flex flex-col justify-center items-center relative group/chaos">
                        <div class="absolute inset-0 bg-red-500/5 opacity-0 group-hover/chaos:opacity-100 transition-opacity rounded-2xl"></div>
                        <h3 class="text-zinc-500 font-bold tracking-widest text-sm mb-6 uppercase">Manual Process (Legacy)</h3>
                        
                        <div class="mb-8">
                            <p class="text-6xl font-extralight text-zinc-600 group-hover/chaos:text-zinc-500 transition-colors">{total_tickets}</p>
                            <p class="text-sm text-zinc-600 mt-2">Incidents Processed</p>
                        </div>
                        
                        <div class="flex items-center gap-2 text-red-900/50 fill-current">
                             <svg class="w-5 h-5" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>
                             <span class="text-xs font-mono uppercase">High Operations Risk</span>
                        </div>
                    </div>

                    <!-- CONTRAST: The "After" State (Axur/Order) -->
                    <div class="glass-panel p-8 flex flex-col justify-center items-center relative overflow-hidden border-orange-500/30">
                        <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-transparent"></div>
                        <div class="absolute -top-10 -right-10 w-40 h-40 bg-orange-500/20 blur-3xl rounded-full"></div>
                        
                        <h3 class="text-orange-500 font-bold tracking-widest text-sm mb-6 uppercase flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-orange-500 animate-pulse"></span>
                            With Axur Automation
                        </h3>

                        <div class="mb-8 relative z-10">
                            <p class="text-7xl font-thin text-white text-glow display-text">{hours_saved:.0}h</p>
                            <p class="text-sm text-zinc-300 mt-2">Productivity Returned to Team</p>
                        </div>

                        <!-- Gap Selling: FTE Equivalent -->
                        <div class="bg-zinc-900/50 rounded-lg p-4 border border-zinc-700/50 backdrop-blur-md relative z-10 flex items-center gap-4">
                            <div class="bg-green-500/20 p-2 rounded-full text-green-400">
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                            </div>
                            <div class="text-left">
                                <p class="text-xl font-bold text-white leading-none">{analysts_saved:.1} FTE</p>
                                <p class="text-[10px] text-zinc-500 uppercase tracking-wider">Workforce Equivalent</p>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Breakdown Footer (Less emphasized now) -->
                <div class="flex justify-center gap-8 text-sm text-zinc-500 border-t border-zinc-800/50 pt-6">
                    <span class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-orange-500"></span> Validation: <b>{val_hours:.0}h</b></span>
                    <span class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-blue-500"></span> Credentials: <b>{cred_hours:.0}h</b></span>
                    <span class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-emerald-500"></span> Takedowns: <b>{td_hours:.0}h</b></span>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header(
                "EFFICIENCY IMPACT",
                "Why Automation Matters"
            ),
            total_tickets = format_number(data.total_tickets),
            hours_saved = hours_saved,
            analysts_saved = analysts_saved,
            val_hours = roi.hours_saved_validation,
            cred_hours = roi.hours_saved_credentials,
            td_hours = roi.hours_saved_takedowns,
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
