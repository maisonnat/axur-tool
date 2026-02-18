//! PoC Data Slide Plugin
//!
//! Displays monitored assets and period information.
//! Redesigned with compact duration banner and full-width asset grid.

use super::helpers::footer_dark;
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

        // Compact duration badge
        let duration_html = if data.is_dynamic_window {
            format!(
                r#"<div class="flex items-center gap-3 px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg">
                    <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5 text-[#FF671F]">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <span class="text-zinc-300 text-sm">{} · <span class="text-[#FF671F] font-semibold">Monitoreo Continuo</span></span>
                </div>"#,
                t.get("poc_period_dynamic_title")
            )
        } else {
            format!(
                r#"<div class="flex items-center gap-4 px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg">
                    <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-5 h-5 text-[#FF671F]">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                    </svg>
                    <span class="text-zinc-400 text-sm">{}</span>
                    <span class="text-white font-semibold">{} → {}</span>
                </div>"#,
                t.get("poc_period_static_title"),
                &data.start_date,
                &data.end_date
            )
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<!-- Background pattern -->
<div class="absolute inset-0" style="background-image:radial-gradient(circle at 25px 25px,rgba(255,75,0,0.08) 2%,transparent 0%),radial-gradient(circle at 75px 75px,rgba(255,75,0,0.08) 2%,transparent 0%);background-size:100px 100px"></div>

<div class="relative h-full flex flex-col">
  <!-- Header Row with Title + Duration -->
  <div class="flex items-start justify-between mb-8">
    <div>
      <span class="bg-[#FF671F] px-4 py-1 text-sm font-bold tracking-wider uppercase">{title_scope}</span>
      <h2 class="text-4xl font-black mt-3 uppercase tracking-tight">{title_assets}</h2>
    </div>
    {duration}
  </div>
  
  <!-- Full-width Asset Grid -->
  <div class="flex-grow">
    <div class="grid grid-cols-4 gap-6 h-full">
      <!-- Brands -->
      <div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 flex flex-col justify-center items-center text-center hover:border-[#FF671F]/50 transition-colors">
        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-12 h-12 text-[#FF671F] mb-4">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.286z"/>
        </svg>
        <p class="text-5xl font-black text-white" style="text-shadow: 0 0 20px rgba(255,75,0,0.3)">{brands}</p>
        <p class="text-sm text-zinc-400 mt-2 uppercase tracking-wider">{brands_label}</p>
      </div>
      
      <!-- Executives -->
      <div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 flex flex-col justify-center items-center text-center hover:border-[#FF671F]/50 transition-colors">
        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-12 h-12 text-[#FF671F] mb-4">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"/>
        </svg>
        <p class="text-5xl font-black text-white" style="text-shadow: 0 0 20px rgba(255,75,0,0.3)">{exec}</p>
        <p class="text-sm text-zinc-400 mt-2 uppercase tracking-wider">{lbl_exec}</p>
      </div>
      
      <!-- IPs -->
      <div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 flex flex-col justify-center items-center text-center hover:border-[#FF671F]/50 transition-colors">
        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-12 h-12 text-[#FF671F] mb-4">
          <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 14.25h13.5m-13.5 0a3 3 0 01-3-3m3 3a3 3 0 100 6h13.5a3 3 0 100-6m-16.5-3a3 3 0 013-3h13.5a3 3 0 013 3m-19.5 0a4.5 4.5 0 01.9-2.7L5.737 5.1a3.375 3.375 0 012.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 01.9 2.7m0 0a3 3 0 01-3 3m0 3h.008v.008h-.008v-.008zm0-6h.008v.008h-.008v-.008zm-3 6h.008v.008h-.008v-.008zm0-6h.008v.008h-.008v-.008z"/>
        </svg>
        <p class="text-5xl font-black text-white" style="text-shadow: 0 0 20px rgba(255,75,0,0.3)">{ips}</p>
        <p class="text-sm text-zinc-400 mt-2 uppercase tracking-wider">{lbl_ips}</p>
      </div>
      
      <!-- Domains -->
      <div class="bg-zinc-900/80 p-8 rounded-xl border border-zinc-800 flex flex-col justify-center items-center text-center hover:border-[#FF671F]/50 transition-colors">
        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-12 h-12 text-[#FF671F] mb-4">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582"/>
        </svg>
        <p class="text-5xl font-black text-white" style="text-shadow: 0 0 20px rgba(255,75,0,0.3)">{domains}</p>
        <p class="text-sm text-zinc-400 mt-2 uppercase tracking-wider">{lbl_domains}</p>
      </div>
    </div>
  </div>
</div>
</div>
{footer}
</div></div>"#,
            title_scope = t.get("poc_scope_title"),
            title_assets = t.get("poc_assets_title"),
            brands = data.brands_count,
            brands_label = t.get("poc_label_brands"),
            exec = data.executives_count,
            lbl_exec = t.get("poc_label_executives"),
            ips = data.ips_count,
            lbl_ips = t.get("poc_label_ips"),
            domains = data.domains_count,
            lbl_domains = t.get("poc_label_domains"),
            duration = duration_html,
            footer = footer_dark(5, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "poc_data".into(),
            html,
        }]
    }
}
