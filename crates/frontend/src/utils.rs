use crate::api;
use wasm_bindgen::JsCast;

pub fn get_today() -> String {
    let date = js_sys::Date::new_0();
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

pub fn get_default_from_date() -> String {
    let date = js_sys::Date::new_0();
    // Subtract 30 days in milliseconds (30 * 24 * 60 * 60 * 1000)
    let thirty_days_ms = 30.0 * 24.0 * 60.0 * 60.0 * 1000.0;
    date.set_time(date.get_time() - thirty_days_ms);
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

pub fn download_html(content: &str, filename: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create blob with proper options
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/html");

    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&content.into()),
        &options,
    )
    .unwrap();

    // Create URL
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    // Create and click download link
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();

    // Cleanup
    web_sys::Url::revoke_object_url(&url).unwrap();
}

/// Parse HTML report to extract slides for Google Slides export
pub fn parse_html_to_slides(html: &str) -> Vec<api::ExportSlideData> {
    let mut slides = Vec::new();

    // Simple regex-like parsing to extract slide sections
    // Look for section patterns in the HTML report

    // Pattern 1: Look for h1/h2 headers and following content
    let mut current_title = String::new();
    let mut current_body: Vec<String> = Vec::new();

    for line in html.lines() {
        let line = line.trim();

        // Check for slide section markers (common patterns in Axur reports)
        if line.contains("<section") || line.contains("class=\"slide\"") {
            // Save previous slide if exists
            if !current_title.is_empty() || !current_body.is_empty() {
                slides.push(api::ExportSlideData {
                    title: if current_title.is_empty() {
                        format!("Slide {}", slides.len() + 1)
                    } else {
                        current_title.clone()
                    },
                    body: current_body.clone(),
                    layout: Some("TITLE_AND_BODY".to_string()),
                });
            }
            current_title = String::new();
            current_body = Vec::new();
        }

        // Extract h1/h2 as titles
        if line.contains("<h1") || line.contains("<h2") {
            // Simple extraction between > and </h
            if let Some(start) = line.find('>') {
                if let Some(end) = line.find("</h") {
                    let title = &line[start + 1..end];
                    // Strip remaining HTML tags
                    current_title = title
                        .replace("<span>", "")
                        .replace("</span>", "")
                        .replace("<strong>", "")
                        .replace("</strong>", "")
                        .trim()
                        .to_string();
                }
            }
        }

        // Extract paragraphs as body text
        if line.contains("<p") && line.contains("</p>") {
            if let Some(start) = line.find('>') {
                if let Some(end) = line.rfind("</p>") {
                    let text = &line[start + 1..end];
                    // Strip HTML tags and get clean text
                    let clean_text: String = text
                        .replace("<span", "")
                        .replace("</span>", "")
                        .replace("<strong>", "")
                        .replace("</strong>", "")
                        .replace("<em>", "")
                        .replace("</em>", "")
                        .chars()
                        .filter(|c| !c.is_control())
                        .collect::<String>()
                        .trim()
                        .to_string();

                    if !clean_text.is_empty() && clean_text.len() > 5 {
                        current_body.push(clean_text);
                    }
                }
            }
        }
    }

    // Don't forget the last slide
    if !current_title.is_empty() || !current_body.is_empty() {
        slides.push(api::ExportSlideData {
            title: if current_title.is_empty() {
                format!("Slide {}", slides.len() + 1)
            } else {
                current_title
            },
            body: current_body,
            layout: Some("TITLE_AND_BODY".to_string()),
        });
    }

    // If no slides were extracted, create a basic one
    if slides.is_empty() {
        slides.push(api::ExportSlideData {
            title: "Threat Intelligence Report".to_string(),
            body: vec!["Report generated by Axur Web".to_string()],
            layout: Some("TITLE".to_string()),
        });
    }

    slides
}
