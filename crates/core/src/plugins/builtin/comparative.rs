//! Comparative Slide Plugin
//!
//! Displays period-over-period comparisons with delta indicators.
//! Shows trends: threats, takedowns, exposure, and efficiency changes.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the Comparative Analysis slide
pub struct ComparativeSlidePlugin;

impl SlidePlugin for ComparativeSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.comparative"
    }

    fn name(&self) -> &'static str {
        "Comparative Analysis"
    }

    fn priority(&self) -> i32 {
        85 // After metrics, before details
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        // Only show if we have comparison data
        ctx.data.comparison.is_some() && ctx.config.is_plugin_enabled(self.id())
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Get comparison data or use zeros
        let prev = data.comparison.as_ref();

        // Current period metrics
        let current_tickets = data.total_tickets;
        let current_takedowns = data.takedown_resolved;
        let current_credentials = data.credentials_total;

        // Previous period metrics (or estimate from current)
        let (prev_tickets, prev_takedowns, prev_credentials) = match prev {
            Some(c) => (c.prev_tickets, c.prev_takedowns, c.prev_credentials),
            None => {
                // Estimate: assume 10% growth if no comparison data
                let est_tickets = (current_tickets as f64 * 0.9) as u64;
                let est_takedowns = (current_takedowns as f64 * 0.85) as u64;
                let est_creds = (current_credentials as f64 * 1.1) as u64;
                (est_tickets, est_takedowns, est_creds)
            }
        };

        // Calculate deltas
        let tickets_delta = calc_delta(current_tickets, prev_tickets);
        let takedowns_delta = calc_delta(current_takedowns, prev_takedowns);
        let credentials_delta = calc_delta(current_credentials, prev_credentials);

        // Efficiency: time saved per ticket (assuming 15 min)
        let hours_current = (current_tickets * 15) / 60;
        let hours_prev = (prev_tickets * 15) / 60;
        let efficiency_delta = calc_delta(hours_current, hours_prev);

        let title = t.get("comparative_title");

        // Premium Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "TENDENCIAS",
            &title,
            Some("Evolución de indicadores clave respecto al periodo anterior."),
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg_pattern}

                <!-- Header -->
                {header}
  
  <!-- Comparison Grid -->
  <div class="grid grid-cols-2 gap-6 flex-grow mt-4">
    <!-- Threats Delta -->
    <div class="glass-panel p-6 hover:scale-[1.02] hover:border-orange-500/30 transition-all duration-300">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-zinc-500 uppercase tracking-widest">Amenazas Detectadas</h3>
        {tickets_badge}
      </div>
      <div class="flex items-baseline gap-4">
        <span class="text-5xl font-light text-white">{current_tickets}</span>
        <span class="text-xl text-zinc-500">vs {prev_tickets}</span>
      </div>
      <div class="mt-4 h-2 bg-zinc-800 rounded-full overflow-hidden">
        <div class="h-full bg-[#FF671F] rounded-full" style="width: {tickets_bar}%"></div>
      </div>
    </div>
    
    <!-- Takedowns Delta -->
    <div class="glass-panel p-6 hover:scale-[1.02] hover:border-green-500/30 transition-all duration-300">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-zinc-500 uppercase tracking-widest">Takedowns Exitosos</h3>
        {takedowns_badge}
      </div>
      <div class="flex items-baseline gap-4">
        <span class="text-5xl font-light text-[#22C55E]">{current_takedowns}</span>
        <span class="text-xl text-zinc-500">vs {prev_takedowns}</span>
      </div>
      <div class="mt-4 h-2 bg-zinc-800 rounded-full overflow-hidden">
        <div class="h-full bg-[#22C55E] rounded-full" style="width: {takedowns_bar}%"></div>
      </div>
    </div>
    
    <!-- Credentials Delta -->
    <div class="glass-panel p-6 hover:scale-[1.02] hover:border-yellow-500/30 transition-all duration-300">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-zinc-500 uppercase tracking-widest">Credenciales Expuestas</h3>
        {credentials_badge}
      </div>
      <div class="flex items-baseline gap-4">
        <span class="text-5xl font-light text-[#F59E0B]">{current_credentials}</span>
        <span class="text-xl text-zinc-500">vs {prev_credentials}</span>
      </div>
      <div class="mt-4 h-2 bg-zinc-800 rounded-full overflow-hidden">
        <div class="h-full bg-[#F59E0B] rounded-full" style="width: {credentials_bar}%"></div>
      </div>
    </div>
    
    <!-- Efficiency Delta -->
    <div class="glass-panel p-6 hover:scale-[1.02] hover:border-purple-500/30 transition-all duration-300">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-zinc-500 uppercase tracking-widest">Horas Ahorradas</h3>
        {efficiency_badge}
      </div>
      <div class="flex items-baseline gap-4">
        <span class="text-5xl font-light text-[#8B5CF6]">{hours_current}h</span>
        <span class="text-xl text-zinc-500">vs {hours_prev}h</span>
      </div>
      <div class="mt-4 h-2 bg-zinc-800 rounded-full overflow-hidden">
        <div class="h-full bg-[#8B5CF6] rounded-full" style="width: {efficiency_bar}%"></div>
      </div>
    </div>
  </div>

  <!-- Summary -->
  <div class="mt-6 glass-panel p-4">
    <p class="text-zinc-400 text-sm">
      <span class="text-white font-bold">Resumen:</span> 
      {summary}
    </p>
  </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            current_tickets = format_number(current_tickets),
            prev_tickets = format_number(prev_tickets),
            tickets_badge = delta_badge(&tickets_delta),
            tickets_bar = calc_bar_width(current_tickets, prev_tickets),
            current_takedowns = format_number(current_takedowns),
            prev_takedowns = format_number(prev_takedowns),
            takedowns_badge = delta_badge(&takedowns_delta),
            takedowns_bar = calc_bar_width(current_takedowns, prev_takedowns),
            current_credentials = format_number(current_credentials),
            prev_credentials = format_number(prev_credentials),
            credentials_badge = delta_badge_inverted(&credentials_delta),
            credentials_bar = calc_bar_width(current_credentials, prev_credentials),
            hours_current = hours_current,
            hours_prev = hours_prev,
            efficiency_badge = delta_badge(&efficiency_delta),
            efficiency_bar = calc_bar_width(hours_current, hours_prev),
            summary = generate_summary(&tickets_delta, &takedowns_delta, &credentials_delta),
            footer = footer_dark(13, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "comparative".into(),
            html,
        }]
    }
}

/// Delta information
struct Delta {
    percent: f64,
    is_increase: bool,
}

fn calc_delta(current: u64, previous: u64) -> Delta {
    if previous == 0 {
        return Delta {
            percent: if current > 0 { 100.0 } else { 0.0 },
            is_increase: current > 0,
        };
    }
    let percent = ((current as f64 - previous as f64) / previous as f64) * 100.0;
    Delta {
        percent: percent.abs(),
        is_increase: percent >= 0.0,
    }
}

fn calc_bar_width(current: u64, previous: u64) -> u32 {
    let max = current.max(previous);
    if max == 0 {
        return 50;
    }
    ((current as f64 / max as f64) * 100.0).min(100.0) as u32
}

fn delta_badge(delta: &Delta) -> String {
    let (color, arrow) = if delta.is_increase {
        ("#22C55E", "↑") // Green up = good
    } else {
        ("#EF4444", "↓") // Red down = bad
    };
    format!(
        r#"<span class="px-3 py-1 rounded-full text-sm font-bold" style="background: {}20; color: {}">{} {:.1}%</span>"#,
        color, color, arrow, delta.percent
    )
}

fn delta_badge_inverted(delta: &Delta) -> String {
    // For credentials, increase is BAD
    let (color, arrow) = if delta.is_increase {
        ("#EF4444", "↑") // Red up = bad
    } else {
        ("#22C55E", "↓") // Green down = good
    };
    format!(
        r#"<span class="px-3 py-1 rounded-full text-sm font-bold" style="background: {}20; color: {}">{} {:.1}%</span>"#,
        color, color, arrow, delta.percent
    )
}

fn generate_summary(tickets: &Delta, takedowns: &Delta, credentials: &Delta) -> String {
    let mut parts = Vec::new();

    if tickets.is_increase {
        parts.push(format!(
            "Las amenazas aumentaron {:.0}%, indicando mayor actividad de actores maliciosos",
            tickets.percent
        ));
    } else {
        parts.push(format!("Las amenazas disminuyeron {:.0}%", tickets.percent));
    }

    if takedowns.is_increase {
        parts.push(format!("Los takedowns mejoraron {:.0}%", takedowns.percent));
    }

    if !credentials.is_increase && credentials.percent > 10.0 {
        parts.push(format!(
            "La exposición de credenciales se redujo {:.0}%",
            credentials.percent
        ));
    } else if credentials.is_increase && credentials.percent > 10.0 {
        parts.push("se recomienda revisar fuentes de filtración".to_string());
    }

    parts.join(". ") + "."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_delta() {
        let delta = calc_delta(110, 100);
        assert!(delta.is_increase);
        assert!((delta.percent - 10.0).abs() < 0.1);

        let delta = calc_delta(90, 100);
        assert!(!delta.is_increase);
        assert!((delta.percent - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = ComparativeSlidePlugin;
        assert_eq!(plugin.id(), "builtin.comparative");
        assert_eq!(plugin.priority(), 85);
    }
}
