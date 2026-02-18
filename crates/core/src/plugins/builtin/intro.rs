//! Intro Slide Plugin — ACT 1: The Ordinary World
//!
//! Narrative Role: Set the context. Explain the monitoring scope and approach.
//! This is the RECIPROCITY moment — give value before asking for anything.
//!
//! Persuasion: Reciprocity (free context) + Authority (methodology credibility)
//! Design: Split layout, glassmorphism text panel, orange accent line

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the intro slide
pub struct IntroSlidePlugin;

impl SlidePlugin for IntroSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.intro"
    }
    fn name(&self) -> &'static str {
        "Introduction"
    }
    fn priority(&self) -> i32 {
        99
    } // Right after cover — RECIPROCITY: give context first

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let text = if data.is_dynamic_window {
            t.get("intro_text_dynamic")
        } else {
            t.get("intro_text_static")
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <!-- Background is Global -->
                {bg_pattern}
                
                <div class="flex-grow grid grid-cols-12 gap-8 h-full relative z-10">
                    <!-- Left: Title (F-PATTERN: Top-left anchor) -->
                    <div class="col-span-4 flex flex-col justify-center">
                         <div class="w-20 h-1 bg-orange-500 mb-8 shadow-[0_0_15px_#FF671F]"></div>
                         <h2 class="text-6xl font-black leading-tight display-text uppercase tracking-tight">{title}</h2>
                    </div>
                    
                    <!-- Right: Text Content in Glass (RECIPROCITY: Free insight) -->
                    <div class="col-span-8 flex flex-col justify-center pl-10">
                        <div class="glass-panel p-10 backdrop-blur-md bg-zinc-900/40 relative">
                             <!-- Decorative accent -->
                             <div class="absolute -top-6 -right-6 w-32 h-32 bg-orange-500/10 rounded-full blur-2xl pointer-events-none"></div>
                             
                             <div class="text-zinc-200 text-xl leading-relaxed font-light space-y-6 relative z-10">
                                <p>{text}</p>
                                <p class="text-white font-medium">{closing}</p>
                             </div>
                        </div>
                    </div>
                </div>
                
                <!-- ZEIGARNIK EFFECT: Open loop to Act 2 -->
                <div class="absolute bottom-24 right-14 z-50">
                    <div class="bg-orange-500/10 border border-orange-500/20 backdrop-blur-md px-5 py-3 rounded-xl flex items-center gap-3 hover:bg-orange-500/20 transition-colors duration-300 cursor-pointer">
                        <div>
                            <p class="text-[10px] text-orange-400 uppercase tracking-widest mb-0.5">Siguiente Capítulo</p>
                            <p class="text-sm font-bold text-white">Panorama de Amenazas →</p>
                        </div>
                    </div>
                </div>

                {footer}
            </div></div>"#,
            title = t.get("intro_title"),
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            text = text,
            closing = t.get("intro_text_closing"),
            footer = crate::plugins::builtin::helpers::footer_dark(2, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "intro".into(),
            html,
        }]
    }
}
