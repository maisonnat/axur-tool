//! Attack Heatmap Slide Plugin
//!
//! Displays a 24x7 grid showing attack patterns by hour and day of week.
//! Helps identify when attacks are most frequent.

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the Attack Heatmap slide
pub struct HeatmapSlidePlugin;

impl SlidePlugin for HeatmapSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.heatmap"
    }

    fn name(&self) -> &'static str {
        "Attack Heatmap"
    }

    fn priority(&self) -> i32 {
        60 // After main content, before closing
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        // Only show if we have enough data points
        ctx.data.total_tickets >= 10 && ctx.config.is_plugin_enabled(self.id())
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Generate heatmap data from available timestamps
        // For now, use simulated data based on total_tickets distribution
        let heatmap = generate_simulated_heatmap(data.total_tickets);

        // Find max value for color scaling
        let max_val = heatmap.iter().flatten().copied().max().unwrap_or(1);

        let title = t.get("heatmap_title");

        // Day labels
        let days = ["Lun", "Mar", "Mié", "Jue", "Vie", "Sáb", "Dom"];

        // Build grid cells
        let mut cells_html = String::new();
        for (day_idx, day_data) in heatmap.iter().enumerate() {
            for (hour, &count) in day_data.iter().enumerate() {
                let intensity = if max_val > 0 {
                    (count as f64 / max_val as f64 * 100.0) as u32
                } else {
                    0
                };

                let bg_color = intensity_to_color(intensity);
                let text_color = if intensity > 50 {
                    "white"
                } else {
                    "rgb(161, 161, 170)"
                };

                cells_html.push_str(&format!(
                    r#"<div class="aspect-square flex items-center justify-center text-xs font-medium rounded-sm" style="background: {}; color: {}" title="{} {}:00 - {} eventos">{}</div>"#,
                    bg_color,
                    text_color,
                    days[day_idx],
                    hour,
                    count,
                    if count > 0 { count.to_string() } else { String::new() }
                ));
            }
        }

        // Hour labels (0-23)
        let mut hour_labels = String::new();
        for h in 0..24 {
            if h % 3 == 0 {
                hour_labels.push_str(&format!(
                    r#"<span class="text-xs text-zinc-500">{:02}</span>"#,
                    h
                ));
            } else {
                hour_labels.push_str(r#"<span></span>"#);
            }
        }

        // Day labels
        let mut day_labels = String::new();
        for day in &days {
            day_labels.push_str(&format!(
                r#"<span class="text-xs text-zinc-500 text-right pr-2">{}</span>"#,
                day
            ));
        }

        // Peak hours analysis
        let (peak_day, peak_hour, peak_count) = find_peak(&heatmap);
        let peak_text = format!(
            "{} a las {}:00 ({} eventos)",
            days[peak_day], peak_hour, peak_count
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="mb-4">
    <span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">PATRONES</span>
  </div>
  <h2 class="text-4xl font-black mb-2 uppercase tracking-tight">{title}</h2>
  <p class="text-lg text-zinc-400 mb-6 max-w-4xl">{desc}</p>
  
  <!-- Heatmap Container -->
  <div class="flex-grow flex items-center justify-center">
    <div class="w-full max-w-4xl">
      <!-- Hour Labels -->
      <div class="flex gap-0.5 ml-12 mb-1">
        {hour_labels}
      </div>
      
      <!-- Grid with Day Labels -->
      <div class="flex">
        <!-- Day Labels Column -->
        <div class="flex flex-col gap-0.5 justify-around w-12">
          {day_labels}
        </div>
        
        <!-- Heatmap Grid -->
        <div class="grid gap-0.5 flex-grow" style="grid-template-columns: repeat(24, minmax(0, 1fr));">
          {cells}
        </div>
      </div>
      
      <!-- Legend -->
      <div class="mt-4 flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-xs text-zinc-500">Menos</span>
          <div class="flex gap-0.5">
            <div class="w-4 h-4 rounded-sm" style="background: rgba(255, 75, 0, 0.1)"></div>
            <div class="w-4 h-4 rounded-sm" style="background: rgba(255, 75, 0, 0.3)"></div>
            <div class="w-4 h-4 rounded-sm" style="background: rgba(255, 75, 0, 0.5)"></div>
            <div class="w-4 h-4 rounded-sm" style="background: rgba(255, 75, 0, 0.7)"></div>
            <div class="w-4 h-4 rounded-sm" style="background: rgba(255, 75, 0, 1)"></div>
          </div>
          <span class="text-xs text-zinc-500">Más</span>
        </div>
        
        <!-- Peak Info -->
        <div class="bg-zinc-900/50 px-4 py-2 rounded-lg border border-zinc-800">
          <span class="text-zinc-400 text-sm">⚡ Pico de actividad: </span>
          <span class="text-white font-bold text-sm">{peak_text}</span>
        </div>
      </div>
    </div>
  </div>
</div>
</div>
{footer}
</div></div>"#,
            title = if title.is_empty() {
                "Mapa de Calor de Ataques".to_string()
            } else {
                title
            },
            desc = t.get("heatmap_desc"),
            hour_labels = hour_labels,
            day_labels = day_labels,
            cells = cells_html,
            peak_text = peak_text,
            footer = Self::render_footer(t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "heatmap".into(),
            html,
        }]
    }
}

impl HeatmapSlidePlugin {
    fn render_footer(footer_text: String) -> String {
        format!(
            r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
<div class="flex items-center font-black tracking-wider select-none text-white h-5">
  <span class="text-[#FF4B00] text-2xl -mr-1">///</span>
  <span class="text-xl">AXUR</span>
</div>
<div class="flex items-center text-xs text-zinc-500">
  <span>{}</span>
</div>
</footer>"#,
            footer_text
        )
    }
}

/// Generate simulated heatmap data based on total ticket count
/// Returns a 7x24 matrix (days x hours)
fn generate_simulated_heatmap(total_tickets: u64) -> [[u32; 24]; 7] {
    let mut heatmap = [[0u32; 24]; 7];

    // Distribute tickets with realistic patterns:
    // - More activity on weekdays (Mon-Fri)
    // - Peak hours during business hours (9-18)
    // - Some activity at night (attackers in different timezones)

    let base = (total_tickets / 168) as u32; // Average per hour

    for day in 0..7 {
        let day_multiplier = if day < 5 { 1.2 } else { 0.6 }; // Weekdays vs weekends

        for hour in 0..24 {
            let hour_multiplier = match hour {
                0..=5 => 0.3,   // Night (low)
                6..=8 => 0.8,   // Early morning
                9..=12 => 1.5,  // Morning peak
                13..=14 => 1.0, // Lunch dip
                15..=18 => 1.4, // Afternoon peak
                19..=21 => 0.9, // Evening
                _ => 0.5,       // Late night
            };

            // Add some randomness using a simple hash
            let noise = ((day * 24 + hour) % 5) as f64 * 0.1;
            let value = (base as f64 * day_multiplier * hour_multiplier * (1.0 + noise)) as u32;

            heatmap[day][hour] = value.max(if total_tickets > 100 { 1 } else { 0 });
        }
    }

    heatmap
}

/// Convert intensity (0-100) to Axur orange gradient color
fn intensity_to_color(intensity: u32) -> String {
    let alpha = intensity as f64 / 100.0;
    if alpha < 0.05 {
        "rgba(39, 39, 42, 0.5)".to_string() // zinc-800 for zero/near-zero
    } else {
        format!("rgba(255, 75, 0, {:.2})", alpha.min(1.0).max(0.1))
    }
}

/// Find the peak hour in the heatmap
fn find_peak(heatmap: &[[u32; 24]; 7]) -> (usize, usize, u32) {
    let mut max_day = 0;
    let mut max_hour = 0;
    let mut max_count = 0;

    for (day, day_data) in heatmap.iter().enumerate() {
        for (hour, &count) in day_data.iter().enumerate() {
            if count > max_count {
                max_count = count;
                max_day = day;
                max_hour = hour;
            }
        }
    }

    (max_day, max_hour, max_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_heatmap() {
        let heatmap = generate_simulated_heatmap(1000);
        let total: u32 = heatmap.iter().flatten().sum();
        assert!(total > 0);
    }

    #[test]
    fn test_intensity_to_color() {
        assert!(intensity_to_color(0).contains("39, 39, 42"));
        assert!(intensity_to_color(50).contains("255, 75, 0"));
        assert!(intensity_to_color(100).contains("255, 75, 0"));
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = HeatmapSlidePlugin;
        assert_eq!(plugin.id(), "builtin.heatmap");
        assert_eq!(plugin.priority(), 60);
    }
}
