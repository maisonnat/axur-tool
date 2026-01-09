//! Threats Chart Slide Plugin
//!
//! Displays threats distribution by type with Axur.com dark theme aesthetics.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct ThreatsSlidePlugin;

impl SlidePlugin for ThreatsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.threats"
    }
    fn name(&self) -> &'static str {
        "Threats by Type"
    }
    fn priority(&self) -> i32 {
        88
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.threats_by_type.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total_threats: u64 = data.threats_by_type.iter().map(|t| t.count).sum();
        let max_count = data
            .threats_by_type
            .iter()
            .map(|t| t.count)
            .max()
            .unwrap_or(1);

        // Generate horizontal bar chart items
        let bars_html: String = data
            .threats_by_type
            .iter()
            .take(8) // Limit to top 8 for visual clarity
            .map(|threat| {
                let percentage = (threat.count as f64 / max_count as f64) * 100.0;
                format!(
                    r#"<div class="flex items-center gap-4 mb-4">
                        <div class="w-40 text-sm text-zinc-300 truncate">{name}</div>
                        <div class="flex-grow h-8 bg-zinc-800 rounded overflow-hidden relative">
                            <div class="h-full bg-gradient-to-r from-[#FF5824] to-[#FF7A4D] rounded transition-all duration-500" 
                                 style="width: {pct}%; box-shadow: 0 0 15px rgba(255, 88, 36, 0.3);"></div>
                        </div>
                        <div class="w-16 text-right font-bold text-[#FF5824] glow-orange-text">{count}</div>
                    </div>"#,
                    name = threat.threat_type,
                    pct = percentage,
                    count = threat.count
                )
            })
            .collect();

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-[#121212] text-white overflow-hidden">
                    <!-- Wireframe background pattern -->
                    <div class="absolute inset-0 wireframe-bg opacity-50"></div>
                    
                    <!-- Content -->
                    <div class="relative z-10 h-full flex flex-col p-14">
                        <!-- Header -->
                        <div class="mb-6">
                            <span class="bg-[#FF5824] text-white px-5 py-2 text-xs font-bold tracking-wider uppercase">
                                ANÁLISIS
                            </span>
                        </div>
                        
                        <!-- Two Column Layout -->
                        <div class="flex gap-12 flex-grow">
                            <!-- Left: Stats and Title -->
                            <div class="w-1/3 flex flex-col">
                                <h2 class="text-4xl font-bold mb-4 text-white">{title}</h2>
                                <p class="text-zinc-400 text-sm mb-8 leading-relaxed">{desc}</p>
                                
                                <!-- Big Number -->
                                <div class="mt-auto">
                                    <div class="text-7xl font-black text-[#FF5824] glow-orange-text">{total}</div>
                                    <div class="text-sm text-zinc-400 uppercase tracking-wider mt-2">Total Threats Detected</div>
                                </div>
                            </div>
                            
                            <!-- Right: Bar Chart -->
                            <div class="w-2/3 flex flex-col justify-center">
                                <div class="bg-zinc-900/50 rounded-xl p-6 border border-zinc-800">
                                    <h3 class="text-lg font-semibold text-white mb-6">Distribution by Type</h3>
                                    {bars}
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            title = t.get("threats_title"),
            desc = t.format("threats_desc", &[("total", &total_threats.to_string())]),
            total = total_threats,
            bars = bars_html,
            footer = footer_dark(7, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "threats".into(),
            html,
        }]
    }
}
