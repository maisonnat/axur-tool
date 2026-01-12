//! Hints System - Passive help tooltips
//!
//! Shows contextual hints when user is idle for a certain time.

use crate::UiLanguage;

/// Position of the hint bubble relative to target
#[derive(Clone, Debug, PartialEq)]
pub enum HintPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// A passive hint that appears after idle time
#[derive(Clone)]
pub struct Hint {
    /// Unique identifier
    pub id: &'static str,
    /// CSS selector for target element
    pub target_selector: &'static str,
    /// Milliseconds of idle time before showing
    pub show_after_idle_ms: u32,
    /// Position relative to target
    pub position: HintPosition,
}

impl Hint {
    /// Get localized message
    pub fn message(&self, lang: UiLanguage) -> &'static str {
        match (self.id, lang) {
            // Editor canvas hint
            ("empty_canvas", UiLanguage::Es) => {
                "💡 Arrastra elementos desde la barra lateral para empezar"
            }
            ("empty_canvas", UiLanguage::En) => "💡 Drag elements from the sidebar to get started",
            ("empty_canvas", UiLanguage::Pt) => {
                "💡 Arraste elementos da barra lateral para começar"
            }

            // Placeholder hint
            ("placeholder_info", UiLanguage::Es) => {
                "💡 Los placeholders se reemplazan automáticamente con datos reales"
            }
            ("placeholder_info", UiLanguage::En) => {
                "💡 Placeholders are automatically replaced with real data"
            }
            ("placeholder_info", UiLanguage::Pt) => {
                "💡 Os placeholders são substituídos automaticamente por dados reais"
            }

            // Export hint
            ("export_options", UiLanguage::Es) => {
                "💡 Puedes exportar a Google Slides o descargar como HTML"
            }
            ("export_options", UiLanguage::En) => {
                "💡 You can export to Google Slides or download as HTML"
            }
            ("export_options", UiLanguage::Pt) => {
                "💡 Você pode exportar para Google Slides ou baixar como HTML"
            }

            // Shortcut hint
            ("shortcuts", UiLanguage::Es) => "💡 Usa Ctrl+Z para deshacer, Ctrl+D para duplicar",
            ("shortcuts", UiLanguage::En) => "💡 Use Ctrl+Z to undo, Ctrl+D to duplicate",
            ("shortcuts", UiLanguage::Pt) => "💡 Use Ctrl+Z para desfazer, Ctrl+D para duplicar",

            _ => "",
        }
    }

    /// Get CSS position style for hint bubble placement
    pub fn position_style(&self) -> String {
        match self.position {
            HintPosition::Top => {
                "bottom: 80px; left: 50%; transform: translateX(-50%);".to_string()
            }
            HintPosition::Bottom => {
                "top: 80px; left: 50%; transform: translateX(-50%);".to_string()
            }
            HintPosition::Left => "right: 20px; top: 50%; transform: translateY(-50%);".to_string(),
            HintPosition::Right => "left: 20px; top: 50%; transform: translateY(-50%);".to_string(),
        }
    }
}

/// All registered hints (extend to add more)
pub static HINTS: &[Hint] = &[
    Hint {
        id: "empty_canvas",
        target_selector: "#fabric-canvas",
        show_after_idle_ms: 15000, // 15 seconds
        position: HintPosition::Top,
    },
    Hint {
        id: "placeholder_info",
        target_selector: "[data-hint='placeholder']",
        show_after_idle_ms: 10000,
        position: HintPosition::Right,
    },
    Hint {
        id: "export_options",
        target_selector: "[data-hint='export']",
        show_after_idle_ms: 20000,
        position: HintPosition::Left,
    },
    Hint {
        id: "shortcuts",
        target_selector: ".toolbar",
        show_after_idle_ms: 30000,
        position: HintPosition::Bottom,
    },
];
