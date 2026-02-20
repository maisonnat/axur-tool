//! Axur Design System Tokens - Cyber Noir
//!
//! Semantic color palette and typography constants.

pub mod colors {
    // Brand Colors
    pub const BRAND_PRIMARY: &str = "#FF671F"; // Axur Orange
    pub const BRAND_PRIMARY_HOVER: &str = "#E6500A";
    pub const BRAND_SECONDARY: &str = "#EF4043"; // Axur Red

    // Surface Colors (Zinc-based)
    pub const SURFACE_BASE: &str = "#09090b"; // Zinc-950
    pub const SURFACE_LAYER: &str = "#18181b"; // Zinc-900
    pub const SURFACE_ELEVATED: &str = "#27272a"; // Zinc-800

    // Content Colors
    pub const TEXT_PRIMARY: &str = "#FFFFFF";
    pub const TEXT_SECONDARY: &str = "#a1a1aa"; // Zinc-400
    pub const TEXT_TERTIARY: &str = "#52525b"; // Zinc-600

    // Status Colors
    pub const SUCCESS: &str = "#22c55e"; // Green-500
    pub const WARNING: &str = "#f59e0b"; // Amber-500
    pub const DANGER: &str = "#ef4444"; // Red-500
    pub const INFO: &str = "#3b82f6"; // Blue-500
}

pub mod shadows {
    pub const GLOW_PRIMARY: &str = "0 0 20px rgba(255, 103, 31, 0.3)";
    pub const GLOW_DANGER: &str = "0 0 20px rgba(239, 68, 68, 0.3)";
}

pub mod component_classes {
    // Glassmorphism
    pub const GLASS_PANEL: &str = "backdrop-blur-md bg-white/5 border border-white/10";
    pub const GLASS_PANEL_HOVER: &str =
        "hover:bg-white/10 hover:border-white/20 transition-all duration-300";

    // Typography
    pub const HEADER_XL: &str = "text-4xl font-light tracking-tight text-white";
    pub const HEADER_LG: &str = "text-2xl font-bold tracking-wide text-white";
    pub const LABEL_SM: &str = "text-xs font-bold uppercase tracking-widest text-zinc-500";
}
