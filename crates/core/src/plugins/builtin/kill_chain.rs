//! Kill Chain Timeline Slide Plugin
//!
//! Shows the lifecycle of 1-2 real takedown cases from detection to resolution,
//! visualized as a horizontal swimlane. Demonstrates Axur's speed with concrete examples.
//!
//! Data: Uses resolved_takedowns with complete date chains (creation→incident→request→resolution).

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct KillChainSlidePlugin;

impl SlidePlugin for KillChainSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.kill_chain"
    }
    fn name(&self) -> &'static str {
        "Kill Chain Timeline"
    }
    fn priority(&self) -> i32 {
        72 // After Takedowns (70), before Metrics (65)
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        // Need at least one takedown with request + resolution dates
        ctx.data
            .resolved_takedowns
            .iter()
            .any(|td| td.request_date.is_some() && td.resolution_date.is_some())
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // ─── Find best examples: takedowns with most complete date chains ───
        let mut candidates: Vec<_> = data
            .resolved_takedowns
            .iter()
            .filter(|td| td.request_date.is_some() && td.resolution_date.is_some())
            .map(|td| {
                // Score by completeness
                let completeness = [
                    td.request_date.is_some(),
                    td.resolution_date.is_some(),
                    !td.host.is_empty(),
                    !td.ticket_type.is_empty(),
                ]
                .iter()
                .filter(|&&x| x)
                .count();
                (td, completeness)
            })
            .collect();

        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top 2 cases for the timeline
        let cases: Vec<_> = candidates.into_iter().take(2).map(|(td, _)| td).collect();

        // ─── Render timeline cases ───
        let cases_html: String = cases.iter().enumerate().map(|(idx, td)| {
            let threat_label = match td.ticket_type.as_str() {
                "phishing" => "PHISHING",
                "fraud" | "brand-abuse" => "FRAUDE DE MARCA",
                "fake-social" | "fake-social-media-profile" => "PERFIL FALSO",
                "malware" => "MALWARE",
                _ => &td.ticket_type,
            };

            let threat_color = match td.ticket_type.as_str() {
                "phishing" => "red",
                "fraud" | "brand-abuse" => "orange",
                "fake-social" | "fake-social-media-profile" => "purple",
                "malware" => "rose",
                _ => "zinc",
            };

            // Format dates for display (extract just date part)
            let fmt_date = |d: &Option<String>| -> String {
                d.as_ref()
                    .map(|s| {
                        // Handle both "2024-01-05" and "2024-01-05T10:30:00Z" formats
                        if s.len() >= 10 { s[..10].to_string() } else { s.clone() }
                    })
                    .unwrap_or_else(|| "—".to_string())
            };

            let request_date = fmt_date(&td.request_date);
            let resolution_date = fmt_date(&td.resolution_date);

            // Calculate duration if both dates are available
            let duration_label = if let (Some(req), Some(res)) = (&td.request_date, &td.resolution_date) {
                compute_duration_label(req, res)
            } else {
                "—".to_string()
            };

            let case_number = idx + 1;
            let host_display = if td.host.len() > 35 {
                format!("{}...", &td.host[..32])
            } else if td.host.is_empty() {
                "—".to_string()
            } else {
                td.host.clone()
            };

            format!(
                r#"<div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 backdrop-blur-sm hover:border-{color}-500/20 transition-all duration-300">
                    <!-- Case header -->
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-3">
                            <span class="bg-{color}-500/10 text-{color}-400 border border-{color}-500/20 px-2.5 py-1 text-xs font-bold tracking-wider uppercase rounded-full">
                                {threat_label}
                            </span>
                            <span class="text-xs text-zinc-500 font-mono">{ticket_key}</span>
                        </div>
                        <span class="text-xs text-zinc-600">Caso #{case_number}</span>
                    </div>

                    <!-- Target -->
                    <div class="text-sm text-zinc-400 mb-4 font-mono truncate">{host}</div>

                    <!-- Timeline swimlane -->
                    <div class="relative">
                        <!-- Timeline track -->
                        <div class="absolute top-4 left-0 right-0 h-0.5 bg-zinc-800"></div>
                        <div class="absolute top-4 left-0 right-0 h-0.5 bg-gradient-to-r from-emerald-500 via-blue-500 to-emerald-500 opacity-60"></div>

                        <div class="flex justify-between relative z-10">
                            <!-- Phase 1: Request -->
                            <div class="flex flex-col items-center">
                                <div class="w-8 h-8 rounded-full bg-blue-500/20 border-2 border-blue-500 flex items-center justify-center mb-2">
                                    <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
                                </div>
                                <span class="text-xs font-bold text-blue-400">SOLICITUD</span>
                                <span class="text-xs text-zinc-500 mt-0.5">{request_date}</span>
                            </div>

                            <!-- Duration arrow -->
                            <div class="flex flex-col items-center justify-center pt-1">
                                <div class="bg-emerald-500/10 border border-emerald-500/20 px-3 py-1 rounded-full">
                                    <span class="text-xs font-bold text-emerald-400 font-mono">{duration}</span>
                                </div>
                                <span class="text-xs text-zinc-600 mt-1">Tiempo de vida</span>
                            </div>

                            <!-- Phase 2: Resolved -->
                            <div class="flex flex-col items-center">
                                <div class="w-8 h-8 rounded-full bg-emerald-500/20 border-2 border-emerald-500 flex items-center justify-center mb-2">
                                    <svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                </div>
                                <span class="text-xs font-bold text-emerald-400">ELIMINADO</span>
                                <span class="text-xs text-zinc-500 mt-0.5">{resolution_date}</span>
                            </div>
                        </div>
                    </div>

                    <!-- Infrastructure context -->
                    <div class="flex gap-4 mt-4 pt-3 border-t border-zinc-800/50">
                        {infra_ip}
                        {infra_isp}
                        {infra_country}
                    </div>
                </div>"#,
                color = threat_color,
                threat_label = threat_label,
                ticket_key = td.ticket_key,
                case_number = case_number,
                host = host_display,
                request_date = request_date,
                resolution_date = resolution_date,
                duration = duration_label,
                infra_ip = if !td.ip.is_empty() {
                    format!(r#"<span class="text-xs text-zinc-500"><span class="text-zinc-600">IP:</span> <span class="text-zinc-400 font-mono">{}</span></span>"#, td.ip)
                } else { String::new() },
                infra_isp = if let Some(isp) = &td.isp {
                    if !isp.is_empty() {
                        format!(r#"<span class="text-xs text-zinc-500"><span class="text-zinc-600">ISP:</span> <span class="text-zinc-400">{}</span></span>"#, isp)
                    } else { String::new() }
                } else { String::new() },
                infra_country = if !td.country.is_empty() {
                    format!(r#"<span class="text-xs text-zinc-500"><span class="text-zinc-600">País:</span> <span class="text-zinc-400">{}</span></span>"#, td.country)
                } else { String::new() },
            )
        }).collect();

        // ─── Summary stats ───
        let total_resolved = data.resolved_takedowns.len();
        let avg_uptime = &data.takedown_median_uptime;
        let success_rate = data.takedown_success_rate;

        let header = crate::plugins::builtin::theme::section_header_premium(
            "CICLO DE VIDA",
            "Kill Chain — De Detección a Eliminación",
            Some("Seguimiento completo de casos reales: cada fase del proceso de takedown documentada."),
        );

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
                    <!-- Background -->
                    {bg}

                    <div class="relative z-10 flex flex-col h-full p-12">
                        <!-- Header -->
                        {header}

                        <div class="grid grid-cols-12 gap-6 flex-grow mt-2">
                            <!-- Left: Timeline Cases (8 cols) -->
                            <div class="col-span-8 flex flex-col gap-4">
                                {cases}
                            </div>

                            <!-- Right: Aggregate Stats (4 cols) -->
                            <div class="col-span-4 flex flex-col gap-4">
                                <!-- Efficiency summary -->
                                <div class="bg-gradient-to-br from-emerald-900/20 to-emerald-950/10 p-5 rounded-2xl border border-emerald-500/20 text-center hover:border-emerald-500/30 transition-all duration-300">
                                    <div class="text-4xl font-light text-emerald-400 font-mono mb-1">{avg_uptime}</div>
                                    <div class="text-xs text-emerald-300/60 uppercase tracking-widest">Tiempo Medio de Vida</div>
                                </div>

                                <div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 text-center hover:border-emerald-500/20 transition-all duration-300">
                                    <div class="text-3xl font-light text-white font-mono mb-1">{success_rate:.1}%</div>
                                    <div class="text-xs text-zinc-500 uppercase tracking-widest">Tasa de Eliminación</div>
                                </div>

                                <div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 text-center hover:border-orange-500/20 transition-all duration-300">
                                    <div class="text-3xl font-light text-orange-400 font-mono mb-1">{total_resolved}</div>
                                    <div class="text-xs text-zinc-500 uppercase tracking-widest">Casos Resueltos</div>
                                </div>

                                <!-- Insight card -->
                                <div class="bg-gradient-to-br from-blue-900/15 to-blue-950/10 p-4 rounded-2xl border border-blue-500/15 flex-grow flex flex-col justify-center">
                                    <div class="flex items-center gap-2 mb-2">
                                        <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        <span class="text-xs font-bold text-blue-300 uppercase tracking-wider">Impacto</span>
                                    </div>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">
                                        Cada hora que un sitio fraudulento permanece activo representa potenciales víctimas. 
                                        La velocidad de eliminación es la métrica más crítica.
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            cases = cases_html,
            avg_uptime = avg_uptime,
            success_rate = success_rate,
            total_resolved = total_resolved,
            footer = footer_dark(12, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "kill_chain".into(),
            html,
        }]
    }
}

/// Compute a human-readable duration between two date strings
fn compute_duration_label(start: &str, end: &str) -> String {
    // Try to parse ISO 8601 dates
    let parse_date = |s: &str| -> Option<chrono::NaiveDate> {
        // Handle "2024-01-05" or "2024-01-05T10:30:00Z"
        let date_part = if s.len() >= 10 { &s[..10] } else { s };
        chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()
    };

    if let (Some(start_d), Some(end_d)) = (parse_date(start), parse_date(end)) {
        let days = (end_d - start_d).num_days();
        if days <= 0 {
            "< 24h".to_string()
        } else if days == 1 {
            "1 día".to_string()
        } else if days < 7 {
            format!("{} días", days)
        } else if days < 30 {
            format!("{} semanas", days / 7)
        } else {
            format!("{} meses", days / 30)
        }
    } else {
        "—".to_string()
    }
}
