//! Data Exposure Slide Plugin
//!
//! Combined view of code leaks and credential exposures.

use super::helpers::{footer_dark, format_number};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct DataExposureSlidePlugin;

impl SlidePlugin for DataExposureSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.data_exposure"
    }
    fn name(&self) -> &'static str {
        "Data Exposure"
    }
    fn priority(&self) -> i32 {
        85
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        ctx.data.secrets_total > 0 || ctx.data.credentials_total > 0
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-8 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden"><div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 70% 30%, #4f46e5 0%, transparent 20%), radial-gradient(circle at 30% 70%, #ea580c 0%, transparent 20%);"></div><div class="relative h-full flex flex-col z-10"><div class="mb-6 border-b border-zinc-800 pb-4"><h2 class="text-3xl font-bold text-white mb-2">{title}</h2><p class="text-lg text-zinc-400">Total external attack surface analysis</p></div><div class="grid grid-cols-2 gap-8 flex-grow"><div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col"><h3 class="text-xl font-semibold text-indigo-400 mb-6 flex items-center gap-2"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>{lbl_sub_code}</h3><div class="space-y-4 flex-grow"><div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg"><div><p class="text-3xl font-bold text-white">{secrets}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_secrets}</p></div></div><div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg"><div><p class="text-3xl font-bold text-white">{repos}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_repos}</p></div></div><div class="flex justify-between items-end p-4 bg-red-900/20 border border-red-500/20 rounded-lg"><div><p class="text-3xl font-bold text-red-400">{prod}</p><p class="text-xs text-red-300 uppercase tracking-wider">{lbl_prod}</p></div></div></div></div><div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col"><h3 class="text-xl font-semibold text-orange-400 mb-6 flex items-center gap-2"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path></svg>{lbl_sub_stealer}</h3><div class="space-y-4 flex-grow"><div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg"><div><p class="text-3xl font-bold text-white">{creds}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_creds}</p></div></div><div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg"><div><p class="text-3xl font-bold text-white">{hosts}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_hosts}</p></div></div><div class="flex justify-between items-end p-4 bg-orange-900/20 border border-orange-500/20 rounded-lg"><div><p class="text-3xl font-bold text-orange-400">{risk}</p><p class="text-xs text-orange-300 uppercase tracking-wider">{lbl_risk}</p></div></div></div></div></div></div></div>{footer}</div></div>"#,
            title = t.get("exposure_title"),
            lbl_sub_code = t.get("exposure_sub_code"),
            secrets = format_number(data.secrets_total),
            lbl_secrets = t.get("code_leak_box_secrets"),
            repos = format_number(data.unique_repos),
            lbl_repos = t.get("code_leak_box_repos"),
            prod = format_number(data.production_secrets),
            lbl_prod = t.get("code_leak_box_prod"),
            lbl_sub_stealer = t.get("exposure_sub_stealer"),
            creds = format_number(data.credentials_total),
            lbl_creds = t.get("stealer_box_creds"),
            hosts = format_number(data.unique_hosts),
            lbl_hosts = t.get("stealer_box_hosts"),
            risk = format_number(data.high_risk_users),
            lbl_risk = t.get("stealer_box_high_risk"),
            footer = footer_dark(8, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "data_exposure".into(),
            html,
        }]
    }
}
