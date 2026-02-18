//! Closing Slide Plugin — ACT 5: The Return (Transformation)
//!
//! Narrative Role: The hero is empowered. This is the COMMITMENT moment.
//! After seeing the crisis and the guide's resolution, the next step must be
//! frictionless. Two clear CTAs, no cognitive overload.
//!
//! Persuasion: Commitment (low-friction next step) + Scarcity (time-limited)
//! Design: Centered layout, two CTA cards, minimal elements for max clarity

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the closing/CTA slide
pub struct ClosingSlidePlugin;

impl SlidePlugin for ClosingSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.closing"
    }
    fn name(&self) -> &'static str {
        "Closing CTA"
    }
    fn priority(&self) -> i32 {
        10
    } // Last slide

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let t = ctx.translations;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative bg-[#09090b] text-white overflow-hidden">
                <!-- Background -->
                {bg_pattern}
                <div class="absolute inset-0 bg-gradient-to-t from-orange-500/5 to-transparent pointer-events-none"></div>

                <div class="flex-grow h-full flex flex-col justify-center items-center text-center relative z-10">
                    
                    <h2 class="text-7xl font-black mb-6 display-text tracking-tight uppercase text-transparent bg-clip-text bg-gradient-to-br from-white to-zinc-400">{title}</h2>
                    <p class="text-2xl text-zinc-400 mb-10 max-w-3xl font-light leading-relaxed">{subtitle}</p>
                    
                    <!-- COMMITMENT: Low-friction badge to prime decision -->
                    <div class="inline-flex items-center gap-3 bg-emerald-500/10 border border-emerald-500/20 px-5 py-2.5 rounded-full mb-16 backdrop-blur-md">
                        <span class="relative flex h-3 w-3">
                          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                          <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
                        </span>
                        <span class="text-sm text-emerald-300 font-medium tracking-wide">Su protección está lista para activarse</span>
                    </div>
                    
                    <!-- FRICTION MINIMIZATION: Two clear, simple CTAs -->
                    <div class="grid grid-cols-2 gap-8 max-w-4xl w-full">
                        <!-- CTA 1: QUICK WIN (lowest friction) -->
                        <div class="glass-panel-premium p-8 flex flex-col items-center hover:scale-[1.02] transition-all duration-500 group/card cursor-pointer ring-1 ring-white/5 hover:ring-orange-500/40">
                            <div class="bg-gradient-to-br from-orange-500 to-red-500 p-4 rounded-xl mb-6 shadow-lg shadow-orange-500/20 group-hover/card:shadow-orange-500/40 transition-all">
                                <svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" class="w-8 h-8 text-white"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3 text-white group-hover/card:text-orange-400 transition-colors uppercase tracking-wide">{cta_activate}</h3>
                            <p class="text-zinc-400 text-sm leading-relaxed">{cta_activate_desc}</p>
                        </div>
                        
                        <!-- CTA 2: DEEPER ENGAGEMENT -->
                        <div class="glass-panel p-8 flex flex-col items-center hover:scale-[1.02] transition-all duration-500 group/card cursor-pointer border border-white/5 hover:border-white/10 hover:bg-white/5">
                            <div class="bg-zinc-800 p-4 rounded-xl mb-6 ring-1 ring-white/10 group-hover/card:bg-zinc-700 transition-all">
                                <svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" class="w-8 h-8 text-zinc-300 group-hover/card:text-white"><path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3 text-white group-hover/card:text-zinc-200 transition-colors uppercase tracking-wide">{cta_meet}</h3>
                            <p class="text-zinc-500 text-sm leading-relaxed">{cta_meet_desc}</p>
                        </div>
                    </div>
                </div>
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            title = t.get("closing_title"),
            subtitle = t.get("closing_subtitle"),
            cta_activate = t.get("closing_cta_activate"),
            cta_activate_desc = t.get("closing_cta_activate_desc"),
            cta_meet = t.get("closing_cta_meet"),
            cta_meet_desc = t.get("closing_cta_meet_desc"),
            footer = footer_dark(15, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "closing".into(),
            html,
        }]
    }
}
