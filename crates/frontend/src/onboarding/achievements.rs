//! Achievement System
//!
//! Tracks user progress through unlockable achievements.
//! Each achievement can be triggered by different actions.

use crate::UiLanguage;

/// How an achievement is triggered/unlocked
#[derive(Clone, Debug, PartialEq)]
pub enum AchievementTrigger {
    /// Unlocked when tutorial is completed
    TutorialComplete,
    /// Unlocked by a manual action identifier
    ManualAction(&'static str),
    /// Unlocked when visiting a specific page
    PageVisit(&'static str),
    /// Unlocked when using a keyboard shortcut
    Shortcut(&'static str),
}

/// A single achievement/milestone
#[derive(Clone)]
pub struct Achievement {
    /// Unique identifier
    pub id: &'static str,
    /// Emoji icon
    pub icon: &'static str,
    /// Trigger condition
    pub trigger: AchievementTrigger,
}

impl Achievement {
    /// Get localized title
    pub fn title(&self, lang: UiLanguage) -> &'static str {
        match (self.id, lang) {
            // Tutorial complete
            ("tutorial", UiLanguage::Es) => "Tutorial completado",
            ("tutorial", UiLanguage::En) => "Tutorial completed",
            ("tutorial", UiLanguage::Pt) => "Tutorial concluído",
            // First import
            ("first_import", UiLanguage::Es) => "Primera importación",
            ("first_import", UiLanguage::En) => "First import",
            ("first_import", UiLanguage::Pt) => "Primeira importação",
            // First export
            ("first_export", UiLanguage::Es) => "Primera exportación",
            ("first_export", UiLanguage::En) => "First export",
            ("first_export", UiLanguage::Pt) => "Primeira exportação",
            // Template created
            ("template_created", UiLanguage::Es) => "Template creado",
            ("template_created", UiLanguage::En) => "Template created",
            ("template_created", UiLanguage::Pt) => "Template criado",
            // Threat hunting
            ("threat_hunting", UiLanguage::Es) => "Threat Hunter",
            ("threat_hunting", UiLanguage::En) => "Threat Hunter",
            ("threat_hunting", UiLanguage::Pt) => "Threat Hunter",
            // Shortcut master
            ("shortcut_master", UiLanguage::Es) => "Maestro de atajos",
            ("shortcut_master", UiLanguage::En) => "Shortcut master",
            ("shortcut_master", UiLanguage::Pt) => "Mestre de atalhos",
            // Default
            _ => self.id,
        }
    }

    /// Get localized description
    pub fn description(&self, lang: UiLanguage) -> &'static str {
        match (self.id, lang) {
            ("tutorial", UiLanguage::Es) => "Completaste el tutorial interactivo",
            ("tutorial", UiLanguage::En) => "You completed the interactive tutorial",
            ("tutorial", UiLanguage::Pt) => "Você concluiu o tutorial interativo",

            ("first_import", UiLanguage::Es) => "Importaste tu primer archivo PPTX",
            ("first_import", UiLanguage::En) => "You imported your first PPTX file",
            ("first_import", UiLanguage::Pt) => "Você importou seu primeiro arquivo PPTX",

            ("first_export", UiLanguage::Es) => "Exportaste un reporte a Google Slides",
            ("first_export", UiLanguage::En) => "You exported a report to Google Slides",
            ("first_export", UiLanguage::Pt) => "Você exportou um relatório para Google Slides",

            ("template_created", UiLanguage::Es) => "Creaste tu primer template personalizado",
            ("template_created", UiLanguage::En) => "You created your first custom template",
            ("template_created", UiLanguage::Pt) => {
                "Você criou seu primeiro template personalizado"
            }

            ("threat_hunting", UiLanguage::Es) => "Usaste Threat Hunting en un reporte",
            ("threat_hunting", UiLanguage::En) => "You used Threat Hunting in a report",
            ("threat_hunting", UiLanguage::Pt) => "Você usou Threat Hunting em um relatório",

            ("shortcut_master", UiLanguage::Es) => "Usaste 5 atajos de teclado diferentes",
            ("shortcut_master", UiLanguage::En) => "You used 5 different keyboard shortcuts",
            ("shortcut_master", UiLanguage::Pt) => "Você usou 5 atalhos de teclado diferentes",

            _ => "",
        }
    }
}

/// All registered achievements (extend this list to add more)
pub static ALL_ACHIEVEMENTS: &[Achievement] = &[
    Achievement {
        id: "tutorial",
        icon: "🎓",
        trigger: AchievementTrigger::TutorialComplete,
    },
    Achievement {
        id: "first_import",
        icon: "📥",
        trigger: AchievementTrigger::ManualAction("import_pptx"),
    },
    Achievement {
        id: "first_export",
        icon: "📤",
        trigger: AchievementTrigger::ManualAction("export_slides"),
    },
    Achievement {
        id: "template_created",
        icon: "✨",
        trigger: AchievementTrigger::ManualAction("create_template"),
    },
    Achievement {
        id: "threat_hunting",
        icon: "🔍",
        trigger: AchievementTrigger::ManualAction("threat_hunting"),
    },
    Achievement {
        id: "shortcut_master",
        icon: "⌨️",
        trigger: AchievementTrigger::ManualAction("shortcut_master"),
    },
];

/// Unlock an achievement by ID (call from anywhere in the app)
pub fn unlock_achievement(id: &str) {
    use crate::onboarding::storage;
    storage::mark_achievement_unlocked(id);
}
