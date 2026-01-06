//! Metrics Slide Plugin
//!
//! Displays general metrics with glowing stat cards.
//! Styled according to Axur Threat Landscape 2025 brand guidelines.

use super::helpers::format_number;
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

        // Calculate efficiency metrics
        let hours_saved = (data.total_tickets * 15) / 60;
        let analysts_saved = (hours_saved as f64) / 160.0;

        // Get translations
        let title_metrics = t.get("metrics_title");
        let title_tickets = t.get("metrics_total_tickets");

        // Format templated translations
        let eff_hours = t.format(
            "eff_text_hours",
            &[
                ("hours", &hours_saved.to_string()),
                ("analysts", &format!("{:.1}", analysts_saved)),
            ],
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="mb-4">
    <span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RESULTADOS</span>
  </div>
  <h2 class="text-4xl font-black mb-8 uppercase tracking-tight">{title_metrics}</h2>
  
  <!-- Stat Cards Grid -->
  <div class="grid grid-cols-3 gap-6 flex-grow">
    <!-- Main Stat - Glowing -->
    <div class="stat-card glow-orange-subtle col-span-2">
      <p class="text-6xl font-black text-[#FF4B00]" style="text-shadow: 0 0 30px rgba(255,75,0,0.5)">{tickets}</p>
      <p class="text-xl font-bold text-white mt-4 uppercase tracking-wider">{title_tickets}</p>
      <p class="text-zinc-400 mt-2">Amenazas detectadas y procesadas automáticamente</p>
    </div>
    
    <!-- Secondary Stats -->
    <div class="space-y-4">
      <div class="stat-card">
        <p class="text-3xl font-bold text-white">{hours}h</p>
        <p class="text-sm text-zinc-400 uppercase">Horas ahorradas</p>
      </div>
      <div class="stat-card">
        <p class="text-3xl font-bold text-[#22C55E]">{analysts}</p>
        <p class="text-sm text-zinc-400 uppercase">FTE equivalente</p>
      </div>
    </div>
    
    <!-- Efficiency Card -->
    <div class="col-span-3 bg-zinc-900/50 p-6 rounded-lg border border-zinc-800">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-bold text-white mb-2">Eficiencia Operacional</h3>
          <p class="text-zinc-400">{eff_hours}</p>
        </div>
        <div class="pill-badge text-white text-xl font-bold">
          15 min/ticket
        </div>
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
            analysts = format!("{:.1}", analysts_saved),
            eff_hours = eff_hours,
            footer = Self::render_footer(6, t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "metrics".into(),
            html,
        }]
    }
}

impl MetricsSlidePlugin {
    /// Render the dark footer
    fn render_footer(page: u32, footer_text: String) -> String {
        format!(
            r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
<div class="flex items-center font-black tracking-wider select-none text-white h-5">
  <span class="text-[#FF4B00] text-2xl -mr-1">///</span>
  <span class="text-xl">AXUR</span>
</div>
<div class="flex items-center text-xs text-zinc-500">
  <span>{}</span>
  <span class="ml-4">{}</span>
</div>
</footer>"#,
            footer_text, page
        )
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
