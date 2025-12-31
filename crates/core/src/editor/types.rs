//! Core types for the Slide Editor Platform

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A complete presentation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationTemplate {
    /// Unique identifier
    pub id: Uuid,
    /// Template name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Ordered list of slides
    pub slides: Vec<SlideDefinition>,
    /// Visual theme settings
    pub theme: Theme,
    /// Template version for compatibility
    pub version: String,
}

impl Default for PresentationTemplate {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Untitled Template".to_string(),
            description: None,
            slides: Vec::new(),
            theme: Theme::default(),
            version: "1.0.0".to_string(),
        }
    }
}

/// Definition of a single slide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideDefinition {
    /// Unique identifier within the template
    pub id: Uuid,
    /// Slide display name
    pub name: String,
    /// Layout preset
    pub layout: LayoutType,
    /// Elements on this slide
    pub elements: Vec<Element>,
    /// Background settings
    pub background: Background,
    /// Order in presentation (0-indexed)
    pub order: i32,
    /// Whether this slide is visible
    pub visible: bool,
    /// Raw Fabric.js JSON (optional, overrides elements if present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_json: Option<String>,
}

impl Default for SlideDefinition {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Slide".to_string(),
            layout: LayoutType::Blank,
            elements: Vec::new(),
            background: Background::default(),
            order: 0,
            visible: true,
            canvas_json: None,
        }
    }
}

/// Layout presets for slides
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LayoutType {
    /// Full width content
    FullWidth,
    /// Two equal columns
    TwoColumn,
    /// Three equal columns
    ThreeColumn,
    /// Title at top, content below
    TitleContent,
    /// Title only centered
    TitleOnly,
    /// Completely blank canvas
    #[default]
    Blank,
}

/// Any element that can be placed on a slide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Element {
    /// Text box with content
    Text {
        id: Uuid,
        content: String,
        style: TextStyle,
        position: Position,
        size: Size,
        locked: bool,
    },
    /// Dynamic placeholder that resolves to data
    Placeholder {
        id: Uuid,
        key: String, // PlaceholderKey as string for flexibility
        position: Position,
        size: Size,
        locked: bool,
    },
    /// Static or dynamic image
    Image {
        id: Uuid,
        src: ImageSource,
        position: Position,
        size: Size,
        locked: bool,
    },
    /// Geometric shape
    Shape {
        id: Uuid,
        shape_type: ShapeType,
        style: ShapeStyle,
        position: Position,
        size: Size,
        locked: bool,
    },
    /// Data-driven chart
    Chart {
        id: Uuid,
        chart_type: ChartType,
        data_source: String, // PlaceholderKey for data
        style: ChartStyle,
        position: Position,
        size: Size,
        locked: bool,
    },
}

/// Position on canvas (percentage-based for responsiveness)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    /// X position as percentage (0-100)
    pub x: f64,
    /// Y position as percentage (0-100)
    pub y: f64,
}

/// Size of an element (percentage-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    /// Width as percentage of slide (0-100)
    pub width: f64,
    /// Height as percentage of slide (0-100)
    pub height: f64,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 20.0,
            height: 10.0,
        }
    }
}

/// Text styling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// Font family (e.g., "Inter", "Roboto")
    pub font_family: String,
    /// Font size in pixels
    pub font_size: u32,
    /// Font weight (e.g., "normal", "bold", "600")
    pub font_weight: String,
    /// Text color as hex (e.g., "#FFFFFF")
    pub color: String,
    /// Text alignment
    pub text_align: TextAlign,
    /// Line height multiplier
    pub line_height: f64,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "Inter".to_string(),
            font_size: 16,
            font_weight: "normal".to_string(),
            color: "#333333".to_string(),
            text_align: TextAlign::Left,
            line_height: 1.5,
        }
    }
}

/// Text alignment options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

/// Theme settings for the presentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Primary brand color
    pub primary_color: String,
    /// Secondary accent color
    pub secondary_color: String,
    /// Background color
    pub background_color: String,
    /// Text color
    pub text_color: String,
    /// Primary font family
    pub font_family: String,
    /// Heading font family
    pub heading_font: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_color: "#6366F1".to_string(),    // Indigo
            secondary_color: "#EC4899".to_string(),  // Pink
            background_color: "#0F172A".to_string(), // Dark slate
            text_color: "#F8FAFC".to_string(),       // Light
            font_family: "Inter".to_string(),
            heading_font: "Inter".to_string(),
        }
    }
}

/// Background settings for a slide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Background {
    /// Solid color background
    Solid { color: String },
    /// Gradient background
    Gradient {
        start_color: String,
        end_color: String,
        direction: GradientDirection,
    },
    /// Image background
    Image { url: String, opacity: f64 },
}

impl Default for Background {
    fn default() -> Self {
        Self::Solid {
            color: "#0F172A".to_string(),
        }
    }
}

/// Gradient direction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GradientDirection {
    #[default]
    ToRight,
    ToBottom,
    ToBottomRight,
    ToBottomLeft,
}

/// Image source options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageSource {
    /// URL to an image
    Url { url: String },
    /// Base64 encoded image
    Base64 { data: String, mime_type: String },
    /// User-uploaded asset (stored in GitHub)
    Asset { path: String },
}

/// Available shape types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShapeType {
    #[default]
    Rectangle,
    RoundedRect,
    Circle,
    Ellipse,
    Triangle,
    Line,
    Arrow,
}

/// Shape styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    /// Fill color
    pub fill_color: String,
    /// Border color
    pub stroke_color: String,
    /// Border width in pixels
    pub stroke_width: f64,
    /// Corner radius (for rectangles)
    pub border_radius: f64,
    /// Opacity (0-1)
    pub opacity: f64,
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            fill_color: "#6366F1".to_string(),
            stroke_color: "#4F46E5".to_string(),
            stroke_width: 0.0,
            border_radius: 8.0,
            opacity: 1.0,
        }
    }
}

/// Chart types available
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    #[default]
    Bar,
    Line,
    Pie,
    Donut,
    Area,
    Gauge,
}

/// Chart styling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
    /// Colors for data series
    pub colors: Vec<String>,
    /// Show legend
    pub show_legend: bool,
    /// Show labels
    pub show_labels: bool,
    /// Title text
    pub title: Option<String>,
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self {
            colors: vec![
                "#6366F1".to_string(),
                "#EC4899".to_string(),
                "#10B981".to_string(),
                "#F59E0B".to_string(),
                "#8B5CF6".to_string(),
            ],
            show_legend: true,
            show_labels: true,
            title: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_serialization() {
        let template = PresentationTemplate::default();
        let json = serde_json::to_string_pretty(&template).unwrap();
        let parsed: PresentationTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(template.name, parsed.name);
    }

    #[test]
    fn test_slide_with_elements() {
        let slide = SlideDefinition {
            id: Uuid::new_v4(),
            name: "Test Slide".to_string(),
            layout: LayoutType::TwoColumn,
            elements: vec![
                Element::Text {
                    id: Uuid::new_v4(),
                    content: "Hello World".to_string(),
                    style: TextStyle::default(),
                    position: Position { x: 10.0, y: 10.0 },
                    size: Size {
                        width: 50.0,
                        height: 20.0,
                    },
                    locked: false,
                },
                Element::Placeholder {
                    id: Uuid::new_v4(),
                    key: "risk_score".to_string(),
                    position: Position { x: 60.0, y: 10.0 },
                    size: Size {
                        width: 30.0,
                        height: 40.0,
                    },
                    locked: false,
                },
            ],
            background: Background::default(),
            order: 0,
            visible: true,
            canvas_json: None,
        };

        let json = serde_json::to_string_pretty(&slide).unwrap();
        let parsed: SlideDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(slide.elements.len(), parsed.elements.len());
    }

    #[test]
    fn test_element_variants() {
        let text = Element::Text {
            id: Uuid::new_v4(),
            content: "Test".to_string(),
            style: TextStyle::default(),
            position: Position::default(),
            size: Size::default(),
            locked: false,
        };

        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("\"type\":\"text\""));
    }
}
