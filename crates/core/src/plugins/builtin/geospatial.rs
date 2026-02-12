//! Geospatial Slide Plugin
//!
//! Shows geographic distribution of threats.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};
use std::collections::HashMap;

pub struct GeospatialSlidePlugin;

impl SlidePlugin for GeospatialSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.geospatial"
    }
    fn name(&self) -> &'static str {
        "Geospatial Analysis"
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

        // Aggregate countries from investigations, incidents, and takedowns
        let mut countries: HashMap<String, u32> = HashMap::new();

        // 1. Deep Investigations (Signal Lake)
        for inv in &data.deep_investigations {
            if let Some(country) = &inv.infrastructure.country {
                *countries.entry(country.clone()).or_insert(0) += 1;
            }
        }

        // 2. Latest Incidents
        for inc in &data.latest_incidents {
            if !inc.country.is_empty() {
                *countries.entry(inc.country.clone()).or_insert(0) += 1;
            }
        }

        // 3. Resolved Takedowns
        for td in &data.resolved_takedowns {
            if !td.country.is_empty() {
                *countries.entry(td.country.clone()).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<_> = countries.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let max_count = sorted.first().map(|(_, c)| *c).unwrap_or(1);

        let rows_html: String = sorted.iter().take(8).map(|(country, count)| {
            let pct = (*count as f64 / max_count as f64) * 100.0;
            format!(
                r#"<div class="mb-5">
                    <div class="flex justify-between items-end mb-2">
                        <span class="font-bold text-lg text-zinc-200">{}</span>
                        <span class="text-orange-500 font-mono font-bold text-xl">{}</span>
                    </div>
                    <div class="h-3 bg-zinc-800/50 rounded-full overflow-hidden border border-zinc-800">
                        <div class="h-full bg-gradient-to-r from-orange-600 to-orange-400 shadow-[0_0_10px_rgba(249,115,22,0.3)] transition-all duration-1000" style="width: {}%"></div>
                    </div>
                   </div>"#,
                country, count, pct
            )
        }).collect();

        // If no data, show a message
        let visualization_html = if sorted.is_empty() {
            format!(
                r#"<div class="flex flex-col items-center justify-center h-full text-zinc-500">
                    <svg class="w-16 h-16 mb-4 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    <p class="text-xl">{}</p>
                   </div>"#,
                t.get("example_no_data")
            )
        } else {
            rows_html
        };

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-[#09090b] text-white overflow-hidden">
                    <!-- Background Grid -->
                    <div class="absolute inset-0" style="background-image: radial-gradient(circle at 1px 1px, #27272a 1px, transparent 0); background-size: 40px 40px; opacity: 0.3;"></div>
                    
                    <div class="relative z-10 flex flex-col h-full p-12">
                        <!-- Header -->
                        <div class="flex items-center gap-4 mb-2">
                            <span class="bg-orange-600/20 text-orange-500 border border-orange-600/30 px-3 py-1 text-xs font-bold tracking-wider uppercase rounded-full">GEOSPATIAL</span>
                        </div>
                        <h2 class="text-5xl font-black mb-10 tracking-tight text-white">{title}</h2>

                        <div class="flex gap-12 flex-grow">
                            <!-- Left: Visualization (Replaces Map) -->
                            <div class="w-2/3 flex flex-col">
                                <div class="bg-zinc-900/40 p-8 rounded-2xl border border-zinc-800/50 flex-grow backdrop-blur-sm shadow-xl">
                                    <h3 class="text-xl font-bold text-zinc-400 mb-6 flex items-center gap-2">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
                                        {lbl_countries}
                                    </h3>
                                    <div class="space-y-1">
                                        {visualization}
                                    </div>
                                </div>
                            </div>

                            <!-- Right: Analysis & Context -->
                            <div class="w-1/3 flex flex-col gap-6">
                                <!-- Why this is important -->
                                <div class="bg-gradient-to-br from-indigo-900/20 to-indigo-950/20 p-6 rounded-2xl border border-indigo-500/20">
                                    <div class="flex items-center gap-3 mb-3">
                                        <div class="p-2 bg-indigo-500/10 rounded-lg">
                                            <svg class="w-6 h-6 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                        </div>
                                        <h3 class="text-lg font-bold text-indigo-300">{why_title}</h3>
                                    </div>
                                    <p class="text-indigo-200/80 leading-relaxed text-sm">
                                        {why_text}
                                    </p>
                                </div>

                                <!-- Key stats or secondary info could go here -->
                            </div>
                        </div>
                    </div>

                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            title = t.get("geo_title"),
            visualization = visualization_html,
            lbl_countries = t.get("geo_lbl_countries"),
            why_title = t.get("geo_why_important_title"),
            why_text = t.get("geo_why_important_text"),
            footer = footer_dark(14, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "geospatial".into(),
            html,
        }]
    }
}
