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

        // Find dominant intent
        let (top_intent, top_count) = intents
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, v)| (*k, *v))
            .unwrap_or(("other", 0));
        let top_pct = if total > 0 {
            (top_count * 100) / total
        } else {
            0
        };

        let top_label = match top_intent {
            "trust" => t.get("intent_cat_trust"),
            "compromised" => t.get("intent_cat_compromised"),
            "data_leak" => t.get("intent_cat_data_leak"),
            _ => t.get("intent_cat_other"),
        };

        // Build bar chart data with better styling
        let bars_html: String = intents.iter().map(|(cat, count)| {
            let pct = if total > 0 { (*count * 100) / total } else { 0 };
            let (label, color, bg_color) = match *cat {
                "trust" => (t.get("intent_cat_trust"), "#f97316", "rgba(249, 115, 22, 0.15)"),
                "compromised" => (t.get("intent_cat_compromised"), "#ef4444", "rgba(239, 68, 68, 0.15)"),
                "data_leak" => (t.get("intent_cat_data_leak"), "#eab308", "rgba(234, 179, 8, 0.15)"),
                _ => (t.get("intent_cat_other"), "#6b7280", "rgba(107, 114, 128, 0.15)"),
            };
            format!(
                r#"<div class="flex items-center gap-4 mb-3 p-3 rounded-lg" style="background: {}">
                    <span class="w-48 text-sm font-medium" style="color: {}">{}</span>
                    <div class="flex-grow bg-zinc-800 rounded-full h-3 overflow-hidden">
                        <div class="h-full rounded-full transition-all" style="width:{}%;background:{}"></div>
                    </div>
                    <span class="w-20 text-right font-bold" style="color: {}">{} ({}%)</span>
                </div>"#,
                bg_color, color, label, pct, color, color, count, pct
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
<div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 20% 80%, #3B82F6 0%, transparent 40%);"></div>
<div class="relative flex-grow h-full overflow-hidden z-10"><div class="h-full flex flex-col">
<div class="mb-4"><span class="bg-gradient-to-r from-blue-600 to-blue-500 px-4 py-1 text-sm font-bold tracking-wider uppercase">AI ANALYSIS</span></div>
<h2 class="text-4xl font-black mb-6 tracking-tight">{title}</h2>

<div class="flex gap-8 flex-grow">
    <!-- Left: Context Panel -->
    <div class="w-2/5 flex flex-col">
        <div class="bg-zinc-900/70 p-6 rounded-xl border border-zinc-800 flex-grow">
            <div class="flex items-center gap-2 mb-4">
                <svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                <h3 class="text-lg font-semibold text-blue-400">¿Qué mide esta slide?</h3>
            </div>
            <p class="text-zinc-400 text-sm leading-relaxed mb-6">Nuestra IA clasifica las amenazas detectadas según la <strong class="text-white">intención del atacante</strong>, no solo el tipo técnico. Esto revela si buscan robar credenciales, dañar la reputación, o comprometer sistemas.</p>
            
            <div class="border-t border-zinc-800 pt-4">
                <div class="flex items-center gap-2 mb-2">
                    <svg class="w-4 h-4 text-orange-400" fill="currentColor" viewBox="0 0 20 20"><path d="M10 2a8 8 0 100 16 8 8 0 000-16zm1 11a1 1 0 11-2 0 1 1 0 012 0zm0-3a1 1 0 01-2 0V7a1 1 0 112 0v3z"></path></svg>
                    <span class="text-sm font-semibold text-orange-400">Insight Clave</span>
                </div>
                <p class="text-white text-sm"><strong class="text-orange-400">{top_pct}%</strong> de ataques apuntan a <strong class="text-white">{top_label}</strong></p>
            </div>
        </div>
    </div>
    
    <!-- Right: Bar Chart -->
    <div class="w-3/5 flex flex-col">
        <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 flex-grow">
            <h3 class="text-lg font-semibold text-white mb-4">Distribución por Intención</h3>
            {bars}
        </div>
    </div>
</div>

</div></div>
{footer}
</div></div>"#,
            title = t.get("intent_title"),
            top_pct = top_pct,
            top_label = top_label,
            bars = bars_html,
            footer = footer_dark(13, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "ai_intent".into(),
            html,
        }]
    }
}
