//! Takedowns Slide Plugin
//!
//! Displays takedown statistics and status with Axur.com dark theme aesthetics.

use super::helpers::footer_dark;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the takedowns results slide
pub struct TakedownsSlidePlugin;

impl SlidePlugin for TakedownsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.takedowns"
    }
    fn name(&self) -> &'static str {
        "Takedowns Results"
    }
    fn priority(&self) -> i32 {
        70
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        let data = ctx.data;
        data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved
            > 0
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let total_takedowns = data.takedown_resolved
            + data.takedown_pending
            + data.takedown_aborted
            + data.takedown_unresolved;

        // Calculate percentages for status bars
        let resolved_pct = if total_takedowns > 0 {
            (data.takedown_resolved as f64 / total_takedowns as f64) * 100.0
        } else {
            0.0
        };
        let pending_pct = if total_takedowns > 0 {
            (data.takedown_pending as f64 / total_takedowns as f64) * 100.0
        } else {
            0.0
        };
        let aborted_pct = if total_takedowns > 0 {
            (data.takedown_aborted as f64 / total_takedowns as f64) * 100.0
        } else {
            0.0
        };
        let unresolved_pct = if total_takedowns > 0 {
            (data.takedown_unresolved as f64 / total_takedowns as f64) * 100.0
        } else {
            0.0
        };

        let html = format!(
            r#"<div class="relative group">
                <div class="printable-slide aspect-[16/9] w-full flex flex-col shadow-lg mb-8 relative bg-[#121212] text-white overflow-hidden">
                    <!-- Wireframe background -->
                    <div class="absolute inset-0 wireframe-bg opacity-30"></div>
                    
                    <!-- Content -->
                    <div class="relative z-10 h-full flex flex-col p-14">
                        <!-- Header -->
                        <div class="mb-6">
                            <span class="bg-[#FF5824] text-white px-5 py-2 text-xs font-bold tracking-wider uppercase">
                                RESULTADOS
                            </span>
                            <h2 class="text-4xl font-bold mt-4">{title}</h2>
                        </div>
                        
                        <!-- Main Grid -->
                        <div class="grid grid-cols-12 gap-6 flex-grow">
                            <!-- Left: Key Stats -->
                            <div class="col-span-5 grid grid-cols-2 gap-4">
                                <!-- Total Takedowns -->
                                <div class="col-span-2 bg-zinc-900 border border-zinc-800 rounded-xl p-6 hover:border-[#FF5824]/50 transition-colors">
                                    <div class="text-6xl font-black text-[#FF5824] glow-orange-text">{total}</div>
                                    <div class="text-sm text-zinc-400 uppercase tracking-wider mt-2">{lbl_req}</div>
                                </div>
                                
                                <!-- Success Rate -->
                                <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-5 hover:border-green-500/50 transition-colors">
                                    <div class="text-4xl font-bold text-green-400">{rate:.1}%</div>
                                    <div class="text-xs text-zinc-500 uppercase tracking-wider mt-1">{lbl_rate}</div>
                                </div>
                                
                                <!-- Median Notify -->
                                <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-5 hover:border-blue-500/50 transition-colors">
                                    <div class="text-3xl font-bold text-blue-400">{notify}</div>
                                    <div class="text-xs text-zinc-500 uppercase tracking-wider mt-1">{lbl_notify}</div>
                                </div>
                                
                                <!-- Uptime -->
                                <div class="col-span-2 bg-zinc-900 border border-zinc-800 rounded-xl p-5 hover:border-purple-500/50 transition-colors">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <div class="text-3xl font-bold text-purple-400">{uptime}</div>
                                            <div class="text-xs text-zinc-500 uppercase tracking-wider mt-1">{lbl_uptime}</div>
                                        </div>
                                        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" class="w-10 h-10 text-purple-400/30">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                    </div>
                                </div>
                            </div>
                            
                            <!-- Right: Status Breakdown -->
                            <div class="col-span-7 bg-zinc-900/50 border border-zinc-800 rounded-xl p-6">
                                <h3 class="text-lg font-semibold text-white mb-6">{status_title}</h3>
                                
                                <!-- Status Bars -->
                                <div class="space-y-4">
                                    <!-- Resolved -->
                                    <div class="flex items-center gap-4">
                                        <span class="pill-badge-ghost text-xs w-28 text-center">{lbl_solved}</span>
                                        <div class="flex-grow h-6 bg-zinc-800 rounded-full overflow-hidden">
                                            <div class="h-full bg-gradient-to-r from-green-500 to-green-400 rounded-full" style="width: {resolved_pct}%;"></div>
                                        </div>
                                        <span class="w-16 text-right font-bold text-green-400">{resolved}</span>
                                    </div>
                                    
                                    <!-- Pending -->
                                    <div class="flex items-center gap-4">
                                        <span class="pill-badge-ghost text-xs w-28 text-center">{lbl_pending}</span>
                                        <div class="flex-grow h-6 bg-zinc-800 rounded-full overflow-hidden">
                                            <div class="h-full bg-gradient-to-r from-amber-500 to-amber-400 rounded-full" style="width: {pending_pct}%;"></div>
                                        </div>
                                        <span class="w-16 text-right font-bold text-amber-400">{pending}</span>
                                    </div>
                                    
                                    <!-- Interrupted -->
                                    <div class="flex items-center gap-4">
                                        <span class="pill-badge-ghost text-xs w-28 text-center">{lbl_aborted}</span>
                                        <div class="flex-grow h-6 bg-zinc-800 rounded-full overflow-hidden">
                                            <div class="h-full bg-gradient-to-r from-red-500 to-red-400 rounded-full" style="width: {aborted_pct}%;"></div>
                                        </div>
                                        <span class="w-16 text-right font-bold text-red-400">{aborted}</span>
                                    </div>
                                    
                                    <!-- Unresolved -->
                                    <div class="flex items-center gap-4">
                                        <span class="pill-badge-ghost text-xs w-28 text-center">{lbl_unresolved}</span>
                                        <div class="flex-grow h-6 bg-zinc-800 rounded-full overflow-hidden">
                                            <div class="h-full bg-gradient-to-r from-zinc-500 to-zinc-400 rounded-full" style="width: {unresolved_pct}%;"></div>
                                        </div>
                                        <span class="w-16 text-right font-bold text-zinc-400">{unresolved}</span>
                                    </div>
                                </div>
                                
                                <!-- Summary pill -->
                                <div class="mt-6 pt-6 border-t border-zinc-800 flex justify-center">
                                    <span class="pill-badge text-sm">
                                        {rate:.1}% Success Rate
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Footer -->
                    {footer}
                </div>
            </div>"#,
            title = t.get("takedowns_title"),
            total = total_takedowns,
            lbl_req = t.get("takedowns_requested"),
            rate = data.takedown_success_rate,
            lbl_rate = t.get("takedowns_success_rate"),
            notify = data.takedown_median_time_to_notify,
            lbl_notify = t.get("takedowns_median_notify"),
            uptime = data.takedown_median_uptime,
            lbl_uptime = t.get("takedowns_median_uptime"),
            status_title = t.get("takedowns_status_title"),
            lbl_solved = t.get("takedowns_solved"),
            lbl_pending = t.get("takedowns_in_progress"),
            lbl_aborted = t.get("takedowns_interrupted"),
            lbl_unresolved = t.get("takedowns_not_solved"),
            resolved = data.takedown_resolved,
            pending = data.takedown_pending,
            aborted = data.takedown_aborted,
            unresolved = data.takedown_unresolved,
            resolved_pct = resolved_pct,
            pending_pct = pending_pct,
            aborted_pct = aborted_pct,
            unresolved_pct = unresolved_pct,
            footer = footer_dark(11, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "takedowns".into(),
            html,
        }]
    }
}
