//! Timeline Slide Plugin
//!
//! Shows story tickets on a timeline.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct TimelineSlidePlugin;

impl SlidePlugin for TimelineSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.timeline"
    }
    fn name(&self) -> &'static str {
        "Timeline"
    }
    fn priority(&self) -> i32 {
        89
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.story_tickets.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let items_html: String = data.story_tickets.iter().take(5).map(|ticket| {
            let status_color = match ticket.status.as_str() {
                "closed" => "bg-green-500",
                "incident" => "bg-red-500",
                _ => "bg-orange-500",
            };
            format!(
                r#"<div class="flex items-center gap-4 mb-4"><div class="w-3 h-3 rounded-full {color}"></div><div class="flex-grow bg-zinc-900 p-4 rounded-lg border border-zinc-800"><p class="font-bold text-white">{key}</p><p class="text-sm text-zinc-400">{desc}</p><p class="text-xs text-zinc-500">{threat_type}</p></div></div>"#,
                color = status_color,
                key = ticket.ticket_key,
                desc = ticket.description,
                threat_type = ticket.threat_type,
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">TIMELINE</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="flex-grow overflow-auto">{items}</div></div></div>{footer}</div></div>"#,
            title = t.get("timeline_title"),
            items = items_html,
            footer = footer_dark(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "timeline".into(),
            html,
        }]
    }
}
