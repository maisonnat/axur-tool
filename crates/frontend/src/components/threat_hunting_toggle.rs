use crate::components::*;
use leptos::*; // For icons or other components if needed

#[component]
pub fn ThreatHuntingToggle(
    include_threat_intel: RwSignal<bool>,
    debug_mode: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="mb-8 relative group cursor-pointer"
            on:click=move |_| include_threat_intel.update(|v| *v = !*v)
        >
            <div class=move || {
                if include_threat_intel.get() {
                    "absolute inset-0 bg-gradient-to-r from-brand-primary/20 to-brand-secondary/20 blur-xl transition-all duration-500 opacity-100"
                } else {
                    "absolute inset-0 bg-brand-primary/0 blur-xl transition-all duration-500 opacity-0"
                }
            }></div>

            <div class=move || {
                let base = "relative p-6 rounded-2xl border transition-all duration-300 overflow-hidden";
                if include_threat_intel.get() {
                    format!("{} bg-surface-elevated/90 border-brand-primary/50 shadow-[0_0_30px_rgba(255,103,31,0.15)]", base)
                } else {
                    format!("{} bg-surface-elevated/50 border-white/5 hover:border-white/10 hover:bg-surface-elevated/70", base)
                }
            }>
                <div class="flex items-center justify-between relative z-10">
                    <div class="flex items-center gap-4">
                        <div class=move || {
                            let base = "w-12 h-12 rounded-xl flex items-center justify-center transition-all duration-500";
                            if include_threat_intel.get() {
                                format!("{} bg-brand-primary text-white shadow-glow", base)
                            } else {
                                format!("{} bg-white/5 text-zinc-500 group-hover:bg-white/10 group-hover:text-zinc-300", base)
                            }
                        }>
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
                        </div>
                        <div>
                            <h3 class=move || {
                                if include_threat_intel.get() {
                                    "text-white font-bold font-display uppercase tracking-wider text-sm transition-colors"
                                } else {
                                    "text-zinc-400 font-bold font-display uppercase tracking-wider text-sm transition-colors group-hover:text-zinc-200"
                                }
                            }>
                                "Active Defense Protocol"
                            </h3>
                            <p class="text-xs text-zinc-500 font-mono mt-1">"DEEP MESH SCANNING"</p>
                        </div>
                    </div>

                    // Toggle Switch Visual
                    <div class="w-14 h-8 rounded-full bg-black/50 border border-white/10 relative p-1 transition-colors duration-300">
                        <div class=move || {
                            if include_threat_intel.get() {
                                "w-6 h-6 rounded-full bg-brand-primary shadow-glow translate-x-6 transition-transform duration-300"
                            } else {
                                "w-6 h-6 rounded-full bg-zinc-600 translate-x-0 transition-transform duration-300"
                            }
                        }></div>
                    </div>
                </div>

                // Expanded Details (Animated)
                <div class=move || {
                    if include_threat_intel.get() {
                        "mt-4 pt-4 border-t border-brand-primary/20 grid grid-cols-2 gap-2 transition-all duration-500 opacity-100 max-h-20"
                    } else {
                        "mt-0 pt-0 border-t-0 border-transparent grid grid-cols-2 gap-2 transition-all duration-500 opacity-0 max-h-0 overflow-hidden"
                    }
                }>
                    <div class="flex items-center gap-2 text-xs text-amber-500 font-mono">
                        <span class="w-1.5 h-1.5 rounded-full bg-amber-500 animate-pulse"></span>
                        "DARK_WEB_FORUMS"
                    </div>
                    <div class="flex items-center gap-2 text-xs text-red-500 font-mono">
                        <span class="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse"></span>
                        "CREDENTIAL_LEAK"
                    </div>
                </div>
            </div>
        </div>

        // Credits warning inside Show
        <Show when=move || include_threat_intel.get()>
            <div class="mb-6 mt-3 flex items-center gap-2 text-amber-400/80 text-xs bg-amber-900/20 rounded px-3 py-2 border border-amber-500/20">
                <span class="text-sm">"⚡"</span>
                <p>
                    <span class="font-bold">"PREMIUM FEATURE:"</span>
                    " This operation consumes "
                    <span class="text-white font-bold">"50 Credits"</span>
                    " from your allocation."
                </p>
            </div>

            // Admin credits toggle
            <div class="mt-3 pt-3 border-t border-zinc-700/50">
                <label class="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        class="rounded border-zinc-700 bg-zinc-800 text-brand-primary focus:ring-brand-primary/50"
                        prop:checked=move || debug_mode.get()
                        on:change=move |ev| debug_mode.set(event_target_checked(&ev))
                    />
                    <span class="text-zinc-500 text-xs">"Admin Mode (Unlimited Credits)"</span>
                </label>
            </div>
        </Show>
    }
}
