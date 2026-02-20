//! Screenshot Service
//!
//! Bindings for html2canvas to capture Safe DOM screenshots.
//! Replaces `window.captureScreenshot`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = captureScreenshot)]
    async fn capture_screenshot_js() -> JsValue;
}

pub struct ScreenshotService;

impl ScreenshotService {
    /// Captures the current screen as a base64 JPEG string
    pub async fn capture() -> Option<String> {
        let result = capture_screenshot_js().await;
        result.as_string()
    }
}
