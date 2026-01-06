//! Closing Slide Plugin
//!
//! Call-to-action slide for report conclusion.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the closing/CTA slide
pub struct ClosingSlidePlugin;

impl SlidePlugin for ClosingSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.closing"
    }
    fn name(&self) -> &'static str {
        "Closing CTA"
    }
    fn priority(&self) -> i32 {
        10
    } // Last slide

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let t = ctx.translations;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col justify-center items-center text-center"><h2 class="text-5xl font-bold mb-6">{title}</h2><p class="text-xl text-zinc-400 mb-12 max-w-3xl">{subtitle}</p><div class="grid grid-cols-2 gap-8 max-w-4xl"><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z"></path></svg></div><h3 class="text-xl font-bold mb-2">{cta_activate}</h3><p class="text-zinc-400 text-sm">{cta_activate_desc}</p></div><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"></path></svg></div><h3 class="text-xl font-bold mb-2">{cta_meet}</h3><p class="text-zinc-400 text-sm">{cta_meet_desc}</p></div></div></div></div>{footer}</div></div>"#,
            title = t.get("closing_title"),
            subtitle = t.get("closing_subtitle"),
            cta_activate = t.get("closing_cta_activate"),
            cta_activate_desc = t.get("closing_cta_activate_desc"),
            cta_meet = t.get("closing_cta_meet"),
            cta_meet_desc = t.get("closing_cta_meet_desc"),
            footer = footer_dark(15, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "closing".into(),
            html,
        }]
    }
}
