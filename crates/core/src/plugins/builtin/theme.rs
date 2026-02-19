//! Axur Brand Theme Constants
//!
//! Design tokens, typography, and CSS based on Axur.com design system.
//! Visual identity: "Cyber Noir Intelligence" — see `.agent/skills/axur-design-system/SKILL.md`
//! Updated with actual colors from axur.com analysis.

/// Brand colors based on Axur.com analysis
pub mod colors {
    /// Electric Orange - Primary brand color (from axur.com)
    pub const ORANGE: &str = "#FF671F";
    pub const ORANGE_LIGHT: &str = "#FF8A4C";
    pub const ORANGE_DARK: &str = "#E55A1B";
    pub const ORANGE_GLOW: &str = "rgba(255, 103, 31, 0.4)";

    /// Dark theme backgrounds (from axur.com)
    pub const BLACK: &str = "#000000";
    pub const DARK_BG: &str = "#121212";
    pub const CHARCOAL: &str = "#1A1A1A";
    pub const SURFACE: &str = "#27272A";
    pub const SURFACE_LIGHT: &str = "#3F3F46";

    /// Light theme
    pub const OFF_WHITE: &str = "#F2F2F2";
    pub const LIGHT_GRAY: &str = "#E4E4E7";

    /// Accents
    pub const BLUE: &str = "#3B82F6";
    pub const RED_CRITICAL: &str = "#EF4444";
    pub const GREEN_SUCCESS: &str = "#22C55E";
    pub const PURPLE: &str = "#A855F7";
    pub const CYAN: &str = "#06B6D4";

    /// Text
    pub const TEXT_WHITE: &str = "#FFFFFF";
    pub const TEXT_LIGHT: &str = "#F2F2F2";
    pub const TEXT_MUTED: &str = "#A1A1AA";
    pub const TEXT_DARK: &str = "#18181B";
    pub const TEXT_BODY_DARK: &str = "#33475B";
}

/// Typography design tokens — font families, sizes, and weights.
/// See `.agent/skills/axur-design-system/themes/axur-dark-premium.md` for full spec.
pub mod typography {
    /// Font families
    pub const FONT_DISPLAY: &str = "Inter";
    pub const FONT_BODY: &str = "Inter";
    pub const FONT_MONO: &str = "JetBrains Mono, monospace";

    /// Font sizes (rem)
    pub const SIZE_HERO_XL: &str = "7rem";
    pub const SIZE_HERO: &str = "4rem";
    pub const SIZE_H1: &str = "3rem";
    pub const SIZE_H2: &str = "2rem";
    pub const SIZE_H3: &str = "1.25rem";
    pub const SIZE_BODY: &str = "0.875rem";
    pub const SIZE_LABEL: &str = "0.65rem";
    pub const SIZE_CAPTION: &str = "0.75rem";

    /// Font weights
    pub const WEIGHT_LIGHT: u16 = 300;
    pub const WEIGHT_REGULAR: u16 = 400;
    pub const WEIGHT_SEMIBOLD: u16 = 600;
    pub const WEIGHT_BOLD: u16 = 700;
    pub const WEIGHT_EXTRABOLD: u16 = 800;
}

/// CSS classes for brand styling - Updated Axur.com design system
/// NOTE: Styles are now injected globally in `html.rs`. This constant is kept for compatibility but empty.
pub const BRAND_CSS: &str = "";

/// Generate the Axur logo HTML with proper styling
pub fn axur_logo_styled(size: &str) -> String {
    let (text_size, slash_size) = match size {
        "sm" => ("text-lg", "text-xl"),
        "lg" => ("text-3xl", "text-4xl"),
        "xl" => ("text-4xl", "text-5xl"),
        _ => ("text-xl", "text-2xl"),
    };

    format!(
        r#"<div class="flex items-center font-black tracking-wider select-none inter-font">
            <span class="{slash_size} text-brand-primary -mr-1">///</span>
            <span class="{text_size}">AXUR</span>
        </div>"#,
        slash_size = slash_size,
        text_size = text_size
    )
}

/// Generate a stat card with glow effect - large number Axur.com style
pub fn stat_card_glow(value: &str, label: &str, glow: bool) -> String {
    let glow_class = if glow { "text-glow" } else { "" };
    format!(
        r#"<div class="glass-panel p-8 flex flex-col justify-between h-full transition-transform hover:scale-[1.02] duration-300">
            <p class="text-5xl font-light text-white {glow_class} display-text">{value}</p>
            <p class="label-text text-zinc-400 mt-2">{label}</p>
        </div>"#,
        glow_class = glow_class,
        value = value,
        label = label
    )
}

/// Generate a large stat card (5xl value)
pub fn stat_card_large(value: &str, label: &str, sublabel: Option<&str>) -> String {
    let sublabel_html = sublabel
        .map(|s| format!(r#"<p class="text-xs text-zinc-500 mt-2">{}</p>"#, s))
        .unwrap_or_default();
    format!(
        r#"<div class="glass-panel p-10 flex flex-col justify-center h-full relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-tr from-orange-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
            <p class="text-7xl font-light text-white display-text relative z-10">{value}</p>
            <p class="label-text text-orange-500 mt-4 relative z-10">{label}</p>
            {sublabel}
        </div>"#,
        value = value,
        label = label,
        sublabel = sublabel_html
    )
}

/// Generate a pill badge for percentages - Axur.com 200px radius style
pub fn pill_badge(text: &str) -> String {
    format!(
        r#"<span class="pill-badge text-white shadow-lg shadow-orange-900/20">{}</span>"#,
        text
    )
}

/// Generate a ghost pill badge
pub fn pill_badge_ghost(text: &str) -> String {
    format!(
        r#"<span class="pill-badge-ghost backdrop-blur-sm">{}</span>"#,
        text
    )
}

/// Generate section header with badge
pub fn section_header(badge: &str, title: &str) -> String {
    format!(
        r#"<div class="mb-12">
            <span class="section-badge mb-4">{badge}</span>
            <h2 class="section-header text-5xl leading-tight display-text">{title}</h2>
        </div>"#,
        badge = badge,
        title = title
    )
}

/// Generate a feature card with icon circle
pub fn feature_card(icon_svg: &str, title: &str, description: &str) -> String {
    format!(
        r#"<div class="glass-panel p-8 hover:bg-white/5 transition-colors">
            <div class="w-12 h-12 rounded-full bg-orange-500/10 flex items-center justify-center mb-6 text-orange-500 border border-orange-500/20">
                {icon}
            </div>
            <h3 class="text-xl font-bold text-white mb-3 tracking-tight">{title}</h3>
            <p class="text-zinc-400 text-sm leading-relaxed">{desc}</p>
        </div>"#,
        icon = icon_svg,
        title = title,
        desc = description
    )
}

/// Generate a threat category tag
pub fn threat_tag(text: &str, orange: bool) -> String {
    let class = if orange {
        "threat-tag threat-tag-orange border border-orange-500/30"
    } else {
        "threat-tag border border-zinc-700"
    };
    format!(
        r#"<span class="{class}">{text}</span>"#,
        class = class,
        text = text
    )
}

/// Generate a horizontal progress bar (default orange)
pub fn progress_bar(percentage: f64, label: Option<&str>) -> String {
    progress_bar_colored(percentage, label, "orange")
}

/// Generate a colored progress bar
/// colors: "orange", "blue", "green"
pub fn progress_bar_colored(percentage: f64, label: Option<&str>, color: &str) -> String {
    let label_html = label
        .map(|l| {
            format!(
                r#"<span class="label-text text-zinc-400 ml-4">{}</span>"#,
                l
            )
        })
        .unwrap_or_default();

    let (gradient, shadow_color) = match color {
        "blue" => ("from-blue-500 to-blue-400", "rgba(59, 130, 246, 0.3)"),
        "green" => ("from-[#22C55E] to-emerald-400", "rgba(34, 197, 94, 0.3)"),
        _ => (
            "from-[var(--color-primary)] to-[#FF8A4C]",
            "rgba(var(--color-primary-rgb), 0.3)",
        ),
    };

    format!(
        r#"<div class="flex items-center">
            <div class="h-2 rounded-full bg-zinc-800 flex-grow overflow-hidden relative">
                <div class="absolute inset-0 bg-white/5"></div>
                <div class="h-full bg-gradient-to-r {} rounded-full relative" style="width: {}%; box-shadow: 0 0 10px {};"></div>
            </div>
            {}
        </div>"#,
        gradient,
        percentage.min(100.0),
        shadow_color,
        label_html
    )
}

// ============================================
// PHASE 2: PREMIUM COMPONENT FUNCTIONS
// ============================================

/// Hero stat card — massive glowing number with shimmer for highest-impact metrics
pub fn stat_card_hero(value: &str, label: &str, sublabel: Option<&str>) -> String {
    let sublabel_html = sublabel
        .map(|s| format!(r#"<p class="metric-context mt-3">{}</p>"#, s))
        .unwrap_or_default();
    format!(
        r#"<div class="glass-panel-premium p-10 flex flex-col justify-center relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-br from-brand-primary/5 via-transparent to-purple-500/3 opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
            <div class="bg-orb-orange w-40 h-40 -top-10 -right-10"></div>
            <p class="hero-number shimmer-text relative z-10">{value}</p>
            <div class="accent-line w-24 relative z-10"></div>
            <p class="label-text text-brand-primary mt-3 relative z-10">{label}</p>
            {sublabel}
        </div>"#,
        value = value,
        label = label,
        sublabel = sublabel_html,
    )
}

/// Hero XL stat card — even larger number for single dominant metrics (cover, ROI)
pub fn stat_card_hero_xl(value: &str, label: &str) -> String {
    format!(
        r#"<div class="glass-panel-premium p-12 flex flex-col justify-center items-center text-center relative overflow-hidden">
            <div class="bg-orb-orange w-60 h-60 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" style="position:absolute;"></div>
            <p class="hero-number-xl shimmer-text relative z-10">{value}</p>
            <div class="accent-line w-32 mx-auto relative z-10"></div>
            <p class="label-text text-brand-primary mt-4 relative z-10 text-center">{label}</p>
        </div>"#,
        value = value,
        label = label,
    )
}

/// Critical severity stat card — red-pulsing for urgent/dangerous data
pub fn stat_card_critical(value: &str, label: &str, sublabel: Option<&str>) -> String {
    let sublabel_html = sublabel
        .map(|s| format!(r#"<p class="metric-context mt-2">{}</p>"#, s))
        .unwrap_or_default();
    format!(
        r#"<div class="card-critical p-8 flex flex-col justify-center relative overflow-hidden">
            <div class="bg-orb-red w-32 h-32 -top-8 -right-8" style="position:absolute;"></div>
            <p class="hero-number-red relative z-10">{value}</p>
            <p class="label-text text-red-400 mt-3 relative z-10">{label}</p>
            {sublabel}
        </div>"#,
        value = value,
        label = label,
        sublabel = sublabel_html,
    )
}

/// Success stat card — green-accented for positive outcomes (takedowns, savings)
pub fn stat_card_success(value: &str, label: &str, sublabel: Option<&str>) -> String {
    let sublabel_html = sublabel
        .map(|s| format!(r#"<p class="metric-context mt-2">{}</p>"#, s))
        .unwrap_or_default();
    format!(
        r#"<div class="card-success p-8 flex flex-col justify-center relative overflow-hidden">
            <div class="bg-orb-purple w-32 h-32 -bottom-8 -right-8" style="position:absolute;opacity:0.5;"></div>
            <p class="hero-number-green relative z-10">{value}</p>
            <p class="label-text text-green-400 mt-3 relative z-10">{label}</p>
            {sublabel}
        </div>"#,
        value = value,
        label = label,
        sublabel = sublabel_html,
    )
}

/// SVG risk gauge (semi-circle) — visual risk indicator 0-100
pub fn risk_gauge_svg(score: u32, label: &str) -> String {
    let clamped = score.min(100);
    // Arc calculation: semi-circle from 180° to 0° (left to right)
    let angle = 180.0 - (clamped as f64 * 1.8); // 0=left, 100=right
    let rad = angle * std::f64::consts::PI / 180.0;
    let cx = 100.0 + 80.0 * rad.cos();
    let cy = 100.0 - 80.0 * rad.sin();

    // Use loop to find color or just conditions (SVG needs hex or var)
    // We can use var(--color-primary) for the orange case.
    let color = if clamped >= 75 {
        "#EF4444"
    } else if clamped >= 50 {
        "#F59E0B"
    } else if clamped >= 25 {
        "var(--color-primary)"
    } else {
        "#22C55E"
    };

    // For glow, we need rgba... using hardcoded for now or use primary-rgb var if possible but SVG drop-shadow logic is tricky with vars in some renderers.
    // Let's stick to hex for knowns, but use var for the "orange" slot if safe.
    // Actually, filter drop-shadow with var() is well supported.

    let glow_color = if clamped >= 75 {
        "rgba(239,68,68,0.4)"
    } else if clamped >= 50 {
        "rgba(245,158,11,0.4)"
    } else {
        "rgba(var(--color-primary-rgb), 0.4)"
    };

    format!(
        r##"<div class="flex flex-col items-center">
            <svg width="200" height="120" viewBox="0 0 200 120">
                <defs>
                    <linearGradient id="gaugeGrad" x1="0" y1="0" x2="1" y2="0">
                        <stop offset="0%" stop-color="#22C55E"/>
                        <stop offset="40%" stop-color="#F59E0B"/>
                        <stop offset="70%" stop-color="var(--color-primary)"/>
                        <stop offset="100%" stop-color="#EF4444"/>
                    </linearGradient>
                </defs>
                <!-- Background arc -->
                <path d="M 20 100 A 80 80 0 0 1 180 100" fill="none" stroke="#27272A" stroke-width="8" stroke-linecap="round"/>
                <!-- Colored arc -->
                <path d="M 20 100 A 80 80 0 0 1 180 100" fill="none" stroke="url(#gaugeGrad)" stroke-width="8" stroke-linecap="round" stroke-dasharray="251" stroke-dashoffset="{offset}" style="transition: stroke-dashoffset 1s ease;"/>
                <!-- Indicator dot -->
                <circle cx="{cx}" cy="{cy}" r="8" fill="{color}" style="filter: drop-shadow(0 0 8px {glow});">
                    <animate attributeName="r" values="7;9;7" dur="2s" repeatCount="indefinite"/>
                </circle>
                <!-- Score text -->
                <text x="100" y="95" text-anchor="middle" fill="white" font-size="32" font-weight="200" font-family="Inter, sans-serif">{score}</text>
                <text x="100" y="112" text-anchor="middle" fill="#71717A" font-size="9" text-transform="uppercase" letter-spacing="2" font-family="Inter, sans-serif">{label}</text>
            </svg>
        </div>"##,
        offset = 251.0 - (clamped as f64 * 2.51),
        cx = cx,
        cy = cy,
        color = color,
        glow = glow_color,
        score = clamped,
        label = label,
    )
}

/// Enhanced section header with animated accent line
pub fn section_header_premium(badge: &str, title: &str, subtitle: Option<&str>) -> String {
    let subtitle_html = subtitle
        .map(|s| {
            format!(
                r#"<p class="text-zinc-400 text-sm mt-2 max-w-2xl leading-relaxed">{}</p>"#,
                s
            )
        })
        .unwrap_or_default();
    format!(
        r#"<div class="mb-10 relative z-10">
            <span class="section-badge mb-4">{badge}</span>
            <h2 class="section-header text-5xl leading-tight display-text mt-3">{title}</h2>
            <div class="accent-line w-48"></div>
            {subtitle}
        </div>"#,
        badge = badge,
        title = title,
        subtitle = subtitle_html,
    )
}

/// Next chapter teaser element (Zeigarnik effect — open loop)
pub fn next_chapter_teaser(label: &str, title: &str) -> String {
    format!(
        r#"<div class="absolute bottom-8 right-14 z-50">
            <div class="next-chapter flex items-center gap-3 cursor-pointer">
                <div>
                    <p class="text-[10px] text-orange-400 uppercase tracking-widest mb-0.5">{label}</p>
                    <p class="text-sm font-bold text-white">{title} →</p>
                </div>
            </div>
        </div>"#,
        label = label,
        title = title,
    )
}

// ============================================
// PHASE 3: NEW LAYOUT COMPONENTS (v3.0)
// ============================================

/// Before/After comparison card — Anchor Contrast technique (Cialdini)
/// Use for showing improvements, threat reduction, or efficiency gains.
pub fn comparison_card(
    before_label: &str,
    before_value: &str,
    after_label: &str,
    after_value: &str,
) -> String {
    format!(
        r#"<div class="grid grid-cols-2 gap-0 rounded-2xl overflow-hidden border border-white/10">
            <div class="p-8 bg-zinc-900/80 flex flex-col justify-center">
                <p class="label-text text-zinc-500 mb-2">{before_label}</p>
                <p class="text-4xl font-light text-zinc-400 display-text">{before_value}</p>
            </div>
            <div class="p-8 bg-gradient-to-br from-orange-500/10 to-transparent flex flex-col justify-center border-l border-white/10">
                <p class="label-text text-orange-400 mb-2">{after_label}</p>
                <p class="text-4xl font-light text-white display-text text-glow">{after_value}</p>
            </div>
        </div>"#,
        before_label = before_label,
        before_value = before_value,
        after_label = after_label,
        after_value = after_value,
    )
}

/// Action recommendation card — CTA with priority badge and effort indicator.
/// Use for "Sugerencias de Acción" slides (Act 5: Need-Payoff).
pub fn action_card(priority: &str, title: &str, description: &str, effort: &str) -> String {
    let priority_color = match priority.to_lowercase().as_str() {
        "critical" | "crítico" => "bg-red-500/20 text-red-400 border-red-500/30",
        "high" | "alto" => "bg-orange-500/20 text-orange-400 border-orange-500/30",
        "medium" | "medio" => "bg-blue-500/20 text-blue-400 border-blue-500/30",
        _ => "bg-zinc-500/20 text-zinc-400 border-zinc-500/30",
    };
    format!(
        r#"<div class="glass-panel p-6 hover:bg-white/5 transition-all duration-300 hover:scale-[1.01] group">
            <div class="flex items-start justify-between mb-4">
                <span class="inline-block px-3 py-1 rounded-full text-[10px] uppercase tracking-widest font-semibold border {priority_color}">{priority}</span>
                <span class="text-xs text-zinc-500">⏱ {effort}</span>
            </div>
            <h4 class="text-lg font-bold text-white mb-2 tracking-tight group-hover:text-orange-400 transition-colors">{title}</h4>
            <p class="text-sm text-zinc-400 leading-relaxed">{description}</p>
        </div>"#,
        priority_color = priority_color,
        priority = priority,
        effort = effort,
        title = title,
        description = description,
    )
}

/// Timeline entry — vertical timeline element for incident chronology.
/// Use for "Incident Timeline" slides (Act 2: Problem).
pub fn timeline_entry(date: &str, title: &str, description: &str, severity: &str) -> String {
    let (dot_color, line_color) = match severity.to_lowercase().as_str() {
        "critical" | "crítico" => ("bg-red-500 shadow-red-500/50", "border-red-500/30"),
        "high" | "alto" => ("bg-orange-500 shadow-orange-500/50", "border-orange-500/30"),
        "medium" | "medio" => ("bg-blue-500 shadow-blue-500/50", "border-blue-500/30"),
        _ => ("bg-zinc-500 shadow-zinc-500/50", "border-zinc-500/30"),
    };
    format!(
        r#"<div class="flex gap-6 relative">
            <div class="flex flex-col items-center">
                <div class="w-4 h-4 rounded-full {dot_color} shadow-lg flex-shrink-0 mt-1"></div>
                <div class="w-px flex-grow {line_color} border-l-2 border-dashed mt-2"></div>
            </div>
            <div class="pb-8">
                <p class="text-[10px] text-zinc-500 uppercase tracking-widest font-semibold">{date}</p>
                <h4 class="text-base font-bold text-white mt-1">{title}</h4>
                <p class="text-sm text-zinc-400 mt-1 leading-relaxed">{description}</p>
            </div>
        </div>"#,
        dot_color = dot_color,
        line_color = line_color,
        date = date,
        title = title,
        description = description,
    )
}
