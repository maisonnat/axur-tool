//! PoC Data Slide Plugin — ACT 1: The Call to Adventure (Scope)
//!
//! Narrative Role: The DIGITAL FORTRESS. Shows the scope of protection not as a list,
//! but as an active, living defense system.
//!
//! Design: "Active Sonar" visualization where the Client is the core,
//! surrounded by concentric rings of protection (Brands > Execs > Infra).

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct PocDataSlidePlugin;

impl SlidePlugin for PocDataSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.poc_data"
    }
    fn name(&self) -> &'static str {
        "Scope & Assets"
    }
    fn priority(&self) -> i32 {
        96
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // ─── Build Asset Stats ───
        struct AssetGroup {
            label: &'static str,
            count: u32,
            icon: &'static str,
            color: &'static str,
        }

        let assets = [
            AssetGroup {
                label: "Marcas",
                count: data.brands_count,
                icon: "🛡️",
                color: "orange",
            },
            AssetGroup {
                label: "Ejecutivos",
                count: data.executives_count,
                icon: "👤",
                color: "blue",
            },
            AssetGroup {
                label: "Infraestructura",
                count: data.ips_count + data.domains_count,
                icon: "🌐",
                color: "emerald",
            },
        ];

        let assets_html: String = assets
            .iter()
            .map(|a| {
                format!(
                    r#"<div class="flex items-center justify-between p-4 bg-zinc-900/50 border border-zinc-800 rounded-xl backdrop-blur-sm">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-lg bg-{color}-500/10 flex items-center justify-center text-2xl border border-{color}-500/20">
                                {icon}
                            </div>
                            <div>
                                <div class="text-3xl font-bold text-white font-mono leading-none">{count}</div>
                                <div class="text-xs text-zinc-500 uppercase tracking-wider mt-1">{label}</div>
                            </div>
                        </div>
                        <div class="flex flex-col items-end">
                            <div class="flex items-center gap-1.5">
                                <span class="relative flex h-2 w-2">
                                  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-{color}-400 opacity-75"></span>
                                  <span class="relative inline-flex rounded-full h-2 w-2 bg-{color}-500"></span>
                                </span>
                                <span class="text-[10px] text-{color}-400 uppercase tracking-widest font-bold">Activo</span>
                            </div>
                        </div>
                    </div>"#,
                    label = a.label,
                    count = format_number(a.count as u64),
                    icon = a.icon,
                    color = a.color
                )
            })
            .collect();

        // ─── Brand Nodes (Orbit 1) ───
        // Position top 3 brands in a triangle layout around the core
        let brand_nodes: String = data
            .brands
            .iter()
            .take(3)
            .enumerate()
            .map(|(i, brand)| {
                // Calculate position on a circle (radius 160px)
                // -90deg is top (0), then 30 (1), 150 (2) for 3 items
                let angle_deg = -90.0 + (i as f64 * 120.0);
                let angle_rad = angle_deg.to_radians();
                let radius = 160.0;
                let x = radius * angle_rad.cos();
                let y = radius * angle_rad.sin();

                format!(
                    r#"<div class="absolute flex flex-col items-center gap-2 transform -translate-x-1/2 -translate-y-1/2 group/node"
                            style="left: calc(50% + {x}px); top: calc(50% + {y}px);">
                        <div class="w-12 h-12 bg-black border border-orange-500/50 rounded-full flex items-center justify-center shadow-[0_0_15px_rgba(255,103,31,0.3)] z-10 group-hover/node:scale-110 transition-transform duration-500">
                             <svg class="w-6 h-6 text-orange-500" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" /></svg>
                        </div>
                        <div class="px-3 py-1 bg-black/80 border border-zinc-800 rounded-full text-xs font-bold text-white tracking-wide whitespace-nowrap backdrop-blur-md opacity-0 group-hover/node:opacity-100 transition-opacity duration-300 transform translate-y-2 group-hover/node:translate-y-0">
                            {brand}
                        </div>
                        <!-- Connecting Line to center -->
                        <div class="absolute top-1/2 left-1/2 w-[160px] h-[1px] bg-gradient-to-r from-orange-500/0 via-orange-500/20 to-orange-500/0 origin-left -z-10"
                             style="transform: rotate({angle}deg) translateX(-160px);"></div>
                   </div>"#,
                    x = x,
                    y = y,
                    brand = brand,
                    angle = angle_deg + 180.0 // Line points inward
                )
            })
            .collect();

        // ─── Radar Scan SVG ───
        let radar_scan = r##"
            <div class="absolute inset-0 rounded-full overflow-hidden animate-[spin_8s_linear_infinite]">
                 <div class="absolute top-0 right-0 w-1/2 h-1/2 bg-gradient-to-bl from-orange-500/10 via-orange-500/0 to-transparent"></div>
            </div>
        "##;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex p-12 relative bg-black text-white overflow-hidden">
                <!-- Background Grid -->
                {bg}

                <div class="flex w-full gap-12 relative z-10">
                    <!-- Left Column: Context & Stats -->
                    <div class="w-1/3 flex flex-col justify-between py-4">
                        <div>
                            <span class="text-orange-500 font-bold tracking-[0.3em] text-xs uppercase mb-2 block">{title_scope}</span>
                            <h2 class="text-5xl font-black uppercase tracking-tight leading-none mb-6">{title_assets}</h2>
                            <div class="w-16 h-1 bg-orange-500 mb-8"></div>
                            
                            <p class="text-zinc-400 text-sm leading-relaxed mb-6">
                                Monitoreo activo y continuo las 24/7 sobre todo el perímetro digital, 
                                detectando amenazas desde la Deep Web hasta redes sociales y dominios fraudulentos.
                            </p>
                        </div>

                        <div class="flex flex-col gap-4">
                            {assets}
                        </div>

                        <!-- Duration Badge -->
                        <div class="mt-8 flex items-center gap-3 text-xs text-zinc-500 font-mono border-t border-zinc-800 pt-4">
                            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                            <span>SISTEMA ACTIVO: {start} → {end}</span>
                        </div>
                    </div>

                    <!-- Right Column: The Digital Fortress Visualization -->
                    <div class="w-2/3 relative flex items-center justify-center">
                        <!-- Orbit System Container -->
                        <div class="relative w-[500px] h-[500px] flex items-center justify-center">
                            
                            <!-- Outer Ring (Infra) -->
                            <div class="absolute inset-0 border border-zinc-800/60 rounded-full"></div>
                            <div class="absolute inset-0 border border-dashed border-zinc-700/30 rounded-full animate-[spin_60s_linear_infinite]"></div>
                            
                            <!-- Middle Ring (Execs) -->
                            <div class="absolute inset-[80px] border border-orange-500/10 rounded-full"></div>
                            <div class="absolute inset-[80px] border border-dashed border-orange-500/20 rounded-full animate-[spin_40s_linear_infinite_reverse]"></div>

                            <!-- Inner Ring (Brands) -->
                            <div class="absolute inset-[160px] border border-orange-500/30 rounded-full shadow-[0_0_30px_rgba(255,103,31,0.1)]"></div>

                            <!-- Radar Scan Effect -->
                            <div class="absolute inset-[20px] rounded-full opacity-50 pointer-events-none mix-blend-screen">
                                {radar}
                            </div>

                            <!-- Core (Company Logo) -->
                            <div class="absolute w-24 h-24 bg-zinc-900 border-2 border-orange-500 rounded-full flex items-center justify-center shadow-[0_0_50px_rgba(255,103,31,0.4)] z-20 relative">
                                <div class="absolute inset-0 bg-orange-500/10 rounded-full animate-ping opacity-20"></div>
                                <div class="text-center">
                                    <div class="text-2xl font-black text-white tracking-widest leading-none">AXUR</div>
                                    <div class="text-[8px] text-orange-400 uppercase tracking-widest mt-1">Core</div>
                                </div>
                            </div>

                            <!-- Floating Brand Nodes -->
                            {brands}

                            <!-- Decorative Data Points (Fake Nodes for Visual Density) -->
                            <div class="absolute top-[10%] right-[20%] w-2 h-2 bg-zinc-700 rounded-full animate-pulse"></div>
                            <div class="absolute bottom-[20%] left-[15%] w-1.5 h-1.5 bg-zinc-600 rounded-full animate-pulse delay-700"></div>
                            <div class="absolute top-[50%] right-[5%] w-1 h-1 bg-orange-500/50 rounded-full animate-ping"></div>
                        </div>
                    </div>
                </div>

                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            title_scope = t.get("poc_scope_title"), // "Alcance del Monitoreo"
            title_assets = t.get("poc_assets_title"), // "Activos Monitoreados"
            assets = assets_html,
            start = data.start_date,
            end = data.end_date,
            radar = radar_scan,
            brands = brand_nodes,
            footer = footer_dark(5, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "poc_data".into(),
            html,
        }]
    }
}
