//! Threat Radar Slide Plugin — ACT 2: The Call to Adventure (Peak)
//!
//! Narrative Role: The CRISIS VISUALIZATION. The spider chart makes the abstract
//! threat landscape tangible — the hero sees their exposure in every direction.
//!
//! Persuasion: Social Proof (benchmark vs sector) + SPIN Implication (cost of inaction)
//! Design: Radar SVG + dimension cards, single orange polygon as Von Restorff element

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

        // ─── Overall risk assessment ───
        let total_score: u32 = dimensions.iter().map(|d| d.score).sum();
        let avg_score = total_score / dimensions.len().max(1) as u32;
        let (risk_label, risk_color) = if avg_score > 75 {
            ("CRÍTICO", "red")
        } else if avg_score > 50 {
            ("ALTO", "orange")
        } else if avg_score > 25 {
            ("MODERADO", "yellow")
        } else {
            ("BAJO", "green")
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-12 mb-8 relative text-white overflow-hidden">
                <!-- Background -->
                {bg}

                <!-- Header -->
                {header}

                <div class="grid grid-cols-12 gap-6 flex-grow mt-2 items-start">
                    <!-- Column 1: Radar Chart (5 cols) -->
                    <div class="col-span-5 flex flex-col items-center justify-center h-full">
                        <div class="bg-zinc-900/40 p-6 rounded-2xl border border-zinc-800/50 backdrop-blur-sm flex flex-col items-center justify-center w-full hover:border-orange-500/10 transition-all duration-300">
                            {svg}
                        </div>
                    </div>
                    
                    <!-- Column 2: Dimension Cards + Risk Summary (7 cols) -->
                    <div class="col-span-7 flex flex-col gap-3 h-full">
                        <!-- Risk assessment badge -->
                        <div class="flex items-center justify-between bg-{risk_color}-500/5 border border-{risk_color}-500/20 rounded-xl px-4 py-2.5">
                            <div class="flex items-center gap-3">
                                <div class="w-2.5 h-2.5 rounded-full bg-{risk_color}-500 animate-pulse"></div>
                                <span class="text-sm text-zinc-300">Nivel de Riesgo Global</span>
                            </div>
                            <span class="text-sm font-bold text-{risk_color}-400 bg-{risk_color}-500/10 px-3 py-1 rounded-full tracking-wider">{risk_label}</span>
                        </div>

                        <!-- Dimension cards in 2-col grid -->
                        <div class="grid grid-cols-2 gap-2.5 flex-grow">
                            {dimension_cards}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header_premium(
                "ANÁLISIS DE RIESGO",
                if title.is_empty() {
                    "Radar de Amenazas"
                } else {
                    &title
                },
                Some("Análisis multidimensional de los vectores de ataque activos. Los puntos más lejanos del centro requieren acción inmediata.")
            ),
            svg = svg,
            risk_color = risk_color,
            risk_label = risk_label,
            dimension_cards = generate_dimension_cards(&dimensions),
            footer = footer_dark(9, &t.get("footer_text")),
        );

        let html = html
            .replace("#FF671F", "var(--color-primary)")
            .replace("bg-orange-500", "bg-brand-primary")
            .replace("text-orange-400", "text-brand-primary") // text-brand-primary is roughly orange-400/500
            .replace("border-orange-500", "border-brand-primary");

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
            icon: "PH",

            detail: format!("{} detecciones", phishing_count),
        },
        ThreatDimension {
            _id: "credentials",
            label: "Credenciales",
            score: creds_score,
            icon: "CR",

            detail: format!("{} expuestas", data.credentials_total),
        },
        ThreatDimension {
            _id: "leaks",
            label: "Filtraciones",
            score: leaks_score,
            icon: "FL",

            detail: format!("{} secretos", data.secrets_total),
        },
        ThreatDimension {
            _id: "social",
            label: "Redes Sociales",
            score: social_score,
            icon: "RS",

            detail: format!("{} perfiles falsos", social_count),
        },
        ThreatDimension {
            _id: "brand",
            label: "Marca",
            score: brand_score,
            icon: "MR",

            detail: format!("{} abusos", brand_count),
        },
        ThreatDimension {
            _id: "exposure",
            label: "Exposición",
            score: efficiency_score,
            icon: "EX",

            detail: format!("{}% sin resolver", efficiency_score),
        },
    ]
}

/// Normalize a count to a 0-100 score
fn normalize_score(value: u64, max_expected: u64) -> u32 {
    ((value as f64 / max_expected as f64) * 100.0).min(100.0) as u32
}

/// Generate SVG radar chart with larger viewBox for labels
fn generate_radar_svg(dimensions: &[ThreatDimension]) -> String {
    let cx = 200.0;
    let cy = 200.0;
    let radius = 130.0;
    let n = dimensions.len() as f64;

    // Background circles (concentric rings)
    let mut circles = String::new();
    for (i, r) in [0.25, 0.5, 0.75, 1.0].iter().enumerate() {
        circles.push_str(&format!(
            r##"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="#27272a" stroke-width="{}"/>"##,
            cx,
            cy,
            radius * r,
            if i == 3 { "1.5" } else { "0.5" }
        ));
    }

    // Scale labels on the axis (25, 50, 75, 100)
    for (val, r) in [(25, 0.25), (50, 0.5), (75, 0.75)] {
        circles.push_str(&format!(
            r##"<text x="{}" y="{}" fill="#3f3f46" font-size="8" text-anchor="end">{}</text>"##,
            cx - 4.0,
            cy - (radius * r) + 3.0,
            val,
        ));
    }

    // Axis lines and labels
    let mut axes = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let x_end = cx + radius * 1.22 * angle.cos();
        let y_end = cy + radius * 1.22 * angle.sin();
        let x_line = cx + radius * angle.cos();
        let y_line = cy + radius * angle.sin();

        // Axis line
        axes.push_str(&format!(
            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#3f3f46" stroke-width="0.5"/>"##,
            cx, cy, x_line, y_line
        ));

        // Label with score
        let anchor = if x_end < cx - 10.0 {
            "end"
        } else if x_end > cx + 10.0 {
            "start"
        } else {
            "middle"
        };

        // Color based on score
        let label_color = if dim.score > 75 {
            "#EF4444"
        } else if dim.score > 50 {
            "#FF671F"
        } else {
            "#a1a1aa"
        };

        axes.push_str(&format!(
            r##"<text x="{}" y="{}" fill="{}" font-size="11" font-weight="700" text-anchor="{}" dominant-baseline="middle" letter-spacing="0.5">{}</text>"##,
            x_end, y_end, label_color, anchor, dim.label.to_uppercase()
        ));

        // Score under label
        let score_y = y_end + 14.0;
        axes.push_str(&format!(
            r##"<text x="{}" y="{}" fill="{}" font-size="10" font-weight="400" text-anchor="{}" dominant-baseline="middle" opacity="0.7">{} pts</text>"##,
            x_end, score_y, label_color, anchor, dim.score
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

    // Data points (dots on vertices)
    let mut dots = String::new();
    for (i, dim) in dimensions.iter().enumerate() {
        let angle = (i as f64 / n) * 2.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
        let r = (dim.score as f64 / 100.0) * radius;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();

        let dot_color = if dim.score > 75 {
            "#EF4444"
        } else if dim.score > 50 {
            "#FF671F"
        } else {
            "#22C55E"
        };

        dots.push_str(&format!(
            r##"<circle cx="{:.1}" cy="{:.1}" r="5" fill="{}" stroke="#18181b" stroke-width="2" style="filter: url(#glow)"/>"##,
            x, y, dot_color
        ));
    }

    format!(
        r##"<svg viewBox="0 0 400 400" class="w-full h-auto max-h-80" preserveAspectRatio="xMidYMid meet">
  <defs>
    <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
    <linearGradient id="polyGrad" x1="0" y1="0" x2="1" y2="1">
       <stop offset="0%" stop-color="rgba(255, 103, 31, 0.35)"/>
       <stop offset="100%" stop-color="rgba(239, 68, 68, 0.08)"/>
    </linearGradient>
  </defs>
  {circles}
  {axes}
  <polygon points="{points}" fill="url(#polyGrad)" stroke="#FF671F" stroke-width="2" stroke-linejoin="round" style="filter: url(#glow)"/>
  {dots}
</svg>"##,
        circles = circles,
        axes = axes,
        points = points,
        dots = dots,
    )
}

/// Generate dimension detail cards — compact 2-column layout
fn generate_dimension_cards(dimensions: &[ThreatDimension]) -> String {
    dimensions.iter().map(|dim| {
        let (border_color, text_color, bg_color, grad_from, grad_to) = if dim.score > 75 {
            ("border-red-500/30", "text-red-400", "bg-red-500/10", "#EF4444", "#F87171")
        } else if dim.score > 50 {
            ("border-orange-500/30", "text-orange-400", "bg-orange-500/10", "#FF671F", "#FF8A4C")
        } else {
            ("border-emerald-500/20", "text-emerald-400", "bg-emerald-500/10", "#22C55E", "#4ADE80")
        };

        format!(
            r#"<div class="bg-zinc-900/40 p-3 rounded-xl border border-zinc-800/50 backdrop-blur-sm hover:{border_color} transition-all duration-300 hover:scale-[1.02] group/card">
                <div class="flex items-center justify-between mb-1.5">
                    <div class="flex items-center gap-2">
                        <span class="text-xs font-bold {text_color} {bg_color} w-7 h-7 flex items-center justify-center rounded-lg group-hover/card:scale-110 transition-transform">{icon}</span>
                        <h4 class="font-semibold text-white text-sm">{label}</h4>
                    </div>
                    <span class="{text_color} font-bold text-xs {bg_color} px-2 py-0.5 rounded-full font-mono">{score}</span>
                </div>
                <div class="h-1 w-full bg-zinc-800 rounded-full overflow-hidden mb-1">
                    <div class="h-full bg-gradient-to-r from-[{grad_from}] to-[{grad_to}] rounded-full transition-all duration-500" style="width: {pct}%"></div>
                </div>
                <p class="text-xs text-zinc-500">{detail}</p>
            </div>"#,
            icon = dim.icon,
            label = dim.label,
            score = dim.score,
            pct = dim.score.min(100),
            detail = dim.detail,
            border_color = border_color,
            text_color = text_color,
            bg_color = bg_color,
            grad_from = grad_from,
            grad_to = grad_to
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
