//! Examples Slide Plugin
//!
//! Shows examples of detected threats and resolved takedowns.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct TakedownExamplesSlidePlugin;

impl SlidePlugin for TakedownExamplesSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.takedown_examples"
    }
    fn name(&self) -> &'static str {
        "Takedown Examples"
    }
    fn priority(&self) -> i32 {
        25
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.takedown_examples.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Take up to 4 examples
        let examples_html: String = data.takedown_examples.iter().take(4).map(|ex| {
            let status_color = match ex.status.to_lowercase().as_str() {
                s if s.contains("solv") || s.contains("remov") || s.contains("success") => "text-emerald-400 bg-emerald-400/10 border-emerald-400/20",
                s if s.contains("pending") || s.contains("progr") => "text-amber-400 bg-amber-400/10 border-amber-400/20",
                _ => "text-red-400 bg-red-400/10 border-red-400/20",
            };

            format!(
                r#"<div class="bg-zinc-900 rounded-lg border border-zinc-800 p-5 flex flex-col gap-3 hover:border-[#FF671F]/40 transition-colors">
                    <div class="flex items-start justify-between">
                        <span class="text-xs font-bold px-2 py-1 rounded bg-zinc-800 text-zinc-400 uppercase tracking-wider">{type_}</span>
                        <span class="text-xs font-bold px-2 py-1 rounded border {status_class}">{status}</span>
                    </div>
                    <div class="flex-grow">
                         <p class="text-sm font-mono text-zinc-300 break-all line-clamp-2" title="{url}">{url}</p>
                    </div>
                    {date_html}
                </div>"#,
                type_ = ex.ticket_type,
                status_class = status_color,
                status = ex.status,
                url = ex.url,
                date_html = ex.request_date.as_ref().map(|d| format!(r#"<div class="pt-3 border-t border-zinc-800 flex items-center gap-2">
                    <svg class="w-3 h-3 text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                    <span class="text-xs text-zinc-500">{}</span>
                </div>"#, d)).unwrap_or_default()
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 80% 80%, #FF671F 0%, transparent 40%);"></div><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-4"><span class="bg-[#FF671F] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RESULTADOS</span></div><h2 class="text-4xl font-black mb-8 uppercase tracking-tight">{title}</h2><div class="grid grid-cols-2 gap-6 flex-grow">{examples}</div></div></div>{footer}</div></div>"#,
            title = t.get("examples_takedowns_title"),
            examples = examples_html,
            footer = footer_dark(13, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "takedown_examples".into(),
            html,
        }]
    }
}

pub struct PocExamplesSlidePlugin;

impl SlidePlugin for PocExamplesSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.poc_examples"
    }
    fn name(&self) -> &'static str {
        "PoC Evidence Examples"
    }
    fn priority(&self) -> i32 {
        20
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.poc_examples.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Calculate metrics for executive summary
        let total_examples = data.poc_examples.len();
        let total_available = data.total_threats.max(total_examples as u64) as usize;
        let high_risk_count = data
            .poc_examples
            .iter()
            .filter(|ex| ex.risk_score.unwrap_or(0.0) >= 0.6)
            .count();
        let high_risk_pct = if total_examples > 0 {
            (high_risk_count * 100) / total_examples
        } else {
            0
        };

        // Average detection time
        let detection_times: Vec<i64> = data
            .poc_examples
            .iter()
            .filter_map(|ex| ex.detection_minutes)
            .collect();
        let avg_detection_mins = if !detection_times.is_empty() {
            detection_times.iter().sum::<i64>() / detection_times.len() as i64
        } else {
            4
        }; // Default fallback

        // Take up to 4 examples with executive-focused card design
        let examples_html: String = data.poc_examples.iter().take(4).map(|ex| {
            // Image or placeholder
            let img_html = ex.screenshot_url.as_ref().map(|url| {
                format!(r#"<div class="relative overflow-hidden rounded-t-lg"><img src="{}" class="w-full h-28 object-cover" alt="screenshot"/><div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent"></div></div>"#, url)
            }).unwrap_or_else(|| r#"<div class="w-full h-28 bg-gradient-to-br from-zinc-800 to-zinc-900 rounded-t-lg flex items-center justify-center"><svg class="w-10 h-10 text-zinc-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg></div>"#.to_string());
            
            // Risk badge with visual indicator (🔴/🟠/🟡)
            let risk = ex.risk_score.unwrap_or(0.5);
            let (risk_emoji, risk_key, risk_color) = if risk >= 0.7 {
                ("🔴", "evidence_risk_high", "bg-red-500/20 text-red-400 border-red-500/40")
            } else if risk >= 0.4 {
                ("🟠", "evidence_risk_medium", "bg-orange-500/20 text-orange-400 border-orange-500/40")
            } else {
                ("🟡", "evidence_risk_low", "bg-yellow-500/20 text-yellow-400 border-yellow-500/40")
            };
            let risk_label = t.get(risk_key);
            
            // Detection time badge
            let detection_mins = ex.detection_minutes.unwrap_or(4);
            let detection_badge = format!("<{} min", detection_mins);
            
            // Impact indicator based on has_login_form
            let impact_html = if ex.has_login_form.unwrap_or(false) {
                format!(r#"<div class="flex items-center gap-1 mt-1"><svg class="w-3 h-3 text-red-400" fill="currentColor" viewBox="0 0 20 20"><path d="M10 2a5 5 0 00-5 5v2a2 2 0 00-2 2v5a2 2 0 002 2h10a2 2 0 002-2v-5a2 2 0 00-2-2H7V7a3 3 0 016 0v2h2V7a5 5 0 00-5-5z"></path></svg><span class="text-xs text-red-400">{}</span></div>"#, t.get("evidence_credential_capture"))
            } else {
                "".to_string()
            };
            
            format!(
                r#"<div class="bg-zinc-900 rounded-lg border border-zinc-800 hover:border-orange-500/50 transition-all overflow-hidden">{img}
<div class="p-3">
    <div class="flex items-center justify-between mb-1">
        <span class="font-semibold text-white text-sm truncate">{ticket}</span>
        <span class="text-xs px-2 py-0.5 rounded border {risk_class}">{emoji} {risk_label}</span>
    </div>
    <div class="flex items-center gap-2 text-xs text-zinc-500 mb-1">
        <span class="px-1.5 py-0.5 bg-zinc-800 rounded">{type_}</span>
        <span class="text-emerald-400 font-medium">{detection}</span>
    </div>
    <p class="text-xs text-zinc-500 truncate" title="{url}">{url}</p>
    {impact}
</div></div>"#,
                img = img_html,
                ticket = ex.ticket_key,
                risk_class = risk_color,
                emoji = risk_emoji,
                risk_label = risk_label,
                type_ = ex.evidence_type,
                detection = detection_badge,
                url = ex.reference_url,
                impact = impact_html,
            )
        }).collect();

        let no_data_html = if data.poc_examples.is_empty() {
            format!(
                r#"<div class="col-span-2 flex items-center justify-center h-full text-zinc-500 text-lg">{}</div>"#,
                t.get("example_no_data")
            )
        } else {
            String::new()
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
<div class="absolute inset-0 opacity-20" style="background-image: radial-gradient(circle at 80% 20%, #FF671F 0%, transparent 30%);"></div>
<div class="relative flex-grow h-full overflow-hidden z-10"><div class="h-full flex flex-col">
<div class="flex items-start justify-between mb-3">
    <div>
        <span class="bg-gradient-to-r from-orange-600 to-orange-500 text-white px-4 py-1 text-sm font-bold tracking-wider uppercase">EVIDENCIA</span>
        <h2 class="text-3xl font-black mt-2 tracking-tight">{title}</h2>
    </div>
    <!-- Context Panel -->
    <div class="bg-zinc-900/70 p-3 rounded-xl border border-zinc-800 max-w-sm">
        <div class="flex items-center gap-2 mb-1">
            <svg class="w-4 h-4 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            <h3 class="text-sm font-semibold text-orange-400">Para el decisor</h3>
        </div>
        <p class="text-zinc-400 text-xs leading-relaxed">Capturas <strong class="text-white">reales</strong> de amenazas detectadas. Los badges muestran nivel de riesgo y tiempo de detección.</p>
    </div>
</div>

<!-- Stats Bar -->
<div class="flex gap-4 mb-3">
    <div class="flex items-center gap-2 bg-zinc-900/50 px-3 py-2 rounded-lg border border-zinc-800">
        <span class="text-xl font-bold text-orange-400">{shown}</span>
        <span class="text-xs text-zinc-400">/ {total} {stats_shown}</span>
    </div>
    <div class="flex items-center gap-2 bg-zinc-900/50 px-3 py-2 rounded-lg border border-red-800/30">
        <span class="text-xl font-bold text-red-400">{high_risk_pct}%</span>
        <span class="text-xs text-zinc-400">{stats_high}</span>
    </div>
    <div class="flex items-center gap-2 bg-zinc-900/50 px-3 py-2 rounded-lg border border-emerald-800/30">
        <span class="text-xl font-bold text-emerald-400">&lt;{avg_time}</span>
        <span class="text-xs text-zinc-400">{stats_det}</span>
    </div>
</div>

<div class="grid grid-cols-2 gap-3 flex-grow">{examples}{no_data}</div>
</div></div>
{footer}
</div></div>"#,
            title = t.get("examples_poc_title"),
            shown = total_examples.min(4),
            total = total_available,
            stats_shown = t.get("evidence_stats_shown"),
            high_risk_pct = high_risk_pct,
            stats_high = t.get("evidence_stats_high_risk"),
            avg_time = avg_detection_mins,
            stats_det = t.get("evidence_stats_detection"),
            examples = examples_html,
            no_data = no_data_html,
            footer = footer_dark(14, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "poc_examples".into(),
            html,
        }]
    }
}
