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

        // Find dominant threat
        let top_threat = data.threats_by_type.first();
        let top_pct = top_threat
            .map(|t| (t.count as f64 * 100.0 / total_threats.max(1) as f64).round() as u64)
            .unwrap_or(0);
        let top_name = top_threat.map(|t| t.threat_type.as_str()).unwrap_or("N/A");

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-[#121212] text-white overflow-hidden">
                    <!-- Wireframe background pattern -->
                    <div class="absolute inset-0 wireframe-bg opacity-50"></div>
                    
                    <!-- Content -->
                    <div class="relative z-10 h-full flex flex-col p-14">
                        <!-- Header -->
                        <div class="mb-4">
                            <span class="bg-gradient-to-r from-[#FF5824] to-[#FF7A4D] text-white px-5 py-2 text-xs font-bold tracking-wider uppercase">
                                ANÁLISIS
                            </span>
                        </div>
                        
                        <h2 class="text-4xl font-black mb-2 text-white tracking-tight">{title}</h2>
                        <p class="text-zinc-400 text-sm mb-6">{desc}</p>
                        
                        <!-- Two Column Layout -->
                        <div class="flex gap-8 flex-grow">
                            <!-- Left: Context + Big Number -->
                            <div class="w-2/5 flex flex-col gap-4">
                                <div class="bg-zinc-900/70 p-5 rounded-xl border border-zinc-800">
                                    <div class="flex items-center gap-2 mb-3">
                                        <svg class="w-5 h-5 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path></svg>
                                        <h3 class="text-base font-semibold text-orange-400">¿Qué son estas amenazas?</h3>
                                    </div>
                                    <p class="text-zinc-400 text-sm leading-relaxed">Son detecciones de <strong class="text-white">actividad maliciosa</strong> contra su marca: sitios falsos, phishing, perfiles fraudulentos, y más. Cada una fue validada por nuestros sistemas de IA.</p>
                                </div>
                                
                                <!-- Big Number Panel -->
                                <div class="bg-gradient-to-br from-orange-500/20 to-orange-600/10 p-6 rounded-xl border border-orange-500/30 flex-grow flex flex-col justify-center">
                                    <div class="text-7xl font-black text-[#FF5824]" style="text-shadow: 0 0 40px rgba(255, 88, 36, 0.5);">{total}</div>
                                    <div class="text-sm text-zinc-300 uppercase tracking-wider mt-2">Amenazas Detectadas</div>
                                    <div class="mt-4 pt-4 border-t border-orange-500/20">
                                        <p class="text-sm text-zinc-400"><strong class="text-orange-400">{top_pct}%</strong> son <strong class="text-white">{top_name}</strong></p>
                                    </div>
                                </div>
                            </div>
                            
                            <!-- Right: Bar Chart -->
                            <div class="w-3/5 flex flex-col">
                                <div class="bg-zinc-900/50 rounded-xl p-6 border border-zinc-800 flex-grow">
                                    <h3 class="text-lg font-semibold text-white mb-4">Distribución por Tipo</h3>
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
            top_pct = top_pct,
            top_name = top_name,
            bars = bars_html,
            footer = footer_dark(7, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "threats".into(),
            html,
        }]
    }
}
