//! Threat Radar Slide Plugin
//!
//! Displays a spider/radar chart showing threat dimensions.
//! Visualizes relative severity across multiple threat categories.

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the Threat Radar slide
pub struct RadarSlidePlugin;

impl SlidePlugin for RadarSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.radar"
    }

    fn name(&self) -> &'static str {
        "Threat Radar"
    }

    fn priority(&self) -> i32 {
        65 // After heatmap, before closing slides
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        // Show if we have meaningful data
        ctx.data.total_tickets >= 5 && ctx.config.is_plugin_enabled(self.id())
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Calculate dimension scores (0-100)
        let dimensions = calculate_dimensions(data);
        
        // Generate SVG radar chart
        let svg = generate_radar_svg(&dimensions);
        
        let title = t.get("radar_title");

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="mb-4">
    <span class="bg-[#FF4B00] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">ANÁLISIS</span>
  </div>
  <h2 class="text-4xl font-black mb-2 uppercase tracking-tight">{title}</h2>
  <p class="text-lg text-zinc-400 mb-6 max-w-4xl">{desc}</p>
  
  <!-- Main Content -->
  <div class="flex-grow flex items-center">
    <div class="flex w-full gap-8">
      <!-- Radar Chart -->
      <div class="flex-1 flex items-center justify-center">
        {svg}
      </div>
      
      <!-- Dimension Details -->
      <div class="w-80 space-y-3">
        {dimension_cards}
      </div>
    </div>
  </div>
</div>
</div>
{footer}
</div></div>"#,
            title = if title.is_empty() { "Radar de Amenazas".to_string() } else { title },
            desc = t.get("radar_desc"),
            svg = svg,
            dimension_cards = generate_dimension_cards(&dimensions),
            footer = Self::render_footer(t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "radar".into(),
            html,
        }]
    }
}

impl RadarSlidePlugin {
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

/// Threat dimension with score and metadata
struct ThreatDimension {
    _id: &'static str,
    label: &'static str,
    score: u32,      // 0-100
    icon: &'static str,
    color: &'static str,
    detail: String,
}

/// Calculate threat dimension scores from report data
fn calculate_dimensions(data: &crate::api::report::PocReportData) -> Vec<ThreatDimension> {
    // Phishing score based on threats
    let phishing_count = data.threats_by_type
        .iter()
        .filter(|t| t.threat_type.to_lowercase().contains("phishing"))
        .map(|t| t.count)
        .sum::<u64>();
    let phishing_score = normalize_score(phishing_count, 100);
    
    // Credentials exposure
    let creds_score = normalize_score(data.credentials_total, 500);
    
    // Code leaks
    let leaks_score = normalize_score(data.secrets_total, 50);
    
    // Social media threats
    let social_count = data.threats_by_type
        .iter()
        .filter(|t| t.threat_type.to_lowercase().contains("social") || 
                    t.threat_type.to_lowercase().contains("fake"))
        .map(|t| t.count)
        .sum::<u64>();
    let social_score = normalize_score(social_count, 50);
    
    // Brand abuse
    let brand_count = data.threats_by_type
        .iter()
        .filter(|t| t.threat_type.to_lowercase().contains("brand") ||
                    t.threat_type.to_lowercase().contains("domain"))
        .map(|t| t.count)
        .sum::<u64>();
    let brand_score = normalize_score(brand_count, 30);
    
    // Takedown efficiency (inverted - high takedowns = lower risk)
    let efficiency_score = if data.total_tickets > 0 {
        100 - (data.takedown_resolved * 100 / data.total_tickets.max(1)) as u32
    } else {
        50
    };

    vec![
        ThreatDimension {
            _id: "phishing",
            label: "Phishing",
            score: phishing_score,
            icon: "🎣",
            color: "#EF4444",
            detail: format!("{} detecciones", phishing_count),
        },
        ThreatDimension {
            _id: "credentials",
            label: "Credenciales",
            score: creds_score,
            icon: "🔑",
            color: "#F59E0B",
            detail: format!("{} expuestas", data.credentials_total),
        },
        ThreatDimension {
            _id: "leaks",
            label: "Filtraciones",
            score: leaks_score,
            icon: "📦",
            color: "#8B5CF6",
            detail: format!("{} secretos", data.secrets_total),
        },
        ThreatDimension {
            _id: "social",
            label: "Redes Sociales",
            score: social_score,
            icon: "👤",
            color: "#3B82F6",
            detail: format!("{} perfiles falsos", social_count),
        },
        ThreatDimension {
            _id: "brand",
            label: "Marca",
            score: brand_score,
            icon: "🏷️",
            color: "#10B981",
            detail: format!("{} abusos", brand_count),
        },
        ThreatDimension {
            _id: "exposure",
            label: "Exposición",
            score: efficiency_score,
            icon: "⚠️",
            color: "#FF4B00",
            detail: format!("{}% sin resolver", efficiency_score),
        },
    ]
}

/// Normalize a count to a 0-100 score
fn normalize_score(value: u64, max_expected: u64) -> u32 {
    ((value as f64 / max_expected as f64) * 100.0).min(100.0) as u32
}

/// Generate SVG radar chart
fn generate_radar_svg(dimensions: &[ThreatDimension]) -> String {
    let cx = 150.0;
    let cy = 150.0;
    let radius = 120.0;
    let n = dimensions.len() as f64;
    
    // Background circles
    let mut circles = String::new();
    for r in [0.25, 0.5, 0.75, 1.0] {
        circles.push_str(&format!(
            r##"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="#3f3f46" stroke-width="1" stroke-dasharray="4 4"/>"##,
            cx, cy, radius * r
        ));
    }
    
    // Axis lines and labels
    let mut axes = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let x_end = cx + radius * 1.15 * angle.cos();
        let y_end = cy + radius * 1.15 * angle.sin();
        let x_line = cx + radius * angle.cos();
        let y_line = cy + radius * angle.sin();
        
        // Axis line
        axes.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#52525b" stroke-width="1"/>"##,
            cx, cy, x_line, y_line
        ));
        
        // Label
        let anchor = if x_end < cx - 10.0 { "end" } else if x_end > cx + 10.0 { "start" } else { "middle" };
        axes.push_str(&format!(
            r##"<text x="{}" y="{}" fill="#a1a1aa" font-size="11" text-anchor="{}" dominant-baseline="middle">{}</text>"##,
            x_end, y_end, anchor, dim.label
        ));
    }
    
    // Data polygon
    let mut points = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let r = (dim.score as f64 / 100.0) * radius;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i > 0 { points.push(' '); }
        points.push_str(&format!("{:.1},{:.1}", x, y));
    }
    
    // Data points
    let mut dots = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let r = (dim.score as f64 / 100.0) * radius;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        dots.push_str(&format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="5" fill="{}" stroke="white" stroke-width="2"/>"#,
            x, y, dim.color
        ));
    }
    
    format!(
        r##"<svg viewBox="0 0 300 300" class="w-80 h-80">
  {circles}
  {axes}
  <polygon points="{points}" fill="rgba(255, 75, 0, 0.2)" stroke="#FF4B00" stroke-width="2"/>
  {dots}
</svg>"##,
        circles = circles,
        axes = axes,
        points = points,
        dots = dots,
    )
}

/// Generate dimension detail cards
fn generate_dimension_cards(dimensions: &[ThreatDimension]) -> String {
    dimensions.iter().map(|dim| {
        let bar_width = dim.score.min(100);
        let severity = if dim.score >= 70 { "Alto" } else if dim.score >= 40 { "Medio" } else { "Bajo" };
        let severity_color = if dim.score >= 70 { "#EF4444" } else if dim.score >= 40 { "#F59E0B" } else { "#22C55E" };
        
        format!(
            r#"<div class="bg-zinc-900/50 p-3 rounded-lg border border-zinc-800">
  <div class="flex items-center justify-between mb-2">
    <span class="text-sm font-medium text-white">{} {}</span>
    <span class="text-xs font-bold px-2 py-0.5 rounded" style="background: {}20; color: {}">{}</span>
  </div>
  <div class="h-1.5 bg-zinc-800 rounded-full overflow-hidden">
    <div class="h-full rounded-full" style="width: {}%; background: {}"></div>
  </div>
  <p class="text-xs text-zinc-500 mt-1">{}</p>
</div>"#,
            dim.icon,
            dim.label,
            severity_color,
            severity_color,
            severity,
            bar_width,
            dim.color,
            dim.detail
        )
    }).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_score() {
        assert_eq!(normalize_score(50, 100), 50);
        assert_eq!(normalize_score(200, 100), 100); // Capped
        assert_eq!(normalize_score(0, 100), 0);
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = RadarSlidePlugin;
        assert_eq!(plugin.id(), "builtin.radar");
        assert_eq!(plugin.priority(), 65);
    }
}
