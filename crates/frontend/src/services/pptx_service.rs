//! PPTX Import Service
//!
//! Handles uploading .pptx files to backend for preview generation and
//! managing the slide data. Replaces legacy inline JS `PPTXImporter`.

use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlidePreview {
    pub slide_number: usize,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub slides: Vec<String>, // URLs
    pub message: Option<String>,
}

pub struct PptxService;

impl PptxService {
    /// Uploads a PPTX file to the backend and returns slide previews
    pub async fn import_file(file: web_sys::File) -> Result<Vec<SlidePreview>, String> {
        let form_data = web_sys::FormData::new().map_err(|e| format!("{:?}", e))?;
        form_data
            .append_with_blob("file", &file)
            .map_err(|e| format!("{:?}", e))?;

        let resp = Request::post("/api/import/pptx")
            .body(form_data)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.ok() {
            return Err(format!("Upload failed: {}", resp.status_text()));
        }

        let result: ImportResult = resp.json().await.map_err(|e| e.to_string())?;

        if !result.success {
            return Err(result.message.unwrap_or_else(|| "Unknown error".into()));
        }

        let slides = result
            .slides
            .into_iter()
            .enumerate()
            .map(|(i, url)| SlidePreview {
                slide_number: i + 1,
                image_url: url,
            })
            .collect();

        Ok(slides)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PptxEdit {
    pub slide_index: usize,
    pub text: Option<String>,
    pub placeholder_key: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PptxService {
    // ... import_file ...

    /// Exports the presentation with injected edits
    pub async fn export_pptx(
        original_file: web_sys::File,
        edits: Vec<PptxEdit>,
    ) -> Result<web_sys::Blob, String> {
        let form_data = web_sys::FormData::new().map_err(|e| format!("{:?}", e))?;
        form_data
            .append_with_blob("file", &original_file)
            .map_err(|e| format!("{:?}", e))?;

        let edits_json = serde_json::to_string(&edits).map_err(|e| e.to_string())?;
        form_data
            .append_with_str("edits", &edits_json)
            .map_err(|e| format!("{:?}", e))?;

        let resp = Request::post("/api/export/inject")
            .body(form_data)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.ok() {
            return Err(format!("Export failed: {}", resp.status_text()));
        }

        let bytes = resp.binary().await.map_err(|e| e.to_string())?;
        let array = js_sys::Uint8Array::from(&bytes[..]);
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &js_sys::Array::of1(&array),
            &web_sys::BlobPropertyBag::new()
                .type_("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        )
        .map_err(|e| format!("{:?}", e))?;
        Ok(blob)
    }
}
