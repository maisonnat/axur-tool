//! Google Slides Export Plugin
//!
//! Exports report slides to Google Slides presentations.
//! This plugin converts HTML slide output to Slides API format.
//!
//! Architecture:
//! - This plugin lives in Core (defines WHAT to export)
//! - Backend's GoogleServices handles HOW to talk to Google API

use crate::plugins::{CloudExportOutput, CloudExportPlugin, SlideOutput};
use std::sync::Arc;

/// Parsed content from HTML slide for Google Slides API
#[derive(Debug, Clone)]
pub struct SlideContent {
    /// Slide title (extracted from h1/h2)
    pub title: String,
    /// Main body text (extracted from paragraphs)
    pub body: Vec<String>,
    /// Image URLs or base64 references
    pub images: Vec<String>,
    /// Slide layout type
    pub layout: SlideLayout,
    /// Background color (hex)
    pub background_color: String,
    /// Text color (hex)
    pub text_color: String,
}

/// Google Slides layout types
#[derive(Debug, Clone, Default)]
pub enum SlideLayout {
    /// Title slide (for cover)
    Title,
    /// Title and body text
    #[default]
    TitleAndBody,
    /// Title with two columns
    TwoColumn,
    /// Blank slide (for custom content)
    Blank,
    /// Section header
    SectionHeader,
}

impl SlideLayout {
    /// Convert to Google Slides predefined layout ID
    pub fn to_google_layout(&self) -> &'static str {
        match self {
            SlideLayout::Title => "TITLE",
            SlideLayout::TitleAndBody => "TITLE_AND_BODY",
            SlideLayout::TwoColumn => "TITLE_AND_TWO_COLUMNS",
            SlideLayout::Blank => "BLANK",
            SlideLayout::SectionHeader => "SECTION_HEADER",
        }
    }
}

impl Default for SlideContent {
    fn default() -> Self {
        Self {
            title: String::new(),
            body: Vec::new(),
            images: Vec::new(),
            layout: SlideLayout::default(),
            background_color: "#0A0A0A".to_string(), // Axur dark
            text_color: "#FFFFFF".to_string(),       // White
        }
    }
}

/// Service interface for Google API operations
/// Implemented by backend's GoogleServices
pub trait GoogleSlidesService: Send + Sync {
    /// Create a new presentation and return its ID
    fn create_presentation(&self, title: &str) -> Result<String, String>;

    /// Add slides to an existing presentation
    fn add_slides(&self, presentation_id: &str, slides: &[SlideContent]) -> Result<(), String>;

    /// Get the shareable URL for a presentation
    fn get_presentation_url(&self, presentation_id: &str) -> String;
}

/// Google Slides Export Plugin
///
/// This plugin implements CloudExportPlugin to enable exporting
/// reports directly to Google Slides presentations.
pub struct GoogleSlidesExportPlugin {
    /// Optional service instance (injected by backend at runtime)
    service: Option<Arc<dyn GoogleSlidesService>>,
}

impl GoogleSlidesExportPlugin {
    /// Create plugin without service (for registration only)
    pub fn new() -> Self {
        Self { service: None }
    }

    /// Create plugin with injected service (for actual export)
    pub fn with_service(service: Arc<dyn GoogleSlidesService>) -> Self {
        Self {
            service: Some(service),
        }
    }

    /// Convert HTML slide output to structured SlideContent
    pub fn parse_slide_html(slide: &SlideOutput) -> SlideContent {
        let html = &slide.html;

        // Extract title (look for h1, h2, or class="title")
        let title = Self::extract_title(html);

        // Extract body paragraphs
        let body = Self::extract_body_text(html);

        // Determine layout based on slide ID
        let layout = Self::determine_layout(&slide.id);

        // Check for dark/light theme
        let (bg, text) = if html.contains("bg-black") || html.contains("bg-zinc") {
            ("#0A0A0A".to_string(), "#FFFFFF".to_string())
        } else {
            ("#FFFFFF".to_string(), "#18181B".to_string())
        };

        SlideContent {
            title,
            body,
            images: Vec::new(), // TODO: Extract images
            layout,
            background_color: bg,
            text_color: text,
        }
    }

    /// Extract title from HTML
    fn extract_title(html: &str) -> String {
        // Simple regex-free extraction
        // Look for content between <h1> and </h1> or <h2> and </h2>
        for tag in ["h1", "h2"] {
            let open = format!("<{}", tag);
            let close = format!("</{}>", tag);

            if let Some(start) = html.find(&open) {
                if let Some(tag_end) = html[start..].find('>') {
                    let content_start = start + tag_end + 1;
                    if let Some(end) = html[content_start..].find(&close) {
                        let raw = &html[content_start..content_start + end];
                        return Self::strip_html_tags(raw);
                    }
                }
            }
        }
        String::new()
    }

    /// Extract body paragraphs from HTML
    fn extract_body_text(html: &str) -> Vec<String> {
        let mut paragraphs = Vec::new();
        let mut search_start = 0;

        while let Some(p_start) = html[search_start..].find("<p") {
            let abs_start = search_start + p_start;
            if let Some(tag_end) = html[abs_start..].find('>') {
                let content_start = abs_start + tag_end + 1;
                if let Some(p_end) = html[content_start..].find("</p>") {
                    let raw = &html[content_start..content_start + p_end];
                    let text = Self::strip_html_tags(raw).trim().to_string();
                    if !text.is_empty() && text.len() > 10 {
                        paragraphs.push(text);
                    }
                    search_start = content_start + p_end + 4;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Limit to first 5 paragraphs
        paragraphs.truncate(5);
        paragraphs
    }

    /// Strip HTML tags from string
    fn strip_html_tags(html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }

        result
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
    }

    /// Determine layout based on slide ID
    fn determine_layout(slide_id: &str) -> SlideLayout {
        match slide_id {
            "cover" => SlideLayout::Title,
            "toc" | "intro" | "solutions" => SlideLayout::SectionHeader,
            "closing" => SlideLayout::Title,
            _ => SlideLayout::TitleAndBody,
        }
    }
}

impl Default for GoogleSlidesExportPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudExportPlugin for GoogleSlidesExportPlugin {
    fn id(&self) -> &'static str {
        "builtin.export.google_slides"
    }

    fn provider(&self) -> &'static str {
        "google_slides"
    }

    fn display_name(&self) -> &'static str {
        "Google Slides"
    }

    fn export(&self, slides: &[SlideOutput], title: &str) -> Result<CloudExportOutput, String> {
        let service = self
            .service
            .as_ref()
            .ok_or("Google Slides service not configured. Please check Google API credentials.")?;

        // 1. Create new presentation
        let presentation_id = service.create_presentation(title)?;

        // 2. Convert HTML slides to SlideContent
        let slide_contents: Vec<SlideContent> = slides.iter().map(Self::parse_slide_html).collect();

        // 3. Add slides to presentation
        service.add_slides(&presentation_id, &slide_contents)?;

        // 4. Return result
        let url = service.get_presentation_url(&presentation_id);

        Ok(CloudExportOutput {
            url,
            provider: "google_slides".to_string(),
            resource_id: presentation_id,
            slides_count: slides.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title() {
        let html = r#"<div><h1 class="text-5xl">My Title</h1><p>Content</p></div>"#;
        let title = GoogleSlidesExportPlugin::extract_title(html);
        assert_eq!(title, "My Title");
    }

    #[test]
    fn test_extract_body() {
        let html = r#"<p class="small">Short</p><p>This is a longer paragraph with content.</p><p>Another paragraph here.</p>"#;
        let body = GoogleSlidesExportPlugin::extract_body_text(html);
        assert_eq!(body.len(), 2); // Short one should be filtered
    }

    #[test]
    fn test_strip_tags() {
        let html = "<span class=\"bold\">Hello <em>World</em></span>";
        let text = GoogleSlidesExportPlugin::strip_html_tags(html);
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_layout_mapping() {
        assert_eq!(SlideLayout::Title.to_google_layout(), "TITLE");
        assert_eq!(
            SlideLayout::TitleAndBody.to_google_layout(),
            "TITLE_AND_BODY"
        );
    }
}
