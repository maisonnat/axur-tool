use axur_core::api::report::PocReportData;
use axur_core::i18n::Translations;
use axur_core::report::html::generate_report_with_plugins;
use std::fs;

fn main() {
    println!("Generating Neuro-Design Mock Report...");

    // 1. Create Demo Data
    let data = PocReportData::demo();

    // 2. Load Translations
    let translations = Translations::load("es").expect("Failed to load translations");

    // 3. Generate HTML with Plugins
    let html = generate_report_with_plugins(&data, &translations, None, None);

    // 4. Save to file
    let output_path = "mock_report.html";
    fs::write(output_path, html).expect("Unable to write file");

    println!("Success! Report generated at: {}", output_path);

    // Print absolute path for clarity
    if let Ok(abs_path) = fs::canonicalize(output_path) {
        println!("Absolute path: {:?}", abs_path);
    }
}
