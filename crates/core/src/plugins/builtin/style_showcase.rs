//! Style Showcase Slide
//!
//! Renders a slide containing all available theme components for visual verification.

use crate::plugins::builtin::{helpers, theme};
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct StyleShowcasePlugin;

impl SlidePlugin for StyleShowcasePlugin {
    fn id(&self) -> &'static str {
        "builtin.style_showcase"
    }
    fn name(&self) -> &'static str {
        "Style Showcase"
    }
    fn priority(&self) -> i32 {
        999
    } // High priority to appear early

    fn is_enabled(&self, _ctx: &PluginContext) -> bool {
        true // Always enabled for now, or could check a flag
    }

    fn generate_slides(&self, _ctx: &PluginContext) -> Vec<SlideOutput> {
        let html = format!(
            r#"<div class="printable-slide aspect-[16/9] w-full flex flex-col p-14 text-white relative overflow-hidden">
                <!-- Background -->
                {bg}

                <!-- Header -->
                {header}

                <!-- Grid Layout -->
                <div class="grid grid-cols-2 gap-8 flex-grow">
                    
                    <!-- Left Column: Metrics -->
                    <div class="flex flex-col gap-6">
                         <h3 class="text-xl font-bold text-zinc-500 uppercase tracking-wider mb-2">Metrics</h3>
                         <div class="grid grid-cols-2 gap-4">
                            {stat1}
                            {stat2}
                         </div>
                         {stat_large}
                         
                         <!-- progress bar -->
                         <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800 mt-2">
                            <div class="flex justify-between mb-2 text-sm">
                                <span class="text-zinc-400">System Load</span>
                                <span class="text-white font-bold">75%</span>
                            </div>
                            {progress}
                         </div>
                    </div>

                    <!-- Right Column: Components -->
                    <div class="flex flex-col gap-6">
                        <h3 class="text-xl font-bold text-zinc-500 uppercase tracking-wider mb-2">Components</h3>
                        
                        <!-- Badges -->
                        <div class="flex flex-wrap gap-2 p-4 bg-zinc-900 rounded-xl border border-zinc-800">
                            {badge1}
                            {badge2}
                            {tag1}
                            {tag2}
                        </div>

                        <!-- Feature Cards -->
                        <div class="grid grid-cols-1 gap-4">
                            {feat1}
                        </div>

                        <!-- Logo -->
                        <div class="mt-auto flex justify-end opacity-50">
                            {logo}
                        </div>
                    </div>
                </div>

                <!-- Footer -->
                {footer}
            </div>"#,
            bg = helpers::geometric_pattern(),
            header = theme::section_header("DESIGN SYSTEM", "Style Showcase"),
            // Metrics
            stat1 = theme::stat_card_glow("100%", "Coverage", true),
            stat2 = theme::stat_card_glow("0", "Errors", false),
            stat_large =
                theme::stat_card_large("A+", "Security Rating", Some("Top 1% of industry")),
            progress = theme::progress_bar(75.0, None),
            // Badges
            badge1 = theme::pill_badge("PILLED"),
            badge2 = theme::pill_badge_ghost("GHOST"),
            tag1 = theme::threat_tag("THREAT", true),
            tag2 = theme::threat_tag("NORMAL", false),
            // Features
            feat1 = theme::feature_card(
                r#"<svg class="w-8 h-8 text-[#FF671F]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>"#,
                "Feature Card",
                "Demonstrates the standard feature card layout with icon circle and description."
            ),
            logo = theme::axur_logo_styled("xl"),
            footer = helpers::footer_dark(1, "Confidential - Axur Design System"),
        );
        vec![SlideOutput {
            id: "style_showcase".into(),
            html,
        }]
    }
}
