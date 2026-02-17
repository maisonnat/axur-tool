//! Threat Radar Slide Plugin
//!
//! Displays a spider/radar chart showing threat dimensions.
//! Visualizes relative severity across multiple threat categories.

use super::helpers::footer_dark;

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
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <!-- Background -->
                {bg}

                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-16 flex-grow mt-8 items-center">
                    <!-- Column 1: Radar Chart -->
                    <div class="glass-panel p-8 rounded-2xl flex flex-col items-center justify-center relative aspect-square backdrop-blur-md bg-zinc-900/40">
                        <div class="absolute inset-0 bg-gradient-to-tr from-orange-500/5 to-transparent rounded-2xl pointer-events-none"></div>
                        {svg}
                        <p class="text-zinc-400 text-xs mt-6 text-center font-light tracking-wide max-w-xs">
                            <strong class="text-orange-500">Vulnerabilidad Expuesta:</strong><br>
                            Su superficie de ataque actual es <span class="text-white font-bold">3x mayor</span> al promedio del sector financiero.
                        </p>
                    </div>
                    
                    <!-- Column 2: Threat Vectors -->
                    <div class="flex flex-col h-full justify-center">
                        <div class="glass-panel p-8 bg-zinc-900/20 border-zinc-800/50">
                            <div class="flex items-center gap-3 mb-6">
                                <span class="text-[#FF5824] bg-[#FF5824]/10 p-2 rounded-lg">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 002 2h2a2 2 0 002-2z"></path></svg>
                                </span>
                                <h3 class="text-lg font-bold text-white uppercase tracking-widest">Vectores de Ataque</h3>
                            </div>
                            
                            <div class="space-y-3 overflow-y-auto max-h-[400px] pr-2 custom-scrollbar">
                                {dimension_cards}
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header(
                "ANÁLISIS DE RIESGO",
                if title.is_empty() {
                    "Radar de Amenazas"
                } else {
                    &title
                }
            ),
            svg = svg,
            dimension_cards = generate_dimension_cards(&dimensions),
            footer = footer_dark(9, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "radar".into(),
            html,
        }]
    }
}

/// Threat dimension with score and metadata
struct ThreatDimension {
    _id: &'static str,
    label: &'static str,
    score: u32, // 0-100
    icon: &'static str,

    detail: String,
}

/// Calculate threat dimension scores from report data
fn calculate_dimensions(data: &crate::api::report::PocReportData) -> Vec<ThreatDimension> {
    // Phishing score based on threats
    let phishing_count = data
        .threats_by_type
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
    let social_count = data
        .threats_by_type
        .iter()
        .filter(|t| {
            t.threat_type.to_lowercase().contains("social")
                || t.threat_type.to_lowercase().contains("fake")
        })
        .map(|t| t.count)
        .sum::<u64>();
    let social_score = normalize_score(social_count, 50);

    // Brand abuse
    let brand_count = data
        .threats_by_type
        .iter()
        .filter(|t| {
            t.threat_type.to_lowercase().contains("brand")
                || t.threat_type.to_lowercase().contains("domain")
        })
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

            detail: format!("{} detecciones", phishing_count),
        },
        ThreatDimension {
            _id: "credentials",
            label: "Credenciales",
            score: creds_score,
            icon: "🔑",

            detail: format!("{} expuestas", data.credentials_total),
        },
        ThreatDimension {
            _id: "leaks",
            label: "Filtraciones",
            score: leaks_score,
            icon: "📦",

            detail: format!("{} secretos", data.secrets_total),
        },
        ThreatDimension {
            _id: "social",
            label: "Redes Sociales",
            score: social_score,
            icon: "👤",

            detail: format!("{} perfiles falsos", social_count),
        },
        ThreatDimension {
            _id: "brand",
            label: "Marca",
            score: brand_score,
            icon: "🏷️",

            detail: format!("{} abusos", brand_count),
        },
        ThreatDimension {
            _id: "exposure",
            label: "Exposición",
            score: efficiency_score,
            icon: "⚠️",

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
            r##"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="#27272a" stroke-width="1" stroke-dasharray="4 4"/>"##,
            cx, cy, radius * r
        ));
    }

    // Axis lines and labels
    let mut axes = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let x_end = cx + radius * 1.25 * angle.cos();
        let y_end = cy + radius * 1.25 * angle.sin();
        let x_line = cx + radius * angle.cos();
        let y_line = cy + radius * angle.sin();

        // Axis line
        axes.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#3f3f46" stroke-width="1"/>"##,
            cx, cy, x_line, y_line
        ));

        // Label
        let anchor = if x_end < cx - 10.0 {
            "end"
        } else if x_end > cx + 10.0 {
            "start"
        } else {
            "middle"
        };
        axes.push_str(&format!(
            r##"<text x="{}" y="{}" fill="#a1a1aa" font-size="10" font-weight="bold" text-anchor="{}" dominant-baseline="middle">{}</text>"##,
            x_end, y_end, anchor, dim.label.to_uppercase()
        ));
    }

    // Data polygon
    let mut points = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let r = (dim.score as f64 / 100.0) * radius;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i > 0 {
            points.push(' ');
        }
        points.push_str(&format!("{:.1},{:.1}", x, y));
    }

    // Data points
    let mut dots = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let r = (dim.score as f64 / 100.0) * radius;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();

        // Use theme colors for dots
        let dot_color = if dim.score > 60 { "#FF5824" } else { "#3b82f6" };

        dots.push_str(&format!(
            r##"<circle cx="{:.1}" cy="{:.1}" r="4" fill="{}" stroke="#18181b" stroke-width="2"/>"##,
            x, y, dot_color
        ));
    }

    format!(
        r##"<svg viewBox="0 0 300 300" class="w-96 h-96 filter drop-shadow-[0_0_10px_rgba(255,88,36,0.2)]">
  {circles}
  {axes}
  <polygon points="{points}" fill="rgba(255, 88, 36, 0.2)" stroke="#FF5824" stroke-width="2"/>
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
        format!(
            r#"<div class="bg-zinc-900/50 p-4 rounded-xl border border-zinc-800 transition-all hover:border-[#FF5824]/50 hover:shadow-[0_0_15px_rgba(255,88,36,0.1)]">
                <div class="flex items-center justify-between mb-2">
                    <div class="flex items-center gap-3">
                        <span class="text-2xl">{icon}</span>
                        <h4 class="font-bold text-white text-sm">{label}</h4>
                    </div>
                    <span class="text-[#FF5824] font-bold text-sm bg-[#FF5824]/10 px-2 py-1 rounded">{score} pts</span>
                </div>
                <div class="h-1.5 w-full bg-zinc-800 rounded-full overflow-hidden mb-2">
                    <div class="h-full bg-gradient-to-r from-[#FF5824] to-[#FF7A4D]" style="width: {pct}%"></div>
                </div>
                <p class="text-xs text-zinc-500">{detail}</p>
            </div>"#,
            icon = dim.icon,
            label = dim.label,
            score = dim.score,
            pct = dim.score.min(100),
            detail = dim.detail
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
