use crate::utils::coords::px_to_emu;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlideEdit {
    pub slide_index: usize, // 1-based index from frontend
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub placeholder_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InjectionRequest {
    pub edits: Vec<SlideEdit>,
}

pub fn inject_edits(original_pptx: &[u8], edits: Vec<SlideEdit>) -> Result<Vec<u8>, anyhow::Error> {
    let cursor = Cursor::new(original_pptx);
    let mut archive = ZipArchive::new(cursor)?;

    let mut out_buffer = Vec::new();
    let mut zip_writer = ZipWriter::new(Cursor::new(&mut out_buffer));

    // Group edits by slide index for faster lookup
    let mut edits_by_slide: std::collections::HashMap<usize, Vec<SlideEdit>> =
        std::collections::HashMap::new();
    for edit in edits {
        edits_by_slide
            .entry(edit.slide_index)
            .or_default()
            .push(edit);
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();
        let options = FileOptions::<()>::default().compression_method(file.compression());

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // Check if this file is a slide that needs editing
        if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
            // Extract number: slide1.xml -> 1
            let num_part = name
                .trim_start_matches("ppt/slides/slide")
                .trim_end_matches(".xml");

            if let Ok(idx) = num_part.parse::<usize>() {
                if let Some(slide_edits) = edits_by_slide.get(&idx) {
                    // INJECT XML HERE
                    let content_str = String::from_utf8(content.clone())?;
                    if content_str.contains("</p:spTree>") {
                        let mut new_shapes_xml = String::new();
                        for edit in slide_edits {
                            new_shapes_xml.push_str(&create_textbox_xml(edit));
                        }

                        let modified_xml = content_str.replace(
                            "</p:spTree>",
                            &format!("{}{}", new_shapes_xml, "</p:spTree>"),
                        );
                        content = modified_xml.into_bytes();
                        tracing::info!("Injected {} edits into {}", slide_edits.len(), name);
                    }
                }
            }
        }

        zip_writer.start_file(name, options)?;
        zip_writer.write_all(&content)?;
    }

    let cursor = zip_writer.finish()?;
    Ok(cursor.into_inner().to_vec())
}

fn create_textbox_xml(edit: &SlideEdit) -> String {
    let x_emu = px_to_emu(edit.x);
    let y_emu = px_to_emu(edit.y);
    let cx_emu = px_to_emu(edit.width);
    let cy_emu = px_to_emu(edit.height);
    let shape_id = 5000 + (edit.x as u32) + (edit.y as u32);
    // Use placeholder_key for name if available, otherwise shape_id
    let shape_name = edit
        .placeholder_key
        .clone()
        .map(|k| format!("Placeholder_{}", k))
        .unwrap_or(format!("Placeholder_{}", shape_id));

    format!(
        r#"
    <p:sp>
        <p:nvSpPr>
            <p:cNvPr id="{}" name="{}"/>
            <p:cNvSpPr txBox="1"/>
            <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
            <a:xfrm>
                <a:off x="{}" y="{}"/>
                <a:ext cx="{}" cy="{}"/>
            </a:xfrm>
            <a:prstGeom prst="rect">
                <a:avLst/>
            </a:prstGeom>
            <a:noFill/> 
        </p:spPr>
        <p:txBody>
            <a:bodyPr wrap="square" rtlCol="0"/>
            <a:lstStyle/>
            <a:p>
                <a:r>
                    <a:rPr lang="en-US" sz="1800" dirty="0">
                        <a:solidFill>
                            <a:srgbClr val="000000"/>
                        </a:solidFill> 
                     </a:rPr>
                    <a:t>{}</a:t>
                </a:r>
            </a:p>
        </p:txBody>
    </p:sp>
    "#,
        shape_id, shape_name, x_emu, y_emu, cx_emu, cy_emu, edit.text
    )
}
