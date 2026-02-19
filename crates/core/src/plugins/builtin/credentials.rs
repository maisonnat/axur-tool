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
                r#"<div class="flex items-center justify-between p-4 glass-panel hover:border-red-500/30 hover:scale-[1.01] transition-all duration-300 mb-3">
                    <div class="flex items-center gap-3">
                        <span class="text-red-500/50">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                        </span>
                        <span class="font-mono text-sm text-white">{}</span>
                    </div>
                    <span class="text-xs text-zinc-500 bg-zinc-800/50 px-2 py-1 rounded">{}</span>
                </div>"#,
                masked_user, source
            )
        }).collect();

        let stealer_count = if data.threat_intelligence.stealer_log_count > 0 {
            data.threat_intelligence.stealer_log_count
        } else {
            data.credential_exposures
                .iter()
                .filter(|c| {
                    c.leak_name
                        .as_deref()
                        .map(|n| n.to_lowercase().contains("stealer"))
                        .unwrap_or(false)
                })
                .count() as u64
        };

        // Premium Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "CREDENCIALES COMPROMETIDAS",
            &t.get("cred_title"),
            Some("Credenciales de su organización detectadas en filtraciones y logs de malware stealer.")
        );

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg_pattern}

                <!-- Header -->
                {header}

                <div class="grid grid-cols-12 gap-8 flex-grow mt-4">
                    <!-- Left: Stats -->
                    <div class="col-span-5 flex flex-col gap-6 justify-center">
                        {card_total}
                        <div class="grid grid-cols-2 gap-4">
                            {card_critical}
                            {card_stealer}
                        </div>
                    </div>

                    <!-- Right: Exposed Credentials -->
                    <div class="col-span-7 flex flex-col">
                        <h3 class="text-xs font-bold text-zinc-500 mb-4 uppercase tracking-widest border-b border-zinc-900 pb-2">
                            Credenciales Detectadas (muestra)
                        </h3>
                        <div class="flex-grow overflow-hidden">
                            {examples}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            card_total = crate::plugins::builtin::theme::stat_card_hero(
                &format_number(total as u64),
                &t.get("cred_total"),
                Some("Credenciales filtradas detectadas")
            ),
            card_critical = crate::plugins::builtin::theme::stat_card_critical(
                &critical.to_string(),
                &t.get("cred_critical"),
                Some("Acceso Privilegiado")
            ),
            card_stealer = crate::plugins::builtin::theme::stat_card_large(
                &format_number(stealer_count),
                &t.get("cred_stealer"),
                Some("Malware Stealer")
            ),
            examples = examples_html,
            footer = footer_dark(15, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "credentials".into(),
            html,
        }]
    }
}
