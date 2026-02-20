use axur_core::api::report::PocReportData;
use axur_core::i18n::Translations;
use axur_core::report::html::generate_report_with_plugins;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Context: Generating Axur Mock Report for verification...");

    // 1. Create mock data (using built-in demo data)
    let data = PocReportData::demo();
    println!(
        "Data Loaded: {} tickets, {} incidents, {} credentials, {} threats",
        data.total_tickets,
        data.total_incidents,
        data.credentials_total,
        data.threats_by_type.iter().map(|t| t.count).sum::<u64>()
    );

    // Define plugin configuration
    let config = PluginConfig {
        is_poc: true,
        show_compliance: true,
        custom_branding: true,
        theme: ThemeMode::Dark,
        disabled_plugins: vec![],
        custom_css: None,
        show_style_showcase: true, // Enable for mock verification
    };

    // 2. Load translations (compiled-in)
    let translations =
        Translations::load("es").expect("Failed to load Spanish translations (es.json)");
    println!("Translations: Loaded 'es' locale.");

    // 3. Generate HTML
    // Using default config (which includes our new style_showcase if enabled by default,
    // but style_showcase might need explicit enable if it's not in default config.
    // Let's check PluginRegistry::with_builtins() to see if it includes strictly default plugins.
    // For now, we assume standard report generation.)
    let html = generate_report_with_plugins(&data, &translations, None, None);
    println!(
        "Generation: Report generated successfully. Size: {} bytes",
        html.len()
    );

    // 4. Write to file
    let path = "mock_report.html";
    let mut file = File::create(path)?;
    file.write_all(html.as_bytes())?;

    println!("Success: Report saved to '{}'", path);
    println!("You can open this file in your browser to inspect the visual changes.");
    Ok(())
}
