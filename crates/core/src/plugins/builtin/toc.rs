//! TOC (Table of Contents) Slide Plugin
//!
//! Displays the report table of contents with brand aesthetics.

use super::helpers::footer_light;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Embedded base64 TOC sidebar image
const TOC_IMAGE_BASE64: &str = include_str!("../../../assets/toc_image_base64.txt");

/// Plugin that generates the table of contents slide
pub struct TocSlidePlugin;

impl SlidePlugin for TocSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.toc"
    }
    fn name(&self) -> &'static str {
        "Table of Contents"
    }
    fn priority(&self) -> i32 {
        97
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let t = ctx.translations;

        // Get TOC items from translations (stored as JSON array)
        let items: Vec<String> = t
            .get_optional("toc_items")
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| {
                vec![
                    "General Metrics".into(),
                    "Digital Fraud".into(),
                    "Data Exposure".into(),
                    "Executives & VIPs".into(),
                    "Deep & Dark Web".into(),
                    "Threat Intelligence".into(),
                    "Operational Efficiency".into(),
                ]
            });

        // Generate TOC items with orange arrow icons
        let items_html: String = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                format!(
                    r#"<div class="flex items-center gap-4 group/item hover:translate-x-2 transition-transform">
                        <div class="flex items-center justify-center w-10 h-10 rounded-full bg-[#FF5824]/10 group-hover/item:bg-[#FF5824]/20 transition-colors">
                            <span class="text-[#FF5824] font-bold text-lg">{}</span>
                        </div>
                        <span class="text-2xl text-zinc-700 font-medium">{}</span>
                    </div>"#,
                    i + 1,
                    item
                )
            })
            .collect();

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex shadow-lg mb-8 relative bg-zinc-100 overflow-hidden">
                    <!-- Content Section -->
                    <div class="w-7/12 p-14 flex flex-col justify-center">
                        <!-- Badge -->
                        <div class="mb-10">
                            <span class="bg-[#FF5824] text-white px-5 py-2 text-sm font-bold tracking-wider uppercase">
                                {title}
                            </span>
                        </div>
                        <!-- TOC Items -->
                        <div class="space-y-5">
                            {items}
                        </div>
                    </div>
                    <!-- Sidebar Image -->
                    <div class="w-5/12 relative">
                        <img 
                            src="data:image/png;base64,{image}" 
                            alt="Brand visual" 
                            class="absolute inset-0 w-full h-full object-cover"
                        />
                        <!-- Gradient overlay for blending -->
                        <div class="absolute inset-0 bg-gradient-to-r from-zinc-100 to-transparent w-1/4"></div>
                    </div>
                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            title = t.get("toc_title"),
            items = items_html,
            image = TOC_IMAGE_BASE64.trim(),
            footer = footer_light(4, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "toc".into(),
            html,
        }]
    }
}
