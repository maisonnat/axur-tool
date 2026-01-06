//! Threat Intelligence Slide Plugin
//!
//! Shows dark web and threat hunting intelligence.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct ThreatIntelSlidePlugin;

impl SlidePlugin for ThreatIntelSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.threat_intel"
    }
    fn name(&self) -> &'static str {
        "Threat Intelligence"
    }
    fn priority(&self) -> i32 {
        30
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        ctx.data.threat_intelligence.data_available
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let ti = &ctx.data.threat_intelligence;
        let t = ctx.translations;

        let sources_html: String = ti
            .dark_web_sources
            .iter()
            .take(5)
            .map(|s| format!(r#"<li class="text-zinc-300 text-sm">{}</li>"#, s))
            .collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-red-800 px-4 py-1 text-sm font-semibold">THREAT INTEL</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-2 gap-8"><div class="space-y-6"><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 class="text-lg font-semibold text-red-400 mb-4">{lbl_dark}</h3><div class="grid grid-cols-2 gap-4"><div><p class="text-3xl font-bold">{dark_mentions}</p><p class="text-xs text-zinc-500">{lbl_mentions}</p></div><div><p class="text-3xl font-bold">{credentials}</p><p class="text-xs text-zinc-500">{lbl_creds}</p></div></div></div><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 class="text-lg font-semibold text-orange-400 mb-4">{lbl_sources}</h3><ul class="list-disc list-inside">{sources}</ul></div></div><div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800"><h3 class="text-lg font-semibold text-blue-400 mb-4">{lbl_quality}</h3><div class="space-y-4"><div class="flex justify-between"><span class="text-zinc-400">Stealer Logs</span><span class="font-bold text-red-400">{stealer_pct:.1}%</span></div><div class="flex justify-between"><span class="text-zinc-400">Plain Passwords</span><span class="font-bold text-orange-400">{plain_pct:.1}%</span></div><div class="flex justify-between"><span class="text-zinc-400">Combolists</span><span class="font-bold text-zinc-400">{combo}</span></div></div></div></div></div></div>{footer}</div></div>"#,
            title = t.get("ti_title"),
            lbl_dark = t.get("ti_dark_web"),
            dark_mentions = ti.dark_web_mentions,
            lbl_mentions = t.get("ti_mentions"),
            credentials = format_number(ti.total_credentials),
            lbl_creds = t.get("ti_credentials"),
            lbl_sources = t.get("ti_sources"),
            sources = sources_html,
            lbl_quality = t.get("ti_quality"),
            stealer_pct = ti.stealer_log_percent,
            plain_pct = ti.plain_password_percent,
            combo = format_number(ti.combolist_count),
            footer = footer_dark(16, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "threat_intel".into(),
            html,
        }]
    }
}
