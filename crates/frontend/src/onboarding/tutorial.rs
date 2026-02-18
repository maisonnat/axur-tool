//! Tutorial Steps System
//!
//! Each step guides the user through a specific action.
//! Steps are validated automatically when the user completes the action.

use crate::UiLanguage;

/// How a tutorial step is validated as complete
#[derive(Clone, Debug, PartialEq)]
pub enum StepValidation {
    /// User clicks the target element
    Click,
    /// User types in an input field
    Input,
    /// User presses a keyboard shortcut (e.g., "Ctrl+D")
    Shortcut(&'static str),
    /// User drags something
    Drag,
    /// Auto-complete after viewing for N milliseconds
    Timeout(u32),
}

/// A single tutorial step
#[derive(Clone)]
pub struct TutorialStep {
    /// Unique identifier (used for i18n key derivation)
    pub id: &'static str,
    /// CSS selector for the target element to highlight
    pub target_selector: &'static str,
    /// How to validate completion
    pub validation: StepValidation,
    /// Position of tooltip relative to target
    pub tooltip_position: &'static str, // "top", "bottom", "left", "right"
}

impl TutorialStep {
    /// Get localized title using i18n key pattern: onboarding_step_{id}_title
    pub fn title(&self, lang: UiLanguage) -> &'static str {
        match (self.id, lang) {
            // Step: edit_title
            ("edit_title", UiLanguage::Es) => "Edita el título",
            ("edit_title", UiLanguage::En) => "Edit the title",
            ("edit_title", UiLanguage::Pt) => "Edite o título",
            // Step: add_placeholder
            ("add_placeholder", UiLanguage::Es) => "Añade un placeholder",
            ("add_placeholder", UiLanguage::En) => "Add a placeholder",
            ("add_placeholder", UiLanguage::Pt) => "Adicione um placeholder",
            // Step: duplicate
            ("duplicate", UiLanguage::Es) => "Duplicar objeto",
            ("duplicate", UiLanguage::En) => "Duplicate object",
            ("duplicate", UiLanguage::Pt) => "Duplicar objeto",
            // Step: export
            ("export", UiLanguage::Es) => "Exportar reporte",
            ("export", UiLanguage::En) => "Export report",
            ("export", UiLanguage::Pt) => "Exportar relatório",
            // Default fallback
            _ => self.id,
        }
    }

    /// Get localized instruction using i18n key pattern: onboarding_step_{id}_instruction
    pub fn instruction(&self, lang: UiLanguage) -> &'static str {
        match (self.id, lang) {
            // Step: edit_title
            ("edit_title", UiLanguage::Es) => "Haz doble clic en el título y escribe algo",
            ("edit_title", UiLanguage::En) => "Double-click the title and type something",
            ("edit_title", UiLanguage::Pt) => "Clique duas vezes no título e digite algo",
            // Step: add_placeholder
            ("add_placeholder", UiLanguage::Es) => {
                "Arrastra un placeholder desde la barra lateral al canvas"
            }
            ("add_placeholder", UiLanguage::En) => {
                "Drag a placeholder from the sidebar to the canvas"
            }
            ("add_placeholder", UiLanguage::Pt) => {
                "Arraste um placeholder da barra lateral para o canvas"
            }
            // Step: duplicate
            ("duplicate", UiLanguage::Es) => "Selecciona un objeto y presiona Ctrl+D",
            ("duplicate", UiLanguage::En) => "Select an object and press Ctrl+D",
            ("duplicate", UiLanguage::Pt) => "Selecione um objeto e pressione Ctrl+D",
            // Step: export
            ("export", UiLanguage::Es) => "Haz clic en el botón 'Exportar' para generar tu reporte",
            ("export", UiLanguage::En) => "Click the 'Export' button to generate your report",
            ("export", UiLanguage::Pt) => "Clique no botão 'Exportar' para gerar seu relatório",
            // Default
            _ => "Complete this step",
        }
    }
}

/// Default tutorial steps (extensible)
pub static TUTORIAL_STEPS: &[TutorialStep] = &[
    TutorialStep {
        id: "edit_title",
        target_selector: ".slide-title, [data-tutorial='title']",
        validation: StepValidation::Input,
        tooltip_position: "bottom",
    },
    TutorialStep {
        id: "add_placeholder",
        target_selector: "[data-tutorial='placeholder-btn']",
        validation: StepValidation::Drag,
        tooltip_position: "right",
    },
    TutorialStep {
        id: "duplicate",
        target_selector: "[data-tutorial='duplicate-btn']",
        validation: StepValidation::Shortcut("Ctrl+D"),
        tooltip_position: "bottom",
    },
    TutorialStep {
        id: "export",
        target_selector: "[data-tutorial='export-btn']",
        validation: StepValidation::Click,
        tooltip_position: "left",
    },
];
