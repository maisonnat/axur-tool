//! ROI (Return on Investment) Slide Plugin
//!
//! Displays operational impact and efficiency metrics.

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

        // Format hours saved
        let (hours_display, hours_unit) = if metrics.hours_saved_total >= 8.0 {
            (
                format!("{:.0}", metrics.person_days_saved),
                t.get("op_unit_person_days"),
            )
        } else {
            (
                format!("{:.1}", metrics.hours_saved_total),
                t.get("op_unit_hours"),
            )
        };

        // Format analysts equivalent
        let analysts_display = if metrics.analysts_equivalent_monthly >= 1.0 {
            format!("{:.1}", metrics.analysts_equivalent_monthly)
        } else {
            format!("{:.0}%", metrics.analysts_equivalent_monthly * 100.0)
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white"><div class="flex-grow h-full overflow-hidden"><div class="h-full flex flex-col"><div class="mb-8"><span class="bg-orange-600 px-4 py-1 text-sm font-semibold">{badge}</span><h2 class="text-4xl font-bold mt-4">{title}</h2></div><div class="grid grid-cols-3 gap-8 flex-grow"><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg></div><h3 class="text-2xl font-bold mb-2">{eff_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">{hours} <span class="text-base font-normal text-zinc-400">{hours_unit}</span></p><p class="text-zinc-400 text-sm leading-relaxed">{eff_desc}</p><div class="mt-4 text-xs text-zinc-500"><p>• {lbl_validation}: {val_hours:.0}h</p><p>• {lbl_monitoring}: {cred_hours:.0}h</p><p>• {lbl_takedowns}: {td_hours:.0}h</p></div></div><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m.94 3.198l.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0112 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 016 18.719m12 0a5.971 5.971 0 00-.941-3.197m0 0A5.995 5.995 0 0012 12.75a5.995 5.995 0 00-5.058 2.772m0 0a3 3 0 00-4.681 2.72 8.986 8.986 0 003.74.477m.94-3.197a5.971 5.971 0 00-.94 3.197M15 6.75a3 3 0 11-6 0 3 3 0 016 0zm6 3a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0zm-13.5 0a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z"></path></svg></div><h3 class="text-2xl font-bold mb-2">{team_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">{analysts}</p><p class="text-zinc-400 text-sm leading-relaxed">{team_desc}</p><div class="mt-4"><div class="flex items-center gap-2 text-xs text-zinc-500"><span class="w-3 h-3 rounded-full bg-green-500"></span><span>{tickets} {lbl_tickets}</span></div><div class="flex items-center gap-2 text-xs text-zinc-500 mt-1"><span class="w-3 h-3 rounded-full bg-blue-500"></span><span>{creds} {lbl_creds}</span></div></div></div><div class="bg-zinc-900 border border-zinc-800 p-8 rounded-xl flex flex-col hover:border-orange-500/50 transition-colors"><div class="bg-orange-600/20 p-4 rounded-full w-16 h-16 flex items-center justify-center mb-6"><svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-8 h-8 text-orange-500"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"></path></svg></div><h3 class="text-2xl font-bold mb-2">{resp_title}</h3><p class="text-4xl font-bold text-orange-500 mb-4">180x</p><p class="text-zinc-400 text-sm leading-relaxed">{resp_desc}</p><div class="mt-4 space-y-2"><div class="flex justify-between text-xs"><span class="text-zinc-500">{lbl_success}</span><span class="text-green-400 font-bold">{success_rate:.1}%</span></div><div class="flex justify-between text-xs"><span class="text-zinc-500">{lbl_td_done}</span><span class="text-white font-bold">{takedowns}</span></div></div></div></div></div></div>{footer}</div></div>"#,
            badge = t.get("op_badge"),
            title = t.get("roi_title"),
            eff_title = t.get("op_time_saved_title"),
            hours = hours_display,
            hours_unit = hours_unit,
            eff_desc = t.get("op_time_saved_desc"),
            lbl_validation = t.get("op_breakdown_validation"),
            val_hours = metrics.hours_saved_validation,
            lbl_monitoring = t.get("op_breakdown_monitoring"),
            cred_hours = metrics.hours_saved_credentials,
            lbl_takedowns = t.get("op_breakdown_takedowns"),
            td_hours = metrics.hours_saved_takedowns,
            team_title = t.get("op_capacity_title"),
            analysts = analysts_display,
            team_desc = t.get("op_capacity_desc"),
            tickets = data.total_tickets,
            lbl_tickets = t.get("op_tickets_processed"),
            creds = data.credentials_total,
            lbl_creds = t.get("op_credentials_monitored"),
            resp_title = t.get("op_response_title"),
            resp_desc = t.get("op_response_desc"),
            lbl_success = t.get("op_success_rate"),
            success_rate = data.takedown_success_rate,
            lbl_td_done = t.get("op_takedowns_completed"),
            takedowns = data.takedown_resolved,
            footer = footer_dark(12, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "roi".into(),
            html,
        }]
    }
}
