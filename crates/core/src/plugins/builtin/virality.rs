//! Virality Slide Plugin
//!
//! Shows threat spread across platforms.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct ViralitySlidePlugin;

impl SlidePlugin for ViralitySlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.virality"
    }
    fn name(&self) -> &'static str {
        "Virality"
    }
    fn priority(&self) -> i32 {
        87
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        let ti = &ctx.data.threat_intelligence;
        ti.data_available && (ti.chat_group_shares > 0 || ti.social_media_mentions > 0)
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let ti = &ctx.data.threat_intelligence;
        let t = ctx.translations;

        let platforms_html: String = ti
            .platforms_detected
            .iter()
            .map(|p| {
                format!(
                    r#"<span class="px-3 py-1 bg-zinc-800 rounded-full text-sm">{}</span>"#,
                    p
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-purple-600 px-4 py-1 text-sm font-semibold">VIRALITY</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-3 gap-8 mb-8"><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 text-center"><p class="text-4xl font-bold text-purple-400">{chat}</p><p class="text-zinc-400 text-sm">{lbl_chat}</p></div><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 text-center"><p class="text-4xl font-bold text-blue-400">{social}</p><p class="text-zinc-400 text-sm">{lbl_social}</p></div><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 text-center"><p class="text-4xl font-bold text-orange-400">{dark}</p><p class="text-zinc-400 text-sm">{lbl_dark}</p></div></div><div class="flex flex-wrap gap-2">{platforms}</div></div></div>{footer}</div></div>"#,
            title = t.get("virality_title"),
            chat = ti.chat_group_shares,
            lbl_chat = t.get("virality_chat"),
            social = ti.social_media_mentions,
            lbl_social = t.get("virality_social"),
            dark = ti.dark_web_mentions,
            lbl_dark = t.get("virality_dark"),
            platforms = platforms_html,
            footer = footer_dark(12, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "virality".into(),
            html,
        }]
    }
}
