//! Timeline Slide Plugin
//!
//! Shows story tickets on a timeline.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

pub struct TimelineSlidePlugin;

impl SlidePlugin for TimelineSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.timeline"
    }
    fn name(&self) -> &'static str {
        "Timeline"
    }
    fn priority(&self) -> i32 {
        89
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        !ctx.data.story_tickets.is_empty()
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total_events = data.story_tickets.len();

        // Count by status for insights
        let closed_count = data
            .story_tickets
            .iter()
            .filter(|t| t.status == "closed")
            .count();
        let incident_count = data
            .story_tickets
            .iter()
            .filter(|t| t.status == "incident")
            .count();
        let _active_count = total_events.saturating_sub(closed_count + incident_count);

        // Find most common threat type
        let mut threat_counts: std::collections::HashMap<&str, usize> =
            std::collections::HashMap::new();
        for ticket in &data.story_tickets {
            *threat_counts
                .entry(ticket.threat_type.as_str())
                .or_insert(0) += 1;
        }
        let top_threat = threat_counts
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, _)| *k)
            .unwrap_or("N/A");

        let items_html: String = data.story_tickets.iter().take(6).map(|ticket| {
            let (status_color, status_icon) = match ticket.status.as_str() {
                "closed" => ("bg-green-500", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>"#),
                "incident" => ("bg-red-500", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>"#),
                _ => ("bg-orange-500", r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>"#),
            };
            format!(
                r#"<div class="flex items-start gap-3 mb-3">
                    <div class="flex flex-col items-center">
                        <div class="w-8 h-8 rounded-full {color} flex items-center justify-center">
                            <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">{icon}</svg>
                        </div>
                        <div class="w-0.5 h-8 bg-zinc-700 mt-1"></div>
                    </div>
                    <div class="flex-grow bg-zinc-900/70 p-3 rounded-lg border border-zinc-800">
                        <div class="flex items-center justify-between">
                            <p class="font-bold text-white text-sm">{key}</p>
                            <span class="text-xs text-zinc-500 px-2 py-0.5 bg-zinc-800 rounded">{threat_type}</span>
                        </div>
                        <p class="text-xs text-zinc-400 mt-1 truncate">{desc}</p>
                    </div>
                </div>"#,
                color = status_color,
                icon = status_icon,
                key = ticket.ticket_key,
                desc = ticket.description,
                threat_type = ticket.threat_type,
            )
        }).collect();

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
<div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 10% 90%, #FF4B00 0%, transparent 40%);"></div>
<div class="relative flex-grow h-full overflow-hidden z-10"><div class="h-full flex flex-col">
<div class="mb-4"><span class="bg-gradient-to-r from-orange-600 to-orange-500 px-4 py-1 text-sm font-bold tracking-wider uppercase">TIMELINE</span></div>
<h2 class="text-4xl font-black mb-6 tracking-tight">{title}</h2>

<div class="flex gap-8 flex-grow">
    <!-- Left: Context Panel -->
    <div class="w-2/5 flex flex-col gap-4">
        <div class="bg-zinc-900/70 p-5 rounded-xl border border-zinc-800">
            <div class="flex items-center gap-2 mb-3">
                <svg class="w-5 h-5 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <h3 class="text-lg font-semibold text-orange-400">¿Qué muestra esta línea de tiempo?</h3>
            </div>
            <p class="text-zinc-400 text-sm leading-relaxed">Esta cronología muestra los <strong class="text-white">eventos de seguridad más recientes</strong> detectados contra su marca. Cada evento representa una amenaza identificada y su estado de resolución actual.</p>
        </div>
        
        <!-- Stats Cards -->
        <div class="grid grid-cols-3 gap-3">
            <div class="bg-zinc-900/50 p-3 rounded-lg border border-zinc-800 text-center">
                <div class="text-2xl font-bold text-orange-400">{total}</div>
                <div class="text-xs text-zinc-500">Eventos</div>
            </div>
            <div class="bg-zinc-900/50 p-3 rounded-lg border border-green-800/50 text-center">
                <div class="text-2xl font-bold text-green-400">{closed}</div>
                <div class="text-xs text-zinc-500">Resueltos</div>
            </div>
            <div class="bg-zinc-900/50 p-3 rounded-lg border border-red-800/50 text-center">
                <div class="text-2xl font-bold text-red-400">{incidents}</div>
                <div class="text-xs text-zinc-500">Incidentes</div>
            </div>
        </div>
        
        <!-- Key Insight -->
        <div class="bg-orange-500/10 p-4 rounded-lg border border-orange-500/20 mt-auto">
            <div class="flex items-center gap-2 mb-2">
                <svg class="w-4 h-4 text-orange-400" fill="currentColor" viewBox="0 0 20 20"><path d="M10 2a8 8 0 100 16 8 8 0 000-16zm1 11a1 1 0 11-2 0 1 1 0 012 0zm0-3a1 1 0 01-2 0V7a1 1 0 112 0v3z"></path></svg>
                <span class="text-sm font-semibold text-orange-400">Patrón Detectado</span>
            </div>
            <p class="text-white text-sm">El vector más común es <strong class="text-orange-400">{top_threat}</strong></p>
        </div>
    </div>
    
    <!-- Right: Timeline -->
    <div class="w-3/5 flex flex-col">
        <div class="bg-zinc-900/30 p-4 rounded-xl border border-zinc-800 flex-grow overflow-auto">
            {items}
        </div>
    </div>
</div>

</div></div>
{footer}
</div></div>"#,
            title = t.get("timeline_title"),
            total = total_events,
            closed = closed_count,
            incidents = incident_count,
            top_threat = top_threat,
            items = items_html,
            footer = footer_dark(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "timeline".into(),
            html,
        }]
    }
}
