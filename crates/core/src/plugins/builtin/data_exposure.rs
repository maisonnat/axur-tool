//! Data Exposure Slide Plugin — ACT 3: The Abyss (Crisis Peak)
//!
//! Narrative Role: The EMOTIONAL PEAK. The hero sees their most sensitive assets
//! (source code + credentials) exposed. This is the Implication phase of SPIN.
//!
//! Persuasion: Scarcity (urgency language) + SPIN Implication (quantify the risk)
//! Design: Twin-column exposure grid, Von Restorff on secrets count only

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

        // Code Leaks Data
        let secrets_count = data.secrets_total;
        let repos_count = data.unique_repos;
        let prod_count = data.production_secrets;

        // Stealer Logs Data
        let creds_count = data.credentials_total;
        let hosts_count = data.unique_hosts;
        let risk_count = data.high_risk_users;

        // Calculate a visual exposure score (0-100) for the gauge
        // Heuristic: Secrets are width 10, Creds are width 0.5
        let exposure_score =
            ((secrets_count as f64 * 10.0 + creds_count as f64 * 0.5) as u32).min(100);
        let exposure_label = if exposure_score > 75 {
            "CRÍTICO"
        } else if exposure_score > 50 {
            "ALTO"
        } else {
            "MEDIO"
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                <!-- Background -->
                {bg}
                
                <!-- Header with premium layout -->
                <div class="flex justify-between items-start mb-8 relative z-10">
                    {header}
                    
                    <!-- Top Right: Risk Gauge -->
                    <div class="scale-75 origin-top-right -mt-4">
                        {risk_gauge}
                        <p class="text-center text-xs text-zinc-500 mt-2 tracking-widest uppercase">Nivel de Exposición</p>
                    </div>
                </div>

                <!-- SPIN IMPLICATION: Quantify what's at stake -->
                <div class="relative z-10 bg-red-500/5 border-l-2 border-red-500/50 p-4 mb-8 -mt-6 max-w-4xl backdrop-blur-sm">
                    <p class="text-red-200 text-sm leading-relaxed">
                        <strong class="text-red-400">IMPACTO DIRECTO:</strong> 
                        Detectamos <strong>{creds} credenciales</strong> activas y <strong>{secrets} secretos</strong> de código fuera de su perímetro. 
                        Esto permite acceso inmediato a sistemas críticos sin necesidad de fuerza bruta.
                    </p>
                </div>
                
                <div class="grid grid-cols-2 gap-12 flex-grow relative z-10 items-stretch">
                    <!-- Column 1: Code Leaks (Internal/Asset Risk) -->
                    <div class="flex flex-col gap-6">
                        <div class="flex items-center gap-4 mb-2">
                            <span class="icon-circle icon-circle-red">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                            </span>
                            <div>
                                <h3 class="text-xl font-bold text-white uppercase tracking-wider">{lbl_sub_code}</h3>
                                <div class="accent-line-thin w-full"></div>
                            </div>
                        </div>
                        
                        <!-- Main Stat: Secrets -->
                        {card_secrets}
                        
                        <!-- Sub Stats -->
                        <div class="grid grid-cols-2 gap-4">
                            {card_repos}
                            {card_prod}
                        </div>
                    </div>
                    
                    <!-- Column 2: Stealer Logs (External/Access Risk) -->
                    <div class="flex flex-col gap-6">
                        <div class="flex items-center gap-4 mb-2">
                            <span class="icon-circle icon-circle-red">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path></svg>
                            </span>
                            <div>
                                <h3 class="text-xl font-bold text-white uppercase tracking-wider">{lbl_sub_stealer}</h3>
                                <div class="accent-line-thin w-full"></div>
                            </div>
                        </div>
                        
                        <!-- Main Stat: Credentials -->
                        {card_creds}
                        
                        <!-- Sub Stats -->
                        <div class="grid grid-cols-2 gap-4">
                            {card_hosts}
                            {card_risk}
                        </div>
                    </div>
                </div>

                <!-- ZEIGARNIK EFFECT: Open loop to next section -->
                {next_teaser}

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            // COGNITIVE EMPTYING HEADER
            header = crate::plugins::builtin::theme::section_header_premium(
                "AMENAZAS ACTIVAS",
                "FUGA DE INFORMACIÓN CRÍTICA",
                None // Subtitle moved to the context box below for better flow
            ),
            risk_gauge =
                crate::plugins::builtin::theme::risk_gauge_svg(exposure_score, exposure_label),
            creds = format_number(creds_count),
            secrets = format_number(secrets_count),
            lbl_sub_code = t.get("exposure_sub_code"),
            // CRITICAL CARD for Secrets
            card_secrets = crate::plugins::builtin::theme::stat_card_critical(
                &format_number(secrets_count),
                &t.get("code_leak_box_secrets"),
                Some("Activos Expuestos")
            ),
            card_repos = crate::plugins::builtin::theme::stat_card_large(
                &format_number(repos_count),
                &t.get("code_leak_box_repos"),
                None
            ),
            card_prod = crate::plugins::builtin::theme::stat_card_large(
                &format_number(prod_count),
                &t.get("code_leak_box_prod"),
                Some("Ambiente Productivo")
            ),
            lbl_sub_stealer = t.get("exposure_sub_stealer"),
            // CRITICAL CARD for Credentials
            card_creds = crate::plugins::builtin::theme::stat_card_critical(
                &format_number(creds_count),
                &t.get("stealer_box_creds"),
                Some("Accesos Comprometidos")
            ),
            card_hosts = crate::plugins::builtin::theme::stat_card_large(
                &format_number(hosts_count),
                &t.get("stealer_box_hosts"),
                None
            ),
            card_risk = crate::plugins::builtin::theme::stat_card_large(
                &format_number(risk_count),
                &t.get("stealer_box_high_risk"),
                Some("Usuarios VIP/Admin")
            ),
            next_teaser = crate::plugins::builtin::theme::next_chapter_teaser(
                "Siguiente Capítulo",
                "Propagación Viral de Amenazas"
            ),
            footer = footer_dark(8, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "data_exposure".into(),
            html,
        }]
    }
}
