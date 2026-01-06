//! AI Intent Slide Plugin
//!
//! AI analysis of attack patterns and intent.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};
use std::collections::HashMap;

pub struct AiIntentSlidePlugin;

impl SlidePlugin for AiIntentSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.ai_intent"
    }
    fn name(&self) -> &'static str {
        "AI Intent Analysis"
    }
    fn priority(&self) -> i32 {
        86
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.threats_by_type.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Group threats into intent categories
        let mut intents: HashMap<&str, u64> = HashMap::new();
        for threat in &data.threats_by_type {
            let cat = match threat.threat_type.as_str() {
                "phishing" | "fake-social-media-profile" | "fake-website" => "trust",
                "stealer-log" | "malware" => "compromised",
                "code-leak" | "credential-leak" => "data_leak",
                _ => "other",
            };
            *intents.entry(cat).or_insert(0) += threat.count;
        }

        let total: u64 = intents.values().sum();

        // Build bar chart data
        let bars_html: String = intents.iter().map(|(cat, count)| {
            let pct = if total > 0 { (*count * 100) / total } else { 0 };
            let (label, color) = match *cat {
                "trust" => (t.get("intent_cat_trust"), "#f97316"),
                "compromised" => (t.get("intent_cat_compromised"), "#ef4444"),
                "data_leak" => (t.get("intent_cat_data_leak"), "#eab308"),
                _ => (t.get("intent_cat_other"), "#6b7280"),
            };
            format!(
                r#"<div class="flex items-center gap-4 mb-4"><span class="w-40 text-right text-sm">{label}</span><div class="flex-grow bg-zinc-800 rounded h-8 overflow-hidden"><div class="h-full" style="width:{}%;background:{}"></div></div><span class="w-16 text-sm">{}</span></div>"#,
                pct, color, count
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-blue-600 px-4 py-1 text-sm font-semibold">AI ANALYSIS</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="flex-grow bg-zinc-900/50 p-6 rounded-xl border border-zinc-800">{bars}</div></div></div>{footer}</div></div>"#,
            title = t.get("intent_title"),
            bars = bars_html,
            footer = footer_dark(13, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "ai_intent".into(),
            html,
        }]
    }
}
