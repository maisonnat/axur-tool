/// Convert screen pixels to English Metric Units (EMUs)
/// Assumption: 96 DPI, where 1 inch = 914400 EMUs / 96 = 9525 EMUs per pixel.
pub fn px_to_emu(px: f64) -> i64 {
    (px * 9525.0).round() as i64
}

/// Convert EMUs to screen pixels
pub fn emu_to_px(emu: i64) -> f64 {
    emu as f64 / 9525.0
}
