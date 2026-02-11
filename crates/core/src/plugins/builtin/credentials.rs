//! Credentials Slide Plugin
//!
//! Shows credential exposure details.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct CredentialsSlidePlugin;

impl SlidePlugin for CredentialsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.credentials"
    }
    fn name(&self) -> &'static str {
        "Credential Exposures"
    }
    fn priority(&self) -> i32 {
        65
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.credential_exposures.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total = data.credential_exposures.len();
        let critical = data.critical_credentials.len();

        // Show top 5 exposures (masked)
        let examples_html: String = data.credential_exposures.iter().take(5).map(|cred| {
            let user = cred.user.as_deref().unwrap_or("unknown");
            let chars: Vec<char> = user.chars().collect();
            let masked_user = if chars.len() > 6 {
                let first: String = chars[..3].iter().collect();
                let last: String = chars[chars.len()-2..].iter().collect();
                format!("{}...{}", first, last)
            } else {
                user.to_string()
            };
            let source = cred.leak_name.as_deref().unwrap_or("unknown");
            format!(
                r#"<div class="flex items-center justify-between p-3 bg-zinc-900 rounded mb-2 border-l-4 border-red-500"><span class="font-mono text-sm">{}</span><span class="text-xs text-zinc-500">{}</span></div>"#,
                masked_user, source
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-red-600 px-4 py-1 text-sm font-semibold">CREDENTIALS</span></div><h2 class="text-4xl font-bold mb-8">{title}</h2><div class="grid grid-cols-3 gap-8 mb-8"><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 text-center"><p class="text-4xl font-bold text-white">{total}</p><p class="text-zinc-400 text-sm">{lbl_total}</p></div><div class="bg-red-900/30 p-6 rounded-xl border border-red-500/30 text-center"><p class="text-4xl font-bold text-red-400">{critical}</p><p class="text-red-300 text-sm">{lbl_critical}</p></div><div class="bg-zinc-900 p-6 rounded-xl border border-zinc-800 text-center"><p class="text-4xl font-bold text-orange-400">{stealer}</p><p class="text-zinc-400 text-sm">{lbl_stealer}</p></div></div><div class="flex-grow">{examples}</div></div></div>{footer}</div></div>"#,
            title = t.get("cred_title"),
            total = format_number(total as u64),
            lbl_total = t.get("cred_total"),
            critical = critical,
            lbl_critical = t.get("cred_critical"),
            stealer = format_number(if data.threat_intelligence.stealer_log_count > 0 {
                data.threat_intelligence.stealer_log_count
            } else {
                // Fallback: count credentials that came from stealer logs
                data.credential_exposures
                    .iter()
                    .filter(|c| {
                        c.leak_name
                            .as_deref()
                            .map(|n| n.to_lowercase().contains("stealer"))
                            .unwrap_or(false)
                    })
                    .count() as u64
            }),
            lbl_stealer = t.get("cred_stealer"),
            examples = examples_html,
            footer = footer_dark(15, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "credentials".into(),
            html,
        }]
    }
}
