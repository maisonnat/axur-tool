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
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full w-full flex flex-col"><div class="h-[25%] w-full flex justify-end flex-shrink-0"><div class="w-7/12 h-full"><div class="w-full h-full relative"><div class="absolute bg-white" style="top:25%;left:10%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:20%;width:10%;height:55%"></div><div class="absolute bg-black" style="top:55%;left:20%;width:20%;height:30%"></div><div class="absolute bg-black" style="top:0%;left:40%;width:10%;height:25%"></div><div class="absolute bg-white" style="top:25%;left:40%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:55%;left:40%;width:10%;height:30%"></div><div class="absolute bg-white" style="top:0%;left:60%;width:10%;height:55%"></div><div class="absolute bg-black" style="top:55%;left:60%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:70%;width:20%;height:25%"></div><div class="absolute bg-black" style="top:25%;left:70%;width:10%;height:30%"></div><div class="absolute bg-white" style="top:55%;left:70%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:25%;left:80%;width:10%;height:30%"></div><div class="absolute bg-black" style="top:55%;left:80%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:90%;width:10%;height:85%"></div></div></div></div><div class="flex-grow grid grid-cols-5 gap-x-12 items-start pt-8"><div class="col-span-2"><h2 class="text-4xl font-bold leading-tight text-orange-500">{title}</h2></div><div class="col-span-3 text-zinc-300 space-y-6 text-base leading-relaxed"><p>{text}</p><p>{closing}</p></div></div></div></div>{footer}</div></div>"#,
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
