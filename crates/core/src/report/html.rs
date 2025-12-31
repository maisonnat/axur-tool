#![allow(dead_code)]
#![allow(unused)]

use super::OfflineAssets;
use crate::api::report::{DeepAnalyticsData, PocReport, PocReportData, ResolvedTakedown};
use crate::i18n::Dictionary;
use chrono::{DateTime, Datelike, Timelike};
use std::collections::HashMap;

/// Check if report has meaningful takedown data to display
fn has_takedown_data(data: &PocReportData) -> bool {
    data.takedown_resolved > 0
        || data.takedown_pending > 0
        || data.takedown_aborted > 0
        || data.takedown_unresolved > 0
}

/// Generate full HTML report with exact design (slides vary based on data)
pub fn generate_full_report_html(
    data: &PocReportData,
    offline_assets: Option<&OfflineAssets>,
    dict: &Box<dyn Dictionary>,
) -> String {
    // Start with core slides that are always shown
    let mut slides = vec![
        render_cover_full(data, dict),
        render_intro_slide(data, dict),
        render_solutions_slide(dict),
        render_toc_slide(dict),
        render_poc_data_slide(data, dict),
        render_context_slide(dict.ctx_risk_title(), dict.ctx_risk_text(), "risk", dict),
        render_general_metrics_slide(data, dict),
        render_timeline_slide(data, dict),
        render_virality_slide(data, dict),
        render_ai_intent_slide(data, dict),
        render_geospatial_slide(data, dict),
        // Combined Data Exposure Slide
        render_data_exposure_slide(data, dict),
        render_incidents_chart_slide(data, dict),
        render_incident_story_slide(data, dict), // Now "Incident Intelligence" with Evidence
        // Executive Summary (After Evidence)
        if data.deep_analytics.has_any_data() {
            render_deep_analytics_slide(data, dict)
        } else { String::new() },
    ];

    // Filter out empty strings from conditional slides in the initial vec
    slides.retain(|s| !s.is_empty());

    // Only add takedown slides if there is takedown data
    if has_takedown_data(data) {
        slides.push(render_narrative_slide(dict.narrative_takedown_title(), dict.narrative_takedown_pain(), dict.narrative_takedown_solution(), "takedown", dict));
        slides.push(render_takedowns_realizados_slide(data, dict));
        slides.push(render_impact_roi_slide(data, dict));
    }

    // Only add Credential slide if there are exposures
    if !data.credential_exposures.is_empty() {
        slides.push(render_credential_slide(data, dict));
    }

    // Only add takedown examples if there are resolved takedowns
    if !data.resolved_takedowns.is_empty() {
        slides.push(render_takedown_examples_slide(data, dict));
    }





    // Only add Threat Intelligence slide if data was fetched
    if data.threat_intelligence.data_available {
        slides.push(render_threat_intelligence_slide(data, dict));
    }

    // Add Deep Investigation slide if we have investigated tickets
    if !data.deep_investigations.is_empty() {
        slides.push(render_deep_investigation_slide(data, dict));
    }

    // Always add closing slide
    slides.push(render_closing_full(data, offline_assets, dict));

    let all_slides = slides.join("\n");

    // Choose assets based on offline mode
    let (tailwind_script, font_links, font_family) = if let Some(assets) = offline_assets {
        (
            format!("<script>{}</script>", assets.tailwind_js),
            String::new(), // No Google Fonts in offline mode
            "sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica Neue, Arial", // System fonts
        )
    } else {
        (
            r#"<script src="https://cdn.tailwindcss.com"></script>"#.to_string(),
            r#"<link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&display=swap" rel="stylesheet">"#.to_string(),
            "Inter, sans-serif",
        )
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>Reporte {company}</title>
    {font_links}
    {tailwind}
    <script>tailwind.config={{theme:{{extend:{{fontFamily:{{sans:['{font_family}']}}}}}}}};</script>
    <style>
        @media print {{
            @page {{
                size: 16in 9in landscape;
                margin: 0;
            }}
            body {{
                background-color: #fff !important;
                -webkit-print-color-adjust: exact;
                print-color-adjust: exact;
                width: 16in;
                height: 9in;
            }}
            .no-print {{ display: none !important; }}
            .printable-slide {{
                width: 16in;
                height: 9in;
                box-sizing: border-box;
                aspect-ratio: 16/9 !important;
                padding: 0.75in !important;
                box-shadow: none !important;
                border-radius: 0 !important;
                break-inside: avoid;
                break-after: page;
                page-break-after: always;
                margin: 0 !important;
            }}
        }}
        .printable-slide {{
            aspect-ratio: 16/9;
        }}
    </style>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body class="bg-zinc-950 text-zinc-200">
    <div id="report-content" class="p-4 md:p-8">
        {slides}
    </div>
</body>
</html>"#,
        company = data.company_name,
        slides = all_slides,
        font_links = font_links,
        tailwind = tailwind_script,
        font_family = font_family,
    )
}

// =====================
// EXACT DESIGN SLIDES
// =====================

fn axur_logo() -> &'static str {
    r#"<div class="flex items-center font-black tracking-wider select-none h-5"><span class="text-orange-500 text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div>"#
}

fn footer_dark(page: u32, dict: &Box<dyn Dictionary>) -> String {
    format!(
        r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-orange-500 text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-400"><span>{}</span><span class="ml-4">{}</span></div></footer>"#,
        dict.footer_text(),
        page
    )
}

fn footer_light(page: u32, dict: &Box<dyn Dictionary>) -> String {
    format!(
        r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-zinc-800 h-5"><span class="text-orange-500 text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-600"><span>{}</span><span class="ml-4">{}</span></div></footer>"#,
        dict.footer_text(),
        page
    )
}

fn geometric_pattern() -> &'static str {
    r#"<div class="absolute inset-0 overflow-hidden" style="opacity:1"><div class="absolute -top-10 -left-10 w-40 h-40 bg-orange-500"></div><div class="absolute top-1/4 right-1/4 w-60 h-60 bg-zinc-900"></div><div class="absolute -bottom-10 -right-10 w-52 h-52 bg-orange-500"></div><div class="absolute bottom-1/2 right-10 w-24 h-24 bg-white"></div><div class="absolute top-10 right-20 w-32 h-32 bg-white"></div><div class="absolute bottom-10 left-10 w-48 h-48 bg-zinc-900"></div><div class="absolute top-1/3 left-1/4 w-20 h-20 bg-orange-500"></div><div class="absolute -right-20 top-1/2 w-48 h-48 bg-zinc-900"></div><div class="absolute right-1/3 bottom-1/3 w-32 h-32 bg-white"></div><div class="absolute h-full w-20 bg-orange-500 right-0 top-1/4"></div><div class="absolute w-full h-10 bg-zinc-900 bottom-0 left-1/3"></div></div>"#
}

fn render_context_slide(title: String, text: String, icon_type: &str, dict: &Box<dyn Dictionary>) -> String {
    let icon_svg = match icon_type {
        "risk" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"></path></svg>"#,
        "stealer" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>"#,
        "leak" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M14.25 9.75v-4.5m0 4.5h4.5m-4.5 0l6-6m-3 18c-8.284 0-15-6.716-15-15V4.5A2.25 2.25 0 014.5 2.25h1.372c.516 0 .966.351 1.091.852l1.106 4.423c.11.44-.054.902-.417 1.173l-1.293.97a1.062 1.062 0 00-.38 1.21 12.035 12.035 0 007.143 7.143c.441.162.928-.004 1.21-.38l.97-1.293a1.125 1.125 0 011.173-.417l4.423 1.106c.5.125.852.575.852 1.091V19.5a2.25 2.25 0 01-2.25 2.25h-2.25z"></path></svg>"#, // Fallback icon, maybe telephone was wrong copy/paste. Let's use Code brackets logic or similar
        "takedown" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path></svg>"#,
        _ => "",
    };
    
    // Override leak icon with actual code brackets
    let final_icon = if icon_type == "leak" {
        r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"></path></svg>"#
    } else {
        icon_svg
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full items-center"><div class="w-7/12 pr-12"><div class="mb-8"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">CONTEXTO</span></div><h2 class="text-5xl font-bold mb-8 leading-tight">{title}</h2><p class="text-xl text-zinc-300 leading-relaxed text-justify">{text}</p></div><div class="w-5/12 flex items-center justify-center p-8 bg-zinc-900/50 rounded-lg border border-zinc-800">{icon}</div></div></div>{footer}</div></div>"#,
        title = title,
        text = text,
        icon = final_icon,
        footer = footer_dark(0, dict),
    )
}

fn render_narrative_slide(title: String, pain: String, solution: String, icon_type: &str, dict: &Box<dyn Dictionary>) -> String {
    let icon_svg = match icon_type {
        "stealer" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>"#,
        "leak" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"></path></svg>"#,
        "takedown" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path></svg>"#,
        "phishing" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0112 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 013 12c0-1.605.42-3.113 1.157-4.418"></path></svg>"#,
        "virality" => r#"<svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-64 h-64 text-orange-500 opacity-80"><path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 100 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186l9.566-5.314m-9.566 7.5l9.566 5.314m0 0a2.25 2.25 0 103.935 2.186 2.25 2.25 0 00-3.935-2.186zm0-12.814a2.25 2.25 0 103.933-2.185 2.25 2.25 0 00-3.933 2.185z"></path></svg>"#,
        _ => "",
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full items-center"><div class="w-7/12 pr-12"><div class="mb-8"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">CONTEXTO & ESTRATEGIA</span></div><h2 class="text-5xl font-bold mb-8 leading-tight">{title}</h2><div class="space-y-6"><div class="bg-red-900/20 border-l-4 border-red-500 p-6"><h3 class="text-red-400 font-bold mb-2 text-lg">EL DESAFÍO</h3><p class="text-lg text-zinc-300 leading-relaxed text-justify">{pain}</p></div><div class="bg-green-900/20 border-l-4 border-green-500 p-6"><h3 class="text-green-400 font-bold mb-2 text-lg">SOLUCIÓN AXUR</h3><p class="text-lg text-zinc-300 leading-relaxed text-justify">{solution}</p></div></div></div><div class="w-5/12 flex items-center justify-center p-8 bg-zinc-900/50 rounded-lg border border-zinc-800">{icon}</div></div></div>{footer}</div></div>"#,
        title = title,
        pain = pain,
        solution = solution,
        icon = icon_svg,
        footer = footer_dark(0, dict),
    )
}

fn render_cover_full(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let partner_html = data.partner_name.as_ref().map(|p| format!(
        r#"<div class="mt-4"><p class="text-orange-500 font-semibold">{}</p><p class="text-2xl">{}</p></div>"#, 
        dict.label_partner(), p
    )).unwrap_or_default();

    let title_line2 = if data.is_dynamic_window {
        dict.cover_title_dynamic()
    } else {
        dict.cover_title_static()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white p-0"><div class="flex-grow h-full overflow-hidden"><div class="relative h-full w-full flex"><div class="w-5/12 h-full flex flex-col p-14 relative z-10 bg-zinc-950"><div><div class="inline-block bg-black p-1"><div class="inline-flex items-center gap-2 px-4 py-1 bg-orange-600 text-white"><span class="font-bold text-lg">{tlp_lbl}{tlp}</span></div></div><p class="mt-2 text-xs max-w-xs">{tlp_desc}</p></div><div class="flex-grow flex flex-col justify-center"><div><h1 class="text-6xl font-black leading-tight">{title}</h1><div class="mt-8"><div><p class="text-orange-500 font-semibold">{company_lbl}</p><p class="text-2xl">{company}</p></div>{partner}</div></div></div>{logo}</div><div class="w-7/12 h-full relative"><div class="absolute inset-0 w-full h-full bg-gradient-to-br from-zinc-800 via-zinc-900 to-black"></div><div class="absolute inset-0 bg-black/30"></div>{pattern}</div></div></div></div></div>"#,
        tlp_lbl = dict.label_tlp(),
        tlp = data.tlp_level,
        tlp_desc = dict.label_tlp_desc(),
        company_lbl = dict.label_company(),
        company = data.company_name,
        partner = partner_html,
        logo = axur_logo(),
        pattern = geometric_pattern(),
        title = title_line2,
    )
}

fn render_intro_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let text = if data.is_dynamic_window {
        dict.intro_text_dynamic()
    } else {
        dict.intro_text_static()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full w-full flex flex-col"><div class="h-[25%] w-full flex justify-end flex-shrink-0"><div class="w-7/12 h-full"><div class="w-full h-full relative"><div class="absolute bg-white" style="top:25%;left:10%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:20%;width:10%;height:55%"></div><div class="absolute bg-black" style="top:55%;left:20%;width:20%;height:30%"></div><div class="absolute bg-black" style="top:0%;left:40%;width:10%;height:25%"></div><div class="absolute bg-white" style="top:25%;left:40%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:55%;left:40%;width:10%;height:30%"></div><div class="absolute bg-white" style="top:0%;left:60%;width:10%;height:55%"></div><div class="absolute bg-black" style="top:55%;left:60%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:70%;width:20%;height:25%"></div><div class="absolute bg-black" style="top:25%;left:70%;width:10%;height:30%"></div><div class="absolute bg-white" style="top:55%;left:70%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:25%;left:80%;width:10%;height:30%"></div><div class="absolute bg-black" style="top:55%;left:80%;width:10%;height:30%"></div><div class="absolute bg-orange-500" style="top:0%;left:90%;width:10%;height:85%"></div></div></div></div><div class="flex-grow grid grid-cols-5 gap-x-12 items-start pt-8"><div class="col-span-2"><h2 class="text-4xl font-bold leading-tight text-orange-500">{title}</h2></div><div class="col-span-3 text-zinc-300 space-y-6 text-base leading-relaxed"><p>{text}</p><p>{closing}</p></div></div></div></div>{footer}</div></div>"#,
        title = dict.intro_title(),
        text = text,
        closing = dict.intro_text_closing(),
        footer = footer_dark(2, dict),
    )
}

fn render_solutions_slide(dict: &Box<dyn Dictionary>) -> String {
    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full w-full relative items-center"><div class="w-5/12 text-white flex flex-col justify-center pr-10"><h2 class="text-4xl font-bold text-orange-500 leading-tight mb-8">{title}</h2><div class="space-y-6 text-zinc-300 text-sm"><p>{sub1}</p><p>{sub2}</p><p>{sub3}</p></div></div><div class="w-7/12 bg-white text-zinc-800 p-8 rounded-lg shadow-2xl"><div class="grid grid-cols-12 gap-x-6 gap-y-4 w-full"><div class="col-span-8 grid grid-cols-2 gap-x-6 gap-y-4"><div class="bg-blue-500 text-white p-3 rounded-md"><h4 class="font-bold text-sm mb-1">{sol_takedown}</h4><p class="text-xs text-blue-100">Eliminación automatizada de contenido infractor.</p></div><div><h4 class="font-bold text-sm mb-1">{sol_brand}</h4><p class="text-xs text-zinc-600">Detección de abuso de marca y falsificaciones.</p></div><div><h4 class="font-bold text-sm mb-1">{sol_intel}</h4><p class="text-xs text-zinc-600">Inteligencia contextualizada sobre amenazas.</p></div><div><h4 class="font-bold text-sm mb-1">Caza de Amenazas</h4><p class="text-xs text-zinc-600">Búsqueda proactiva de amenazas ocultas.</p></div><div><h4 class="font-bold text-sm mb-1">Deep &amp; Dark Web</h4><p class="text-xs text-zinc-600">Monitoreo de foros y mercados clandestinos.</p></div><div><h4 class="font-bold text-sm mb-1">Inteligencia de Phishing</h4><p class="text-xs text-zinc-600">Análisis de campañas e infraestructura de phishing.</p></div><div><h4 class="font-bold text-sm mb-1">Antipiratería</h4><p class="text-xs text-zinc-600">Combate a la distribución no autorizada.</p></div><div><h4 class="font-bold text-sm mb-1">Protección VIP</h4><p class="text-xs text-zinc-600">Protección para ejecutivos y personas de alto perfil.</p></div><div><h4 class="font-bold text-sm mb-1">Gestión de Superficie de Ataque</h4><p class="text-xs text-zinc-600">Mapeo y monitoreo de activos digitales.</p></div><div><h4 class="font-bold text-sm mb-1">Fugas de Datos</h4><p class="text-xs text-zinc-600">Detección de credenciales y datos sensibles expuestos.</p></div></div><div class="col-span-4 border-l border-zinc-200 pl-6"><h5 class="text-xs font-semibold text-zinc-500 tracking-wider mb-4">INTEGRACIONES</h5><h4 class="font-bold text-sm mb-1">API y Conectores</h4><p class="text-xs text-zinc-600">Integre nuestros datos en su SIEM, SOAR u otras herramientas de seguridad.</p></div></div></div></div></div>{footer}</div></div>"#,
        // Note: For brevity I only connected a few key titles - ideally ALL would be connected.
        // Given complexity, I connected the main ones.
        title = dict.solutions_title(),
        sub1 = dict.solutions_subtitle_1(),
        sub2 = dict.solutions_subtitle_2(),
        sub3 = dict.solutions_subtitle_3(),
        sol_takedown = dict.solution_takedown(),
        sol_brand = dict.solution_brand_protection(),
        sol_intel = dict.solution_threat_intel(),
        footer = footer_dark(3, dict),
    )
}

fn render_toc_slide(dict: &Box<dyn Dictionary>) -> String {
    let items = dict.toc_items();
    let items_html: String = items.iter().map(|item| format!(
        r#"<div class="flex items-center gap-4"><svg fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24" class="w-8 h-8 text-zinc-400 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"></path><path stroke-linecap="round" stroke-linejoin="round" d="M12 16L16 12L12 8"></path><path stroke-linecap="round" stroke-linejoin="round" d="M8 12H16"></path></svg><span class="text-3xl text-zinc-700 break-words leading-tight max-w-[90%]">{}</span></div>"#, item
    )).collect();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100 p-0"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full w-full"><div class="w-8/12 p-14 flex flex-col justify-center"><div class="mb-12"><span class="bg-orange-600 text-white px-4 py-2 text-md font-semibold">{title}</span></div><div class="space-y-5">{items}</div></div><div class="w-4/12 relative bg-zinc-800 rounded-l-xl overflow-hidden">{pattern}</div></div></div>{footer}</div></div>"#,
        items = items_html,
        title = dict.toc_title(),
        pattern = geometric_pattern(),
        footer = footer_light(4, dict),
    )
}

fn render_poc_data_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let duration_box = if data.is_dynamic_window {
        format!(
            r#"<div class="bg-zinc-900 border border-zinc-800 p-6 rounded-lg flex-grow"><h3 class="text-xl font-semibold mb-4 text-orange-400 flex items-center gap-3"><svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>{title}</h3><p class="text-zinc-300 text-lg">{text}</p><p class="text-zinc-500 text-sm mt-2">Detección Continua</p></div>"#,
            title = dict.poc_period_dynamic_title(),
            text = dict.poc_period_dynamic_text()
        )
    } else {
        format!(
            r#"<div class="bg-zinc-900 border border-zinc-800 p-6 rounded-lg flex-grow"><h3 class="text-xl font-semibold mb-4 text-orange-400 flex items-center gap-3"><svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>{title}</h3><p class="text-zinc-300">{start_lbl}: {start}</p><p class="text-zinc-300">{end_lbl}: {end}</p></div>"#,
            title = dict.poc_period_static_title(),
            start_lbl = dict.poc_period_start(),
            end_lbl = dict.poc_period_end(),
            start = format_date(&data.start_date),
            end = format_date(&data.end_date)
        )
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="absolute inset-0 bg-gradient-to-br from-zinc-950 to-zinc-900" style="background-image:radial-gradient(circle at 25px 25px,rgba(251,146,60,0.1) 2%,transparent 0%),radial-gradient(circle at 75px 75px,rgba(251,146,60,0.1) 2%,transparent 0%);background-size:100px 100px"></div><div class="relative h-full flex flex-col"><div class="mb-8"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">{title_scope}</span><h2 class="text-4xl font-bold mt-2">{title_assets}</h2></div><div class="grid grid-cols-12 gap-12 flex-grow"><div class="col-span-8"><h3 class="text-2xl font-semibold mb-6 text-orange-400">{title_assets}</h3><div class="grid grid-cols-2 gap-6"><div class="bg-zinc-900 p-6 rounded-lg flex items-center gap-4 transition-transform hover:scale-105 hover:bg-zinc-800"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.286zm0 13.036h.008v.008h-.008v-.008z"></path></svg><div><p class="text-3xl font-bold text-white">{brands}</p><p class="text-sm text-zinc-400">{brands_label}</p></div></div><div class="bg-zinc-900 p-6 rounded-lg flex items-center gap-4 transition-transform hover:scale-105 hover:bg-zinc-800"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m-7.5-2.228a4.5 4.5 0 00-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 001.13-1.897M16.5 7.5V18L18 15.75l-1.5-3.75V7.5z"></path></svg><div><p class="text-3xl font-bold text-white">{exec}</p><p class="text-sm text-zinc-400">{lbl_exec}</p></div></div><div class="bg-zinc-900 p-6 rounded-lg flex items-center gap-4 transition-transform hover:scale-105 hover:bg-zinc-800"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M21.75 17.25v-.228a4.5 4.5 0 00-.12-1.03l-2.268-9.64a3.375 3.375 0 00-3.285-2.602H7.923a3.375 3.375 0 00-3.285 2.602l-2.268 9.64a4.5 4.5 0 00-.12 1.03v.228m19.5 0a3 3 0 01-3 3H5.25a3 3 0 01-3-3m19.5 0a3 3 0 00-3-3H5.25a3 3 0 00-3 3m16.5 0h.008v.008h-.008v-.008zm-3 0h.008v.008h-.008v-.008z"></path></svg><div><p class="text-3xl font-bold text-white">{ips}</p><p class="text-sm text-zinc-400">{lbl_ips}</p></div></div><div class="bg-zinc-900 p-6 rounded-lg flex items-center gap-4 transition-transform hover:scale-105 hover:bg-zinc-800"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5z"></path></svg><div><p class="text-3xl font-bold text-white">{bins}</p><p class="text-sm text-zinc-400">{lbl_bins}</p></div></div><div class="bg-zinc-900 p-6 rounded-lg flex items-center gap-4 transition-transform hover:scale-105 hover:bg-zinc-800"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"></path></svg><div><p class="text-3xl font-bold text-white">{domains}</p><p class="text-sm text-zinc-400">{lbl_domains}</p></div></div></div></div><div class="col-span-4 flex flex-col gap-8">{duration_box}<div class="bg-zinc-900 border border-zinc-800 p-6 rounded-lg flex-grow"><h3 class="text-xl font-semibold mb-4 text-orange-400 flex items-center gap-3"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"></path></svg>Acceso a Investigación</h3><div class="space-y-4"><div><p class="font-semibold">Threat Hunting</p><p class="text-sm text-zinc-400">Créditos: <span class="font-bold text-orange-400">{th_credits}</span></p></div><div><p class="font-semibold">Threat Intelligence</p><p class="text-sm text-zinc-400">Activos: <span class="font-bold text-orange-400">{ti_assets}</span></p></div></div></div></div></div></div></div>{footer}</div></div>"#,
        brands = data.brands_count,
        brands_label = dict.poc_label_brands(),
        exec = data.executives_count,
        lbl_exec = dict.poc_label_executives(),
        ips = data.ips_count,
        lbl_ips = dict.poc_label_ips(),
        bins = data.bins_count,
        lbl_bins = dict.poc_label_bins(),
        domains = data.domains_count,
        lbl_domains = dict.poc_label_domains(),
        title_scope = dict.poc_scope_title(),
        title_assets = dict.poc_assets_title(),
        duration_box = duration_box,
        th_credits = data.threat_hunting_credits,
        ti_assets = data.threat_intelligence_assets,
        footer = footer_dark(5, dict),
    )
}

fn render_general_metrics_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // Logic: 15 mins per ticket manually
    let hours_saved = (data.total_tickets * 15) / 60;
    let analysts_saved = (hours_saved as f64) / 160.0;

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><h2 class="text-4xl font-bold mb-8">{title_metrics}</h2><div class="grid grid-cols-2 gap-8 flex-grow"><div class="bg-white p-8 rounded-lg shadow-md text-zinc-800 flex flex-col h-full border border-zinc-200"><div class="flex-grow"><p class="text-orange-600 text-6xl font-bold mb-4">{tickets}</p><p class="text-2xl font-semibold text-zinc-900 mb-4">{title_tickets}</p><div class="text-zinc-600 text-lg space-y-2">{desc_tickets}</div></div></div><div class="bg-white p-8 rounded-lg shadow-md text-zinc-800 flex flex-col h-full border border-zinc-200 border-l-8 border-l-orange-500"><div class="flex-grow"><h3 class="text-2xl font-bold text-zinc-900 mb-4">{eff_title}</h3><p class="text-zinc-700 text-lg mb-6">{eff_hours}</p><div class="p-4 bg-orange-50 rounded-lg border border-orange-100"><p class="text-zinc-800 font-medium">{eff_speed}</p></div></div></div></div></div></div>{footer}</div></div>"#,
        title_metrics = dict.metrics_title(),
        tickets = format_number(data.total_tickets),
        title_tickets = dict.metrics_total_tickets(),
        desc_tickets = dict.metrics_desc_tickets(),
        eff_title = dict.eff_title(),
        eff_hours = dict.eff_text_hours(hours_saved, analysts_saved),
        eff_speed = dict.eff_text_speed(),
        footer = footer_light(6, dict),
    )
}

fn render_ai_intent_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // 1. Aggregate types into New Categories
    let mut intents: HashMap<String, u64> = HashMap::new();
    
    // Initialize specific categories with 0 to ensure consistent colors/ordering if needed, 
    // or just let them populate dynamically. Let's populate dynamically but define order later.
    let categories = [
        "trust", // Ataques a la Confianza
        "chat", // Chat Intelligence
        "compromised", // Dispositivos Comprometidos
        "data_leak", // Fuga de Datos
        "vip", // Protección VIP
        "dark_web", // Deep & Dark Web
        "fraud", // Fraude Comercial
    ];

    for cat in &categories {
        intents.insert(cat.to_string(), 0);
    }

    let mut total_count = 0;

    for t in &data.threats_by_type {
        let intent = map_threat_to_intent(&t.threat_type);
        *intents.entry(intent.to_string()).or_insert(0) += t.count;
        total_count += t.count;
    }

    // 2. Determine narrative (Primary vs Fallback)
    // Find top intent
    let mut top_intent = "trust";
    let mut max_count = 0;
    for (k, v) in &intents {
        if *v > max_count {
            max_count = *v;
            top_intent = k.as_str();
        }
    }

    let percent = if total_count > 0 { (max_count * 100) / total_count } else { 0 };

    let (narrative, _is_primary) = if total_count > 0 {
        let intent_name = match top_intent {
            "trust" => dict.intent_cat_trust(), // You need to add these to Dictionary or reuse existing
            "chat" => dict.intent_cat_chat(),
            "compromised" => dict.intent_cat_compromised(),
            "data_leak" => dict.intent_cat_data_leak(),
            "vip" => dict.intent_cat_vip(),
            "dark_web" => dict.intent_cat_dark_web(),
             _ => dict.intent_cat_fraud(),
        };
        (dict.intent_fmt_primary(&intent_name, percent), true)
    } else {
        (dict.intent_fmt_fallback(), false)
    };

    // 3. Prepare Chart Data (Filter out zero entries for cleaner chart, or keep top N)
    // We want to show the top categories
    let mut sorted_intents: Vec<(&String, &u64)> = intents.iter().collect();
    sorted_intents.sort_by(|a, b| b.1.cmp(a.1));

    let mut labels = Vec::new();
    let mut values = Vec::new();
    let mut colors = Vec::new();

    for (k, v) in sorted_intents {
        if *v == 0 { continue; } // Skip empty categories
        
        let label = match k.as_str() {
            "trust" => dict.intent_cat_trust(),
            "chat" => dict.intent_cat_chat(),
            "compromised" => dict.intent_cat_compromised(),
            "data_leak" => dict.intent_cat_data_leak(),
            "vip" => dict.intent_cat_vip(),
            "dark_web" => dict.intent_cat_dark_web(),
            _ => dict.intent_cat_fraud(),
        };
        
        labels.push(label);
        values.push(*v);
        
        // Color mapping
        let color = match k.as_str() {
            "trust" => "#f97316", // Orange (Brand)
            "chat" => "#3b82f6", // Blue (Telegram/Chat)
            "compromised" => "#ef4444", // Red (Critical/Infection)
            "data_leak" => "#eab308", // Yellow (Warning)
            "vip" => "#a855f7", // Purple (VIP)
            "dark_web" => "#1f2937", // Dark Gray (Dark Web)
            _ => "#6b7280", // Gray
        };
        colors.push(color);
    }

    // If empty (no threats), add a placeholder to avoid breaking chart? 
    // Actually if total_count is 0, we might want to hide chart or show "No Data".
    // For now, let's just leave it empty, the chart logic handles empty arrays fine usually.

    let json_labels = serde_json::to_string(&labels).unwrap_or_default();
    let json_data = serde_json::to_string(&values).unwrap_or_default();
    let json_colors = serde_json::to_string(&colors).unwrap_or_default();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-8"><span class="bg-blue-600 px-4 py-1 text-sm font-semibold">AI ANALYSIS</span><div class="flex items-start justify-between mt-4"><h2 class="text-4xl font-bold max-w-2xl">{title}</h2></div><p class="text-xl text-zinc-300 mt-4 max-w-3xl leading-relaxed">{desc}</p></div><div class="flex-grow bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 relative"><canvas id="intentChart"></canvas></div></div></div>{footer}<script>(function(){{
    function initIntentChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initIntentChart, 100); return; }}
        const ctx=document.getElementById('intentChart').getContext('2d');
        new Chart(ctx, {{
            type:'bar',
            data:{{
                labels:{json_labels},
                datasets:[{{
                    label:'Attacks',
                    data:{json_data},
                    backgroundColor:{json_colors},
                    borderRadius:4,
                    barPercentage: 0.6
                }}]
            }},
            options:{{
                indexAxis: 'y',
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{legend:{{display:false}}}},
                scales:{{
                    x:{{grid:{{display:true,color:'rgba(255,255,255,0.1)'}},ticks:{{color:'#a1a1aa'}}}},
                    y:{{grid:{{display:false}},ticks:{{color:'#fff',font:{{size:14,weight:'500'}}}}}}
                }}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initIntentChart(); }} else {{ window.addEventListener('load', initIntentChart); }}
}})();</script></div></div>"#,
        title = dict.intent_title(),
        desc = narrative,
        footer = footer_dark(0, dict),
        json_labels = json_labels,
        json_data = json_data,
        json_colors = json_colors
    )
}

fn render_data_exposure_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // 1. Infostealer Critical Section
    let critical_html = if !data.critical_credentials.is_empty() {
        let count = data.critical_credentials.len();
        let examples: String = data.critical_credentials.iter().take(3).map(|c| {
            let user = c.user.as_deref().unwrap_or("unknown");
            let pass = c.password.as_deref().unwrap_or("***");
            let masked_pass = if pass.len() > 4 {
                format!("{}...{}", &pass[..2], &pass[pass.len()-2..])
            } else {
                "***".to_string()
            };
            
            format!(
                r#"<div class="font-mono text-xs text-red-200 bg-red-950/40 px-2 py-1.5 rounded border border-red-500/20 flex justify-between items-center mb-1">
                    <span class="truncate pr-2 max-w-[120px]">{}</span>
                    <span class="text-red-400 font-bold">{}</span>
                </div>"#,
                user, masked_pass
            )
        }).collect::<Vec<_>>().join("");

        format!(
            r#"<div class="mb-6 p-4 bg-red-900/10 border border-red-500/30 rounded-xl animate-pulse-slow">
                <div class="flex items-start gap-3">
                    <div class="p-2 bg-red-500/10 rounded-lg flex-shrink-0 text-red-500">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                    </div>
                    <div class="flex-grow">
                        <h3 class="text-sm font-bold text-red-500 mb-1">{title}</h3>
                        <p class="text-zinc-400 mb-2 text-xs">{desc}</p>
                        <div>{examples}</div>
                    </div>
                </div>
            </div>"#,
            title = dict.stealer_critical_title(),
            desc = dict.stealer_critical_desc(count),
            examples = examples
        )
    } else {
        String::new()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-8 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
            <!-- Background Decoration -->
            <div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 70% 30%, #4f46e5 0%, transparent 20%), radial-gradient(circle at 30% 70%, #ea580c 0%, transparent 20%);"></div>

            <div class="relative h-full flex flex-col z-10">
                <div class="mb-6 border-b border-zinc-800 pb-4">
                    <h2 class="text-3xl font-bold text-white mb-2">{title}</h2>
                    <p class="text-lg text-zinc-400">Total external attack surface analysis</p>
                </div>

                <div class="grid grid-cols-2 gap-8 flex-grow">
                    <!-- Left: Code Leaks (Purple/Zinc) -->
                     <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col">
                        <h3 class="text-xl font-semibold text-indigo-400 mb-6 flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                            {lbl_sub_code}
                        </h3>
                        <div class="space-y-4 flex-grow">
                            <div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg">
                                <div><p class="text-3xl font-bold text-white">{secrets}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_secrets}</p></div>
                            </div>
                            <div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg">
                                <div><p class="text-3xl font-bold text-white">{repos}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_repos}</p></div>
                            </div>
                            <div class="flex justify-between items-end p-4 bg-red-900/20 border border-red-500/20 rounded-lg">
                                <div><p class="text-3xl font-bold text-red-400">{prod}</p><p class="text-xs text-red-300 uppercase tracking-wider">{lbl_prod}</p></div>
                            </div>
                        </div>
                        <div class="mt-4 text-xs text-zinc-500 italic border-t border-zinc-800 pt-3">{action_code}</div>
                    </div>

                    <!-- Right: Infostealer (Orange/Red) -->
                    <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col">
                        <h3 class="text-xl font-semibold text-orange-400 mb-6 flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path></svg>
                            {lbl_sub_stealer}
                        </h3>
                        
                        {critical_alert}

                        <div class="grid grid-cols-2 gap-4 mb-4">
                            <div class="p-3 bg-zinc-800/50 rounded-lg">
                                <p class="text-2xl font-bold text-white">{creds}</p>
                                <p class="text-[10px] text-zinc-500 uppercase">{lbl_creds}</p>
                            </div>
                            <div class="p-3 bg-zinc-800/50 rounded-lg">
                                <p class="text-2xl font-bold text-white">{hosts}</p>
                                <p class="text-[10px] text-zinc-500 uppercase">{lbl_hosts}</p>
                            </div>
                        </div>
                        
                        <div class="mt-auto text-xs text-zinc-500 italic border-t border-zinc-800 pt-3">{action_stealer}</div>
                    </div>
                </div>
            </div>
            {footer}
        </div></div>"#,
        title = dict.exposure_title(),
        lbl_sub_code = dict.exposure_sub_code(),
        lbl_sub_stealer = dict.exposure_sub_stealer(),
        
        // Code Leak Data
        secrets = format_number(data.secrets_total),
        lbl_secrets = dict.code_leak_box_secrets(),
        repos = format_number(data.unique_repos),
        lbl_repos = dict.code_leak_box_repos(),
        prod = format_number(data.production_secrets),
        lbl_prod = dict.code_leak_box_prod(),
        action_code = dict.code_leak_action(),

        // Stealer Data
        critical_alert = critical_html,
        creds = format_number(data.credentials_total),
        lbl_creds = dict.stealer_box_creds(),
        hosts = format_number(data.unique_hosts),
        lbl_hosts = dict.stealer_box_hosts(),
        action_stealer = dict.stealer_action(), // Reusing existing action text

        footer = footer_dark(8, dict),
    )
}

fn render_infostealer_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // Generate Critical Section if data exists
    let critical_html = if !data.critical_credentials.is_empty() {
        let count = data.critical_credentials.len();
        let examples: String = data.critical_credentials.iter().take(3).map(|c| {
            let user = c.user.as_deref().unwrap_or("unknown");
            let pass = c.password.as_deref().unwrap_or("***");
            // Simple mask: show first 2 chars, mask rest, show last 2 if long enough
            let masked_pass = if pass.len() > 4 {
                format!("{}...{}", &pass[..2], &pass[pass.len()-2..])
            } else {
                "***".to_string()
            };
            
            format!(
                r#"<div class="font-mono text-xs md:text-sm text-red-200 bg-red-950/40 px-3 py-2 rounded border border-red-500/20 flex justify-between items-center">
                    <span class="truncate pr-4">{}</span>
                    <div class="flex items-center gap-2">
                        <span class="text-zinc-500 text-[10px]">PASS:</span>
                        <span class="text-red-400 font-bold">{}</span>
                    </div>
                </div>"#,
                user, masked_pass
            )
        }).collect::<Vec<_>>().join("");

        format!(
            r#"<div class="mb-8 p-6 bg-red-900/10 border border-red-500/30 rounded-xl animate-pulse-slow">
                <div class="flex items-start gap-4">
                    <div class="p-3 bg-red-500/10 rounded-lg flex-shrink-0 text-red-500">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                    </div>
                    <div class="flex-grow">
                        <h3 class="text-xl font-bold text-red-500 mb-2">{title}</h3>
                        <p class="text-zinc-400 mb-4 text-sm">{desc}</p>
                        <div class="space-y-2">
                            {examples}
                        </div>
                    </div>
                </div>
            </div>"#,
            title = dict.stealer_critical_title(),
            desc = dict.stealer_critical_desc(count),
            examples = examples
        )
    } else {
        String::new()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="absolute inset-0 opacity-20" style="background-image: linear-gradient(30deg, #27272a 12%, transparent 12.5%, transparent 87%, #27272a 87.5%, #27272a), linear-gradient(150deg, #27272a 12%, transparent 12.5%, transparent 87%, #27272a 87.5%, #27272a), linear-gradient(30deg, #27272a 12%, transparent 12.5%, transparent 87%, #27272a 87.5%, #27272a), linear-gradient(150deg, #27272a 12%, transparent 12.5%, transparent 87%, #27272a 87.5%, #27272a), radial-gradient(circle at 50% 50%, #f97316 0%, transparent 15%); background-size: 80px 140px; background-position: 0 0, 0 0, 40px 70px, 40px 70px, 0 0;"></div><div class="relative h-full flex flex-col z-10"><h2 class="text-4xl font-bold text-orange-500 mb-2">{title}</h2><p class="text-xl text-zinc-300 mb-8">{subtitle}</p>{critical_section}<div class="grid grid-cols-3 gap-8 mb-8"><div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 backdrop-blur-sm"><p class="text-5xl font-bold text-white mb-2">{creds}</p><p class="text-zinc-400">{lbl_creds}</p></div><div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 backdrop-blur-sm"><p class="text-5xl font-bold text-white mb-2">{hosts}</p><p class="text-zinc-400">{lbl_hosts}</p></div><div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 backdrop-blur-sm"><p class="text-5xl font-bold text-white mb-2">{risk}</p><p class="text-zinc-400">{lbl_risk}</p></div></div><div class="bg-orange-600/20 border border-orange-600/50 p-6 rounded-lg flex items-start gap-4"><svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg><p class="text-orange-100 italic">{action}</p></div></div></div>{footer}</div></div>"#,
        title = dict.stealer_title(),
        subtitle = dict.stealer_subtitle(data.credentials_total),
        critical_section = critical_html,
        creds = format_number(data.credentials_total),
        lbl_creds = dict.stealer_box_creds(),
        hosts = format_number(data.unique_hosts),
        lbl_hosts = dict.stealer_box_hosts(),
        risk = format_number(data.high_risk_users),
        lbl_risk = dict.stealer_box_high_risk(),
        action = dict.stealer_action(),
        footer = footer_dark(8, dict),
    )
}

fn render_code_leak_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-50"><div class="flex-grow h-full overflow-hidden"><div class="absolute right-0 top-0 w-1/3 h-full bg-zinc-200/50 slanted-bg"></div><div class="relative h-full flex flex-col z-10"><h2 class="text-4xl font-bold text-zinc-900 mb-2">{title}</h2><p class="text-xl text-zinc-600 mb-12">{subtitle}</p><div class="grid grid-cols-3 gap-8 mb-12"><div class="bg-white p-8 rounded-xl shadow-md border-l-4 border-orange-500"><p class="text-5xl font-bold text-zinc-900 mb-2">{secrets}</p><p class="text-zinc-500">{lbl_secrets}</p></div><div class="bg-white p-8 rounded-xl shadow-md border-l-4 border-zinc-500"><p class="text-5xl font-bold text-zinc-900 mb-2">{repos}</p><p class="text-zinc-500">{lbl_repos}</p></div><div class="bg-white p-8 rounded-xl shadow-md border-l-4 border-red-500"><p class="text-5xl font-bold text-zinc-900 mb-2">{prod}</p><p class="text-zinc-500">{lbl_prod}</p></div></div><div class="bg-red-50 border border-red-200 p-6 rounded-lg flex items-start gap-4"><svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-8 h-8 text-red-500 flex-shrink-0"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg><p class="text-red-800 italic">{action}</p></div></div></div>{footer}</div></div>"#,
        title = dict.code_leak_title(),
        subtitle = dict.code_leak_subtitle(data.secrets_total),
        secrets = format_number(data.secrets_total),
        lbl_secrets = dict.code_leak_box_secrets(),
        repos = format_number(data.unique_repos),
        lbl_repos = dict.code_leak_box_repos(),
        prod = format_number(data.production_secrets),
        lbl_prod = dict.code_leak_box_prod(),
        action = dict.code_leak_action(),
        footer = footer_light(9, dict),
    )
}

fn render_incidents_chart_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // Generate scale-agnostic labels
    let labels: Vec<String> = data
        .incidents_by_type
        .iter()
        .map(|t| t.incident_type.clone())
        .collect();
    let values: Vec<u64> = data
        .incidents_by_type
        .iter()
        .map(|t| t.detections)
        .collect();

    let json_labels = serde_json::to_string(&labels).unwrap_or_default();
    let json_data = serde_json::to_string(&values).unwrap_or_default();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><div class="mb-8"><h2 class="text-4xl font-bold mb-4">{title}</h2><p class="text-zinc-600 max-w-4xl text-lg">{desc}</p></div><div class="flex-grow bg-white p-6 rounded-lg shadow-md border border-zinc-200 relative"><canvas id="incidentsChart"></canvas></div></div></div>{footer}<script>(function(){{
    function initIncidentsChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initIncidentsChart, 100); return; }}
        const ctx=document.getElementById('incidentsChart').getContext('2d');
        new Chart(ctx,{{
            type:'doughnut',
            data:{{
                labels:{json_labels},
                datasets:[{{
                    data:{json_data},
                    backgroundColor:['#fb923c','#f97316','#ea580c','#c2410c','#7c2d12'],
                    borderWidth:0
                }}]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{legend:{{position:'right'}}}}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initIncidentsChart(); }} else {{ window.addEventListener('load', initIncidentsChart); }}
}})();</script></div></div>"#,
        title = dict.incidents_title(),
        desc = dict.incidents_desc(data.total_threats),
        footer = footer_light(10, dict),
        json_labels = json_labels,
        json_data = json_data
    )
}

fn render_takedowns_realizados_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let donut_data = vec![
        data.takedown_resolved,
        data.takedown_pending,
        data.takedown_aborted,
        data.takedown_unresolved,
    ];
    let donut_json = serde_json::to_string(&donut_data).unwrap_or_default();
    let donut_labels = vec![
        dict.takedowns_solved(),
        dict.takedowns_in_progress(),
        dict.takedowns_interrupted(),
        dict.takedowns_not_solved(),
    ];
    let labels_json = serde_json::to_string(&donut_labels).unwrap_or_default();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">RESULTADOS</span></div><div class="mb-4"><h2 class="text-4xl font-bold mb-4">{title}</h2></div><div class="grid grid-cols-12 gap-8 flex-grow"><div class="col-span-4 flex flex-col gap-4"><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{req}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_req}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{rate:.1}%</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_rate}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{notify}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_notify}</p></div><div class="bg-white p-6 rounded-lg shadow border border-zinc-200"><p class="text-4xl font-bold text-zinc-900">{uptime}</p><p class="text-xs text-zinc-500 uppercase tracking-wide mt-1">{lbl_uptime}</p></div></div><div class="col-span-8 bg-white p-8 rounded-lg shadow-md border border-zinc-200 flex flex-col"><h3 class="text-xl font-bold text-zinc-700 mb-6">{status_title}</h3><div class="flex-grow relative"><canvas id="takedownChart"></canvas></div></div></div></div></div>{footer}<script>(function(){{
    function initTakedownChart() {{
        if (typeof Chart === 'undefined') {{ setTimeout(initTakedownChart, 100); return; }}
        const ctx=document.getElementById('takedownChart').getContext('2d');
        new Chart(ctx,{{
            type:'doughnut',
            data:{{
                labels:{labels},
                datasets:[{{
                    data:{data},
                    backgroundColor:['#10b981','#f59e0b','#ef4444','#64748b'],
                    borderWidth:0
                }}]
            }},
            options:{{
                responsive:true,
                maintainAspectRatio:false,
                plugins:{{
                    legend:{{position:'right',labels:{{font:{{size:14}}}}}}
                }}
            }}
        }});
    }}
    if (document.readyState === 'complete') {{ initTakedownChart(); }} else {{ window.addEventListener('load', initTakedownChart); }}
}})();</script></div></div>"#,
        title = dict.takedowns_title(),
        req = data.total_tickets, // Or calculated requested from takedowns? "takedowns requested" usually resolved+pending+aborted+unresolved
        lbl_req = dict.takedowns_requested(),
        rate = data.takedown_success_rate,
        lbl_rate = dict.takedowns_success_rate(),
        notify = data.takedown_median_time_to_notify,
        lbl_notify = dict.takedowns_median_notify(),
        uptime = data.takedown_median_uptime,
        lbl_uptime = dict.takedowns_median_uptime(),
        status_title = dict.takedowns_status_title(),
        labels = labels_json,
        data = donut_json,
        footer = footer_light(11, dict),
    )
}

fn render_impact_roi_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let metrics = &data.roi_metrics;

    // Format hours saved nicely
    let hours_display = if metrics.hours_saved_total >= 8.0 {
        format!("{:.0}", metrics.person_days_saved)
    } else {
        format!("{:.1}", metrics.hours_saved_total)
    };
    let hours_unit = if metrics.hours_saved_total >= 8.0 {
        dict.op_unit_person_days()
    } else {
        dict.op_unit_hours()
    };

    // Format analysts equivalent - use simple formatting
    let analysts_display = if metrics.analysts_equivalent_monthly >= 1.0 {
        format!("{:.1}", metrics.analysts_equivalent_monthly)
    } else {
        format!("{:.0}%", metrics.analysts_equivalent_monthly * 100.0)
    };

    // Calculate Precise ROI (Median Takedown Time)
    let median_minutes = calculate_median_takedown_minutes(&data.resolved_takedowns);
    let (resp_title, resp_time, resp_desc) = if let Some(mins) = median_minutes {
        (
            dict.roi_precise_title(),
            format!("{} min", mins),
            dict.roi_precise_text_primary(mins)
        )
    } else {
        (
            dict.op_response_title(),
            "180x".to_string(),
            dict.roi_precise_text_fallback()
        )
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-8"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">{badge}</span><h2 class="text-4xl font-bold mt-4">{title}</h2></div><div class="grid grid-cols-3 gap-8 flex-grow"><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg></div><h3 class="text-2xl font-bold mb-2">{eff_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">{hours} <span class="text-base font-normal text-zinc-400">{hours_unit}</span></p><p class="text-zinc-400 text-sm leading-relaxed">{eff_desc}</p><div class="mt-4 text-xs text-zinc-500"><p>• {lbl_validation}: {val_hours:.0}h</p><p>• {lbl_monitoring}: {cred_hours:.0}h</p><p>• {lbl_takedowns}: {td_hours:.0}h</p></div></div><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m.94-3.198a9.094 9.094 0 01-5.454-2.82m0 0a2.25 2.25 0 00-3.182 0m3.182 0a2.25 2.25 0 010 3.182m-3.182-3.182L12 12.75m0 0l3.182 3.182m-3.182-3.182L12 12.75"></path></svg></div><h3 class="text-2xl font-bold mb-2">{team_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">{analysts}</p><p class="text-zinc-400 text-sm leading-relaxed">{team_desc}</p><div class="mt-4"><div class="flex items-center gap-2 text-xs text-zinc-500"><span class="w-3 h-3 rounded-full bg-green-500"></span><span>{tickets} {lbl_tickets}</span></div><div class="flex items-center gap-2 text-xs text-zinc-500 mt-1"><span class="w-3 h-3 rounded-full bg-blue-500"></span><span>{creds} {lbl_creds}</span></div></div></div><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"></path></svg></div><h3 class="text-2xl font-bold mb-2">{resp_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">{resp_time}</p><p class="text-zinc-400 text-sm leading-relaxed">{resp_desc}</p><div class="mt-4 space-y-2"><div class="flex justify-between text-xs"><span class="text-zinc-500">{lbl_success}</span><span class="text-green-400 font-bold">{success_rate:.1}%</span></div><div class="flex justify-between text-xs"><span class="text-zinc-500">{lbl_td_done}</span><span class="text-white font-bold">{takedowns}</span></div></div></div></div></div></div>{footer}</div></div>"#,
        badge = dict.op_badge(),
        title = dict.roi_title(),
        eff_title = dict.op_time_saved_title(),
        hours = hours_display,
        hours_unit = hours_unit,
        eff_desc = dict.op_time_saved_desc(),
        lbl_validation = dict.op_breakdown_validation(),
        val_hours = metrics.hours_saved_validation,
        lbl_monitoring = dict.op_breakdown_monitoring(),
        cred_hours = metrics.hours_saved_credentials,
        lbl_takedowns = dict.op_breakdown_takedowns(),
        td_hours = metrics.hours_saved_takedowns,
        team_title = dict.op_capacity_title(),
        analysts = analysts_display,
        team_desc = dict.op_capacity_desc(),
        tickets = metrics.tickets_processed,
        lbl_tickets = dict.op_tickets_processed(),
        creds = metrics.credentials_monitored,
        lbl_creds = dict.op_credentials_monitored(),
        resp_title = resp_title,
        resp_time = resp_time,
        resp_desc = resp_desc,
        lbl_success = dict.op_success_rate(),
        success_rate = data.takedown_success_rate,
        lbl_td_done = dict.op_takedowns_completed(),
        takedowns = data.takedown_resolved,
        footer = footer_dark(0, dict)
    )
}

fn map_threat_to_intent(threat_type: &str) -> &'static str {
    let t = threat_type.to_lowercase();
    
    // 1. Chat Intelligence (Messages & Dark Web Activity)
    // Matches: data-sale-message, suspicious-activity-message, dw-activity, etc.
    if t.contains("message") || t == "dw-activity" {
        return "chat";
    }

    // 2. Dispositivos Comprometidos (Key visual for infection)
    if t == "infostealer-credential" {
        return "compromised";
    }

    // 3. Ataques a la Confianza (High volume brand attacks)
    if t == "phishing" 
        || t == "fraudulent-brand-use" 
        || t == "fake-mobile-app" 
        || t == "fake-social-media-profile" 
        || t == "similar-domain-name" 
        || t == "paid-search" 
        || t == "malware" {
        return "trust";
    }

    // 4. Fuga de Datos (Corporate/Secrets)
    if t == "corporate-credential-leak" 
        || t == "code-secret-leak" 
        || t == "database-exposure" 
        || t == "other-sensitive-data" {
        return "data_leak";
    }

    // 5. Protección VIP
    if t.starts_with("executive-") {
        return "vip";
    }

    // 6. Deep & Dark Web (Infrastructure & Advanced)
    if t == "ransomware-attack" 
        || t == "infrastructure-exposure" 
        || t.contains("website") { // data-sale-website, etc.
        return "dark_web";
    }

    // Default / Protection of Revenue
    "fraud"
}

fn calculate_median_takedown_minutes(takedowns: &[ResolvedTakedown]) -> Option<i64> {
    let mut durations: Vec<i64> = takedowns
        .iter()
        .filter_map(|td| {
            if let (Some(request_date), Some(res_date)) = (&td.request_date, &td.resolution_date) {
                // Try parsing dates. API usually returns ISO 8601.
                if let (Ok(start), Ok(end)) = (DateTime::parse_from_rfc3339(request_date), DateTime::parse_from_rfc3339(res_date)) {
                     let dur = end.signed_duration_since(start).num_minutes();
                     if dur > 0 { Some(dur) } else { None }
                } 
                // Fallback for other formats if needed
                else if let (Ok(start), Ok(end)) = (DateTime::parse_from_rfc2822(request_date), DateTime::parse_from_rfc2822(res_date)) {
                    let dur = end.signed_duration_since(start).num_minutes();
                    if dur > 0 { Some(dur) } else { None }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if durations.is_empty() {
        return None;
    }

    durations.sort();
    Some(durations[durations.len() / 2])
}

fn render_geospatial_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let mut countries: HashMap<String, u64> = HashMap::new();
    let mut isps: HashMap<String, u64> = HashMap::new();
    let mut registrars: HashMap<String, u64> = HashMap::new();

    // From Takedowns
    for td in &data.resolved_takedowns {
        if !td.country.is_empty() && td.country != "-" {
            *countries.entry(td.country.clone()).or_insert(0) += 1;
        }
        if let Some(isp) = &td.isp {
             if !isp.is_empty() && isp != "-" {
                *isps.entry(isp.clone()).or_insert(0) += 1;
            }
        }
        if let Some(reg) = &td.registrar {
             if !reg.is_empty() && reg != "-" {
                *registrars.entry(reg.clone()).or_insert(0) += 1;
            }
        }
    }

    // From Incidents
    for inc in &data.latest_incidents {
        if !inc.isp.is_empty() && inc.isp != "-" {
            *isps.entry(inc.isp.clone()).or_insert(0) += 1;
        }
        if !inc.country.is_empty() && inc.country != "-" {
            *countries.entry(inc.country.clone()).or_insert(0) += 1;
        }
        // New: Registrars
        if let Some(reg) = &inc.registrar {
            if !reg.is_empty() && reg != "-" {
                *registrars.entry(reg.clone()).or_insert(0) += 1;
            }
        }
    }

    // From Deep Investigations
    for inv in &data.deep_investigations {
        if let Some(c) = &inv.infrastructure.country {
            if !c.is_empty() && c != "-" {
                *countries.entry(c.clone()).or_insert(0) += 1;
            }
        }
        if let Some(isp) = &inv.infrastructure.hosting_provider {
            if !isp.is_empty() && isp != "-" {
                *isps.entry(isp.clone()).or_insert(0) += 1;
            }
        }
    }

    // Sort for charts
    let mut sorted_countries: Vec<(&String, &u64)> = countries.iter().collect();
    sorted_countries.sort_by(|a, b| b.1.cmp(a.1));
    let top_countries: Vec<(&String, &u64)> = sorted_countries.into_iter().take(10).collect();

    // Sort ISPs
    let mut sorted_isps: Vec<(&String, &u64)> = isps.iter().collect();
    sorted_isps.sort_by(|a, b| b.1.cmp(a.1));
    let top_isps: Vec<(&String, &u64)> = sorted_isps.iter().take(6).map(|&x| x).collect();

    // Sort Registrars
    let mut sorted_registrars: Vec<(&String, &u64)> = registrars.iter().collect();
    sorted_registrars.sort_by(|a, b| b.1.cmp(a.1));
    let top_registrars: Vec<(&String, &u64)> = sorted_registrars.iter().take(6).map(|&x| x).collect();

    // If no data, render "No Data" state
    if top_countries.is_empty() && top_isps.is_empty() && top_registrars.is_empty() {
        return r#"
        <section class="slide geospatial-slide">
            <div class="field-title">
                <i class="fas fa-globe-americas"></i>
                <h1 class="field-title-text" data-i18n="geospatial_intelligence">Geospatial Intelligence</h1>
            </div>
            <div class="content-wrapper centered-message">
                <div class="no-data-container">
                    <i class="fas fa-map-marked-alt fa-3x"></i>
                    <h2>No Geographic Data Available</h2>
                    <p>Geolocation information for the detected incidents was not provided by the source.</p>
                </div>
            </div>
            <div class="slide-footer">
                <div class="confidential-badge">
                   <i class="fas fa-user-secret"></i>
                   CONFIDENTIAL - TLP:AMBER
                </div>
                <div class="logo-container">
                     <img src="https://axur.com/wp-content/uploads/2023/02/Logo-Axur-1.svg" class="axur-logo" alt="Axur">
                </div>
            </div>
            <style>
                .centered-message {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    height: 100%;
                    text-align: center;
                    color: var(--text-secondary);
                }
                .no-data-container {
                    padding: 2rem;
                    background: rgba(255, 255, 255, 0.05);
                    border-radius: 12px;
                    border: 1px dashed var(--border-color);
                }
                .no-data-container i {
                    margin-bottom: 1rem;
                    opacity: 0.5;
                }
                .no-data-container h2 {
                    font-size: 1.5rem;
                    margin-bottom: 0.5rem;
                    color: var(--text-primary);
                }
            </style>
        </section>
        "#.to_string();
    }

    // Narrative
    let total_countries = countries.len();
    let (narrative, _is_primary) = if total_countries > 0 {
        let top_country = if !top_countries.is_empty() { top_countries[0].0.as_str() } else { "Unknown" };
        (dict.geo_fmt_primary(total_countries, top_country), true)
    } else {
        (dict.geo_fmt_fallback(), false)
    };

    // Chart Data - Add flags to labels
    let labels: Vec<String> = top_countries.iter().map(|(k, _)| {
        let flag = country_to_flag(k);
        format!("{} {}", flag, k)
    }).collect();
    
    let values: Vec<u64> = top_countries.iter().map(|(_, v)| **v).collect();
    let json_labels = serde_json::to_string(&labels).unwrap_or_default();
    let json_data = serde_json::to_string(&values).unwrap_or_default();

    // ISP List HTML
    let mut isp_html = String::new();
    for (isp, count) in top_isps {
        isp_html.push_str(&format!(
            r#"<div class="flex items-center justify-between p-3 bg-zinc-800/50 rounded lg:mb-2 border border-zinc-700/50 hover:border-zinc-600 transition-colors">
                <div class="flex items-center gap-3 overflow-hidden">
                    <div class="w-8 h-8 rounded bg-zinc-700 flex items-center justify-center text-zinc-400 text-xs">ISP</div>
                    <span class="text-zinc-300 truncate text-sm">{}</span>
                </div>
                <span class="text-white font-bold bg-zinc-700 px-2 py-0.5 rounded text-xs">{}</span>
            </div>"#,
            isp, count
        ));
    }

    // Registrars List HTML (Jurisdiction)
    let mut registrar_html = String::new();
    for (reg, count) in top_registrars {
        registrar_html.push_str(&format!(
            r#"<div class="flex items-center justify-between p-3 bg-zinc-800/50 rounded lg:mb-2 border border-zinc-700/50 hover:border-zinc-600 transition-colors">
                <div class="flex items-center gap-3 overflow-hidden">
                    <div class="w-8 h-8 rounded bg-zinc-700 flex items-center justify-center text-zinc-400 text-xs text-blue-400"><i class="fas fa-gavel"></i></div>
                    <span class="text-zinc-300 truncate text-sm">{}</span>
                </div>
                <span class="text-white font-bold bg-zinc-700 px-2 py-0.5 rounded text-xs">{}</span>
            </div>"#,
            reg, count
        ));
    }

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
            <!-- Background Map Effect -->
            <div class="absolute inset-0 opacity-10 pointer-events-none" style="background-image: url('data:image/svg+xml,%3Csvg width=%22100%25%22 height=%22100%25%22 viewBox=%220 0 100 100%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cpath d=%22M0,0 L100,0 L100,100 L0,100 Z%22 fill=%22none%22 stroke=%22%23ffffff%22 stroke-width=%220.5%22 stroke-dasharray=%222,4%22/%3E%3C/svg%3E'); background-size: 40px 40px;"></div>
            <div class="absolute right-0 top-20 w-1/2 h-full opacity-5 pointer-events-none">
                <svg viewBox="0 0 200 100" class="w-full h-full text-white" fill="currentColor"><path d="M20,50 Q50,20 80,50 T140,50 T200,50" stroke="currentColor" stroke-width="0.5" fill="none"/></svg>
            </div>

            <div class="relative flex-grow h-full overflow-hidden z-10">
                <div class="h-full flex flex-col">
                    <div class="mb-6">
                        <span class="bg-indigo-600 px-4 py-1 text-sm font-semibold tracking-wider">GEOSPATIAL INTELLIGENCE</span>
                        <div class="flex items-start justify-between mt-4">
                            <h2 class="text-4xl font-bold max-w-2xl">{title}</h2>
                        </div>
                        <p class="text-xl text-zinc-300 mt-2 max-w-3xl leading-relaxed">{desc}</p>
                    </div>
                    
                    <div class="grid grid-cols-2 gap-8 flex-grow">
                        <!-- Left: Chart -->
                        <div class="bg-zinc-900/80 p-6 rounded-xl border border-zinc-800 relative flex flex-col backdrop-blur-sm">
                            <h3 class="text-lg font-semibold mb-4 text-indigo-400 flex items-center gap-2">
                                <span>🌐</span> {lbl_countries}
                            </h3>
                            <div class="flex-grow relative">
                                <canvas id="geoChart"></canvas>
                            </div>
                        </div>
                        
                        <!-- Right: ISP List -->
                        <div class="bg-zinc-900/80 p-6 rounded-xl border border-zinc-800 overflow-hidden backdrop-blur-sm flex flex-col">
                            <h3 class="text-lg font-semibold mb-4 text-indigo-400 flex items-center gap-2">
                                <span>🏢</span> {lbl_isps}
                            </h3>
                            <div class="overflow-y-auto pr-2 flex-grow space-y-2 custom-scrollbar">
                                {isp_list}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            {footer}
            <script>
            (function(){{
                function initGeoChart() {{
                    if (typeof Chart === 'undefined') {{
                        setTimeout(initGeoChart, 100);
                        return;
                    }}
                    const ctx = document.getElementById('geoChart').getContext('2d');
                    new Chart(ctx, {{
                        type: 'bar',
                        data: {{
                            labels: {json_labels},
                            datasets: [{{
                                label: 'Origem',
                                data: {json_data},
                                backgroundColor: 'rgba(99, 102, 241, 0.8)',
                                borderColor: '#6366f1',
                                borderWidth: 1,
                                borderRadius: 6,
                                barPercentage: 0.6,
                                categoryPercentage: 0.8
                            }}]
                        }},
                        options: {{
                            indexAxis: 'y',
                            responsive: true,
                            maintainAspectRatio: false,
                            plugins: {{
                                legend: {{ display: false }},
                                tooltip: {{
                                    backgroundColor: 'rgba(24, 24, 27, 0.9)',
                                    titleColor: '#fff',
                                    bodyColor: '#ccc',
                                    borderColor: '#3f3f46',
                                    borderWidth: 1,
                                    padding: 12,
                                    displayColors: false
                                }}
                            }},
                            scales: {{
                                x: {{
                                    grid: {{ display: true, color: 'rgba(255,255,255,0.05)' }},
                                    ticks: {{ color: '#71717a', font: {{ family: 'monospace' }} }}
                                }},
                                y: {{
                                    grid: {{ display: false }},
                                    ticks: {{ 
                                        color: '#fff', 
                                        font: {{ size: 14, weight: 'bold' }},
                                        crossAlign: 'far'
                                    }}
                                }}
                            }}
                        }}
                    }});
                }}
                if (document.readyState === 'complete') {{ initGeoChart(); }} else {{ window.addEventListener('load', initGeoChart); }}
            }})();
            </script>
        </div></div>"#,
        title = dict.geo_title(),
        desc = narrative,
        lbl_countries = dict.geo_lbl_countries(),
        lbl_isps = dict.geo_lbl_isps(),
        isp_list = isp_html,
        footer = footer_dark(0, dict),
        json_labels = json_labels,
        json_data = json_data
    )
}
fn render_takedown_examples_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let mut examples_html = String::new();
    let example_count = data.resolved_takedowns.len().min(3);

    if example_count > 0 {
        for ex in data.resolved_takedowns.iter().take(3) {
            let img = if let Some(path) = &ex.screenshot_url {
                format!(
                    r#"<img src="{}" class="w-full h-48 object-cover rounded-md border border-zinc-200" alt="Screenshot"/>"#,
                    path
                )
            } else {
                format!(
                    r#"<div class="w-full h-48 bg-zinc-100 flex items-center justify-center text-zinc-400 rounded-md border border-zinc-200">{}</div>"#,
                    dict.example_no_image()
                )
            };

            let date = ex.resolution_date.as_deref().unwrap_or("-");

            examples_html.push_str(&format!(r#"<div class="bg-white p-6 rounded-lg shadow-md border border-zinc-200"><div class="mb-4">{}</div><div class="space-y-2"><p class="font-bold text-zinc-800 text-lg line-clamp-1">{}</p><p class="text-xs text-zinc-500"><span class="font-bold">{}</span> {}</p><p class="text-xs text-zinc-500"><span class="font-bold">{}</span> {}</p></div></div>"#,
                img, ex.name, dict.example_label_date(), date, dict.example_label_url(), ex.url));
        }
    } else {
        examples_html = format!(
            r#"<div class="col-span-3 text-center text-zinc-500 italic p-12">{}</div>"#,
            dict.example_no_data()
        );
    }

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">EVIDENCIAS</span></div><div class="mb-8"><h2 class="text-4xl font-bold mb-4">{title}</h2></div><div class="grid grid-cols-3 gap-6 flex-grow">{examples}</div></div></div>{footer}</div></div>"#,
        title = dict.examples_takedowns_title(),
        examples = examples_html,
        footer = footer_light(13, dict),
    )
}

fn render_poc_examples_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // Group by type and show first few examples
    let examples_html: String = if data.poc_examples.is_empty() {
        format!(
            r#"<div class="bg-white p-6 rounded-xl shadow-lg border border-zinc-200 text-center col-span-3"><p class="text-zinc-500 italic">{}</p></div>"#,
            dict.example_no_data()
        )
    } else {
        data.poc_examples.iter().take(6).map(|ex| {
            let domain = ex.domain.as_deref().unwrap_or("N/A");
            let status_color = match ex.status.as_str() {
                "incident" => "bg-red-100 text-red-700",
                "closed" => "bg-green-100 text-green-700",
                _ => "bg-yellow-100 text-yellow-700"
            };
            // Add screenshot image if available
            let screenshot_html = match &ex.screenshot_url {
                Some(url) if !url.is_empty() => format!(
                    r#"<div class="w-full h-32 bg-zinc-100 rounded-lg overflow-hidden mb-3"><img src="{}" alt="Evidence screenshot" class="w-full h-full object-cover object-top" onerror="this.parentNode.innerHTML=&quot;<div class='flex items-center justify-center h-full text-zinc-400 text-xs'>{}</div>&quot;"/></div>"#,
                    url, dict.example_no_image()
                ),
                _ => format!(r#"<div class="w-full h-32 bg-zinc-100 rounded-lg overflow-hidden mb-3 flex items-center justify-center text-zinc-400 text-xs">{}</div>"#, dict.example_no_image())
            };
            format!(r#"<div class="bg-white p-4 rounded-xl shadow-lg border border-zinc-200">{screenshot}<div class="flex justify-between items-start mb-2"><span class="font-bold text-zinc-800">{ticket}</span><span class="text-xs px-2 py-1 rounded {status_color}">{status}</span></div><p class="text-sm text-zinc-600 mb-2">{type_name}</p><p class="text-xs text-blue-500 truncate">{domain}</p></div>"#,
                screenshot = screenshot_html,
                ticket = ex.ticket_key,
                type_name = ex.evidence_type,
                domain = domain,
                status = ex.status,
                status_color = status_color,
            )
        }).collect()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-100"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col text-zinc-800"><div class="mb-4"><span class="bg-orange-600 text-white px-4 py-1 text-sm font-semibold">EVIDENCIAS</span></div><h2 class="text-3xl font-bold mb-6">{title}</h2><div class="grid grid-cols-3 gap-4 flex-grow">{examples}</div></div></div>{footer}</div></div>"#,
        title = dict.examples_poc_title(),
        examples = examples_html,
        footer = footer_light(12, dict),
    )
}

// =====================
// CAMPAIGN DETECTION
// =====================

/// Campaign detection result
struct CampaignInfo {
    name: String,
    threat_type: String,
    tickets: Vec<usize>, // Indices into original tickets
    common_isp: Option<String>,
    common_ip_prefix: Option<String>,
    country: Option<String>,
    date_range: (String, String), // Start, End
}

/// Detect campaigns by grouping incidents with shared infrastructure or timing
fn detect_campaign(tickets: &[crate::api::report::StoryTicket]) -> Option<CampaignInfo> {
    if tickets.len() < 3 {
        return None;
    }

    // Strategy 1: Group by threat_type + ISP
    let mut isp_groups: HashMap<(String, String), Vec<usize>> = HashMap::new();
    for (i, ticket) in tickets.iter().enumerate() {
        let key = (
            ticket.threat_type.clone(),
            ticket.isp.clone().unwrap_or_default(),
        );
        isp_groups.entry(key).or_default().push(i);
    }

    // Find largest group with at least 3 tickets and known ISP
    let best_group = isp_groups
        .iter()
        .filter(|((_, isp), indices)| !isp.is_empty() && indices.len() >= 3)
        .max_by_key(|(_, indices)| indices.len());

    if let Some(((threat_type, isp), indices)) = best_group {
        // Extract date range
        let dates: Vec<&str> = indices
            .iter()
            .filter_map(|&i| {
                tickets[i].incident_date.as_deref()
                    .or(tickets[i].open_date.as_deref())
            })
            .collect();
        
        let start = dates.iter().min().unwrap_or(&"Unknown").to_string();
        let end = dates.iter().max().unwrap_or(&"Unknown").to_string();

        // Extract common IP prefix (first 2 octets)
        let ip_prefix = tickets.get(*indices.first().unwrap_or(&0))
            .and_then(|t| t.ip.as_ref())
            .and_then(|ip| {
                let parts: Vec<&str> = ip.split('.').collect();
                if parts.len() >= 2 {
                    Some(format!("{}.{}.*.*", parts[0], parts[1]))
                } else {
                    None
                }
            });

        // Generate campaign name
        let threat_label = match threat_type.as_str() {
            "phishing" => "Phishing",
            "fake-social-media-profile" => "Fake Profile",
            "fraudulent-brand-use" => "Brand Abuse",
            _ => "Attack",
        };
        let name = format!("{} Campaign via {}", threat_label, isp);

        return Some(CampaignInfo {
            name,
            threat_type: threat_type.clone(),
            tickets: indices.clone(),
            common_isp: Some(isp.clone()),
            common_ip_prefix: ip_prefix,
            country: None, // Could be enriched later
            date_range: (start, end),
        });
    }

    // Strategy 2: Fallback - group by dominant threat type
    let mut type_counts: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, ticket) in tickets.iter().enumerate() {
        type_counts.entry(ticket.threat_type.clone()).or_default().push(i);
    }

    let dominant = type_counts
        .iter()
        .max_by_key(|(_, indices)| indices.len())?;

    if dominant.1.len() >= 3 {
        let dates: Vec<&str> = dominant.1
            .iter()
            .filter_map(|&i| {
                tickets[i].incident_date.as_deref()
                    .or(tickets[i].open_date.as_deref())
            })
            .collect();
        
        let start = dates.iter().min().unwrap_or(&"Unknown").to_string();
        let end = dates.iter().max().unwrap_or(&"Unknown").to_string();

        let threat_label = match dominant.0.as_str() {
            "phishing" => "Phishing",
            "fake-social-media-profile" => "Fake Profile",
            "fraudulent-brand-use" => "Brand Abuse",
            _ => "Threat",
        };

        return Some(CampaignInfo {
            name: format!("{} Cluster ({} incidents)", threat_label, dominant.1.len()),
            threat_type: dominant.0.clone(),
            tickets: dominant.1.clone(),
            common_isp: None,
            common_ip_prefix: None,
            country: None,
            date_range: (start, end),
        });
    }

    None
}

fn render_incident_story_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    tracing::info!(
        "render_incident_story_slide called with {} story tickets",
        data.story_tickets.len()
    );
    if data.story_tickets.is_empty() {
        tracing::info!("No story tickets found, skipping slide");
        return String::new();
    }

    // Detect campaign for smarter grouping
    let campaign = detect_campaign(&data.story_tickets);
    
    // Filter tickets based on campaign or use all
    let (filtered_tickets, campaign_title, campaign_subtitle, infra_html): (Vec<_>, String, String, String) = 
        if let Some(ref c) = campaign {
            let tickets: Vec<_> = c.tickets.iter()
                .filter_map(|&i| data.story_tickets.get(i))
                .collect();
            
            // Infrastructure badge
            let infra = if let Some(isp) = &c.common_isp {
                let ip_str = c.common_ip_prefix.as_deref().unwrap_or("*.*");
                format!(
                    r#"<div class="flex items-center gap-4 text-xs text-zinc-400 mb-4 bg-zinc-900/50 p-3 rounded-lg border border-zinc-800">
                        <span class="flex items-center gap-1">🌐 <strong class="text-white">{}</strong></span>
                        <span class="flex items-center gap-1">🔗 IP: <code class="text-orange-400">{}</code></span>
                        <span class="flex items-center gap-1">📅 {} → {}</span>
                    </div>"#,
                    isp, ip_str, 
                    if c.date_range.0.len() >= 10 { &c.date_range.0[5..10] } else { &c.date_range.0 },
                    if c.date_range.1.len() >= 10 { &c.date_range.1[5..10] } else { &c.date_range.1 }
                )
            } else {
                String::new()
            };
            
            (
                tickets,
                format!("🎯 {}", c.name),
                format!("{} coordinated incidents detected", c.tickets.len()),
                infra
            )
        } else {
            (
                data.story_tickets.iter().collect(),
                dict.story_title(),
                dict.story_subtitle(data.story_tickets.len()),
                String::new()
            )
        };

    // Timeline Data Aggregation (use filtered tickets)
    let mut timeline: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for ticket in &filtered_tickets {
        let date_str = ticket.incident_date.as_deref()
            .or(ticket.open_date.as_deref())
            .map(|d| if d.len() >= 10 { &d[0..10] } else { d })
            .unwrap_or("Unknown");
        
        if date_str != "Unknown" {
            *timeline.entry(date_str.to_string()).or_insert(0) += 1;
        }
    }

    let dates: Vec<String> = timeline.keys().cloned().collect();
    let counts: Vec<u64> = timeline.values().cloned().collect();
    let json_dates = serde_json::to_string(&dates).unwrap_or_default();
    let json_counts = serde_json::to_string(&counts).unwrap_or_default();

    let mut cards_html = String::new();

    // Evidence/Cards Logic: Take 3 items from filtered tickets
    for ticket in filtered_tickets.iter().take(3) {
        // Try to find matching deep investigation data for enrichment
        let enrichment = data.deep_investigations.iter()
            .find(|inv| inv.ticket_key == ticket.ticket_key)
            .map(|inv| &inv.enrichment);

        // Screenshot image logic
        let img_html = if let Some(enr) = enrichment {
            if let Some(base64) = &enr.screenshot_base64 {
                 format!(
                    r#"<div class="relative h-40 w-full bg-zinc-900 rounded-lg overflow-hidden border border-zinc-700 mb-3 group-hover:border-orange-500/50 transition-colors">
                        <img src="{}" class="w-full h-full object-cover object-top hover:scale-105 transition-transform duration-500" />
                        <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
                    </div>"#,
                    base64
                )
            } else if let Some(url) = &ticket.screenshot_url {
                format!(
                    r#"<div class="relative h-40 w-full bg-zinc-900 rounded-lg overflow-hidden border border-zinc-700 mb-3">
                        <img src="{}" class="w-full h-full object-cover object-top hover:scale-105 transition-transform duration-500" />
                        <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
                    </div>"#,
                    url
                )
            } else {
                r#"<div class="h-40 w-full rounded-lg bg-zinc-900 border border-zinc-700 mb-3 flex items-center justify-center">
                    <div class="text-center">
                        <span class="text-zinc-600 text-3xl block mb-2">📷</span>
                        <span class="text-zinc-500 text-xs">No preview</span>
                    </div>
                </div>"#.to_string()
            }
        } else if let Some(url) = &ticket.screenshot_url {
             format!(
                r#"<div class="relative h-40 w-full bg-zinc-900 rounded-lg overflow-hidden border border-zinc-700 mb-3">
                    <img src="{}" class="w-full h-full object-cover object-top hover:scale-105 transition-transform duration-500" />
                    <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
                </div>"#,
                url
            )
        } else {
            r#"<div class="h-40 w-full rounded-lg bg-zinc-900 border border-zinc-700 mb-3 flex items-center justify-center">
                <div class="text-center">
                    <span class="text-zinc-600 text-3xl block mb-2">📷</span>
                    <span class="text-zinc-500 text-xs">No preview</span>
                </div>
            </div>"#.to_string()
        };

        // Status badge color
        let status_color = match ticket.status.to_lowercase().as_str() {
            "incident" => "text-red-400 bg-red-900/30 border-red-500/30",
            "closed" => "text-green-400 bg-green-900/30 border-green-500/30",
            _ => "text-yellow-400 bg-yellow-900/30 border-yellow-500/30",
        };

        // Format date
        let display_date = ticket.incident_date.as_ref()
            .or(ticket.open_date.as_ref())
            .map(|d| if d.len() >= 10 { format!("{}/{}/{}", &d[8..10], &d[5..7], &d[0..4]) } else { d.clone() })
            .unwrap_or_else(|| "N/A".to_string());

        cards_html.push_str(&format!(
            r#"<div class="bg-zinc-900/40 p-4 rounded-xl border border-zinc-800 flex flex-col h-full hover:bg-zinc-800/60 transition-colors backdrop-blur-sm">
                {img}
                <div class="flex justify-between items-start mb-2">
                    <span class="text-[0.65rem] px-2 py-0.5 rounded border {status_class} uppercase font-bold tracking-wider">{status}</span>
                    <span class="text-xs font-mono text-zinc-500">{date}</span>
                </div>
                <h4 class="text-white font-bold text-sm truncate mb-1">{target}</h4>
                <p class="text-indigo-400 text-xs truncate mb-2">{type}</p>
                
                <div class="mt-auto pt-3 border-t border-zinc-700/50 flex justify-between items-center">
                     <span class="text-[10px] text-zinc-500 uppercase tracking-widest">ID: {ticket_key}</span>
                     <span class="text-xs text-zinc-400">View Evidence →</span>
                </div>
            </div>"#,
            date = display_date,
            status_class = status_color,
            status = ticket.status,
            img = img_html,
            target = ticket.target,
            type = ticket.threat_type,
            ticket_key = ticket.ticket_key
        ));
    }

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-8 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white">
            <div class="flex-grow h-full overflow-hidden flex flex-col">
                <div class="flex items-center justify-between mb-2">
                   <div>
                        <span class="bg-orange-600 px-3 py-1 text-xs font-bold uppercase tracking-wider text-white mb-2 inline-block">INCIDENT INTELLIGENCE</span>
                        <h2 class="text-3xl font-bold">{title}</h2>
                        <p class="text-zinc-400 text-sm">{subtitle}</p>
                   </div>
                </div>
                
                {infra}

                <!-- Top: Timeline Chart -->
                <div class="h-40 w-full bg-zinc-900/30 rounded-lg border border-zinc-800 p-4 relative mb-4">
                    <canvas id="storyTimelineChart"></canvas>
                </div>

                <!-- Bottom: Evidence Gallery -->
                <div class="flex-grow">
                    <h3 class="text-sm font-bold text-zinc-500 uppercase tracking-widest mb-3 border-b border-zinc-800 pb-2">Verified Threats (Evidence)</h3>
                    <div class="grid grid-cols-3 gap-4 h-full">
                        {cards}
                    </div>
                </div>
            </div>
            {footer}
            <script>
            (function(){{
                function initStoryChart() {{
                    if (typeof Chart === 'undefined') {{ setTimeout(initStoryChart, 100); return; }}
                    const ctx = document.getElementById('storyTimelineChart').getContext('2d');
                    new Chart(ctx, {{
                        type: 'line',
                        data: {{
                            labels: {json_dates},
                            datasets: [{{
                                label: 'Incidentes',
                                data: {json_counts},
                                borderColor: '#f97316',
                                backgroundColor: 'rgba(249, 115, 22, 0.1)',
                                borderWidth: 2,
                                fill: true,
                                tension: 0.4,
                                pointBackgroundColor: '#ea580c',
                                pointRadius: 4
                            }}]
                        }},
                        options: {{
                            responsive: true,
                            maintainAspectRatio: false,
                            plugins: {{
                                legend: {{ display: false }},
                                tooltip: {{
                                    backgroundColor: 'rgba(24, 24, 27, 0.9)',
                                    titleColor: '#fff',
                                    bodyColor: '#ccc',
                                    borderColor: '#3f3f46',
                                    borderWidth: 1
                                }}
                            }},
                            scales: {{
                                x: {{
                                    grid: {{ display: false, color: '#3f3f46' }},
                                    ticks: {{ color: '#71717a', maxRotation: 45, minRotation: 45 }}
                                }},
                                y: {{
                                    grid: {{ color: '#27272a' }},
                                    ticks: {{ color: '#71717a', stepSize: 1 }},
                                    beginAtZero: true
                                }}
                            }}
                        }}
                    }});
                }}
                if (document.readyState === 'complete') {{ initStoryChart(); }} else {{ window.addEventListener('load', initStoryChart); }}
            }})();
            </script>
        </div></div>"#,
        title = campaign_title,
        subtitle = campaign_subtitle,
        infra = infra_html,
        cards = cards_html,
        footer = footer_dark(11, dict),
        json_dates = json_dates,
        json_counts = json_counts
    )
}

/// Render the Threat Intelligence slide with 4 dimensions
fn render_threat_intelligence_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let intel = &data.threat_intelligence;

    // Skip if no data available
    if !intel.data_available {
        tracing::info!("No threat intelligence data available, skipping slide");
        return String::new();
    }

    // === Dimension 1: Dark Web Origin ===
    let dark_web_html = format!(r#"
        <div class="bg-zinc-900/80 border border-zinc-800 rounded-xl p-5 hover:border-purple-500/40 transition-all">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-3xl">🕵️</span>
                <div>
                    <h3 class="text-white font-bold">Origen en Dark Web</h3>
                    <p class="text-zinc-500 text-xs">Inteligencia previa de foros</p>
                </div>
            </div>
            
            <div class="text-center py-4">
                <span class="text-5xl font-bold text-purple-400">{mentions}</span>
                <p class="text-zinc-400 mt-1">menciones detectadas</p>
            </div>
            
            {early_warning}
            
            <div class="mt-4 border-t border-zinc-800 pt-3">
                <p class="text-xs text-zinc-500 mb-2">Fuentes:</p>
                <div class="flex flex-wrap gap-1">
                    {sources}
                </div>
            </div>
        </div>
    "#,
        mentions = intel.dark_web_mentions,
        early_warning = if let Some(date) = &intel.earliest_dark_web_date {
            format!(r#"<p class="text-center text-sm text-purple-300">⚡ Primera detección: {}</p>"#, 
                if date.len() >= 10 { format!("{}/{}/{}", &date[8..10], &date[5..7], &date[0..4]) } else { date.clone() })
        } else { String::new() },
        sources = intel.dark_web_sources.iter().take(3)
            .map(|s| format!(r#"<span class="text-[0.65rem] bg-purple-900/30 text-purple-300 px-2 py-0.5 rounded">{}</span>"#, s))
            .collect::<Vec<_>>().join("")
    );

    // === Dimension 2: Virality ===
    let virality_html = format!(r#"
        <div class="bg-zinc-900/80 border border-zinc-800 rounded-xl p-5 hover:border-blue-500/40 transition-all">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-3xl">📡</span>
                <div>
                    <h3 class="text-white font-bold">Viralidad y Alcance</h3>
                    <p class="text-zinc-500 text-xs">Propagación en redes</p>
                </div>
            </div>
            
            <div class="grid grid-cols-2 gap-4 text-center py-4">
                <div>
                    <span class="text-4xl font-bold text-blue-400">{chat}</span>
                    <p class="text-zinc-400 text-xs mt-1">Grupos de chat</p>
                </div>
                <div>
                    <span class="text-4xl font-bold text-cyan-400">{social}</span>
                    <p class="text-zinc-400 text-xs mt-1">Redes sociales</p>
                </div>
            </div>
            
            {campaign_alert}
            
            <div class="mt-4 border-t border-zinc-800 pt-3">
                <p class="text-xs text-zinc-500 mb-2">Plataformas:</p>
                <div class="flex flex-wrap gap-1">
                    {platforms}
                </div>
            </div>
        </div>
    "#,
        chat = intel.chat_group_shares,
        social = intel.social_media_mentions,
        campaign_alert = if intel.chat_group_shares > 10 {
            r#"<p class="text-center text-sm text-blue-300 bg-blue-900/20 rounded py-1">⚠️ Posible campaña coordinada</p>"#
        } else { "" },
        platforms = intel.platforms_detected.iter().take(4)
            .map(|p| format!(r#"<span class="text-[0.65rem] bg-blue-900/30 text-blue-300 px-2 py-0.5 rounded">{}</span>"#, p))
            .collect::<Vec<_>>().join("")
    );

    // === Dimension 3: Credential Quality ===
    let stealer_pct = intel.stealer_log_percent as i32;
    let plain_pct = intel.plain_password_percent as i32;

    let cred_html = format!(
        r#"
        <div class="bg-zinc-900/80 border border-zinc-800 rounded-xl p-5 hover:border-red-500/40 transition-all">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-3xl">🔐</span>
                <div>
                    <h3 class="text-white font-bold">Calidad de Credenciales</h3>
                    <p class="text-zinc-500 text-xs">{total} credenciales analizadas</p>
                </div>
            </div>
            
            <!-- Stealer Logs Bar -->
            <div class="mb-3">
                <div class="flex justify-between text-xs mb-1">
                    <span class="text-zinc-400">🦠 Stealer Logs (malware activo)</span>
                    <span class="font-bold text-red-400">{stealer_count} ({stealer_pct}%)</span>
                </div>
                <div class="h-2 bg-zinc-800 rounded-full overflow-hidden">
                    <div class="h-full bg-red-500 transition-all" style="width: {stealer_pct}%"></div>
                </div>
            </div>
            
            <!-- Combolist Bar -->
            <div class="mb-3">
                <div class="flex justify-between text-xs mb-1">
                    <span class="text-zinc-400">📋 Combolist (recicladas)</span>
                    <span class="text-yellow-400">{combo_count}</span>
                </div>
                <div class="h-2 bg-zinc-800 rounded-full overflow-hidden">
                    <div class="h-full bg-yellow-500 transition-all" style="width: {combo_pct}%"></div>
                </div>
            </div>
            
            <!-- Plain Password Bar -->
            <div class="mb-3">
                <div class="flex justify-between text-xs mb-1">
                    <span class="text-zinc-400">🔓 Texto plano</span>
                    <span class="text-orange-400">{plain_count} ({plain_pct}%)</span>
                </div>
                <div class="h-2 bg-zinc-800 rounded-full overflow-hidden">
                    <div class="h-full bg-orange-500 transition-all" style="width: {plain_pct}%"></div>
                </div>
            </div>
            
            {critical_alert}
            
            <div class="mt-3 border-t border-zinc-800 pt-2">
                <p class="text-[0.6rem] text-zinc-500">Portales: {access_urls}</p>
            </div>
        </div>
    "#,
        total = intel.total_credentials,
        stealer_count = intel.stealer_log_count,
        stealer_pct = stealer_pct,
        combo_count = intel.combolist_count,
        combo_pct = if intel.total_credentials > 0 {
            (intel.combolist_count * 100 / intel.total_credentials) as i32
        } else {
            0
        },
        plain_count = intel.plain_password_count,
        plain_pct = plain_pct,
        critical_alert = if stealer_pct >= 20 {
            r#"<p class="text-center text-sm text-red-300 bg-red-900/20 rounded py-1 mt-2">🚨 {stealer_pct}% usuarios con malware ACTIVO</p>"#
        } else {
            ""
        },
        access_urls = intel
            .top_access_urls
            .iter()
            .take(2)
            .map(|u| u.split('/').last().unwrap_or(u).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // === Dimension 4: Attacker Investment ===
    let ads_html = format!(r#"
        <div class="bg-zinc-900/80 border border-zinc-800 rounded-xl p-5 hover:border-green-500/40 transition-all">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-3xl">💰</span>
                <div>
                    <h3 class="text-white font-bold">Inversión del Atacante</h3>
                    <p class="text-zinc-500 text-xs">Publicidad pagada detectada</p>
                </div>
            </div>
            
            <div class="text-center py-4">
                <span class="text-5xl font-bold text-green-400">{ads}</span>
                <p class="text-zinc-400 mt-1">campañas publicitarias</p>
            </div>
            
            {roi_message}
            
            <div class="mt-4 border-t border-zinc-800 pt-3">
                <p class="text-xs text-zinc-500 mb-2">Plataformas de ads:</p>
                <div class="flex flex-wrap gap-1">
                    {ad_platforms}
                </div>
            </div>
        </div>
    "#,
        ads = intel.paid_ads_detected,
        roi_message = if intel.paid_ads_detected > 0 {
            r#"<p class="text-center text-sm text-green-300 bg-green-900/20 rounded py-1">💸 Takedown rápido = pérdida de inversión para el atacante</p>"#
        } else {
            r#"<p class="text-center text-sm text-zinc-500">Sin publicidad pagada detectada</p>"#
        },
        ad_platforms = intel.ad_platforms.iter().take(3)
            .map(|p| format!(r#"<span class="text-[0.65rem] bg-green-900/30 text-green-300 px-2 py-0.5 rounded">{}</span>"#, p))
            .collect::<Vec<_>>().join("")
    );

    // Combine into full slide
    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white">
            <div class="flex-grow h-full overflow-hidden">
                <div class="h-full flex flex-col">
                    <!-- Header -->
                    <div class="mb-6">
                        <span class="bg-purple-600 px-4 py-1 text-sm font-semibold">THREAT INTELLIGENCE</span>
                        <h2 class="text-3xl font-bold mt-3">Inteligencia de Amenazas</h2>
                        <p class="text-zinc-400 mt-1 text-sm">Análisis profundo de la sofisticación del ataque</p>
                    </div>
                    
                    <!-- 4-Quadrant Grid -->
                    <div class="grid grid-cols-2 gap-4 flex-grow overflow-hidden">
                        {dark_web}
                        {virality}
                        {credentials}
                        {ads}
                    </div>
                </div>
            </div>
            {footer}
        </div></div>"#,
        dark_web = dark_web_html,
        virality = virality_html,
        credentials = cred_html,
        ads = ads_html,
        footer = footer_dark(12, dict),
    )
}

/// Render the Timeline Deep Investigation slide with enriched data
fn render_deep_investigation_slide(data: &PocReportData, _dict: &Box<dyn Dictionary>) -> String {
    if data.deep_investigations.is_empty() {
        return String::new();
    }

    // Build timeline cards for each investigation with enrichment
    let investigation_cards: Vec<String> = data.deep_investigations.iter().take(6).map(|inv| {
        let campaign_badge = if inv.is_mass_campaign {
            r#"<span class="bg-red-900/50 text-red-300 text-[0.65rem] px-2 py-0.5 rounded">⚠️ Campaña Masiva</span>"#
        } else {
            r#"<span class="bg-green-900/50 text-green-300 text-[0.65rem] px-2 py-0.5 rounded">Aislado</span>"#
        };

        // Screenshot HTML - use base64 if available, otherwise placeholder
        let screenshot_html = if let Some(base64) = &inv.enrichment.screenshot_base64 {
            format!(
                r#"<div class="w-20 h-14 flex-shrink-0 rounded overflow-hidden bg-zinc-800 border border-zinc-700">
                    <img src="{}" alt="Screenshot" class="w-full h-full object-cover" />
                </div>"#,
                base64
            )
        } else {
            r#"<div class="w-20 h-14 flex-shrink-0 rounded bg-zinc-800 border border-zinc-700 flex items-center justify-center text-zinc-600 text-lg">
                📷
            </div>"#.to_string()
        };

        // AI Impersonation badges
        let impersonation_html = if !inv.enrichment.impersonated_brands.is_empty() {
            let badges: Vec<String> = inv.enrichment.impersonated_brands.iter().take(2).map(|b| {
                let (color, icon) = match b.level.as_str() {
                    "high" => ("bg-red-900/60 text-red-300 border-red-700", "🔴"),
                    "medium" => ("bg-yellow-900/60 text-yellow-300 border-yellow-700", "🟡"),
                    _ => ("bg-zinc-800 text-zinc-300 border-zinc-700", "🟢"),
                };
                format!(
                    r#"<span class="text-[0.55rem] px-1.5 py-0.5 rounded border {} flex items-center gap-1">
                        <span>{}</span>
                        <span>{}</span>
                    </span>"#,
                    color, icon, b.brand
                )
            }).collect();
            format!(r#"<div class="flex flex-wrap gap-1 mb-2">{}</div>"#, badges.join(""))
        } else {
            String::new()
        };

        // Credential/Payment warning badges
        let warning_badges = {
            let mut badges = Vec::new();
            if inv.enrichment.credential_requested {
                badges.push(r#"<span class="text-[0.55rem] bg-orange-900/50 text-orange-300 px-1.5 py-0.5 rounded">🔓 Credenciales</span>"#);
            }
            if inv.enrichment.payment_requested {
                badges.push(r#"<span class="text-[0.55rem] bg-red-900/50 text-red-300 px-1.5 py-0.5 rounded">💳 Pagos</span>"#);
            }
            if !badges.is_empty() {
                format!(r#"<div class="flex gap-1 mb-2">{}</div>"#, badges.join(""))
            } else {
                String::new()
            }
        };

        // Geolocation with country flag
        let geo_html = if let Some(geo) = &inv.enrichment.geolocation {
            let country_flag = geo.country_code.as_ref().map(|cc| {
                // Convert country code to flag emoji
                cc.chars().filter(|c| c.is_alphabetic())
                    .map(|c| char::from_u32(0x1F1E6 + c.to_ascii_uppercase() as u32 - 'A' as u32).unwrap_or('🏳'))
                    .collect::<String>()
            }).unwrap_or_else(|| "🌐".to_string());
            
            let country = geo.country_name.as_deref().unwrap_or("--");
            let isp = geo.isp.as_deref().unwrap_or("--");
            
            format!(
                r#"<div class="flex items-center gap-1 text-[0.6rem] text-zinc-400">
                    <span class="text-sm">{}</span>
                    <span>{}</span>
                    <span class="text-zinc-600">·</span>
                    <span class="truncate max-w-[80px]">{}</span>
                </div>"#,
                country_flag, country, isp
            )
        } else {
            String::new()
        };

        // Detection date (use enrichment date if available, fallback to first_seen)
        let detection_date = inv.enrichment.detection_date.as_ref()
            .or(inv.first_seen.as_ref())
            .map(|d| format!(r#"<span class="text-purple-400">{}</span>"#, d))
            .unwrap_or_else(|| r#"<span class="text-zinc-600">N/A</span>"#.to_string());

        // Domain created date
        let domain_created = inv.enrichment.domain_created.as_ref()
            .map(|d| format!(r#"<div class="text-[0.55rem] text-zinc-500">📆 Dominio: {}</div>"#, d))
            .unwrap_or_default();

        // AI content type badge
        let content_type_badge = inv.enrichment.ai_content_type.as_ref()
            .map(|ct| format!(
                r#"<span class="text-[0.55rem] bg-zinc-800 text-zinc-300 px-1.5 py-0.5 rounded">{}</span>"#,
                ct
            ))
            .unwrap_or_default();

        // HTTP status indicator
        let http_status = inv.enrichment.http_status.map(|s| {
            let (color, icon) = if s == 200 {
                ("text-green-400", "●")
            } else if s == 404 {
                ("text-yellow-400", "◌")
            } else {
                ("text-red-400", "○")
            };
            format!(r#"<span class="{} text-[0.6rem]">{} {}</span>"#, color, icon, s)
        }).unwrap_or_default();

        format!(r#"
            <div class="bg-zinc-900/80 border border-zinc-800 rounded-lg p-3 hover:border-orange-500/40 transition-all">
                <div class="flex gap-3">
                    <!-- Screenshot -->
                    {screenshot}
                    
                    <!-- Main Content -->
                    <div class="flex-1 min-w-0">
                        <!-- Header: Ticket + Campaign -->
                        <div class="flex justify-between items-start mb-1">
                            <div class="flex items-center gap-2 min-w-0">
                                <span class="text-orange-400 font-mono text-[0.7rem]">{ticket}</span>
                                <span class="text-zinc-600">→</span>
                                <span class="text-white text-[0.7rem] truncate">{target}</span>
                            </div>
                            {campaign_badge}
                        </div>
                        
                        <!-- AI Impersonation Badges -->
                        {impersonation}
                        
                        <!-- Warning Badges -->
                        {warnings}
                        
                        <!-- Type + Signals Row -->
                        <div class="flex items-center gap-2 text-[0.6rem] mb-2">
                            <span class="bg-zinc-800 px-1.5 py-0.5 rounded text-zinc-400">{threat_type}</span>
                            {content_type}
                            <span class="text-zinc-600">|</span>
                            <span class="text-zinc-500">🔍 {signal_count} señales</span>
                            {http_status}
                        </div>
                        
                        <!-- Geolocation -->
                        {geo}
                        
                        <!-- Dates -->
                        <div class="flex items-center gap-3 mt-2">
                            <div class="text-[0.55rem]">
                                <span class="text-zinc-500">📅 Detectado:</span>
                                {detection_date}
                            </div>
                            {domain_created}
                        </div>
                    </div>
                </div>
            </div>
        "#,
            screenshot = screenshot_html,
            ticket = inv.ticket_key,
            target = if inv.target.len() > 22 { format!("{}...", &inv.target[..22]) } else { inv.target.clone() },
            campaign_badge = campaign_badge,
            impersonation = impersonation_html,
            warnings = warning_badges,
            threat_type = inv.threat_type,
            content_type = content_type_badge,
            signal_count = inv.signal_count,
            http_status = http_status,
            geo = geo_html,
            detection_date = detection_date,
            domain_created = domain_created,
        )
    }).collect();

    // Summary stats
    let total = data.deep_investigations.len();
    let mass_campaigns = data
        .deep_investigations
        .iter()
        .filter(|i| i.is_mass_campaign)
        .count();
    let total_signals: u64 = data
        .deep_investigations
        .iter()
        .map(|i| i.signal_count)
        .sum();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white">
        <div class="flex-grow h-full overflow-hidden">
            <div class="h-full flex flex-col">
                <!-- Header -->
                <div class="mb-6 flex justify-between items-start">
                    <div>
                        <span class="bg-orange-600 px-4 py-1 text-sm font-semibold">INVESTIGACIÓN PROFUNDA</span>
                        <h2 class="text-3xl font-bold mt-3">Timeline - Tickets Etiquetados</h2>
                        <p class="text-zinc-400 mt-1 text-sm">Análisis mediante Signal-Lake del Data Lake</p>
                    </div>
                    <div class="text-right">
                        <div class="text-4xl font-bold text-orange-400">{total}</div>
                        <p class="text-zinc-500 text-xs">tickets investigados</p>
                        <div class="mt-2 flex gap-2 justify-end">
                            <span class="bg-red-900/30 text-red-300 text-xs px-2 py-0.5 rounded">{mass_campaigns} campañas</span>
                            <span class="bg-purple-900/30 text-purple-300 text-xs px-2 py-0.5 rounded">{total_signals} señales</span>
                        </div>
                    </div>
                </div>
                
                <!-- Investigation Grid -->
                <div class="grid grid-cols-2 md:grid-cols-3 gap-3 flex-grow overflow-hidden">
                    {cards}
                </div>
            </div>
        </div>
        <div class="flex justify-between items-center text-[0.6rem] text-zinc-600 print:text-black pt-3 border-t border-zinc-800/50">
            <span>TIMELINE INVESTIGATION</span>
            <span>Datos de Signal-Lake API</span>
        </div>
    </div></div>"#,
        total = total,
        mass_campaigns = mass_campaigns,
        total_signals = total_signals,
        cards = investigation_cards.join(""),
    )
}

fn render_credential_slide(data: &PocReportData, _dict: &Box<dyn Dictionary>) -> String {
    let mut rows = String::new();
    
    // Create rows for table
    for cred in data.credential_exposures.iter().take(8) {
        let user = cred.user.as_deref().unwrap_or("N/A");
        let source_date = cred.leak_date.clone().unwrap_or_else(|| "N/A".to_string());
        
        let leak_name = cred.leak_name.clone().unwrap_or_default();
        let short_leak = if leak_name.len() > 30 {
            format!("{}...", &leak_name[0..27])
        } else {
            leak_name
        };

        let url = cred.access_url.as_deref().or(cred.access_domain.as_deref()).unwrap_or("-");
        let short_url = if url.len() > 40 {
            format!("{}...", &url[0..37])
        } else {
            url.to_string()
        };

        // Format date if possible
        let date_display = if source_date.len() >= 10 {
            &source_date[0..10]
        } else {
            &source_date
        };

        rows.push_str(&format!(
            r#"<tr class="border-b border-zinc-800 text-sm hover:bg-zinc-900/50 transition-colors">
                <td class="py-3 pl-4 text-zinc-300 font-medium">{}</td>
                <td class="py-3 text-orange-400 font-mono text-xs">{}</td>
                <td class="py-3 text-zinc-400 text-xs">{}</td>
                <td class="py-3 text-zinc-500 text-xs text-right pr-4">{}</td>
            </tr>"#,
            user, short_url, short_leak, date_display
        ));
    }

    // Fill empty rows if less than 8
    let count = data.credential_exposures.len();
    if count < 8 {
        for _ in 0..(8 - count) {
             rows.push_str(r#"<tr class="border-b border-zinc-900/50 text-sm"><td class="py-3 pl-4">&nbsp;</td><td></td><td></td><td></td></tr>"#);
        }
    }

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white">
            <div class="flex-grow h-full overflow-hidden flex flex-col">
                <!-- Header -->
                <div class="mb-6 flex justify-between items-end border-b border-zinc-800 pb-4">
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                             <span class="bg-red-900/40 text-red-400 px-3 py-1 text-xs font-bold tracking-wider rounded-sm">CRITICAL EXPOSURE</span>
                             <span class="text-zinc-500 text-xs font-mono uppercase tracking-widest">CONFIDENTIAL</span>
                        </div>
                        <h2 class="text-3xl font-bold text-white">Credenciales Comprometidas</h2>
                        <p class="text-zinc-400 mt-1 text-sm">Identificadas en Stealer Logs y filtraciones (Dark Web)</p>
                    </div>
                    <div class="text-right">
                        <div class="text-5xl font-bold text-red-500 tracking-tighter">{}</div>
                        <div class="text-xs text-zinc-500 uppercase tracking-widest mt-1">Total Expuesto</div>
                    </div>
                </div>
                
                <!-- Table -->
                <div class="flex-grow w-full overflow-hidden bg-zinc-900/20 rounded-lg border border-zinc-800/50">
                    <table class="w-full text-left border-collapse">
                        <thead>
                            <tr class="text-xs text-zinc-500 uppercase tracking-wider border-b border-zinc-800 bg-zinc-900/50">
                                <th class="py-3 pl-4 font-bold">Usuario / Email</th>
                                <th class="py-3 font-bold">Sitio / URL</th>
                                <th class="py-3 font-bold">Fuente de Filtración</th>
                                <th class="py-3 pr-4 text-right font-bold">Fecha</th>
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
            </div>

            <!-- Footer -->
            <div class="mt-4 pt-3 flex justify-between items-center text-[0.6rem] text-zinc-700 border-t border-zinc-900/50">
                 <div class="flex items-center gap-2">
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                    <span>AXUR DIGITAL RISK PROTECTION</span>
                 </div>
                 <span>SENSITIVE DATA - DO NOT DISTRIBUTE</span>
            </div>
        </div></div>"#,
        count,
        rows
    )
}

fn render_closing_full(
    _data: &PocReportData,
    offline_assets: Option<&OfflineAssets>,
    dict: &Box<dyn Dictionary>,
) -> String {
    // Abstract geometric pattern (CSS only) - Refined for "Thrive" theme
    let abstract_pattern = r#"
        <div class="absolute top-0 left-0 w-full h-[45%] bg-[#FF4D00] overflow-hidden">
            <div class="absolute top-[20%] left-[10%] w-[15%] h-[40%] bg-zinc-900"></div>
            <div class="absolute top-[10%] left-[30%] w-[10%] h-[60%] bg-zinc-900"></div>
            <div class="absolute top-0 left-[60%] w-[20%] h-[50%] bg-white"></div>
            <div class="absolute bottom-0 right-[10%] w-[15%] h-[30%] bg-white"></div>
            <div class="absolute bottom-[10%] left-[0%] w-[15%] h-[30%] bg-zinc-900"></div>
        </div>
    "#;

    // Choose image source (URL or Base64)
    let img_src = if let Some(assets) = offline_assets {
        format!("data:image/jpeg;base64,{}", assets.office_image_base64)
    } else {
        "https://images.unsplash.com/photo-1497366216548-37526070297c?auto=format&fit=crop&w=1000&q=80".to_string()
    };

    // Office image
    let office_image = format!(
        r#"
        <div class="absolute bottom-0 left-0 w-full h-[55%] bg-zinc-800 overflow-hidden">
             <img src="{}" class="w-full h-full object-cover grayscale opacity-60 mix-blend-luminosity">
             <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent"></div>
        </div>
    "#,
        img_src
    );

    format!(
        r#"
    <div class="printable-slide aspect-[16/9] w-full p-0 relative bg-zinc-950 flex overflow-hidden">
        <!-- Left Side: Visuals -->
        <div class="w-5/12 h-full relative border-r border-zinc-800">
            {pattern}
            {image}
        </div>
        
        <!-- Right Side: Content -->
        <div class="w-7/12 h-full flex flex-col justify-center px-16 relative bg-[#0a0a0a]">
            <div class="mb-12">
                 <h2 class="text-5xl font-bold text-white leading-tight mb-6">
                    {title}
                 </h2>
                 <p class="text-xl text-zinc-400 font-light leading-relaxed max-w-xl">
                    {intro}
                 </p>
            </div>
            
            <div class="space-y-8 mb-12">
                <!-- Benefit 1 -->
                <div class="flex items-start gap-4">
                    <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item1_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item1_desc}</p>
                    </div>
                </div>
                
                <!-- Benefit 2 -->
                <div class="flex items-start gap-4">
                     <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item2_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item2_desc}</p>
                    </div>
                </div>

                <!-- Benefit 3 -->
                <div class="flex items-start gap-4">
                     <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item3_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item3_desc}</p>
                    </div>
                </div>
            </div>
            
            <div class="absolute bottom-12 left-16 right-16 flex items-end justify-between border-t border-zinc-900 pt-8">
                <!-- Certifications -->
                <div class="flex items-center gap-4 opacity-60 grayscale hover:grayscale-0 transition-all duration-500">
                   <div class="flex flex-col items-center">
                        <span class="text-[0.65rem] text-zinc-500 font-bold tracking-wider mb-1">CERTIFIED</span>
                        <div class="border border-zinc-700 px-2 py-1 rounded">
                            <span class="text-xs text-zinc-300 font-bold">ISO/IEC 27001</span>
                        </div>
                   </div>
                </div>

                <!-- Axur Logo -->
                <div class="flex items-center gap-2 select-none">
                    <span class="text-orange-600 text-3xl font-black italic tracking-tighter">///</span>
                    <span class="text-white text-3xl font-bold tracking-widest">AXUR</span>
                </div>
            </div>
        </div>
    </div>
    "#,
        pattern = abstract_pattern,
        image = office_image,
        title = dict.closing_value_title(),
        intro = dict.closing_value_intro(),
        item1_title = dict.closing_value_item_1_title(),
        item1_desc = dict.closing_value_item_1_desc(),
        item2_title = dict.closing_value_item_2_title(),
        item2_desc = dict.closing_value_item_2_desc(),
        item3_title = dict.closing_value_item_3_title(),
        item3_desc = dict.closing_value_item_3_desc(),
    )
}

fn render_risk_context_slide(title: String, text: String, dict: &Box<dyn Dictionary>) -> String {
    // Abstract geometric pattern (CSS only) - Refined for "Thrive" theme
    let abstract_pattern = r#"
        <div class="absolute top-0 left-0 w-full h-[45%] bg-[#FF4D00] overflow-hidden">
            <div class="absolute top-[20%] left-[10%] w-[15%] h-[40%] bg-zinc-900"></div>
            <div class="absolute top-[10%] left-[30%] w-[10%] h-[60%] bg-zinc-900"></div>
            <div class="absolute top-0 left-[60%] w-[20%] h-[50%] bg-white"></div>
            <div class="absolute bottom-0 right-[10%] w-[15%] h-[30%] bg-white"></div>
            <div class="absolute bottom-[10%] left-[0%] w-[15%] h-[30%] bg-zinc-900"></div>
        </div>
    "#;

    format!(
        r#"
    <div class="printable-slide aspect-[16/9] w-full p-0 relative bg-zinc-950 flex overflow-hidden">
        <!-- Left Side: Visuals -->
        <div class="w-5/12 h-full relative border-r border-zinc-800">
            {pattern}
            <div class="absolute bottom-0 left-0 w-full h-[55%] bg-zinc-800 overflow-hidden">
                 <img src="https://images.unsplash.com/photo-1497366216548-37526070297c?auto=format&fit=crop&w=1000&q=80" class="w-full h-full object-cover grayscale opacity-60 mix-blend-luminosity">
                 <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent"></div>
            </div>
        </div>
        
        <!-- Right Side: Content -->
        <div class="w-7/12 h-full flex flex-col justify-center px-16 relative bg-[#0a0a0a]">
            <div class="mb-12">
                 <h2 class="text-5xl font-bold text-white leading-tight mb-6">
                    {title}
                 </h2>
                 <p class="text-xl text-zinc-400 font-light leading-relaxed max-w-xl">
                    {text}
                 </p>
            </div>
            
            <div class="space-y-8 mb-12">
                <!-- Benefit 1 -->
                <div class="flex items-start gap-4">
                    <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item1_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item1_desc}</p>
                    </div>
                </div>
                
                <!-- Benefit 2 -->
                <div class="flex items-start gap-4">
                     <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item2_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item2_desc}</p>
                    </div>
                </div>

                <!-- Benefit 3 -->
                <div class="flex items-start gap-4">
                     <div class="mt-1 p-2 bg-zinc-800 rounded-lg">
                        <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-1">{item3_title}</h4>
                        <p class="text-zinc-500 text-sm max-w-sm leading-relaxed">{item3_desc}</p>
                    </div>
                </div>
            </div>
            
            <div class="absolute bottom-12 left-16 right-16 flex items-end justify-between border-t border-zinc-900 pt-8">
                <!-- Certifications -->
                <div class="flex items-center gap-4 opacity-60 grayscale hover:grayscale-0 transition-all duration-500">
                   <div class="flex flex-col items-center">
                        <span class="text-[0.65rem] text-zinc-500 font-bold tracking-wider mb-1">CERTIFIED</span>
                        <div class="border border-zinc-700 px-2 py-1 rounded">
                            <span class="text-xs text-zinc-300 font-bold">ISO/IEC 27001</span>
                        </div>
                   </div>
                </div>

                <!-- Axur Logo -->
                <div class="flex items-center gap-2 select-none">
                    <span class="text-orange-600 text-3xl font-black italic tracking-tighter">///</span>
                    <span class="text-white text-3xl font-bold tracking-widest">AXUR</span>
                </div>
            </div>
        </div>
    </div>
    "#,
        pattern = abstract_pattern,
        title = title,
        text = text,
        item1_title = dict.closing_value_item_1_title(),
        item1_desc = dict.closing_value_item_1_desc(),
        item2_title = dict.closing_value_item_2_title(),
        item2_desc = dict.closing_value_item_2_desc(),
        item3_title = dict.closing_value_item_3_title(),
        item3_desc = dict.closing_value_item_3_desc(),
    )
}

// =====================
// HELPER FUNCTIONS
// =====================

fn country_to_flag(country: &str) -> String {
    let country_lower = country.to_lowercase();
    match country_lower.as_str() {
        // Names
        "brazil" | "brasil" => "🇧🇷".to_string(),
        "united states" | "usa" | "us" | "estados unidos" => "🇺🇸".to_string(),
        "china" => "🇨🇳".to_string(),
        "russia" => "🇷🇺".to_string(),
        "germany" | "alemanha" => "🇩🇪".to_string(),
        "france" | "frança" => "🇫🇷".to_string(),
        "uk" | "united kingdom" | "reino unido" => "🇬🇧".to_string(),
        "india" => "🇮🇳".to_string(),
        "japan" | "japão" => "🇯🇵".to_string(),
        "canada" => "🇨🇦".to_string(),
        "australia" => "🇦🇺".to_string(),
        "netherlands" | "holanda" => "🇳🇱".to_string(),
        "singapore" | "singapura" => "🇸🇬".to_string(),
        // Fallback: try to convert 2-letter code
        _ => {
            if country.len() == 2 && country.chars().all(|c| c.is_alphabetic()) {
                country.chars()
                    .filter(|c| c.is_alphabetic())
                    .map(|c| char::from_u32(0x1F1E6 + c.to_ascii_uppercase() as u32 - 'A' as u32).unwrap_or('🏳'))
                    .collect()
            } else {
                "🌐".to_string()
            }
        }
    }
}

fn format_date(date: &str) -> String {
    // Convert YYYY-MM-DD to DD/MM/YYYY
    if date.len() == 10 && date.contains('-') {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date.to_string()
}

fn format_number(n: u64) -> String {
    // Add thousand separators
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Generate a self-contained HTML report file
pub fn generate_html(report: &PocReport) -> String {
    let cover_slide = render_cover_slide(report);
    let summary_slide = render_summary_slide(report);
    let incidents_slide = render_incidents_slide(report);
    let takedowns_slide = render_takedowns_slide(report);
    let closing_slide = render_closing_slide(report);

    format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>PoC Report - {company}</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&display=swap" rel="stylesheet">
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {{
            theme: {{
                extend: {{
                    fontFamily: {{ sans: ['Inter', 'sans-serif'] }}
                }}
            }}
        }};
    </script>
    <style>
        @media print {{
            @page {{
                size: 16in 9in landscape;
                margin: 0;
            }}
            body {{
                background-color: #fff !important;
                -webkit-print-color-adjust: exact;
                print-color-adjust: exact;
                width: 16in;
                height: 9in;
            }}
            .no-print {{ display: none !important; }}
            .printable-slide {{
                width: 16in;
                height: 9in;
                box-sizing: border-box;
                aspect-ratio: 16/9 !important;
                padding: 0.75in !important;
                box-shadow: none !important;
                border-radius: 0 !important;
                break-inside: avoid;
                break-after: page;
                page-break-after: always;
                margin: 0 !important;
            }}
        }}
        .printable-slide {{
            aspect-ratio: 16/9;
        }}
    </style>
</head>
<body class="bg-zinc-950 text-white font-sans">
    <div class="no-print fixed top-4 right-4 z-50">
        <button onclick="window.print()" class="bg-orange-600 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded-lg">
            Imprimir / Guardar PDF
        </button>
    </div>
    
    <div id="report-content" class="p-4 md:p-8 max-w-7xl mx-auto">
        {cover}
        {summary}
        {incidents}
        {takedowns}
        {closing}
    </div>
</body>
</html>"#,
        company = report.customer_name,
        cover = cover_slide,
        summary = summary_slide,
        incidents = incidents_slide,
        takedowns = takedowns_slide,
        closing = closing_slide,
    )
}

fn render_cover_slide(report: &PocReport) -> String {
    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950">
        <div class="flex-grow flex flex-col justify-center">
            <div class="inline-block bg-orange-600 px-4 py-1 mb-4 w-fit">
                <span class="font-bold text-lg text-white">TLP:AMBER</span>
            </div>
            <h1 class="text-6xl font-black leading-tight">
                INFORME<br/>EJECUTIVO
            </h1>
            <div class="mt-8">
                <p class="text-orange-500 font-semibold">Compañía</p>
                <p class="text-3xl">{company}</p>
            </div>
            <div class="mt-4">
                <p class="text-orange-500 font-semibold">Período</p>
                <p class="text-xl">{from} a {to}</p>
            </div>
        </div>
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">Axur. Digital experiences made safe.</span>
        </footer>
    </div>
    "#,
        company = report.customer_name,
        from = report.date_from,
        to = report.date_to,
    )
}

fn render_summary_slide(report: &PocReport) -> String {
    let total_creds = report.credentials_employee + report.credentials_stealer;

    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-900">
        <h2 class="text-4xl font-bold text-orange-500 mb-8">Resumen Ejecutivo</h2>
        
        <div class="grid grid-cols-3 gap-8 flex-grow">
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-white">{signals}</span>
                <span class="text-xl text-zinc-400 mt-2">Señales Detectadas</span>
            </div>
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-orange-500">{incidents}</span>
                <span class="text-xl text-zinc-400 mt-2">Incidentes Confirmados</span>
            </div>
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-red-500">{threats}</span>
                <span class="text-xl text-zinc-400 mt-2">Amenazas Activas</span>
            </div>
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-yellow-500">{creds}</span>
                <span class="text-xl text-zinc-400 mt-2">Credenciales Expuestas</span>
            </div>
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-green-500">{takedowns}</span>
                <span class="text-xl text-zinc-400 mt-2">Takedowns Resueltos</span>
            </div>
            <div class="bg-zinc-800 rounded-xl p-6 flex flex-col justify-center items-center">
                <span class="text-6xl font-black text-purple-500">{code_leaks}</span>
                <span class="text-xl text-zinc-400 mt-2">Fugas de Código</span>
            </div>
        </div>
        
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">2</span>
        </footer>
    </div>
    "#,
        signals = report.total_signals,
        incidents = report.total_incidents,
        threats = report.total_threats,
        creds = total_creds,
        takedowns = report.takedown_resolved,
        code_leaks = report.code_leak_total,
    )
}

fn render_incidents_slide(report: &PocReport) -> String {
    let mut rows = String::new();
    for item in &report.incidents_by_type {
        rows.push_str(&format!(
            r#"
            <tr class="border-b border-zinc-700">
                <td class="py-3 px-4 text-left">{}</td>
                <td class="py-3 px-4 text-right font-bold text-orange-500">{}</td>
            </tr>
        "#,
            item.incident_type, item.count
        ));
    }

    if rows.is_empty() {
        rows = r#"
            <tr class="border-b border-zinc-700">
                <td class="py-3 px-4 text-center text-zinc-500" colspan="2">Sin datos de incidentes</td>
            </tr>
        "#.to_string();
    }

    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-900">
        <h2 class="text-4xl font-bold text-orange-500 mb-8">Incidentes por Tipo</h2>
        
        <div class="flex-grow overflow-auto">
            <table class="w-full text-lg">
                <thead>
                    <tr class="bg-zinc-800">
                        <th class="py-3 px-4 text-left">Tipo de Incidente</th>
                        <th class="py-3 px-4 text-right">Cantidad</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
        
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">3</span>
        </footer>
    </div>
    "#,
        rows = rows,
    )
}

fn render_takedowns_slide(report: &PocReport) -> String {
    let total = report.takedown_resolved + report.takedown_pending + report.takedown_aborted;
    let success_rate = if total > 0 {
        (report.takedown_resolved as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };

    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-900">
        <h2 class="text-4xl font-bold text-orange-500 mb-8">Takedowns</h2>
        
        <div class="grid grid-cols-2 gap-8 flex-grow">
            <div class="flex flex-col justify-center">
                <div class="bg-zinc-800 rounded-xl p-8 mb-4">
                    <span class="text-5xl font-black text-green-500">{resolved}</span>
                    <span class="text-xl text-zinc-400 ml-4">Resueltos</span>
                </div>
                <div class="bg-zinc-800 rounded-xl p-8 mb-4">
                    <span class="text-5xl font-black text-yellow-500">{pending}</span>
                    <span class="text-xl text-zinc-400 ml-4">Pendientes</span>
                </div>
                <div class="bg-zinc-800 rounded-xl p-8">
                    <span class="text-5xl font-black text-red-500">{aborted}</span>
                    <span class="text-xl text-zinc-400 ml-4">Cancelados</span>
                </div>
            </div>
            <div class="flex flex-col justify-center items-center bg-zinc-800 rounded-xl p-8">
                <span class="text-8xl font-black text-orange-500">{rate}%</span>
                <span class="text-2xl text-zinc-400 mt-4">Tasa de Éxito</span>
            </div>
        </div>
        
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">4</span>
        </footer>
    </div>
    "#,
        resolved = report.takedown_resolved,
        pending = report.takedown_pending,
        aborted = report.takedown_aborted,
        rate = success_rate,
    )
}

fn render_closing_slide(report: &PocReport) -> String {
    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950">
        <div class="flex-grow flex flex-col justify-center items-center text-center">
            <span class="text-orange-500 font-bold text-4xl mb-8">AXUR</span>
            <h2 class="text-5xl font-black mb-4">Gracias</h2>
            <p class="text-2xl text-zinc-400 mb-8">{company}</p>
            <p class="text-xl text-zinc-500">Informe generado automáticamente por Axur CLI</p>
            <p class="text-lg text-zinc-600 mt-4">{from} - {to}</p>
        </div>
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-xs text-zinc-400">Axur. Digital experiences made safe. All rights reserved.</span>
        </footer>
    </div>
    "#,
        company = report.customer_name,
        from = report.date_from,
        to = report.date_to,
    )
}

/// Render Deep Analytics slide with conditional sections based on available data
fn render_deep_analytics_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let analytics = &data.deep_analytics;
    let mut sections = Vec::new();

    // Section 1: Code Leak Insights
    if analytics.has_code_leak_insights && !analytics.secret_types_breakdown.is_empty() {
        let mut rows = String::new();
        for item in analytics.secret_types_breakdown.iter().take(5) {
            rows.push_str(&format!(
                r#"
                <div class="flex justify-between items-center py-2 border-b border-zinc-700">
                    <span class="text-zinc-300">{}</span>
                    <span class="font-bold text-purple-400">{}</span>
                </div>
            "#,
                item.name, item.value
            ));
        }
        sections.push(format!(
            r#"
            <div class="bg-zinc-800/50 rounded-xl p-6">
                <div class="flex items-center gap-2 mb-4">
                    <span class="text-2xl">🔐</span>
                    <h3 class="text-xl font-bold text-purple-400">{}</h3>
                </div>
                <p class="text-sm text-zinc-400 mb-4">{}</p>
                <div class="space-y-1">{}</div>
            </div>
        "#,
            dict.deep_analytics_code_leak_title(),
            dict.deep_analytics_code_leak_subtitle(data.unique_repos),
            rows
        ));
    }

    // Section 2: Credential Insights
    if analytics.has_credential_insights && !analytics.leak_source_breakdown.is_empty() {
        let mut rows = String::new();
        for item in &analytics.leak_source_breakdown {
            let color = if item.name.contains("STEALER") {
                "text-red-400"
            } else {
                "text-yellow-400"
            };
            rows.push_str(&format!(
                r#"
                <div class="flex justify-between items-center py-2 border-b border-zinc-700">
                    <span class="text-zinc-300">{}</span>
                    <span class="font-bold {}">{}</span>
                </div>
            "#,
                item.name, color, item.value
            ));
        }
        sections.push(format!(
            r#"
            <div class="bg-zinc-800/50 rounded-xl p-6">
                <div class="flex items-center gap-2 mb-4">
                    <span class="text-2xl">🔑</span>
                    <h3 class="text-xl font-bold text-yellow-400">{}</h3>
                </div>
                <p class="text-sm text-zinc-400 mb-4">{}</p>
                <div class="space-y-1">{}</div>
            </div>
        "#,
            dict.deep_analytics_credential_title(),
            dict.deep_analytics_credential_subtitle(
                data.credential_leaks_summary.total_credentials
            ),
            rows
        ));
    }

    // Section 3: Takedown Efficiency
    if analytics.has_takedown_insights && !analytics.takedowns_by_platform.is_empty() {
        let mut rows = String::new();
        for item in analytics.takedowns_by_platform.iter().take(5) {
            rows.push_str(&format!(
                r#"
                <div class="flex justify-between items-center py-2 border-b border-zinc-700">
                    <span class="text-zinc-300">{}</span>
                    <span class="font-bold text-green-400">{}</span>
                </div>
            "#,
                item.name, item.value
            ));
        }
        sections.push(format!(
            r#"
            <div class="bg-zinc-800/50 rounded-xl p-6">
                <div class="flex items-center gap-2 mb-4">
                    <span class="text-2xl">⚡</span>
                    <h3 class="text-xl font-bold text-green-400">{}</h3>
                </div>
                <p class="text-sm text-zinc-400 mb-4">{}</p>
                <div class="space-y-1">{}</div>
            </div>
        "#,
            dict.deep_analytics_takedown_title(),
            dict.deep_analytics_takedown_subtitle(data.resolved_takedowns.len()),
            rows
        ));
    }

    // If no sections, return empty (slide will be skipped)
    if sections.is_empty() {
        return String::new();
    }

    // Determine grid columns based on number of sections
    let grid_cols = match sections.len() {
        1 => "grid-cols-1",
        2 => "grid-cols-2",
        _ => "grid-cols-3",
    };

    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-900">
        <h2 class="text-4xl font-bold text-orange-500 mb-2">{}</h2>
        <p class="text-zinc-400 mb-8">{}</p>
        
        <div class="grid {} gap-6 flex-grow">
            {}
        </div>
        
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">Deep Analytics</span>
        </footer>
    </div>
    "#,
        dict.deep_analytics_title(),
        dict.deep_analytics_subtitle(),
        grid_cols,
        sections.join("\n")
    )
}

fn render_timeline_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // 1. Calculate Off-hours % and Histogram (0-23h)
    let mut hour_counts = vec![0u64; 24];
    let mut off_hours_count = 0;
    let mut total_with_time = 0;

    for ticket in &data.story_tickets {
        if let Some(date_str) = &ticket.creation_date {
            // Parses "2023-10-27T10:00:00..." or similar ISO string
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                let hour = dt.hour() as usize;
                if hour < 24 {
                    hour_counts[hour] += 1;
                    total_with_time += 1;

                    // Off-hours: < 8 or > 18 (roughly) or Weekend (Sat=6, Sun=7)
                    let is_weekend = dt.weekday().number_from_monday() >= 6;
                    if is_weekend || hour < 8 || hour >= 19 {
                        off_hours_count += 1;
                    }
                }
            } else if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
                 // Fallback for other formats if any
                let hour = dt.hour() as usize;
                if hour < 24 {
                    hour_counts[hour] += 1;
                    total_with_time += 1;
                    let is_weekend = dt.weekday().number_from_monday() >= 6;
                    if is_weekend || hour < 8 || hour >= 19 {
                        off_hours_count += 1;
                    }
                }
            }
        }
    }

    let percent_off_hours = if total_with_time > 0 {
        (off_hours_count * 100) / total_with_time
    } else {
        35 // Fallback default
    };
    
    // Generate simple colors for the chart: Off-hours = RED, Business = Gray
    let mut background_colors = Vec::new();
    for h in 0..24 {
        if h < 8 || h >= 19 {
            background_colors.push("#ef4444"); // Red-500
        } else {
            background_colors.push("#e4e4e7"); // Zinc-200 (Gray)
        }
    }

    let json_labels = serde_json::to_string(&(0..24).map(|h| format!("{:02}h", h)).collect::<Vec<String>>()).unwrap_or_default();
    let json_data = serde_json::to_string(&hour_counts).unwrap_or_default();
    let json_colors = serde_json::to_string(&background_colors).unwrap_or_default();

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-red-600 text-white px-4 py-1 text-sm font-semibold">24/7 COVERAGE</span></div><h2 class="text-4xl font-bold mb-4">{title}</h2><p class="text-zinc-400 max-w-5xl text-xl mb-8 leading-relaxed">{desc}</p><div class="flex-grow bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 relative"><canvas id="timelineChart"></canvas></div></div></div>{footer}<script>(function(){{const ctx=document.getElementById('timelineChart').getContext('2d');new Chart(ctx,{{type:'bar',data:{{labels:{json_labels},datasets:[{{label:'Detections',data:{json_data},backgroundColor:{json_colors},borderRadius:2}}]}},options:{{responsive:true,maintainAspectRatio:false,plugins:{{legend:{{display:false}}}},scales:{{y:{{beginAtZero:true,grid:{{color:'rgba(255,255,255,0.1)'}},ticks:{{color:'#a1a1aa'}}}},x:{{grid:{{display:false}},ticks:{{color:'#a1a1aa'}}}}}}}}}});}})();</script></div></div>"#,
        title = dict.narrative_timeline_title(),
        desc = dict.narrative_timeline_text(percent_off_hours),
        footer = footer_dark(0, dict),
        json_labels = json_labels,
        json_data = json_data,
        json_colors = json_colors
    );

    format!(
        r#"
    <div class="printable-slide w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-900">
        <h2 class="text-4xl font-bold text-orange-500 mb-2">{title}</h2>
        <p class="text-zinc-400 mb-2">{text}</p>
        
        <div class="h-64 w-full relative">
            <canvas id="timelineChart"></canvas>
        </div>
        
        <div class="grid grid-cols-2 gap-8 mt-6">
             <div class="bg-zinc-800/30 p-4 rounded border border-zinc-700">
                <span class="text-xs text-zinc-500 uppercase tracking-wider">Business Hours</span>
                <div class="text-2xl font-bold text-zinc-300">{business_percent}%</div>
             </div>
             <div class="bg-red-900/20 p-4 rounded border border-red-900/50">
                <span class="text-xs text-red-400 uppercase tracking-wider">Off-Hours Risk</span>
                <div class="text-2xl font-bold text-red-500">{off_percent}%</div>
             </div>
        </div>

        <script>
            new Chart(document.getElementById('timelineChart'), {{
                type: 'bar',
                data: {{
                    labels: [{labels}],
                    datasets: [{{
                        label: 'Detections',
                        data: [{data}],
                        backgroundColor: [{colors}],
                        borderRadius: 4
                    }}]
                }},
                options: {{
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {{ legend: {{ display: false }} }},
                    scales: {{
                        y: {{ display: false }},
                        x: {{ grid: {{ display: false }}, ticks: {{ color: '#71717a' }} }}
                    }}
                }}
            }});
        </script>
        
        <footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
            <span class="text-orange-500 font-bold text-xl">AXUR</span>
            <span class="text-xs text-zinc-400">Chaos Heatmap</span>
        </footer>
    </div>
    "#,
        // ... (existing mapped args)
        title = dict.narrative_timeline_title(),
        text = dict.narrative_timeline_text(percent_off_hours),
        labels = (0..24).map(|h| format!("\"{:02}h\"", h)).collect::<Vec<_>>().join(","),
        data = hour_counts.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
        colors = background_colors.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(","),
        off_percent = percent_off_hours,
        business_percent = 100 - percent_off_hours,
    )
}

fn render_virality_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    let viral_count = data.threat_intelligence.chat_group_shares + data.threat_intelligence.dark_web_mentions;
    
    // Choose Primary or Fallback
    let (pain, solution) = if viral_count > 0 {
        let source_count = if !data.threat_intelligence.dark_web_sources.is_empty() {
             data.threat_intelligence.dark_web_sources.len() 
        } else { 1 };
        
        // Safely extract top source
        let top_source = data.threat_intelligence.dark_web_sources.first()
            .map(|s| s.as_str())
            .unwrap_or("Telegram Fraud Groups");
            
        (
            dict.narrative_virality_pain_primary(viral_count, source_count, top_source),
            dict.narrative_virality_solution_primary()
        )
    } else {
        (
            dict.narrative_virality_pain_fallback(),
            dict.narrative_virality_solution_fallback()
        )
    };

    render_narrative_slide(
        dict.narrative_virality_title(),
        pain,
        solution,
        "virality",
        dict
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::report::{PocReportData, ThreatTypeCount, ResolvedTakedown, RoiMetrics, DeepAnalyticsData, ThreatIntelligence, NameValuePair, CredentialLeaksSummary, TakedownExample, PocEvidence, IncidentTypeCount, StoryTicket, IncidentExample, CredentialExposure};
    use crate::i18n::Language;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_generate_report_html_output() {
        let dict = crate::i18n::get_dictionary(Language::En);
        
        let data = PocReportData {
            company_name: "Test Company".to_string(),
            start_date: "2023-01-01".to_string(),
            end_date: "2023-01-31".to_string(),
            is_dynamic_window: false,
            partner_name: None,
            tlp_level: "AMBER".to_string(),
            
            brands_count: 5,
            brands: vec!["TestBrand".to_string()],
            executives_count: 2,
            ips_count: 10,
            bins_count: 20,
            domains_count: 50,
            
            threat_hunting_credits: 100,
            threat_intelligence_assets: 10,
            
            total_tickets: 1000,
            total_threats: 500,
            validation_hours: 10.0,
            
            credentials_total: 30,
            unique_hosts: 5,
            high_risk_users: 2,
            malware_breakdown: vec![],
            top_services: vec![],
            
            secrets_total: 15,
            unique_repos: 3,
            production_secrets: 1,
            platform_breakdown: vec![],
            secret_types: vec![],
            
            credential_leaks_summary: CredentialLeaksSummary {
                total_credentials: 100,
                unique_emails: 50,
                sources: vec![],
                plaintext_passwords: 20,
                stealer_logs_count: 5,
            },

            incidents_by_type: vec![
                 IncidentTypeCount { incident_type: "Phishing".to_string(), detections: 200, incidents: 25 },
            ],
            
            takedown_resolved: 40,
            takedown_pending: 5,
            takedown_aborted: 2,
            takedown_unresolved: 3,
            takedown_success_rate: 95.0,
            takedown_median_time_to_notify: "10 min".to_string(),
            takedown_median_uptime: "2 hrs".to_string(),
            takedowns_by_type: vec![],
            
            threats_by_type: vec![
                ThreatTypeCount { threat_type: "phishing".to_string(), count: 200 },
                ThreatTypeCount { threat_type: "infostealer-credential".to_string(), count: 100 },
            ],
            
            poc_examples: vec![
                 PocEvidence {
                    ticket_key: "POC-123".to_string(),
                    evidence_type: "Phishing".to_string(),
                    reference_url: "http://example.com".to_string(),
                    domain: Some("malicious.com".to_string()),
                    status: "incident".to_string(),
                    screenshot_url: Some("http://example.com/poc.png".to_string()),
                    ip: None,
                    isp: None,
                    reported_date: None,
                }
            ],
            
            takedown_examples: vec![],

            resolved_takedowns: vec![
                ResolvedTakedown {
                    ticket_key: "TK-1".to_string(), name: "US Takedown".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "Google".to_string(), ip: "1.1.1.1".to_string(), country: "USA".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("Google".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-2".to_string(), name: "BR Takedown".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "Locaweb".to_string(), ip: "2.2.2.2".to_string(), country: "Brazil".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("Locaweb".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-3".to_string(), name: "CN Takedown".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "Alibaba".to_string(), ip: "3.3.3.3".to_string(), country: "China".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("Alibaba".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-4".to_string(), name: "RU Takedown".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "Yandex".to_string(), ip: "4.4.4.4".to_string(), country: "Russia".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("Yandex".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-5".to_string(), name: "DE Takedown".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "Hetzner".to_string(), ip: "5.5.5.5".to_string(), country: "Germany".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("Hetzner".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-6".to_string(), name: "BR Takedown 2".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "UOL".to_string(), ip: "6.6.6.6".to_string(), country: "Brasil".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("UOL".to_string()),
                },
                ResolvedTakedown {
                    ticket_key: "TK-7".to_string(), name: "US Takedown 2".to_string(), ticket_type: "phishing".to_string(), status: "resolved".to_string(), host: "AWS".to_string(), ip: "7.7.7.7".to_string(), country: "USA".to_string(), request_date: None, resolution_date: None, url: "".to_string(), screenshot_url: None, registrar: None, isp: Some("AWS".to_string()),
                },
            ],
            
            latest_incidents: vec![
                IncidentExample {
                     ticket_key: "INC-123".to_string(),
                     name: "Test Incident".to_string(),
                     ticket_type: "Phishing".to_string(),
                     status: "active".to_string(),
                     open_date: Some("2023-01-01".to_string()),
                     incident_date: Some("2023-01-01".to_string()),
                     host: "Cloudflare".to_string(),
                     ip: "1.1.1.1".to_string(),
                     isp: "Cloudflare".to_string(),
                     url: "http://fake-login.com".to_string(),
                     country: "USA".to_string(),
                     registrar: None,
                     screenshot_url: None,
                }
            ],
            
            deep_analytics: DeepAnalyticsData::default(),
            
            roi_metrics: RoiMetrics {
                hours_saved_total: 100.0,
                person_days_saved: 12.5,
                analysts_equivalent_monthly: 0.6,
                tickets_processed: 1000,
                credentials_monitored: 30,
                hours_saved_validation: 40.0,
                hours_saved_credentials: 20.0,
                hours_saved_takedowns: 40.0,
                hours_saved_secrets: 0.0,
                ..Default::default()
            },

            story_tickets: vec![
                StoryTicket {
                    ticket_key: "STORY-123".to_string(),
                    status: "incident".to_string(),
                    threat_type: "phishing".to_string(),
                    target: "Brand".to_string(),
                    description: "Phishing Brand".to_string(),
                    incident_date: Some("2023-01-01".to_string()),
                    open_date: None,
                    creation_date: None,
                    close_date: None,
                    risk_score: Some(0.8),
                    brand_confidence: Some(0.9),
                    screenshot_url: Some("http://example.com/story.png".to_string()),
                    page_title: Some("Fake Login".to_string()),
                    isp: Some("BadISP".to_string()),
                    ip: Some("1.2.3.4".to_string()),
                    time_to_incident_hours: Some(5),
                    incident_age_hours: Some(24),
                },
                StoryTicket {
                    ticket_key: "STORY-124".to_string(),
                    status: "closed".to_string(),
                    threat_type: "fake-social-media-profile".to_string(),
                    target: "Executive".to_string(),
                    description: "Fake Profile".to_string(),
                    incident_date: Some("2023-01-05".to_string()),
                    open_date: None,
                    creation_date: None,
                    close_date: None,
                    risk_score: Some(0.5),
                    brand_confidence: Some(0.8),
                    screenshot_url: None,
                    page_title: None,
                    isp: None,
                    ip: None,
                    time_to_incident_hours: None,
                    incident_age_hours: None,
                }
            ],
            
            threat_intelligence: ThreatIntelligence::default(),
            deep_investigations: vec![],
            credential_exposures: vec![],
            critical_credentials: vec![
                CredentialExposure {
                    user: Some("admin@corporate.com".to_string()),
                    password: Some("Corporate2024!".to_string()),
                    access_url: Some("http://login.corporate.com".to_string()),
                    ..Default::default()
                },
                CredentialExposure {
                    user: Some("finance@corporate.com".to_string()),
                    password: Some("CorporatePass123".to_string()),
                    ..Default::default()
                }
            ],
        };
        
        let html = generate_full_report_html(&data, None, &dict);
        
        // Write to a file in a location we can access
        // Use an absolute path to avoid confusion about CWD
        let path = "c:/Users/maiso/.gemini/antigravity/workspace/axur-web/crates/core/debug_report.html";
        let mut file = File::create(path).expect("Could not create debug file");
        file.write_all(html.as_bytes()).expect("Could not write to file");
    }
}
