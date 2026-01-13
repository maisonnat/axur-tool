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
#[derive(Clone, PartialEq)]
pub struct UiDict {
    // Login page
    pub login_title: &'static str,
    pub email_label: &'static str,
    pub email_placeholder: &'static str,
    pub password_label: &'static str,
    pub password_placeholder: &'static str,
    pub continue_btn: &'static str,
    pub email_password_required: &'static str,

    // Landing page
    pub landing_tagline: &'static str,
    pub landing_tagline_highlight: &'static str,
    pub landing_subtitle: &'static str,
    pub landing_login_btn: &'static str,
    pub landing_beta_btn: &'static str,
    pub landing_rights: &'static str,

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

    // Onboarding
    pub onboarding_title: &'static str,
    pub onboarding_subtitle: &'static str,
    pub jump_in_title: &'static str,
    pub jump_in_desc: &'static str,
    pub enter_dashboard: &'static str,
    pub learning_hub_title: &'static str,
    pub learning_hub_desc: &'static str,
    pub tutorial_disclaimer: &'static str,

    // Skills
    pub skill_th_title: &'static str,
    pub skill_th_desc: &'static str,
    pub skill_templates_title: &'static str,
    pub skill_templates_desc: &'static str,
    pub skill_reports_title: &'static str,
    pub skill_reports_desc: &'static str,
    pub skill_editor_title: &'static str,
    pub skill_editor_desc: &'static str,

    // Sandbox Mode
    pub sandbox_banner: &'static str,
    pub sandbox_exit: &'static str,
    pub sandbox_welcome_title: &'static str,
    pub sandbox_welcome_desc: &'static str,
    pub sandbox_start_tutorial: &'static str,
    pub sandbox_explore: &'static str,
    pub sandbox_mode_label: &'static str,

    // Achievements
    pub achievement_unlocked: &'static str,
    pub achievement_progress: &'static str,
    pub achievement_completed: &'static str,

    // Marketplace
    pub mkt_title: &'static str,
    pub mkt_subtitle: &'static str,
    pub cat_all: &'static str,
    pub cat_exec: &'static str,
    pub cat_tech: &'static str,
    pub cat_comp: &'static str,
    pub cat_risk: &'static str,
    pub cat_custom: &'static str,
    pub mkt_author_axur: &'static str,
    pub mkt_author_community: &'static str,
    pub mkt_search_placeholder: &'static str,
    pub mkt_loading: &'static str,
    pub mkt_no_results: &'static str,
    pub mkt_use_template: &'static str,
    pub mkt_close: &'static str,
    pub mkt_featured: &'static str,

    // Logs
    pub logs_title: &'static str,
    pub logs_subtitle: &'static str,
    pub logs_col_date: &'static str,
    pub logs_col_category: &'static str,
    pub logs_col_search: &'static str,
    pub logs_placeholder_search: &'static str,
    pub logs_all_categories: &'static str,
    pub logs_error_prefix: &'static str,
    pub logs_failed_load: &'static str,
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

    // Landing page
    landing_tagline: " para proteger tu negocio",
    landing_tagline_highlight: "Soluciones IA",
    landing_subtitle: "Más allá del perímetro",
    landing_login_btn: "Iniciar Sesión",
    landing_beta_btn: "Solicitar Acceso Beta",
    landing_rights: "© 2026 Axur. Todos los derechos reservados.",

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

    copyright: "Axur Web © 2025",
    external_threat_intel: "External Threat Intelligence",

    // Onboarding
    onboarding_title: "Protección de Riesgo Digital",
    onboarding_subtitle:
        "Experimenta la próxima generación de detección y respuesta automatizada ante amenazas.",
    jump_in_title: "Acceder a la Plataforma",
    jump_in_desc: "Entrar al centro de comando. Recomendado para usuarios experimentados.",
    enter_dashboard: "Entrar al Dashboard",
    learning_hub_title: "Onboarding",
    learning_hub_desc: "Aprende a proteger tus activos digitales. Selecciona un módulo:",
    tutorial_disclaimer: "Nota: Este es un ejemplo interactivo con fines demostrativos.",

    // Skills
    skill_th_title: "Threat Hunting",
    skill_th_desc: "Aprende a encontrar amenazas usando filtros avanzados.",
    skill_templates_title: "Plantillas",
    skill_templates_desc: "Explora y usa plantillas de la comunidad.",
    skill_reports_title: "Reportes",
    skill_reports_desc: "Crea y exporta reportes de seguridad detallados.",
    skill_editor_title: "Editor",
    skill_editor_desc: "Personaliza diseños de reportes con el editor visual.",

    // Sandbox Mode
    sandbox_banner: "⚠️ MODO PRÁCTICA - Los cambios no se guardan",
    sandbox_exit: "Salir del modo práctica",
    sandbox_welcome_title: "🎓 ¡Bienvenido a Axur Academy!",
    sandbox_welcome_desc: "Aprende a crear reportes de seguridad con este template de ejemplo. Tus cambios no se guardarán, así que puedes experimentar libremente.",
    sandbox_start_tutorial: "🎯 Empezar Tutorial",
    sandbox_explore: "🔍 Explorar Libremente",
    sandbox_mode_label: "Modo Práctica",

    // Achievements
    achievement_unlocked: "🏆 ¡Logro Desbloqueado!",
    achievement_progress: "Tu Progreso",
    achievement_completed: "completados",

    // Marketplace
    mkt_title: "Marketplace de Plantillas",
    mkt_subtitle: "Explora plantillas de la comunidad para acelerar tus reportes.",
    cat_all: "Todos",
    cat_exec: "Ejecutivo",
    cat_tech: "Técnico",
    cat_comp: "Compliance",
    cat_risk: "Riesgo",
    cat_custom: "Personalizado",
    mkt_author_axur: "Oficial Axur",
    mkt_author_community: "Comunidad",
    mkt_search_placeholder: "Buscar plantillas...",
    mkt_loading: "Cargando...",
    mkt_no_results: "No se encontraron plantillas",
    mkt_use_template: "Usar Plantilla",
    mkt_close: "Cerrar",
    mkt_featured: "Destacado",

    // Logs
    logs_title: "Registros del Sistema",
    logs_subtitle: "Explora y busca en los registros de la aplicación.",
    logs_col_date: "Fecha",
    logs_col_category: "Categoría",
    logs_col_search: "Búsqueda",
    logs_placeholder_search: "Buscar en logs...",
    logs_all_categories: "Todas las categorías",
    logs_error_prefix: "Error: ",
    logs_failed_load: "Error al cargar: ",
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

    // Landing page
    landing_tagline: " to protect your business",
    landing_tagline_highlight: "AI Solutions",
    landing_subtitle: "Beyond the perimeter",
    landing_login_btn: "Log In",
    landing_beta_btn: "Request Beta Access",
    landing_rights: "© 2026 Axur. All rights reserved.",

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

    copyright: "Axur Web © 2025",
    external_threat_intel: "External Threat Intelligence",

    // Onboarding
    onboarding_title: "Digital Risk Protection",
    onboarding_subtitle:
        "Experience the next generation of automated threat detection and response.",
    jump_in_title: "Access Platform",
    jump_in_desc: "Enter the command center. Recommended for experienced users.",
    enter_dashboard: "Enter Dashboard",
    learning_hub_title: "Onboarding",
    learning_hub_desc: "Learn how to protect your digital assets. Select a module:",
    tutorial_disclaimer: "Note: This is an interactive example for demonstration purposes.",

    // Skills
    skill_th_title: "Threat Hunting",
    skill_th_desc: "Learn how to find threats using advanced filters.",
    skill_templates_title: "Templates",
    skill_templates_desc: "Browse and use community templates.",
    skill_reports_title: "Reports",
    skill_reports_desc: "Create and export detailed security reports.",
    skill_editor_title: "Editor",
    skill_editor_desc: "Customize report layouts with the visual editor.",

    // Sandbox Mode
    sandbox_banner: "⚠️ PRACTICE MODE - Changes are not saved",
    sandbox_exit: "Exit practice mode",
    sandbox_welcome_title: "🎓 Welcome to Axur Academy!",
    sandbox_welcome_desc: "Learn to create security reports with this demo template. Your changes won't be saved, so feel free to experiment.",
    sandbox_start_tutorial: "🎯 Start Tutorial",
    sandbox_explore: "🔍 Explore Freely",
    sandbox_mode_label: "Practice Mode",

    // Achievements
    achievement_unlocked: "🏆 Achievement Unlocked!",
    achievement_progress: "Your Progress",
    achievement_completed: "completed",

    // Marketplace
    mkt_title: "Template Marketplace",
    mkt_subtitle: "Browse community templates to verify reporting.",
    cat_all: "All",
    cat_exec: "Executive",
    cat_tech: "Technical",
    cat_comp: "Compliance",
    cat_risk: "Risk",
    cat_custom: "Custom",
    mkt_author_axur: "Axur Official",
    mkt_author_community: "Community",
    mkt_search_placeholder: "Search templates...",
    mkt_loading: "Loading...",
    mkt_no_results: "No templates found",
    mkt_use_template: "Use Template",
    mkt_close: "Close",
    mkt_featured: "Featured",

    // Logs
    logs_title: "System Logs",
    logs_subtitle: "Browse and search application logs.",
    logs_col_date: "Date",
    logs_col_category: "Category",
    logs_col_search: "Search",
    logs_placeholder_search: "Search logs...",
    logs_all_categories: "All Categories",
    logs_error_prefix: "Error: ",
    logs_failed_load: "Failed to load: ",
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

    // Landing page
    landing_tagline: " para proteger seu negócio",
    landing_tagline_highlight: "Soluções IA",
    landing_subtitle: "Além do perímetro",
    landing_login_btn: "Entrar",
    landing_beta_btn: "Solicitar Acesso Beta",
    landing_rights: "© 2026 Axur. Todos os direitos reservados.",

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

    copyright: "Axur Web © 2025",
    external_threat_intel: "External Threat Intelligence",

    // Onboarding
    onboarding_title: "Proteção de Risco Digital",
    onboarding_subtitle:
        "Experimente a próxima geração de detecção e resposta automatizada a ameaças.",
    jump_in_title: "Acessar Plataforma",
    jump_in_desc: "Entrar no centro de comando. Recomendado para usuários experientes.",
    enter_dashboard: "Entrar no Dashboard",
    learning_hub_title: "Onboarding",
    learning_hub_desc: "Aprenda a proteger seus ativos digitais. Selecione um módulo:",
    tutorial_disclaimer: "Nota: Este é um exemplo interativo para fins de demonstração.",

    // Skills
    skill_th_title: "Threat Hunting",
    skill_th_desc: "Aprenda a encontrar ameaças usando filtros avançados.",
    skill_templates_title: "Modelos",
    skill_templates_desc: "Navegue e use modelos da comunidade.",
    skill_reports_title: "Relatórios",
    skill_reports_desc: "Crie e exporte relatórios de segurança detalhados.",
    skill_editor_title: "Editor",
    skill_editor_desc: "Personalize layouts de relatórios com o editor visual.",

    // Sandbox Mode
    sandbox_banner: "⚠️ MODO PRÁTICA - As alterações não são salvas",
    sandbox_exit: "Sair do modo prática",
    sandbox_welcome_title: "🎓 Bem-vindo à Axur Academy!",
    sandbox_welcome_desc: "Aprenda a criar relatórios de segurança com este modelo de exemplo. Suas alterações não serão salvas, então sinta-se à vontade para experimentar.",
    sandbox_start_tutorial: "🎯 Iniciar Tutorial",
    sandbox_explore: "🔍 Explorar Livremente",
    sandbox_mode_label: "Modo Prática",

    // Achievements
    achievement_unlocked: "🏆 Conquista Desbloqueada!",
    achievement_progress: "Seu Progresso",
    achievement_completed: "concluídos",

    // Marketplace
    mkt_title: "Marketplace de Modelos",
    mkt_subtitle: "Navegue por modelos da comunidade para agilizar seus relatórios.",
    cat_all: "Todos",
    cat_exec: "Executivo",
    cat_tech: "Técnico",
    cat_comp: "Compliance",
    cat_risk: "Risco",
    cat_custom: "Personalizado",
    mkt_author_axur: "Oficial Axur",
    mkt_author_community: "Comunidade",
    mkt_search_placeholder: "Buscar modelos...",
    mkt_loading: "Carregando...",
    mkt_no_results: "Nenhum modelo encontrado",
    mkt_use_template: "Usar Modelo",
    mkt_close: "Fechar",
    mkt_featured: "Destaque",

    // Logs
    logs_title: "Logs do Sistema",
    logs_subtitle: "Navegue e pesquise nos logs da aplicação.",
    logs_col_date: "Data",
    logs_col_category: "Categoria",
    logs_col_search: "Busca",
    logs_placeholder_search: "Buscar logs...",
    logs_all_categories: "Todas as categorias",
    logs_error_prefix: "Erro: ",
    logs_failed_load: "Falha ao carregar: ",
};
