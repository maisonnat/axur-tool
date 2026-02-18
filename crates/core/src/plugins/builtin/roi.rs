//! ROI (Return on Investment) Slide Plugin
//!
//! Displays operational impact and efficiency metrics with Axur.com aesthetics.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the ROI/impact slide
pub struct RoiSlidePlugin;

impl SlidePlugin for RoiSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.roi"
    }
    fn name(&self) -> &'static str {
        "Impact & ROI"
    }
    fn priority(&self) -> i32 {
        60
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;
        let metrics = &data.roi_metrics;

        // Format analysts equivalent
        let analysts_display = if metrics.analysts_equivalent_monthly >= 1.0 {
            format!("{:.1}", metrics.analysts_equivalent_monthly)
        } else {
            format!("{:.0}%", metrics.analysts_equivalent_monthly * 100.0)
        };

        // Premium Header
        let header = crate::plugins::builtin::theme::section_header_premium(
            "VALOR ESTRATÉGICO",
            "El Retorno de su Inversión en Seguridad",
            Some("Más allá de la protección, Axur devuelve recursos críticos a su organización. Transformamos el gasto en seguridad en eficiencia operativa.")
        );

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-[#09090b] text-white overflow-hidden">
                    <!-- Background -->
                    {bg_pattern}
                    <div class="absolute inset-0 bg-gradient-to-br from-orange-500/5 to-transparent pointer-events-none"></div>
                    
                    <!-- Content -->
                    <div class="relative z-10 h-full flex flex-col p-14">
                        {header}
                        
                        <div class="grid grid-cols-12 gap-12 flex-grow mt-4">
                            <!-- Left Column: The "Old Way" (Pain) -->
                            <div class="col-span-5 flex flex-col justify-center">
                                <div class="p-8 rounded-3xl border border-zinc-800 bg-zinc-900/50 backdrop-blur-sm grayscale opacity-70 hover:grayscale-0 hover:opacity-100 transition-all duration-500 group/pain">
                                    <h3 class="text-zinc-500 font-bold tracking-widest text-xs mb-6 uppercase flex items-center gap-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                        Sin Automatización
                                    </h3>
                                    <ul class="space-y-4 text-zinc-400">
                                        <li class="flex items-start gap-3">
                                            <span class="text-red-900/50 mt-1">✖</span>
                                            <span><strong>{tickets} incidentes</strong> revisados manualmente uno a uno.</span>
                                        </li>
                                        <li class="flex items-start gap-3">
                                            <span class="text-red-900/50 mt-1">✖</span>
                                            <span>Tiempos de respuesta lentos (días, no minutos).</span>
                                        </li>
                                        <li class="flex items-start gap-3">
                                            <span class="text-red-900/50 mt-1">✖</span>
                                            <span>Fatiga de alertas y burnout del equipo SOC.</span>
                                        </li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Center: Arrow -->
                            <div class="col-span-2 flex items-center justify-center relative">
                                <div class="w-full h-px bg-zinc-800 absolute top-1/2 left-0"></div>
                                <div class="bg-zinc-900 border border-zinc-700 p-2 rounded-full relative z-10 text-orange-500">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                                </div>
                            </div>

                            <!-- Right Column: The "Axur Way" (Value) -->
                            <div class="col-span-5 flex flex-col justify-center">
                                <div class="glass-panel-premium p-8 h-full flex flex-col justify-center relative overflow-hidden ring-1 ring-orange-500/30 shadow-[0_0_40px_rgba(255,103,31,0.1)]">
                                    <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-transparent"></div>
                                    <div class="absolute -top-10 -right-10 w-32 h-32 bg-orange-500/20 blur-3xl rounded-full"></div>
                                    
                                    <h3 class="text-orange-500 font-bold tracking-widest text-xs mb-8 uppercase flex items-center gap-2">
                                        <span class="w-1.5 h-1.5 rounded-full bg-orange-500 animate-pulse"></span>
                                        Eficiencia Axur
                                    </h3>
                                    
                                    <div class="text-center mb-8 relative z-10">
                                        <span class="hero-number shimmer-text">{analysts}</span>
                                        <p class="text-sm font-bold text-white mt-2">Analistas Dedicados (Full-Time)</p>
                                        <p class="text-xs text-zinc-400 mt-1">Equivalente en capacidad operativa ganada</p>
                                    </div>

                                    <div class="bg-zinc-950/50 rounded-xl p-4 border border-zinc-700/50 backdrop-blur-md relative z-10 text-center">
                                        <p class="text-green-400 font-bold text-lg mb-1">{success_rate:.1}%</p>
                                        <p class="text-[10px] text-zinc-500 uppercase tracking-widest">Tasa de Éxito en Takedowns</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Bottom CTA -->
                        <div class="mt-8 bg-zinc-900/30 border border-zinc-800 rounded-2xl p-6 flex justify-between items-center group/cta hover:bg-zinc-900 hover:border-orange-500/30 transition-all">
                            <div>
                                <h4 class="text-white font-bold text-lg mb-1">Maximice su Protección</h4>
                                <p class="text-zinc-500 text-sm">Nuestro equipo de expertos está listo para escalar estos resultados.</p>
                            </div>
                            <div class="bg-[#FF671F] text-white px-6 py-3 rounded-lg font-bold text-sm tracking-wide shadow-lg shadow-orange-900/20 group-hover/cta:shadow-orange-500/20 transition-all cursor-pointer">
                                AGENDAR REVISIÓN ESTRATÉGICA
                            </div>
                        </div>

                        <!-- Footer -->
                        {footer}
                    </div>
                </div>
            </div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = header,
            tickets = data.total_tickets,
            analysts = analysts_display,
            success_rate = data.takedown_success_rate,
            footer = footer_dark(12, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "roi".into(),
            html,
        }]
    }
}
