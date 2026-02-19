//! Virality Slide Plugin — ACT 3: The Abyss (Spread)
//!
//! Narrative Role: Show HOW FAR the threat has spread. After seeing the exposure,
//! the hero now sees the threats propagating across platforms. Maximum urgency.
//!
//! Persuasion: SPIN Implication (cost of viral spread) + Cognitive Emptying (dynamic insight)
//! Design: Triple Stat layout with dynamic executive summary card

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct ViralitySlidePlugin;

impl SlidePlugin for ViralitySlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.virality"
    }
    fn name(&self) -> &'static str {
        "Virality"
    }
    fn priority(&self) -> i32 {
        87
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        let ti = &ctx.data.threat_intelligence;
        ti.data_available && (ti.chat_group_shares > 0 || ti.social_media_mentions > 0)
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let ti = &ctx.data.threat_intelligence;
        let t = ctx.translations;

        let platforms_html: String = ti
            .platforms_detected
            .iter()
            .map(|p| {
                format!(
                    r#"<span class="px-3 py-1 bg-zinc-800 rounded-full text-sm">{}</span>"#,
                    p
                )
            })
            .collect::<Vec<_>>()
            .join("");

        // Generate Dynamic Insights (Storytelling)
        // This is where we translate raw data into business intelligence
        let total_mentions = ti.chat_group_shares + ti.social_media_mentions + ti.dark_web_mentions;

        let (insight_title, insight_text, insight_color) = if total_mentions == 0 {
            ("Baja Actividad Viral", "No se detectaron menciones significativas de sus activos en canales públicos o privados de alto riesgo.", "text-zinc-500")
        } else if ti.chat_group_shares > ti.social_media_mentions * 2 {
            ("Foco en Canales Privados", "La actividad se concentra en grupos de chat (Telegram/WhatsApp), lo que sugiere una campaña de fraude dirigida y coordinada fuera del radar público.", "text-purple-400")
        } else if ti.social_media_mentions > ti.chat_group_shares {
            ("Alta Exposición Pública", "La amenaza tiene alta visibilidad en redes sociales, lo que aumenta el riesgo de daño reputacional y requiere contención mediática inmediata.", "text-blue-400")
        } else {
            ("Actividad Viral Diversificada", "Se detectó presencia simultánea en múltiples canales, indicando una campaña compleja con vectores tanto públicos como privados.", "text-orange-400")
        };

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 mb-8 relative text-white overflow-hidden">
                <!-- Background -->
                {bg_pattern}

                <div class="flex-grow flex flex-col z-10 relative">
                    <!-- Header: section_header_premium -->
                    {header}
                    <!-- Header insight card -->
                    <div class="flex items-center justify-between mb-8 -mt-6">
                         <div></div>
                         <!-- RECIPROCITY: Dynamic insight card — free analysis -->
                         <div class="glass-panel p-6 max-w-md border-l-4 border-l-orange-500">
                            <h4 class="text-sm font-bold {insight_color} uppercase tracking-wider mb-2 flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                {insight_title}
                            </h4>
                            <p class="text-zinc-300 text-sm leading-relaxed">
                                {insight_text}
                            </p>
                         </div>
                    </div>

                    <!-- Metrics Grid -->
                    <div class="grid grid-cols-3 gap-8 mb-12 flex-grow items-center">
                        <!-- Chat Groups -->
                        <div class="glass-panel p-8 flex flex-col items-center justify-center text-center h-full hover:bg-white/5 hover:scale-[1.02] transition-all duration-300 group/stat">
                            <div class="bg-purple-500/10 p-4 rounded-full mb-4 group-hover/stat:bg-purple-500/20 transition-colors">
                                <svg class="w-8 h-8 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path></svg>
                            </div>
                            <p class="text-5xl font-light text-white mb-2 display-text">{chat}</p>
                            <p class="text-zinc-400 text-sm uppercase tracking-widest">{lbl_chat}</p>
                            <p class="text-zinc-600 text-xs mt-1">Telegram / WhatsApp</p>
                        </div>
                        
                        <!-- Social Media -->
                        <div class="glass-panel p-8 flex flex-col items-center justify-center text-center h-full hover:bg-white/5 hover:scale-[1.02] transition-all duration-300 group/stat">
                            <div class="bg-blue-500/10 p-4 rounded-full mb-4 group-hover/stat:bg-blue-500/20 transition-colors">
                                <svg class="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 11.5V14m0-2.5v-6a1.5 1.5 0 113 0m-3 6a1.5 1.5 0 00-3 0v2a7.5 7.5 0 0015 0v-5a1.5 1.5 0 00-3 0m-6-3V11m0-5.5v-1a1.5 1.5 0 013 0v1m0 0V11m0-5.5a1.5 1.5 0 013 0v3m0 0V11"></path></svg>
                            </div>
                            <p class="text-5xl font-light text-white mb-2 display-text">{social}</p>
                            <p class="text-zinc-400 text-sm uppercase tracking-widest">{lbl_social}</p>
                            <p class="text-zinc-600 text-xs mt-1">Redes Públicas</p>
                        </div>
                        
                        <!-- Dark Web -->
                        <div class="glass-panel p-8 flex flex-col items-center justify-center text-center h-full hover:bg-white/5 hover:scale-[1.02] transition-all duration-300 group/stat">
                            <div class="bg-orange-500/10 p-4 rounded-full mb-4 group-hover/stat:bg-orange-500/20 transition-colors">
                                <svg class="w-8 h-8 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                            </div>
                            <p class="text-5xl font-light text-white mb-2 display-text">{dark}</p>
                            <p class="text-zinc-400 text-sm uppercase tracking-widest">{lbl_dark}</p>
                            <p class="text-zinc-600 text-xs mt-1">Foros / Markets</p>
                        </div>
                    </div>
                    
                <div class="flex flex-wrap gap-2 justify-center opacity-80">
                        {platforms}
                    </div>

                </div>
                
                {footer}
            </div></div>"#,
            bg_pattern = crate::plugins::builtin::helpers::geometric_pattern(),
            header = crate::plugins::builtin::theme::section_header_premium(
                "PROPAGACIÓN DE AMENAZAS",
                &t.get("virality_title"),
                None
            ),
            chat = ti.chat_group_shares,
            lbl_chat = t.get("virality_chat"),
            social = ti.social_media_mentions,
            lbl_social = t.get("virality_social"),
            dark = ti.dark_web_mentions,
            lbl_dark = t.get("virality_dark"),
            platforms = platforms_html,
            insight_text = insight_text,
            insight_color = insight_color,
            insight_title = insight_title,
            footer = footer_dark(12, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "virality".into(),
            html,
        }]
    }
}
