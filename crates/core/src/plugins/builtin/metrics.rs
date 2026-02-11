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

        // Use ROI metrics for consistent calculations (5 min/ticket from constant)
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

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="mb-4">
    <span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RESULTADOS</span>
  </div>
  <h2 class="text-4xl font-black mb-6 uppercase tracking-tight">{title_metrics}</h2>
  
  <!-- Stat Cards Grid -->
  <div class="grid grid-cols-3 gap-5 mb-6">
    <!-- Main Stat - Tickets -->
    <div class="stat-card glow-orange-subtle">
      <p class="text-5xl font-black text-[#FF4B00]" style="text-shadow: 0 0 30px rgba(255,75,0,0.5)">{tickets}</p>
      <p class="text-sm font-bold text-white mt-3 uppercase tracking-wider">{title_tickets}</p>
    </div>
    
    <!-- Hours Saved -->
    <div class="stat-card">
      <p class="text-5xl font-black text-white">{hours:.0}<span class="text-2xl text-zinc-400 ml-1">h</span></p>
      <p class="text-sm text-zinc-400 uppercase mt-3">Horas Ahorradas</p>
    </div>

    <!-- FTE Equivalent -->
    <div class="stat-card">
      <p class="text-5xl font-black text-[#22C55E]">{analysts}</p>
      <p class="text-sm text-zinc-400 uppercase mt-3">FTE Equivalente</p>
      <p class="text-xs text-zinc-600 mt-1">{days:.0} persona-días</p>
    </div>
  </div>

  <!-- Operational Breakdown -->
  <div class="bg-zinc-900/50 p-5 rounded-lg border border-zinc-800 flex-grow">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-base font-bold text-white">Desglose de Ahorro Operacional</h3>
      <span class="text-xs text-zinc-500 bg-zinc-800 px-3 py-1 rounded-full">5 min/ticket · 2 min/credencial · 2h/takedown</span>
    </div>
    <div class="space-y-3">
      <!-- Validation -->
      <div class="flex items-center gap-3">
        <span class="w-28 text-xs text-zinc-400 truncate">Validación</span>
        <div class="flex-grow h-5 bg-zinc-800 rounded-full overflow-hidden">
          <div class="h-full bg-gradient-to-r from-[#FF5824] to-[#FF7A4D] rounded-full" style="width: {val_pct:.0}%; box-shadow: 0 0 10px rgba(255,88,36,0.3);"></div>
        </div>
        <span class="w-14 text-right text-sm font-bold text-[#FF5824]">{val_hours:.0}h</span>
      </div>
      <!-- Credentials -->
      <div class="flex items-center gap-3">
        <span class="w-28 text-xs text-zinc-400 truncate">Credenciales</span>
        <div class="flex-grow h-5 bg-zinc-800 rounded-full overflow-hidden">
          <div class="h-full bg-gradient-to-r from-blue-500 to-blue-400 rounded-full" style="width: {cred_pct:.0}%;"></div>
        </div>
        <span class="w-14 text-right text-sm font-bold text-blue-400">{cred_hours:.0}h</span>
      </div>
      <!-- Takedowns -->
      <div class="flex items-center gap-3">
        <span class="w-28 text-xs text-zinc-400 truncate">Takedowns</span>
        <div class="flex-grow h-5 bg-zinc-800 rounded-full overflow-hidden">
          <div class="h-full bg-gradient-to-r from-green-500 to-green-400 rounded-full" style="width: {td_pct:.0}%;"></div>
        </div>
        <span class="w-14 text-right text-sm font-bold text-green-400">{td_hours:.0}h</span>
      </div>
    </div>
  </div>
</div>
</div>
{footer}
</div></div>"#,
            title_metrics = title_metrics,
            tickets = format_number(data.total_tickets),
            title_tickets = title_tickets,
            hours = hours_saved,
            analysts = if analysts_saved >= 1.0 {
                format!("{:.1}", analysts_saved)
            } else {
                format!("{:.0}%", analysts_saved * 100.0)
            },
            days = person_days,
            val_pct = val_pct,
            val_hours = roi.hours_saved_validation,
            cred_pct = cred_pct,
            cred_hours = roi.hours_saved_credentials,
            td_pct = td_pct,
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
