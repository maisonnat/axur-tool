//! TOC (Table of Contents) Slide Plugin — ACT 1: The Ordinary World → Threshold
//!
//! Narrative Role: The ROADMAP. Gives the reader a curated preview of what's coming,
//! with each chapter teasing its hero metric to build curiosity and urgency.
//!
//! Design: Full-width journey map with connecting timeline, dynamic KPI badges,
//! and a summary stats bar. No wasted space.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

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

        // ─── Build dynamic TOC items from actual report data ───
        struct TocChapter {
            num: &'static str,
            label: &'static str,
            desc: &'static str,
            kpi_value: String,
            kpi_label: &'static str,
            color: &'static str,
            enabled: bool,
        }

        let total_takedowns = data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved;

        let total_threats: u64 = data.threats_by_type.iter().map(|t| t.count).sum();

        let chapters = [
            TocChapter {
                num: "01",
                label: "Panorama de Amenazas",
                desc: "Distribución y tipología de amenazas detectadas",
                kpi_value: format_number(total_threats),
                kpi_label: "amenazas",
                color: "orange",
                enabled: !data.threats_by_type.is_empty(),
            },
            TocChapter {
                num: "02",
                label: "Exposición de Datos",
                desc: "Credenciales filtradas y secretos expuestos",
                kpi_value: format_number(data.credentials_total + data.secrets_total),
                kpi_label: "expuestos",
                color: "red",
                enabled: data.credentials_total > 0 || data.secrets_total > 0,
            },
            TocChapter {
                num: "03",
                label: "Incidentes Activos",
                desc: "Clasificación y evidencia de incidentes detectados",
                kpi_value: format_number(data.incidents_by_type.iter().map(|i| i.incidents).sum()),
                kpi_label: "incidentes",
                color: "yellow",
                enabled: !data.incidents_by_type.is_empty(),
            },
            TocChapter {
                num: "04",
                label: "Gestión de Takedowns",
                desc: "Proceso de eliminación de amenazas y resultados",
                kpi_value: format!("{:.0}%", data.takedown_success_rate),
                kpi_label: "éxito",
                color: "emerald",
                enabled: total_takedowns > 0,
            },
            TocChapter {
                num: "05",
                label: "Velocidad de Respuesta",
                desc: "Tiempos de reacción y ciclo de vida de amenazas",
                kpi_value: data.takedown_median_uptime.clone(),
                kpi_label: "mediana",
                color: "cyan",
                enabled: data
                    .resolved_takedowns
                    .iter()
                    .any(|td| td.request_date.is_some() && td.resolution_date.is_some()),
            },
            TocChapter {
                num: "06",
                label: "Análisis de Riesgo",
                desc: "Radar multidimensional de vectores de ataque",
                kpi_value: format!("{}", data.total_tickets),
                kpi_label: "vectores",
                color: "purple",
                enabled: data.total_tickets >= 5,
            },
            TocChapter {
                num: "07",
                label: "Retorno de Inversión",
                desc: "Impacto financiero y valor de la protección",
                kpi_value: "ROI".to_string(),
                kpi_label: "análisis",
                color: "emerald",
                enabled: true,
            },
        ];

        let enabled: Vec<&TocChapter> = chapters.iter().filter(|c| c.enabled).collect();

        // ─── Render chapter cards ───
        let chapters_html: String = enabled
            .iter()
            .enumerate()
            .map(|(idx, ch)| {
                // Alternate subtle highlight for visual rhythm
                let is_highlight = idx % 2 == 0;
                let card_bg = if is_highlight {
                    "bg-zinc-900/50"
                } else {
                    "bg-zinc-900/30"
                };

                format!(
                    r#"<div class="{card_bg} rounded-xl border border-zinc-800/40 p-3.5 backdrop-blur-sm hover:border-{color}-500/30 transition-all duration-300 hover:scale-[1.02] group/ch relative overflow-hidden">
                        <!-- Hover glow -->
                        <div class="absolute inset-0 bg-gradient-to-r from-{color}-500/0 to-{color}-500/5 opacity-0 group-hover/ch:opacity-100 transition-opacity rounded-xl"></div>

                        <div class="relative z-10 flex items-center gap-3">
                            <!-- Number -->
                            <span class="text-{color}-500/60 text-2xl font-black font-mono leading-none min-w-[28px]">{num}</span>

                            <!-- Content -->
                            <div class="flex-grow min-w-0">
                                <h4 class="text-white font-semibold text-sm leading-tight group-hover/ch:text-{color}-300 transition-colors">{label}</h4>
                                <p class="text-zinc-500 text-xs mt-0.5 leading-snug truncate">{desc}</p>
                            </div>

                            <!-- KPI Badge -->
                            <div class="flex flex-col items-end shrink-0">
                                <span class="text-{color}-400 font-bold text-sm font-mono leading-tight">{kpi_value}</span>
                                <span class="text-zinc-600 text-[10px] uppercase tracking-wider">{kpi_label}</span>
                            </div>
                        </div>
                    </div>"#,
                    card_bg = card_bg,
                    color = ch.color,
                    num = ch.num,
                    label = ch.label,
                    desc = ch.desc,
                    kpi_value = ch.kpi_value,
                    kpi_label = ch.kpi_label,
                )
            })
            .collect();

        // ─── Summary stats bar (top-level numbers) ───
        let stats_html = format!(
            r#"<div class="flex gap-3">
                <div class="flex-1 bg-zinc-900/40 border border-zinc-800/40 rounded-xl p-3 text-center backdrop-blur-sm">
                    <div class="text-2xl font-light text-orange-400 font-mono">{threats}</div>
                    <div class="text-[10px] text-zinc-500 uppercase tracking-widest mt-0.5">Amenazas</div>
                </div>
                <div class="flex-1 bg-zinc-900/40 border border-zinc-800/40 rounded-xl p-3 text-center backdrop-blur-sm">
                    <div class="text-2xl font-light text-red-400 font-mono">{exposed}</div>
                    <div class="text-[10px] text-zinc-500 uppercase tracking-widest mt-0.5">Expuestos</div>
                </div>
                <div class="flex-1 bg-zinc-900/40 border border-zinc-800/40 rounded-xl p-3 text-center backdrop-blur-sm">
                    <div class="text-2xl font-light text-emerald-400 font-mono">{rate:.0}%</div>
                    <div class="text-[10px] text-zinc-500 uppercase tracking-widest mt-0.5">Éxito</div>
                </div>
                <div class="flex-1 bg-zinc-900/40 border border-zinc-800/40 rounded-xl p-3 text-center backdrop-blur-sm">
                    <div class="text-2xl font-light text-cyan-400 font-mono">{uptime}</div>
                    <div class="text-[10px] text-zinc-500 uppercase tracking-widest mt-0.5">Mediana</div>
                </div>
            </div>"#,
            threats = format_number(total_threats),
            exposed = format_number(data.credentials_total + data.secrets_total),
            rate = data.takedown_success_rate,
            uptime = data.takedown_median_uptime,
        );

        // ─── Chapter count ──
        let chapter_count = enabled.len();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-12 mb-8 relative text-white overflow-hidden">
                <!-- Background -->
                {bg}

                <div class="relative z-10 flex flex-col h-full">
                    <!-- Header row -->
                    <div class="flex items-start justify-between mb-4">
                        <div>
                            <span class="text-orange-500 font-bold tracking-[0.3em] text-xs uppercase mb-1.5 block">Reporte Digital</span>
                            <h2 class="text-4xl font-black uppercase tracking-tight">{title}</h2>
                            <div class="w-20 h-1 bg-gradient-to-r from-orange-500 to-transparent mt-3"></div>
                        </div>
                        <div class="text-right">
                            <span class="text-zinc-600 text-xs uppercase tracking-widest">{count} capítulos</span>
                            <div class="flex items-center gap-1.5 mt-1.5 justify-end">
                                <span class="w-2 h-2 rounded-full bg-orange-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-red-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-yellow-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-emerald-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-cyan-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-purple-500/60"></span>
                                <span class="w-2 h-2 rounded-full bg-emerald-500/60"></span>
                            </div>
                        </div>
                    </div>

                    <!-- Stats bar -->
                    {stats}

                    <!-- Chapter grid (2 columns) -->
                    <div class="grid grid-cols-2 gap-2.5 mt-4 flex-grow">
                        {chapters}
                    </div>

                    <!-- Bottom quote -->
                    <div class="flex items-center justify-between mt-3 pt-2.5 border-t border-zinc-800/30">
                        <p class="text-zinc-600 text-xs italic">"La visibilidad es el primer paso hacia la seguridad."</p>
                        <div class="flex items-center gap-2">
                            <span class="text-[10px] text-zinc-700 uppercase tracking-wider">Confidencial</span>
                            <div class="w-1.5 h-1.5 rounded-full bg-orange-500/40"></div>
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div></div>"#,
            bg = crate::plugins::builtin::helpers::geometric_pattern(),
            title = t.get("toc_title"),
            count = chapter_count,
            stats = stats_html,
            chapters = chapters_html,
            footer = footer_dark(2, &t.get("footer_text")),
        );

        // Semantic replacements for static brand elements
        let html = html
            .replace(
                "text-orange-500 font-bold tracking-[0.3em]",
                "text-brand-primary font-bold tracking-[0.3em]",
            ) // Title label
            .replace(
                "from-orange-500 to-transparent",
                "from-[var(--color-primary)] to-transparent",
            ) // Underline gradient
            .replace("bg-orange-500/40", "bg-[rgba(255,103,31,0.4)]"); // Footer dot

        vec![SlideOutput {
            id: "toc".into(),
            html,
        }]
    }
}
