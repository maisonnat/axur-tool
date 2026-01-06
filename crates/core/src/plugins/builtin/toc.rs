//! TOC (Table of Contents) Slide Plugin

use super::helpers::{footer_light, geometric_pattern};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

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

        let items_html: String = items.iter().map(|item| format!(
            r#"<div class="flex items-center gap-4"><svg fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24" class="w-8 h-8 text-zinc-400 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"></path><path stroke-linecap="round" stroke-linejoin="round" d="M12 16L16 12L12 8"></path><path stroke-linecap="round" stroke-linejoin="round" d="M8 12H16"></path></svg><span class="text-3xl text-zinc-700 break-words leading-tight max-w-[90%]">{}</span></div>"#, item
        )).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100 p-0"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full w-full"><div class="w-8/12 p-14 flex flex-col justify-center"><div class="mb-12"><span class="bg-orange-600 text-white px-4 py-2 text-md font-semibold">{title}</span></div><div class="space-y-5">{items}</div></div><div class="w-4/12 relative bg-zinc-800 rounded-l-xl overflow-hidden">{pattern}</div></div></div>{footer}</div></div>"#,
            items = items_html,
            title = t.get("toc_title"),
            pattern = geometric_pattern(),
            footer = footer_light(4, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "toc".into(),
            html,
        }]
    }
}
