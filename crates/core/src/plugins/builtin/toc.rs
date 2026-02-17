//! TOC (Table of Contents) Slide Plugin
//!
//! Displays the report table of contents with brand aesthetics.

use super::helpers::footer_light;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Embedded base64 TOC sidebar image
const TOC_IMAGE_BASE64: &str = include_str!("../../../assets/toc_image_base64.txt");

/// Plugin that generates the table of contents slide
pub struct TocSlidePlugin;

impl SlidePlugin for TocSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.toc"
    }
    fn name(&self) -> &'static str {
        "Table of Contents"
    }
    fn priority(&self) -> i32 {
        97
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        struct TocItem {
            label: &'static str,
            icon: &'static str,
            enabled: bool,
        }

        let total_takedowns = data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved;

        let items = [
            TocItem {
                label: "Visión General",
                icon: "📊",
                enabled: true, // Always show main metrics
            },
            TocItem {
                label: "Panorama de Amenazas",
                icon: "🌐",
                enabled: !data.threats_by_type.is_empty(),
            },
            TocItem {
                label: "Exposición de Datos",
                icon: "🔓",
                enabled: data.credentials_total > 0 || data.secrets_total > 0,
            },
            TocItem {
                label: "Resultados de Incidentes",
                icon: "⚠️",
                enabled: !data.incidents_by_type.is_empty(),
            },
            TocItem {
                label: "Gestión de Takedowns",
                icon: "🛡️",
                enabled: total_takedowns > 0,
            },
            TocItem {
                label: "Radar de Riesgo",
                icon: "🎯",
                enabled: data.total_tickets >= 5,
            },
        ];

        // Filter enabled items
        let enabled_items: Vec<&TocItem> = items.iter().filter(|i| i.enabled).collect();

        // Generate TOC items HTML
        let items_html: String = enabled_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                format!(
                    r#"<div class="glass-panel p-5 flex items-center gap-5 group/item hover:border-orange-500/50 transition-all duration-300 relative overflow-hidden backdrop-blur-md bg-zinc-900/40">
                        <div class="absolute inset-0 bg-gradient-to-r from-orange-500/0 via-orange-500/5 to-orange-500/0 translate-x-[-100%] group-hover/item:translate-x-[100%] transition-transform duration-1000"></div>
                        
                        <div class="relative">
                            <div class="absolute inset-0 bg-orange-500/20 blur-lg opacity-0 group-hover/item:opacity-100 transition-opacity"></div>
                            <div class="flex items-center justify-center w-12 h-12 rounded-xl bg-zinc-900/80 text-2xl group-hover/item:text-glow transition-all ring-1 ring-white/10 group-hover/item:ring-orange-500/50 relative z-10">
                                {icon}
                            </div>
                        </div>
                        
                        <div class="flex flex-col">
                           <span class="text-orange-500 text-[10px] font-bold uppercase tracking-widest mb-0.5">0{num}</span>
                           <span class="text-xl text-zinc-200 font-light group-hover/item:text-white transition-colors">{label}</span>
                        </div>
                        
                        <!-- Connector Dot -->
                        <div class="ml-auto w-1.5 h-1.5 rounded-full bg-zinc-700 group-hover/item:bg-orange-500 transition-colors shadow-[0_0_5px_rgba(255,88,36,0.5)]"></div>
                    </div>"#,
                    num = i + 1,
                    icon = item.icon,
                    label = item.label
                )
            })
            .collect();

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex shadow-lg mb-8 relative text-white overflow-hidden">
                    <!-- Background: Global Gradient + Pattern -->
                    <div class="absolute inset-0 z-0">
                        <div class="absolute inset-0 opacity-20" style="background-image: radial-gradient(circle, #3f3f46 1px, transparent 1px); background-size: 32px 32px;"></div>
                    </div>

                    <!-- Content Section -->
                    <div class="w-7/12 p-14 flex flex-col z-10 pl-20">
                        
                        <!-- Header -->
                        <div class="mb-10">
                            <span class="text-orange-500 font-bold tracking-[0.3em] text-sm uppercase mb-2 block">Reporte Digital</span>
                            <h2 class="text-5xl font-black display-text uppercase tracking-tight">{title}</h2>
                            <div class="w-24 h-1.5 bg-gradient-to-r from-orange-500 to-transparent mt-6"></div>
                        </div>
                        
                        <!-- TOC Grid -->
                        <div class="grid grid-cols-2 gap-4 relative">
                            {items}
                        </div>
                    </div>
                    
                    <!-- Sidebar Image -->
                    <div class="w-5/12 relative h-full">
                        <img 
                            src="data:image/png;base64,{image}" 
                            alt="Brand visual" 
                            class="absolute inset-0 w-full h-full object-cover opacity-50 mix-blend-overlay grayscale"
                        />
                        <div class="absolute inset-0 bg-gradient-to-r from-zinc-950 via-zinc-950/80 to-orange-900/20"></div>
                        
                        <!-- Decorative Quote -->
                        <div class="absolute bottom-20 left-10 right-10 text-right">
                            <p class="text-3xl font-thin italic text-white/90 leading-tight display-text">"La visibilidad es el<br>primer paso hacia<br>la seguridad."</p>
                            <div class="flex justify-end mt-6">
                                <div class="w-12 h-1 bg-[#FF5824] shadow-[0_0_15px_#FF5824]"></div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            title = t.get("toc_title"),
            items = items_html,
            image = TOC_IMAGE_BASE64.trim(),
            footer = footer_light(4, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "toc".into(),
            html,
        }]
    }
}
