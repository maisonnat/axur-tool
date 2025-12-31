//! GitHub-based template storage
//!
//! Templates are stored as JSON files in the private GitHub repository
//! to preserve database storage (Leapcell free tier).

use serde::{Deserialize, Serialize};

/// Path structure for template storage in GitHub
pub const TEMPLATES_BASE_PATH: &str = "templates";
pub const OFFICIAL_TEMPLATES_PATH: &str = "templates/official";
pub const USER_TEMPLATES_PATH: &str = "templates/users";
pub const MARKETPLACE_TEMPLATES_PATH: &str = "templates/marketplace";

/// Template metadata stored in the database (lightweight)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template UUID
    pub id: String,
    /// User-facing name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Path to JSON file in GitHub repo
    pub github_path: String,
    /// Preview image URL (optional, for marketplace cards)
    pub preview_image_url: Option<String>,
    /// Whether this template is public
    pub is_public: bool,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Build the GitHub path for a user template
pub fn get_user_template_path(tenant_id: &str, template_id: &str) -> String {
    format!("{}/{}/{}.json", USER_TEMPLATES_PATH, tenant_id, template_id)
}

/// Build the GitHub path for an official template
pub fn get_official_template_path(template_name: &str) -> String {
    format!("{}/{}.json", OFFICIAL_TEMPLATES_PATH, template_name)
}

/// Build the GitHub path for a marketplace template
pub fn get_marketplace_template_path(template_id: &str) -> String {
    format!("{}/{}.json", MARKETPLACE_TEMPLATES_PATH, template_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_template_path() {
        let path = get_user_template_path("tenant-123", "abc-def-ghi");
        assert_eq!(path, "templates/users/tenant-123/abc-def-ghi.json");
    }

    #[test]
    fn test_official_template_path() {
        let path = get_official_template_path("default");
        assert_eq!(path, "templates/official/default.json");
    }

    #[test]
    fn test_marketplace_template_path() {
        let path = get_marketplace_template_path("template-456");
        assert_eq!(path, "templates/marketplace/template-456.json");
    }
}
