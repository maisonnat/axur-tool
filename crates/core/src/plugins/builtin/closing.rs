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
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <div class="flex-grow h-full flex flex-col justify-center items-center text-center relative z-10">
                    
                    <h2 class="text-6xl font-black mb-6 display-text tracking-tight uppercase">{title}</h2>
                    <p class="text-2xl text-zinc-300 mb-16 max-w-3xl font-light">{subtitle}</p>
                    
                    <div class="grid grid-cols-2 gap-10 max-w-5xl w-full">
                        <!-- Card 1 -->
                        <div class="glass-panel p-10 flex flex-col items-center hover:scale-[1.02] transition-transform duration-300 group/card">
                            <div class="bg-orange-500/10 p-5 rounded-2xl mb-6 ring-1 ring-orange-500/20 group-hover/card:ring-orange-500/50 transition-all">
                                <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z"></path></svg>
                            </div>
                            <h3 class="text-2xl font-bold mb-3 text-white">{cta_activate}</h3>
                            <p class="text-zinc-400 text-sm leading-relaxed">{cta_activate_desc}</p>
                        </div>
                        
                        <!-- Card 2 -->
                        <div class="glass-panel p-10 flex flex-col items-center hover:scale-[1.02] transition-transform duration-300 group/card">
                            <div class="bg-orange-500/10 p-5 rounded-2xl mb-6 ring-1 ring-orange-500/20 group-hover/card:ring-orange-500/50 transition-all">
                                <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"></path></svg>
                            </div>
                            <h3 class="text-2xl font-bold mb-3 text-white">{cta_meet}</h3>
                            <p class="text-zinc-400 text-sm leading-relaxed">{cta_meet_desc}</p>
                        </div>
                    </div>
                </div>
                {footer}
            </div></div>"#,
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
