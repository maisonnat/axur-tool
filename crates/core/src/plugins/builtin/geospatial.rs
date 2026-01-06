//! Geospatial Slide Plugin
//!
//! Shows geographic distribution of threats.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};
use std::collections::HashMap;

pub struct GeospatialSlidePlugin;

impl SlidePlugin for GeospatialSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.geospatial"
    }
    fn name(&self) -> &'static str {
        "Geospatial Analysis"
    }
    fn priority(&self) -> i32 {
        84
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.deep_investigations.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Aggregate countries from investigations
        let mut countries: HashMap<String, u32> = HashMap::new();
        for inv in &data.deep_investigations {
            if let Some(country) = &inv.infrastructure.country {
                *countries.entry(country.clone()).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<_> = countries.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let rows_html: String = sorted.iter().take(10).map(|(country, count)| {
            format!(
                r#"<div class="flex items-center justify-between p-3 bg-zinc-900 rounded mb-2"><span class="font-medium">{}</span><span class="px-3 py-1 bg-orange-600 rounded text-sm">{}</span></div>"#,
                country, count
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-green-600 px-4 py-1 text-sm font-semibold">GEOSPATIAL</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-2 gap-8"><div class="flex-grow">{rows}</div><div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 flex items-center justify-center"><p class="text-zinc-500 text-center">{map_placeholder}</p></div></div></div></div>{footer}</div></div>"#,
            title = t.get("geo_title"),
            rows = rows_html,
            map_placeholder = t.get("geo_map_placeholder"),
            footer = footer_dark(14, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "geospatial".into(),
            html,
        }]
    }
}
