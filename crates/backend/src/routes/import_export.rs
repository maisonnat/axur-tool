//! Import/Export routes for templates
//! Handles PPTX import and various export formats

use axum::{response::Json, routing::post, Router};
use axum_extra::extract::Multipart;
use serde::Serialize;

/// Response for PPTX import
#[derive(Debug, Serialize)]
pub struct ImportPptxResponse {
    pub success: bool,
    pub slides: Vec<ImportedSlide>,
    pub message: String,
}

/// Imported slide structure
#[derive(Debug, Serialize)]
pub struct ImportedSlide {
    pub id: String,
    pub name: String,
    /// Fabric.js compatible canvas JSON
    pub canvas_json: String,
    /// Any images extracted (base64)
    pub images: Vec<String>,
}

/// Parse PPTX and extract slides
///
/// For MVP, we'll convert PPTX to basic slide structures.
/// Full PPTX parsing is complex - this is a simplified version.
pub async fn import_pptx(mut multipart: Multipart) -> Json<ImportPptxResponse> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name = String::new();

    // Extract file from multipart
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            file_name = field.file_name().unwrap_or("upload.pptx").to_string();
            if let Ok(data) = field.bytes().await {
                file_data = Some(data.to_vec());
            }
        }
    }

    let Some(data) = file_data else {
        return Json(ImportPptxResponse {
            success: false,
            slides: vec![],
            message: "No file uploaded".to_string(),
        });
    };

    tracing::info!("Importing PPTX: {} ({} bytes)", file_name, data.len());

    // Try to parse PPTX (ZIP file with XML inside)
    match parse_pptx(&data) {
        Ok(slides) => Json(ImportPptxResponse {
            success: true,
            slides,
            message: format!("Imported {} slides", file_name),
        }),
        Err(e) => {
            tracing::error!("PPTX import failed: {}", e);
            Json(ImportPptxResponse {
                success: false,
                slides: vec![],
                message: format!("Import failed: {}", e),
            })
        }
    }
}

/// Parse PPTX file and extract slide data
fn parse_pptx(data: &[u8]) -> Result<Vec<ImportedSlide>, String> {
    use std::io::{Cursor, Read};
    use zip::ZipArchive;

    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Invalid PPTX file: {}", e))?;

    let mut slides = Vec::new();
    let mut slide_count = 0;

    // Find all slide*.xml files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();

        if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
            slide_count += 1;

            // Read slide XML
            let mut xml_content = String::new();
            file.read_to_string(&mut xml_content).ok();

            // Extract basic text and shapes from XML
            let elements = extract_elements_from_xml(&xml_content);

            // Create Fabric.js compatible JSON
            let canvas_json = create_fabric_json(&elements);

            slides.push(ImportedSlide {
                id: format!("slide-{}", slide_count),
                name: format!("Slide {}", slide_count),
                canvas_json,
                images: vec![],
            });
        }
    }

    // Extract images from media folder
    let images = extract_images_from_pptx(&mut archive);

    // Associate images with slides (simplified - just add to first slide)
    if !slides.is_empty() && !images.is_empty() {
        slides[0].images = images;
    }

    if slides.is_empty() {
        // If no slides found, create a placeholder
        slides.push(ImportedSlide {
            id: "slide-1".to_string(),
            name: "Imported Slide".to_string(),
            canvas_json: "{}".to_string(),
            images: vec![],
        });
    }

    Ok(slides)
}

/// Extract text and shape elements from PPTX slide XML
fn extract_elements_from_xml(xml: &str) -> Vec<SlideElement> {
    let mut elements = Vec::new();

    // Very basic XML parsing - look for text runs
    // In production, use quick-xml or similar
    let mut y_pos = 100.0;

    // Find text content (simplified regex-like approach)
    for line in xml.lines() {
        if line.contains("<a:t>") && line.contains("</a:t>") {
            if let Some(start) = line.find("<a:t>") {
                if let Some(end) = line.find("</a:t>") {
                    let text = &line[start + 5..end];
                    if !text.trim().is_empty() {
                        elements.push(SlideElement::Text {
                            content: text.to_string(),
                            x: 100.0,
                            y: y_pos,
                            font_size: 24.0,
                        });
                        y_pos += 40.0;
                    }
                }
            }
        }
    }

    elements
}

/// Create Fabric.js compatible JSON from elements
fn create_fabric_json(elements: &[SlideElement]) -> String {
    let mut objects = Vec::new();

    for element in elements {
        match element {
            SlideElement::Text {
                content,
                x,
                y,
                font_size,
            } => {
                objects.push(serde_json::json!({
                    "type": "i-text",
                    "left": x,
                    "top": y,
                    "text": content,
                    "fontSize": font_size,
                    "fill": "#f8fafc",
                    "fontFamily": "Inter, sans-serif"
                }));
            }
            SlideElement::Rect {
                x,
                y,
                width,
                height,
            } => {
                objects.push(serde_json::json!({
                    "type": "rect",
                    "left": x,
                    "top": y,
                    "width": width,
                    "height": height,
                    "fill": "#3f3f46",
                    "stroke": "#52525b",
                    "strokeWidth": 1
                }));
            }
            SlideElement::Image { data, x, y, .. } => {
                objects.push(serde_json::json!({
                    "type": "image",
                    "left": x,
                    "top": y,
                    "src": data
                }));
            }
        }
    }

    let canvas = serde_json::json!({
        "version": "5.3.0",
        "objects": objects,
        "background": "#1e293b"
    });

    serde_json::to_string(&canvas).unwrap_or_else(|_| "{}".to_string())
}

/// Extract images from PPTX media folder
fn extract_images_from_pptx<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> Vec<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use std::io::Read;

    let mut images = Vec::new();

    for i in 0..archive.len() {
        if let Ok(mut file) = archive.by_index(i) {
            let name = file.name().to_string();

            if name.starts_with("ppt/media/") {
                let ext = name.rsplit('.').next().unwrap_or("");
                let mime = match ext.to_lowercase().as_str() {
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "gif" => "image/gif",
                    _ => continue,
                };

                let mut data = Vec::new();
                if file.read_to_end(&mut data).is_ok() {
                    let b64 = STANDARD.encode(&data);
                    images.push(format!("data:{};base64,{}", mime, b64));
                }
            }
        }
    }

    images
}

/// Slide element types
#[derive(Debug)]
#[allow(dead_code)]
enum SlideElement {
    Text {
        content: String,
        x: f64,
        y: f64,
        font_size: f64,
    },
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Image {
        data: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
}

/// Create router for import/export endpoints
pub fn router() -> Router {
    Router::new().route("/pptx", post(import_pptx))
}
