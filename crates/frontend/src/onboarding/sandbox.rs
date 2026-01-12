//! Sandbox Mode - Demo environment for learning
//!
//! Provides a safe environment where changes are not saved.
//! Includes rich demo content for practicing all achievement activities.

/// Global sandbox mode signal (set once at component mount)
static mut SANDBOX_MODE: bool = false;

/// Check if sandbox mode is active
pub fn is_sandbox_mode() -> bool {
    unsafe { SANDBOX_MODE }
}

/// Set sandbox mode (should only be called once at startup)
pub fn set_sandbox_mode(enabled: bool) {
    unsafe {
        SANDBOX_MODE = enabled;
    }
}

/// Demo slides data for sandbox mode
/// 5 realistic slides for practicing editor features
pub fn get_sandbox_slides() -> Vec<SandboxSlide> {
    vec![
        SandboxSlide {
            id: "cover",
            name: "Portada",
            elements: vec![
                SlideElement::text("ACME Corporation", 80, 120, 48, "#ffffff", true),
                SlideElement::text(
                    "Reporte de Amenazas Digitales",
                    80,
                    180,
                    24,
                    "#f97316",
                    false,
                ),
                SlideElement::placeholder("report_date", "Enero 2026", 80, 230),
                SlideElement::text("/// AXUR", 80, 450, 20, "#f97316", true),
            ],
        },
        SandboxSlide {
            id: "metrics",
            name: "Métricas Clave",
            elements: vec![
                SlideElement::text("📊 Métricas del Período", 40, 30, 28, "#ffffff", true),
                SlideElement::metric_card("127", "Amenazas Detectadas", "#ef4444", 40, 80),
                SlideElement::metric_card("89", "Takedowns Exitosos", "#22c55e", 260, 80),
                SlideElement::metric_card("4.2h", "Tiempo Respuesta", "#f97316", 480, 80),
            ],
        },
        SandboxSlide {
            id: "threats",
            name: "Análisis de Amenazas",
            elements: vec![
                SlideElement::text("🔍 Análisis de Amenazas", 40, 30, 28, "#ffffff", true),
                SlideElement::text("Top 5 Tipos de Incidentes:", 40, 80, 16, "#d4d4d8", false),
                SlideElement::text("1. Phishing (45 casos)", 60, 120, 14, "#fbbf24", false),
                SlideElement::text("2. Fake Domains (32 casos)", 60, 150, 14, "#fb923c", false),
                SlideElement::text("3. Data Leaks (28 casos)", 60, 180, 14, "#f87171", false),
                SlideElement::text("4. Brand Abuse (15 casos)", 60, 210, 14, "#a78bfa", false),
                SlideElement::text(
                    "5. Social Media Fraud (7 casos)",
                    60,
                    240,
                    14,
                    "#60a5fa",
                    false,
                ),
                SlideElement::tip(
                    "💡 Tip: Haz doble clic en cualquier texto para editarlo",
                    40,
                    300,
                ),
            ],
        },
        SandboxSlide {
            id: "evidence",
            name: "Evidencia",
            elements: vec![
                SlideElement::text("📸 Evidencia de Incidentes", 40, 30, 28, "#ffffff", true),
                SlideElement::evidence_card("Phishing - fake-acme.com", "#ef4444", 40, 80),
                SlideElement::evidence_card("Fake App - Play Store", "#f97316", 340, 80),
            ],
        },
        SandboxSlide {
            id: "recommendations",
            name: "Recomendaciones",
            elements: vec![
                SlideElement::text("✅ Recomendaciones", 40, 30, 28, "#ffffff", true),
                SlideElement::text(
                    "Acciones prioritarias para el próximo período:",
                    40,
                    80,
                    14,
                    "#d4d4d8",
                    false,
                ),
                SlideElement::recommendation(
                    "🔒 Implementar DMARC estricto para prevenir phishing",
                    "#22c55e",
                    40,
                    110,
                ),
                SlideElement::recommendation(
                    "📱 Monitorear app stores para detectar apps falsas",
                    "#f97316",
                    40,
                    180,
                ),
                SlideElement::recommendation(
                    "🔍 Activar alertas de Signal-Lake para credenciales",
                    "#3b82f6",
                    40,
                    250,
                ),
                SlideElement::tip(
                    "💡 Practica: Usa Ctrl+D para duplicar una recomendación",
                    40,
                    340,
                ),
            ],
        },
    ]
}

/// Sandbox slide structure
#[derive(Clone)]
pub struct SandboxSlide {
    pub id: &'static str,
    pub name: &'static str,
    pub elements: Vec<SlideElement>,
}

/// Slide element types
#[derive(Clone)]
pub enum SlideElement {
    Text {
        content: &'static str,
        x: u32,
        y: u32,
        size: u32,
        color: &'static str,
        bold: bool,
    },
    Placeholder {
        key: &'static str,
        value: &'static str,
        x: u32,
        y: u32,
    },
    MetricCard {
        value: &'static str,
        label: &'static str,
        color: &'static str,
        x: u32,
        y: u32,
    },
    EvidenceCard {
        label: &'static str,
        border_color: &'static str,
        x: u32,
        y: u32,
    },
    Recommendation {
        text: &'static str,
        color: &'static str,
        x: u32,
        y: u32,
    },
    Tip {
        text: &'static str,
        x: u32,
        y: u32,
    },
}

impl SlideElement {
    pub fn text(
        content: &'static str,
        x: u32,
        y: u32,
        size: u32,
        color: &'static str,
        bold: bool,
    ) -> Self {
        SlideElement::Text {
            content,
            x,
            y,
            size,
            color,
            bold,
        }
    }

    pub fn placeholder(key: &'static str, value: &'static str, x: u32, y: u32) -> Self {
        SlideElement::Placeholder { key, value, x, y }
    }

    pub fn metric_card(
        value: &'static str,
        label: &'static str,
        color: &'static str,
        x: u32,
        y: u32,
    ) -> Self {
        SlideElement::MetricCard {
            value,
            label,
            color,
            x,
            y,
        }
    }

    pub fn evidence_card(label: &'static str, border_color: &'static str, x: u32, y: u32) -> Self {
        SlideElement::EvidenceCard {
            label,
            border_color,
            x,
            y,
        }
    }

    pub fn recommendation(text: &'static str, color: &'static str, x: u32, y: u32) -> Self {
        SlideElement::Recommendation { text, color, x, y }
    }

    pub fn tip(text: &'static str, x: u32, y: u32) -> Self {
        SlideElement::Tip { text, x, y }
    }
}

/// Practice activities for the sandbox
pub fn get_practice_activities(lang: crate::UiLanguage) -> Vec<PracticeActivity> {
    match lang {
        crate::UiLanguage::Es => vec![
            PracticeActivity {
                id: "edit_text",
                title: "Editar texto",
                instruction: "Haz doble clic en 'ACME Corporation' y cambia el nombre",
                icon: "📝",
            },
            PracticeActivity {
                id: "navigate",
                title: "Navegar slides",
                instruction: "Usa las flechas o miniaturas para moverte entre slides",
                icon: "🔄",
            },
            PracticeActivity {
                id: "duplicate",
                title: "Duplicar objeto",
                instruction: "Selecciona un texto y presiona Ctrl+D",
                icon: "📋",
            },
            PracticeActivity {
                id: "export",
                title: "Exportar",
                instruction: "Haz clic en 'Google Slides' para exportar",
                icon: "📤",
            },
        ],
        crate::UiLanguage::En => vec![
            PracticeActivity {
                id: "edit_text",
                title: "Edit text",
                instruction: "Double-click on 'ACME Corporation' and change the name",
                icon: "📝",
            },
            PracticeActivity {
                id: "navigate",
                title: "Navigate slides",
                instruction: "Use arrows or thumbnails to move between slides",
                icon: "🔄",
            },
            PracticeActivity {
                id: "duplicate",
                title: "Duplicate object",
                instruction: "Select a text and press Ctrl+D",
                icon: "📋",
            },
            PracticeActivity {
                id: "export",
                title: "Export",
                instruction: "Click 'Google Slides' to export",
                icon: "📤",
            },
        ],
        crate::UiLanguage::Pt => vec![
            PracticeActivity {
                id: "edit_text",
                title: "Editar texto",
                instruction: "Clique duas vezes em 'ACME Corporation' e mude o nome",
                icon: "📝",
            },
            PracticeActivity {
                id: "navigate",
                title: "Navegar slides",
                instruction: "Use as setas ou miniaturas para se mover entre slides",
                icon: "🔄",
            },
            PracticeActivity {
                id: "duplicate",
                title: "Duplicar objeto",
                instruction: "Selecione um texto e pressione Ctrl+D",
                icon: "📋",
            },
            PracticeActivity {
                id: "export",
                title: "Exportar",
                instruction: "Clique em 'Google Slides' para exportar",
                icon: "📤",
            },
        ],
    }
}

/// Practice activity structure
#[derive(Clone)]
pub struct PracticeActivity {
    pub id: &'static str,
    pub title: &'static str,
    pub instruction: &'static str,
    pub icon: &'static str,
}

// ==================== LOCALIZED TEXTS ====================

/// Get localized sandbox banner text
pub fn sandbox_banner_text(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "⚠️ MODO PRÁCTICA - Los cambios no se guardan",
        crate::UiLanguage::En => "⚠️ PRACTICE MODE - Changes are not saved",
        crate::UiLanguage::Pt => "⚠️ MODO PRÁTICA - As alterações não são salvas",
    }
}

/// Get localized "Exit sandbox" button text
pub fn exit_sandbox_text(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "Salir del modo práctica",
        crate::UiLanguage::En => "Exit practice mode",
        crate::UiLanguage::Pt => "Sair do modo prática",
    }
}

/// Get localized welcome modal title
pub fn sandbox_welcome_title(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "🎓 ¡Bienvenido a Axur Academy!",
        crate::UiLanguage::En => "🎓 Welcome to Axur Academy!",
        crate::UiLanguage::Pt => "🎓 Bem-vindo à Axur Academy!",
    }
}

/// Get localized welcome modal description
pub fn sandbox_welcome_desc(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "Aprende a crear reportes de seguridad con este template de ejemplo. Tus cambios no se guardarán, así que puedes experimentar libremente.",
        crate::UiLanguage::En => "Learn to create security reports with this sample template. Your changes won't be saved, so feel free to experiment.",
        crate::UiLanguage::Pt => "Aprenda a criar relatórios de segurança com este template de exemplo. Suas alterações não serão salvas, então fique à vontade para experimentar.",
    }
}

/// Get localized "Start Tutorial" button text
pub fn sandbox_start_tutorial(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "🎯 Empezar Tutorial",
        crate::UiLanguage::En => "🎯 Start Tutorial",
        crate::UiLanguage::Pt => "🎯 Iniciar Tutorial",
    }
}

/// Get localized "Explore Freely" button text
pub fn sandbox_explore_freely(lang: crate::UiLanguage) -> &'static str {
    match lang {
        crate::UiLanguage::Es => "🔍 Explorar Libremente",
        crate::UiLanguage::En => "🔍 Explore Freely",
        crate::UiLanguage::Pt => "🔍 Explorar Livremente",
    }
}
