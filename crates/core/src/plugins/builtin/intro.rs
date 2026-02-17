//! Intro Slide Plugin
//!
//! Introduction slide explaining Axur's approach.

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
    } // Right after cover

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
                
                <div class="flex-grow grid grid-cols-12 gap-8 h-full relative z-10">
                    <!-- Left: Title -->
                    <div class="col-span-4 flex flex-col justify-center">
                         <div class="w-20 h-1 bg-orange-500 mb-8 shadow-[0_0_15px_#FF5824]"></div>
                         <h2 class="text-6xl font-black leading-tight display-text uppercase tracking-tight">{title}</h2>
                    </div>
                    
                    <!-- Right: Text Content in Glass -->
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
                
                {footer}
            </div></div>"#,
            title = t.get("intro_title"),
            text = text,
            closing = t.get("intro_text_closing"),
            footer = Self::footer_dark(2, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "intro".into(),
            html,
        }]
    }
}

impl IntroSlidePlugin {
    fn footer_dark(page: u32, footer_text: &str) -> String {
        format!(
            r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-orange-500 text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-400"><span>{}</span><span class="ml-4">{}</span></div></footer>"#,
            footer_text, page
        )
    }
}
