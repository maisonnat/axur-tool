//! Cover Slide Plugin — ACT 1: The Ordinary World
//!
//! Narrative Role: This is the OPENING. First impression, brand authority,
//! and professional trust established in 7 seconds.
//!
//! Persuasion: Authority (brand power) + Unity (shared identity with client)
//! Design: Full-bleed dark, split layout, prominent brand + client identity

use super::theme;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Base64 encoded cover image (person with orange smoke)
const COVER_IMAGE_BASE64: &str = include_str!("../../../assets/cover_image_base64.txt");

/// Plugin that generates the cover slide
pub struct CoverSlidePlugin;

impl SlidePlugin for CoverSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.cover"
    }
    fn name(&self) -> &'static str {
        "Cover Slide"
    }
    fn priority(&self) -> i32 {
        100
    } // First slide — AUTHORITY anchor

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // UNITY: Show the client prominently — they are the hero
        let partner_html = data.partner_name.as_ref().map(|p| format!(
            r##"<div class="mt-8 border-l-2 border-[#FF671F] pl-4"><p class="text-[#FF671F] font-semibold uppercase tracking-widest text-xs mb-1">{}</p><p class="text-2xl font-bold text-white">{}</p></div>"##,
            t.get("label_partner"), p
        )).unwrap_or_default();

        let title = if data.is_dynamic_window {
            t.get("cover_title_dynamic")
        } else {
            t.get("cover_title_static")
        };

        // Format dates for "Analysis Period"
        // start_date and end_date are already strings (YYYY-MM-DD)
        let date_range = format!("{} — {}", data.start_date, data.end_date);

        let html = format!(
            r##"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex shadow-2xl mb-8 relative bg-black text-white overflow-hidden">
                
                <!-- BACKGROUND: Base64 Image + Animated Overlay -->
                <!-- Image with slow zoom effect -->
                <div class="absolute inset-0 z-0 overflow-hidden">
                    <img src="data:image/png;base64,{image}" alt="Cover" class="absolute inset-0 w-full h-full object-cover object-center scale-110" style="filter: contrast(1.2) brightness(0.6); animation: scale-in 20s ease-out forwards;"/>
                    
                    <!-- Animated Gradient Overlay -->
                    <div class="absolute inset-0 bg-gradient-to-r from-black via-black/60 to-transparent z-10"></div>
                    <div class="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent z-10"></div>
                    
                    <!-- Floating Orbs for depth -->
                    <div class="bg-orb-orange top-1/4 right-1/4 w-[600px] h-[600px] opacity-20 animate-[pulse-orange_10s_ease-in-out_infinite] z-10"></div>
                </div>

                <!-- CONTENT PANEL (Left) -->
                <div class="w-7/12 h-full flex flex-col justify-between p-16 z-20 relative">
                    
                    <!-- TOP: TLP Badge + Brand -->
                    <div class="flex items-start justify-between">
                         <!-- TLP Badge (AUTHORITY: Pulse animation) -->
                        <div>
                            <div class="inline-flex items-center gap-3 px-5 py-2.5 bg-[#FF671F] text-white rounded-r-full shadow-[0_0_20px_rgba(255,103,31,0.4)] animate-[pulse-orange_3s_ease-in-out_infinite]">
                                <span class="w-2 h-2 rounded-full bg-white animate-pulse"></span>
                                <span class="font-bold text-lg tracking-widest">{tlp_lbl}{tlp}</span>
                            </div>
                            <p class="mt-3 text-[10px] text-zinc-400 max-w-xs uppercase tracking-widest pl-1">{tlp_desc}</p>
                        </div>
                        
                        <!-- Header Logo -->
                        <div class="opacity-80 scale-90 origin-top-right">
                            {logo}
                        </div>
                    </div>
                  
                    <!-- CENTER: Hero Title -->
                    <div class="flex-grow flex flex-col justify-center mt-10">
                        <!-- Date Range -->
                        <p class="text-zinc-400 font-mono text-sm tracking-widest mb-6 opacity-80 border-b border-white/10 pb-2 inline-block w-fit">
                            Analyzed Period: <span class="text-white">{dates}</span>
                        </p>

                        <h1 class="text-7xl font-black leading-[0.9] display-text uppercase text-white tracking-tight drop-shadow-2xl">
                            {title}
                        </h1>
                        
                        <!-- Accent Line -->
                        <div class="accent-line w-32 mt-8 mb-8"></div>
                    </div>
                  
                    <!-- BOTTOM: Client + Partner -->
                    <div class="mt-auto">
                        <div>
                            <p class="text-[#FF671F] font-semibold uppercase tracking-widest text-xs mb-2 flex items-center gap-2">
                                <span class="w-8 h-[1px] bg-[#FF671F]"></span>
                                {company_lbl}
                            </p>
                            <p class="text-4xl font-bold display-text text-white tracking-wide">{company}</p>
                        </div>
                        {partner}
                    </div>
                </div>
                
                <!-- RIGHT PANEL: Visual Anchor -->
                <div class="w-5/12 h-full z-20 relative flex flex-col justify-end p-16">
                     <!-- Tech decorative elements -->
                     <svg class="absolute top-10 right-10 w-32 h-32 opacity-20 animate-[spin_20s_linear_infinite]" viewBox="0 0 100 100">
                        <circle cx="50" cy="50" r="48" stroke="white" stroke-width="1" fill="none" stroke-dasharray="10 5"/>
                        <circle cx="50" cy="50" r="30" stroke="#FF671F" stroke-width="1" fill="none"/>
                     </svg>
                </div>
                
            </div></div>"##,
            tlp_lbl = t.get("label_tlp"),
            tlp = data.tlp_level,
            tlp_desc = t.get("label_tlp_desc"),
            company_lbl = t.get("label_company"),
            company = data.company_name,
            partner = partner_html,
            logo = theme::axur_logo_styled("lg"),
            image = COVER_IMAGE_BASE64.trim(),
            title = title,
            dates = date_range,
        );

        vec![SlideOutput {
            id: "cover".into(),
            html,
        }]
    }
}
