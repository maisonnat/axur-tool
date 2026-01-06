//! Examples Slide Plugin
//!
//! Shows examples of detected threats and resolved takedowns.

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};
use super::helpers::footer_light;

pub struct TakedownExamplesSlidePlugin;

impl SlidePlugin for TakedownExamplesSlidePlugin {
    fn id(&self) -> &'static str { "builtin.takedown_examples" }
    fn name(&self) -> &'static str { "Takedown Examples" }
    fn priority(&self) -> i32 { 25 }
    
    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.takedown_examples.is_empty()
    }
    
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;
        
        // Take up to 4 examples
        let examples_html: String = data.takedown_examples.iter().take(4).map(|ex| {
            format!(
                r#"<div class="bg-white p-4 rounded-lg shadow border border-zinc-200"><p class="font-semibold text-zinc-900 truncate">{}</p><p class="text-xs text-zinc-500 mt-1">{} {}</p><p class="text-xs text-zinc-400 truncate">{} {}</p></div>"#,
                ex.name,
                t.get("example_label_type"),
                ex.ticket_type,
                t.get("example_label_url"),
                ex.url
            )
        }).collect();
        
        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-green-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-2 gap-6 flex-grow">{examples}</div></div></div>{footer}</div></div>"#,
            title = t.get("examples_takedowns_title"),
            examples = examples_html,
            footer = footer_light(13, &t.get("footer_text")),
        );
        
        vec![SlideOutput { id: "takedown_examples".into(), html }]
    }
}

pub struct PocExamplesSlidePlugin;

impl SlidePlugin for PocExamplesSlidePlugin {
    fn id(&self) -> &'static str { "builtin.poc_examples" }
    fn name(&self) -> &'static str { "PoC Evidence Examples" }
    fn priority(&self) -> i32 { 20 }
    
    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.poc_examples.is_empty()
    }
    
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;
        
        // Take up to 4 examples
        let examples_html: String = data.poc_examples.iter().take(4).map(|ex| {
            let img_html = ex.screenshot_url.as_ref().map(|url| {
                format!(r#"<img src="{}" class="w-full h-24 object-cover rounded mb-2" alt="screenshot"/>"#, url)
            }).unwrap_or_else(|| format!(r#"<div class="w-full h-24 bg-zinc-200 rounded mb-2 flex items-center justify-center text-zinc-400 text-xs">{}</div>"#, t.get("example_no_image")));
            
            format!(
                r#"<div class="bg-white p-4 rounded-lg shadow border border-zinc-200">{}<p class="font-semibold text-zinc-900 truncate">{}</p><p class="text-xs text-zinc-500 mt-1">{} {}</p><p class="text-xs text-zinc-400 truncate">{}</p></div>"#,
                img_html,
                ex.ticket_key,
                t.get("example_label_type"),
                ex.evidence_type,
                ex.reference_url
            )
        }).collect();
        
        let no_data_html = if data.poc_examples.is_empty() {
            format!(r#"<div class="col-span-2 flex items-center justify-center h-full text-zinc-500 text-lg">{}</div>"#, t.get("example_no_data"))
        } else {
            String::new()
        };
        
        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">EVIDENCIA</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-2 gap-6 flex-grow">{examples}{no_data}</div></div></div>{footer}</div></div>"#,
            title = t.get("examples_poc_title"),
            examples = examples_html,
            no_data = no_data_html,
            footer = footer_light(14, &t.get("footer_text")),
        );
        
        vec![SlideOutput { id: "poc_examples".into(), html }]
    }
}
