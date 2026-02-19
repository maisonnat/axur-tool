//! Threat DNA Slide Plugin
//!
//! Replaces the old Geospatial slide with an attacker profile analysis.
//! Aggregates forensic indicators (ISPs, countries, registrars, threat types)
//! from investigations, incidents, and takedowns to build a "Threat DNA" fingerprint.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};
use std::collections::HashMap;

pub struct GeospatialSlidePlugin;

impl SlidePlugin for GeospatialSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.geospatial"
    }
    fn name(&self) -> &'static str {
        "Threat DNA"
    }
    fn priority(&self) -> i32 {
        84
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.deep_investigations.is_empty()
            || !ctx.data.latest_incidents.is_empty()
            || !ctx.data.resolved_takedowns.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // ─── Aggregate forensic indicators ───
        let mut countries: HashMap<String, u32> = HashMap::new();
        let mut isps: HashMap<String, u32> = HashMap::new();
        let mut registrars: HashMap<String, u32> = HashMap::new();
        let mut threat_types: HashMap<String, u32> = HashMap::new();
        let mut total_unique_ips: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let mut total_unique_hosts: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        // 1. Deep Investigations (Signal Lake)
        for inv in &data.deep_investigations {
            if let Some(country) = &inv.infrastructure.country {
                if !country.is_empty() {
                    *countries.entry(country.clone()).or_insert(0) += 1;
                }
            }
            if let Some(hosting) = &inv.infrastructure.hosting_provider {
                if !hosting.is_empty() {
                    *isps.entry(hosting.clone()).or_insert(0) += 1;
                }
            }
            if let Some(ip) = &inv.infrastructure.ip {
                total_unique_ips.insert(ip.clone());
            }
            if !inv.threat_type.is_empty() {
                *threat_types.entry(inv.threat_type.clone()).or_insert(0) += 1;
            }
            if !inv.target.is_empty() {
                total_unique_hosts.insert(inv.target.clone());
            }
            if let Some(reg) = &inv.enrichment.registrar {
                if !reg.is_empty() {
                    *registrars.entry(reg.clone()).or_insert(0) += 1;
                }
            }
        }

        // 2. Latest Incidents
        for inc in &data.latest_incidents {
            if !inc.country.is_empty() {
                *countries.entry(inc.country.clone()).or_insert(0) += 1;
            }
            if !inc.isp.is_empty() {
                *isps.entry(inc.isp.clone()).or_insert(0) += 1;
            }
            if !inc.ip.is_empty() {
                total_unique_ips.insert(inc.ip.clone());
            }
            if !inc.host.is_empty() {
                total_unique_hosts.insert(inc.host.clone());
            }
            if !inc.ticket_type.is_empty() {
                *threat_types.entry(inc.ticket_type.clone()).or_insert(0) += 1;
            }
            if let Some(reg) = &inc.registrar {
                if !reg.is_empty() {
                    *registrars.entry(reg.clone()).or_insert(0) += 1;
                }
            }
        }

        // 3. Resolved Takedowns
        for td in &data.resolved_takedowns {
            if !td.country.is_empty() {
                *countries.entry(td.country.clone()).or_insert(0) += 1;
            }
            if let Some(isp) = &td.isp {
                if !isp.is_empty() {
                    *isps.entry(isp.clone()).or_insert(0) += 1;
                }
            }
            if !td.ip.is_empty() {
                total_unique_ips.insert(td.ip.clone());
            }
            if !td.host.is_empty() {
                total_unique_hosts.insert(td.host.clone());
            }
            if !td.ticket_type.is_empty() {
                *threat_types.entry(td.ticket_type.clone()).or_insert(0) += 1;
            }
            if let Some(reg) = &td.registrar {
                if !reg.is_empty() {
                    *registrars.entry(reg.clone()).or_insert(0) += 1;
                }
            }
        }

        // ─── Sort all aggregations ───
        let sort_top = |map: HashMap<String, u32>, limit: usize| -> Vec<(String, u32)> {
            let mut sorted: Vec<_> = map.into_iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
            sorted.into_iter().take(limit).collect()
        };

        let top_countries = sort_top(countries, 5);
        let top_isps = sort_top(isps, 4);
        let top_registrars = sort_top(registrars, 3);
        let top_threats = sort_top(threat_types, 4);
        let ip_count = total_unique_ips.len();
        let host_count = total_unique_hosts.len();

        // ─── Render forensic indicator rows ───
        let render_indicator_list = |items: &[(String, u32)], color: &str| -> String {
            if items.is_empty() {
                return r#"<div class="text-zinc-600 text-sm italic">Sin datos suficientes</div>"#
                    .to_string();
            }
            let total: u32 = items.iter().map(|(_, c)| *c).sum();
            items.iter().map(|(name, count)| {
                let pct = if total > 0 { (*count as f64 / total as f64) * 100.0 } else { 0.0 };
                format!(
                    r#"<div class="flex items-center gap-3 group/row">
                        <div class="w-full">
                            <div class="flex justify-between items-baseline mb-1">
                                <span class="text-sm font-medium text-zinc-200">{name}</span>
                                <span class="text-xs text-{color}-400 font-mono font-bold">{pct:.0}%</span>
                            </div>
                            <div class="h-1.5 bg-zinc-800/50 rounded-full overflow-hidden">
                                <div class="h-full bg-{color}-500/60 rounded-full transition-all duration-500" style="width: {pct}%"></div>
                            </div>
                        </div>
                    </div>"#,
                    name = name,
                    color = color,
                    pct = pct,
                )
            }).collect::<Vec<_>>().join("")
        };

        // ─── Threat type badges ───
        let threat_badges_html = if top_threats.is_empty() {
            String::new()
        } else {
            top_threats.iter().map(|(tt, count)| {
                let (label, badge_color) = match tt.as_str() {
                    "phishing" => ("Phishing", "red"),
                    "brand-abuse" | "fraud" => ("Fraude de Marca", "orange"),
                    "fake-social" | "fake-social-media-profile" => ("Perfiles Falsos", "purple"),
                    "malware" => ("Malware", "rose"),
                    "typosquatting" => ("Typosquatting", "amber"),
                    _ => (tt.as_str(), "zinc"),
                };
                format!(
                    r#"<span class="inline-flex items-center gap-1.5 bg-{color}-500/10 text-{color}-400 border border-{color}-500/20 px-3 py-1.5 text-xs font-bold tracking-wider uppercase rounded-full">
                        {label} <span class="text-{color}-500/60">({count})</span>
                    </span>"#,
                    color = badge_color,
                    label = label,
                    count = count,
                )
            }).collect::<Vec<_>>().join(" ")
        };

        // ─── Top country flag-style badges ───
        let country_badges_html = if top_countries.is_empty() {
            r#"<div class="text-zinc-600 text-sm italic">Sin datos geográficos</div>"#.to_string()
        } else {
            top_countries.iter().map(|(country, count)| {
                format!(
                    r#"<div class="flex items-center gap-3 bg-zinc-800/40 hover:bg-zinc-800/60 border border-zinc-700/30 hover:border-orange-500/20 px-4 py-2.5 rounded-xl transition-all duration-300 hover:scale-[1.02]">
                        <span class="text-sm font-bold text-zinc-300 tracking-wide">{country}</span>
                        <span class="text-lg font-light text-orange-400 font-mono">{count}</span>
                    </div>"#,
                    country = country,
                    count = count,
                )
            }).collect::<Vec<_>>().join("")
        };

        // Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "INTELIGENCIA DE AMENAZAS",
            &t.get("geo_title"),
            Some("Perfil forense agregado del atacante basado en infraestructura, registradores y patrones de hosting."),
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
                            <!-- Left: Threat DNA Profile (8 cols) -->
                            <div class="col-span-8 flex flex-col gap-4">
                                <!-- Threat Type Badges -->
                                <div class="flex flex-wrap gap-2 mb-1">
                                    {threat_badges}
                                </div>

                                <!-- Two-column forensic grid -->
                                <div class="grid grid-cols-2 gap-4 flex-grow">
                                    <!-- ISP / Hosting Providers -->
                                    <div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 backdrop-blur-sm hover:border-blue-500/20 transition-all duration-300">
                                        <h4 class="text-xs font-bold text-zinc-500 uppercase tracking-widest mb-4 flex items-center gap-2">
                                            <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2"></path></svg>
                                            Hosting / ISP
                                        </h4>
                                        <div class="space-y-3">
                                            {isps_html}
                                        </div>
                                    </div>

                                    <!-- Registrars -->
                                    <div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 backdrop-blur-sm hover:border-purple-500/20 transition-all duration-300">
                                        <h4 class="text-xs font-bold text-zinc-500 uppercase tracking-widest mb-4 flex items-center gap-2">
                                            <svg class="w-4 h-4 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                                            Registradores
                                        </h4>
                                        <div class="space-y-3">
                                            {registrars_html}
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <!-- Right: Country Origins + Stats (4 cols) -->
                            <div class="col-span-4 flex flex-col gap-4">
                                <!-- Stats mini cards -->
                                <div class="grid grid-cols-2 gap-3">
                                    <div class="bg-zinc-900/40 p-4 rounded-xl border border-zinc-800/50 text-center hover:border-orange-500/20 transition-all duration-300">
                                        <div class="text-2xl font-light text-orange-400 font-mono">{ip_count}</div>
                                        <div class="text-xs text-zinc-500 mt-1">IPs Únicas</div>
                                    </div>
                                    <div class="bg-zinc-900/40 p-4 rounded-xl border border-zinc-800/50 text-center hover:border-orange-500/20 transition-all duration-300">
                                        <div class="text-2xl font-light text-orange-400 font-mono">{host_count}</div>
                                        <div class="text-xs text-zinc-500 mt-1">Hosts Únicos</div>
                                    </div>
                                </div>

                                <!-- Country origins -->
                                <div class="bg-zinc-900/40 p-5 rounded-2xl border border-zinc-800/50 backdrop-blur-sm flex-grow hover:border-orange-500/20 transition-all duration-300">
                                    <h4 class="text-xs font-bold text-zinc-500 uppercase tracking-widest mb-4 flex items-center gap-2">
                                        <svg class="w-4 h-4 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        Países de Origen
                                    </h4>
                                    <div class="space-y-2">
                                        {country_badges}
                                    </div>
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
            threat_badges = threat_badges_html,
            isps_html = render_indicator_list(&top_isps, "blue"),
            registrars_html = render_indicator_list(&top_registrars, "purple"),
            ip_count = ip_count,
            host_count = host_count,
            country_badges = country_badges_html,
            footer = footer_dark(14, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "threat_dna".into(),
            html,
        }]
    }
}
