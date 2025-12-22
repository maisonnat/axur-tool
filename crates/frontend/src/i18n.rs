//! Frontend internationalization (i18n) module
//!
//! Provides translations for UI text in Spanish, English, and Portuguese.

/// Supported UI languages
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum UiLanguage {
    #[default]
    Es,
    En,
    Pt,
}

impl UiLanguage {
    pub fn code(&self) -> &'static str {
        match self {
            UiLanguage::Es => "es",
            UiLanguage::En => "en",
            UiLanguage::Pt => "pt",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            UiLanguage::Es => "Español",
            UiLanguage::En => "English",
            UiLanguage::Pt => "Português",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "en" => UiLanguage::En,
            "pt" | "pt-br" => UiLanguage::Pt,
            _ => UiLanguage::Es,
        }
    }

    pub fn all() -> Vec<UiLanguage> {
        vec![UiLanguage::Es, UiLanguage::En, UiLanguage::Pt]
    }
}

/// UI Dictionary - contains all translatable strings
#[derive(Clone)]
pub struct UiDict {
    // Login page
    pub login_title: &'static str,
    pub email_label: &'static str,
    pub email_placeholder: &'static str,
    pub password_label: &'static str,
    pub password_placeholder: &'static str,
    pub continue_btn: &'static str,
    pub email_password_required: &'static str,

    // 2FA
    pub tfa_title: &'static str,
    pub tfa_subtitle: &'static str,
    pub tfa_label: &'static str,
    pub tfa_placeholder: &'static str,
    pub verify_btn: &'static str,
    pub tfa_required: &'static str,

    // Finalizing
    pub finalizing: &'static str,

    // Dashboard
    pub generate_report: &'static str,
    pub configuration: &'static str,
    pub tenant_label: &'static str,
    pub tenant_placeholder: &'static str,
    pub from_date: &'static str,
    pub to_date: &'static str,
    pub language_label: &'static str,
    pub generate_btn: &'static str,
    pub preview: &'static str,
    pub download_html: &'static str,
    pub loading_tenants: &'static str,
    pub select_tenant: &'static str,
    pub logout: &'static str,

    // Common
    pub copyright: &'static str,
    pub external_threat_intel: &'static str,
}

/// Get dictionary for a specific language
pub fn get_ui_dict(lang: UiLanguage) -> UiDict {
    match lang {
        UiLanguage::Es => DICT_ES,
        UiLanguage::En => DICT_EN,
        UiLanguage::Pt => DICT_PT,
    }
}

// Spanish dictionary
const DICT_ES: UiDict = UiDict {
    login_title: "Iniciar Sesión",
    email_label: "Email",
    email_placeholder: "tu@email.com",
    password_label: "Contraseña",
    password_placeholder: "••••••••",
    continue_btn: "Continuar",
    email_password_required: "Email y contraseña son requeridos",

    tfa_title: "Verificación 2FA",
    tfa_subtitle: "Ingresa el código de tu app autenticadora",
    tfa_label: "Código 2FA",
    tfa_placeholder: "123456",
    verify_btn: "Verificar",
    tfa_required: "Código 2FA requerido",

    finalizing: "Finalizando sesión...",

    generate_report: "Generar Reporte",
    configuration: "Configuración",
    tenant_label: "Tenant",
    tenant_placeholder: "Buscar tenant...",
    from_date: "Fecha Inicio",
    to_date: "Fecha Fin",
    language_label: "Idioma",
    generate_btn: "Generar Reporte",
    preview: "Vista Previa",
    download_html: "Descargar HTML",
    loading_tenants: "Cargando tenants...",
    select_tenant: "Selecciona un tenant",
    logout: "Cerrar Sesión",

    copyright: "Axur Web © 2024",
    external_threat_intel: "External Threat Intelligence",
};

// English dictionary
const DICT_EN: UiDict = UiDict {
    login_title: "Sign In",
    email_label: "Email",
    email_placeholder: "you@email.com",
    password_label: "Password",
    password_placeholder: "••••••••",
    continue_btn: "Continue",
    email_password_required: "Email and password are required",

    tfa_title: "2FA Verification",
    tfa_subtitle: "Enter the code from your authenticator app",
    tfa_label: "2FA Code",
    tfa_placeholder: "123456",
    verify_btn: "Verify",
    tfa_required: "2FA code required",

    finalizing: "Finalizing session...",

    generate_report: "Generate Report",
    configuration: "Configuration",
    tenant_label: "Tenant",
    tenant_placeholder: "Search tenant...",
    from_date: "From Date",
    to_date: "To Date",
    language_label: "Language",
    generate_btn: "Generate Report",
    preview: "Preview",
    download_html: "Download HTML",
    loading_tenants: "Loading tenants...",
    select_tenant: "Select a tenant",
    logout: "Log Out",

    copyright: "Axur Web © 2024",
    external_threat_intel: "External Threat Intelligence",
};

// Portuguese dictionary
const DICT_PT: UiDict = UiDict {
    login_title: "Entrar",
    email_label: "Email",
    email_placeholder: "voce@email.com",
    password_label: "Senha",
    password_placeholder: "••••••••",
    continue_btn: "Continuar",
    email_password_required: "Email e senha são obrigatórios",

    tfa_title: "Verificação 2FA",
    tfa_subtitle: "Digite o código do seu app autenticador",
    tfa_label: "Código 2FA",
    tfa_placeholder: "123456",
    verify_btn: "Verificar",
    tfa_required: "Código 2FA obrigatório",

    finalizing: "Finalizando sessão...",

    generate_report: "Gerar Relatório",
    configuration: "Configuração",
    tenant_label: "Tenant",
    tenant_placeholder: "Buscar tenant...",
    from_date: "Data Início",
    to_date: "Data Fim",
    language_label: "Idioma",
    generate_btn: "Gerar Relatório",
    preview: "Prévia",
    download_html: "Baixar HTML",
    loading_tenants: "Carregando tenants...",
    select_tenant: "Selecione um tenant",
    logout: "Sair",

    copyright: "Axur Web © 2024",
    external_threat_intel: "External Threat Intelligence",
};
