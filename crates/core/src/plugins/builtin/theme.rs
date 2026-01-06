//! Axur Brand Theme Constants
//!
//! Design tokens and CSS based on Axur Threat Landscape 2025.

/// Brand colors based on Threat Landscape 2025
pub mod colors {
    /// Electric Orange - Primary brand color
    pub const ORANGE: &str = "#FF4B00";
    pub const ORANGE_LIGHT: &str = "#FF6B1A";
    pub const ORANGE_DARK: &str = "#D94000";

    /// Dark theme backgrounds
    pub const BLACK: &str = "#000000";
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

    /// Text
    pub const TEXT_WHITE: &str = "#FFFFFF";
    pub const TEXT_MUTED: &str = "#A1A1AA";
    pub const TEXT_DARK: &str = "#18181B";
}

/// CSS classes for brand styling
pub const BRAND_CSS: &str = r#"
<style>
/* Axur Brand Theme - Based on Threat Landscape 2025 */
:root {
  --axur-orange: #FF4B00;
  --axur-orange-glow: rgba(255, 75, 0, 0.4);
  --axur-black: #000000;
  --axur-charcoal: #1A1A1A;
  --axur-surface: #27272A;
}

/* Glow Effects */
.glow-orange {
  box-shadow: 0 0 30px var(--axur-orange-glow), 0 0 60px rgba(255, 75, 0, 0.2);
}

.glow-orange-subtle {
  box-shadow: 0 0 20px rgba(255, 75, 0, 0.25);
}

.glow-red {
  box-shadow: 0 0 20px rgba(239, 68, 68, 0.4);
}

/* Pill Badge - for stats like +65% */
.pill-badge {
  background: linear-gradient(135deg, #FF4B00 0%, #FF6B1A 100%);
  padding: 0.5rem 1.25rem;
  border-radius: 9999px;
  font-weight: 700;
  display: inline-block;
  box-shadow: 0 0 15px rgba(255, 75, 0, 0.3);
}

/* Stat Cards with Glow */
.stat-card {
  background: #1A1A1A;
  border: 1px solid #3F3F46;
  border-radius: 0.75rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.stat-card:hover {
  border-color: #FF4B00;
  box-shadow: 0 0 20px rgba(255, 75, 0, 0.2);
}

.stat-card .value {
  font-size: 3rem;
  font-weight: 800;
  color: #FF4B00;
  text-shadow: 0 0 30px rgba(255, 75, 0, 0.5);
}

/* Section Headers - ALL CAPS Bold */
.section-header {
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #FFFFFF;
}

.section-badge {
  background: #FF4B00;
  padding: 0.375rem 1rem;
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

/* Wireframe Background Pattern */
.wireframe-bg {
  background-image: 
    linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
  background-size: 50px 50px;
}

/* Gradient Backgrounds */
.bg-gradient-dark {
  background: linear-gradient(180deg, #000000 0%, #1A1A1A 100%);
}

.bg-gradient-radial {
  background: radial-gradient(ellipse at center top, #27272A 0%, #000000 70%);
}

/* Bar Chart Styles */
.chart-bar {
  background: linear-gradient(90deg, #FF4B00 0%, #FF6B1A 100%);
  border-radius: 0.25rem;
  box-shadow: 0 0 10px rgba(255, 75, 0, 0.3);
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

/* Logo styling with double slash */
.axur-logo {
  font-weight: 900;
  letter-spacing: 0.1em;
}

.axur-logo::before {
  content: '///';
  color: #FF4B00;
  margin-right: 0.25rem;
}
</style>
"#;

/// Generate the Axur logo HTML with proper styling
pub fn axur_logo_styled(size: &str) -> String {
    let (text_size, slash_size) = match size {
        "sm" => ("text-lg", "text-xl"),
        "lg" => ("text-3xl", "text-4xl"),
        _ => ("text-xl", "text-2xl"),
    };

    format!(
        r#"<div class="flex items-center font-black tracking-wider select-none">
            <span class="{slash_size} text-[#FF4B00] -mr-1">///</span>
            <span class="{text_size}">AXUR</span>
        </div>"#,
        slash_size = slash_size,
        text_size = text_size
    )
}

/// Generate a stat card with glow effect
pub fn stat_card_glow(value: &str, label: &str, glow: bool) -> String {
    let glow_class = if glow { "glow-orange-subtle" } else { "" };
    format!(
        r#"<div class="stat-card {glow_class}">
            <p class="value">{value}</p>
            <p class="text-sm text-zinc-400 uppercase tracking-wider mt-2">{label}</p>
        </div>"#,
        glow_class = glow_class,
        value = value,
        label = label
    )
}

/// Generate a pill badge for percentages
pub fn pill_badge(text: &str) -> String {
    format!(r#"<span class="pill-badge text-white">{}</span>"#, text)
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
