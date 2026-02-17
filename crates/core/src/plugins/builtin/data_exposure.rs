//! Data Exposure Slide Plugin
//!
//! Combined view of code leaks and credential exposures.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct DataExposureSlidePlugin;

impl SlidePlugin for DataExposureSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.data_exposure"
    }
    fn name(&self) -> &'static str {
        "Data Exposure"
    }
    fn priority(&self) -> i32 {
        85
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        ctx.data.secrets_total > 0 || ctx.data.credentials_total > 0
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Translations & Data
        let title = t.get("exposure_title");

        // Code Leaks Data
        let secrets_count = data.secrets_total;
        let repos_count = data.unique_repos;
        let prod_count = data.production_secrets;

        // Stealer Logs Data
        let creds_count = data.credentials_total;
        let hosts_count = data.unique_hosts;
        let risk_count = data.high_risk_users;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}
                
                <!-- Header -->
                {header}
                
                <div class="grid grid-cols-2 gap-12 flex-grow mt-4">
                    <!-- Column 1: Code Leaks (Internal/Asset Risk) -->
                    <div class="flex flex-col gap-6">
                        <div class="flex items-center gap-3 border-b border-zinc-800 pb-2">
                            <span class="text-[#FF5824]">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                            </span>
                            <h3 class="text-xl font-bold text-white uppercase tracking-wider">{lbl_sub_code}</h3>
                        </div>
                        
                        <!-- Main Stat -->
                        {card_secrets}
                        
                        <!-- Sub Stats -->
                        <div class="grid grid-cols-2 gap-4">
                            {card_repos}
                            {card_prod}
                        </div>
                    </div>
                    
                    <!-- Column 2: Stealer Logs (External/Access Risk) -->
                    <div class="flex flex-col gap-6">
                        <div class="flex items-center gap-3 border-b border-zinc-800 pb-2">
                            <span class="text-[#FF5824]">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path></svg>
                            </span>
                            <h3 class="text-xl font-bold text-white uppercase tracking-wider">{lbl_sub_stealer}</h3>
                        </div>
                        
                        <!-- Main Stat -->
                        {card_creds}
                        
                        <!-- Sub Stats -->
                        <div class="grid grid-cols-2 gap-4">
                            {card_hosts}
                            {card_risk}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header(
                "FUGA DE INFORMACIÓN CRÍTICA",
                "Activos Comprometidos"
            ),
            lbl_sub_code = t.get("exposure_sub_code"),
            card_secrets = crate::plugins::builtin::theme::stat_card_glow(
                &format_number(secrets_count),
                &t.get("code_leak_box_secrets"),
                true
            ),
            card_repos = crate::plugins::builtin::theme::stat_card_large(
                &format_number(repos_count),
                &t.get("code_leak_box_repos"),
                None
            ),
            card_prod = crate::plugins::builtin::theme::stat_card_large(
                &format_number(prod_count),
                &t.get("code_leak_box_prod"),
                Some("Críticos")
            ),
            lbl_sub_stealer = t.get("exposure_sub_stealer"),
            card_creds = crate::plugins::builtin::theme::stat_card_glow(
                &format_number(creds_count),
                &t.get("stealer_box_creds"),
                true
            ),
            card_hosts = crate::plugins::builtin::theme::stat_card_large(
                &format_number(hosts_count),
                &t.get("stealer_box_hosts"),
                None
            ),
            card_risk = crate::plugins::builtin::theme::stat_card_large(
                &format_number(risk_count),
                &t.get("stealer_box_high_risk"),
                Some("Urgente")
            ),
            footer = footer_dark(8, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "data_exposure".into(),
            html,
        }]
    }
}
