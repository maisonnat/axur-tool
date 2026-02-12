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
pub const BRAND_CSS: &str = r#"
<style>
/* Axur Brand Theme - Based on axur.com design analysis */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&display=swap');

:root {
  --axur-orange: #FF5824;
  --axur-orange-glow: rgba(255, 88, 36, 0.4);
  --axur-orange-subtle: rgba(255, 88, 36, 0.15);
  --axur-black: #000000;
  --axur-dark-bg: #121212;
  --axur-charcoal: #1A1A1A;
  --axur-surface: #27272A;
  --axur-surface-light: #3F3F46;
  --axur-text-light: #F2F2F2;
  --axur-text-muted: #A1A1AA;
}

/* Global Font */
.inter-font {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
}

/* Glow Effects */
.glow-orange {
  box-shadow: 0 0 30px var(--axur-orange-glow), 0 0 60px rgba(255, 88, 36, 0.2);
}

.glow-orange-subtle {
  box-shadow: 0 0 20px rgba(255, 88, 36, 0.25);
}

.glow-orange-text {
  text-shadow: 0 0 30px rgba(255, 88, 36, 0.5);
}

.glow-red {
  box-shadow: 0 0 20px rgba(239, 68, 68, 0.4);
}

/* Pill Badge - Axur.com style (200px border radius) */
.pill-badge {
  background: linear-gradient(135deg, #FF5824 0%, #FF7A4D 100%);
  padding: 0.5rem 1.5rem;
  border-radius: 200px;
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 0 15px rgba(255, 88, 36, 0.3);
  font-family: 'Inter', sans-serif;
}

.pill-badge-ghost {
  background: transparent;
  border: 1px solid var(--axur-surface-light);
  padding: 0.5rem 1.5rem;
  border-radius: 200px;
  font-weight: 500;
  color: var(--axur-text-muted);
}

/* Stat Cards with Glow - Axur.com style */
.stat-card {
  background: var(--axur-charcoal);
  border: 1px solid var(--axur-surface-light);
  border-radius: 12px;
  padding: 2rem;
  transition: all 0.3s ease;
  font-family: 'Inter', sans-serif;
}

.stat-card:hover {
  border-color: var(--axur-orange);
  box-shadow: 0 0 25px rgba(255, 88, 36, 0.2);
  transform: translateY(-2px);
}

.stat-card .value {
  font-size: 4rem;
  font-weight: 800;
  color: var(--axur-orange);
  text-shadow: 0 0 40px rgba(255, 88, 36, 0.5);
  line-height: 1;
}

.stat-card .value-lg {
  font-size: 5rem;
}

.stat-card .label {
  font-size: 0.875rem;
  color: var(--axur-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.1em;
  margin-top: 0.75rem;
}

/* Feature Cards - Axur.com style */
.feature-card {
  background: var(--axur-charcoal);
  border: 1px solid var(--axur-surface-light);
  border-radius: 12px;
  padding: 2rem;
  transition: all 0.3s ease;
}

.feature-card:hover {
  border-color: var(--axur-orange);
  box-shadow: 0 0 20px rgba(255, 88, 36, 0.15);
}

.feature-card .icon-circle {
  width: 4rem;
  height: 4rem;
  border-radius: 50%;
  background: var(--axur-orange-subtle);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
}

.feature-card .icon-circle svg {
  width: 2rem;
  height: 2rem;
  color: var(--axur-orange);
}

/* Section Headers - ALL CAPS Bold */
.section-header {
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #FFFFFF;
  font-family: 'Inter', sans-serif;
}

.section-badge {
  background: var(--axur-orange);
  padding: 0.5rem 1.25rem;
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: white;
  display: inline-block;
}

/* Wireframe Background Pattern */
.wireframe-bg {
  background-image: 
    linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
  background-size: 50px 50px;
}

/* Orange Wireframe */
.wireframe-orange {
  background-image: 
    linear-gradient(rgba(255, 88, 36, 0.05) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 88, 36, 0.05) 1px, transparent 1px);
  background-size: 40px 40px;
}

/* Gradient Backgrounds */
.bg-gradient-dark {
  background: linear-gradient(180deg, #121212 0%, #1A1A1A 100%);
}

.bg-gradient-radial {
  background: radial-gradient(ellipse at center top, #27272A 0%, #121212 70%);
}

.bg-gradient-orange {
  background: linear-gradient(135deg, #FF5824 0%, #FF7A4D 100%);
}

/* Bar Chart Styles */
.chart-bar {
  background: linear-gradient(90deg, #FF5824 0%, #FF7A4D 100%);
  border-radius: 4px;
  box-shadow: 0 0 10px rgba(255, 88, 36, 0.3);
}

/* Horizontal Bar for stats */
.h-bar {
  height: 8px;
  border-radius: 4px;
  background: var(--axur-surface);
  overflow: hidden;
}

.h-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #FF5824 0%, #FF7A4D 100%);
  border-radius: 4px;
  transition: width 0.5s ease;
}

/* Critical Alert */
.alert-critical {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-left: 4px solid #EF4444;
  animation: pulse-glow 2s ease-in-out infinite;
}

@keyframes pulse-glow {
  0%, 100% { box-shadow: 0 0 10px rgba(239, 68, 68, 0.2); }
  50% { box-shadow: 0 0 20px rgba(239, 68, 68, 0.4); }
}

/* Logo styling with triple slash */
.axur-logo {
  font-weight: 900;
  letter-spacing: 0.1em;
  font-family: 'Inter', sans-serif;
}

.axur-logo::before {
  /* content: '///'; Removed to prevent duplication with explicit HTML */
  color: var(--axur-orange);
  margin-right: 0.25rem;
}

/* Tag/Badge styles for threat types */
.threat-tag {
  background: var(--axur-surface);
  color: var(--axur-text-light);
  padding: 0.375rem 0.875rem;
  border-radius: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  display: inline-block;
}

.threat-tag-orange {
  background: var(--axur-orange-subtle);
  color: var(--axur-orange);
}

/* Number counter animation */
@keyframes countUp {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-count {
  animation: countUp 0.5s ease-out forwards;
}
</style>
"#;

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
    let glow_class = if glow { "glow-orange-subtle" } else { "" };
    format!(
        r#"<div class="stat-card {glow_class}">
            <p class="value">{value}</p>
            <p class="label">{label}</p>
        </div>"#,
        glow_class = glow_class,
        value = value,
        label = label
    )
}

/// Generate a large stat card (5xl value)
pub fn stat_card_large(value: &str, label: &str, sublabel: Option<&str>) -> String {
    let sublabel_html = sublabel
        .map(|s| format!(r#"<p class="text-xs text-zinc-500 mt-1">{}</p>"#, s))
        .unwrap_or_default();
    format!(
        r#"<div class="stat-card glow-orange-subtle">
            <p class="value value-lg">{value}</p>
            <p class="label">{label}</p>
            {sublabel}
        </div>"#,
        value = value,
        label = label,
        sublabel = sublabel_html
    )
}

/// Generate a pill badge for percentages - Axur.com 200px radius style
pub fn pill_badge(text: &str) -> String {
    format!(r#"<span class="pill-badge text-white">{}</span>"#, text)
}

/// Generate a ghost pill badge
pub fn pill_badge_ghost(text: &str) -> String {
    format!(r#"<span class="pill-badge-ghost">{}</span>"#, text)
}

/// Generate section header with badge
pub fn section_header(badge: &str, title: &str) -> String {
    format!(
        r#"<div class="mb-8">
            <span class="section-badge">{badge}</span>
            <h2 class="section-header text-4xl mt-4">{title}</h2>
        </div>"#,
        badge = badge,
        title = title
    )
}

/// Generate a feature card with icon circle
pub fn feature_card(icon_svg: &str, title: &str, description: &str) -> String {
    format!(
        r#"<div class="feature-card">
            <div class="icon-circle">{icon}</div>
            <h3 class="text-xl font-bold text-white mb-2">{title}</h3>
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
        "threat-tag threat-tag-orange"
    } else {
        "threat-tag"
    };
    format!(
        r#"<span class="{class}">{text}</span>"#,
        class = class,
        text = text
    )
}

/// Generate a horizontal progress bar
pub fn progress_bar(percentage: f64, label: Option<&str>) -> String {
    let label_html = label
        .map(|l| format!(r#"<span class="text-xs text-zinc-400 ml-2">{}</span>"#, l))
        .unwrap_or_default();
    format!(
        r#"<div class="flex items-center">
            <div class="h-bar flex-grow">
                <div class="h-bar-fill" style="width: {pct}%;"></div>
            </div>
            {label}
        </div>"#,
        pct = percentage.min(100.0),
        label = label_html
    )
}
