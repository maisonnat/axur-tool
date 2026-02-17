//! Axur Brand Theme Constants
//!
//! Design tokens and CSS based on Axur.com design system.
//! Updated with actual colors from axur.com analysis.

/// Brand colors based on Axur.com analysis
pub mod colors {
    /// Electric Orange - Primary brand color (from axur.com)
    pub const ORANGE: &str = "#FF5824";
    pub const ORANGE_LIGHT: &str = "#FF7A4D";
    pub const ORANGE_DARK: &str = "#D94000";
    pub const ORANGE_GLOW: &str = "rgba(255, 88, 36, 0.4)";

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
            <span class="{slash_size} text-[#FF5824] -mr-1">///</span>
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
        _ => ("from-[#FF5824] to-[#FF7A4D]", "rgba(255, 88, 36, 0.3)"),
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
