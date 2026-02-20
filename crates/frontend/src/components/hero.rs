use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class="relative mb-12 py-12 overflow-hidden rounded-3xl border border-white/5 bg-surface-base/50 backdrop-blur-sm group">
            <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
            <div class="absolute -top-24 -right-24 w-96 h-96 bg-brand-primary/20 blur-[100px] rounded-full animate-pulse-slow"></div>
            <div class="absolute -bottom-24 -left-24 w-64 h-64 bg-brand-secondary/10 blur-[80px] rounded-full"></div>

            <div class="relative z-10 px-8 flex flex-col lg:flex-row items-center justify-between gap-8">
                // Left side: Text & Titles
                <div class="flex flex-col items-start gap-4 lg:w-2/3">
                    <div class="flex items-center gap-3">
                        <span class="px-3 py-1 bg-brand-primary/10 border border-brand-primary/30 text-brand-primary text-xs font-bold font-mono tracking-widest uppercase rounded">
                            "System Ready"
                        </span>
                        <div class="h-px w-12 bg-gradient-to-r from-brand-primary/50 to-transparent"></div>
                    </div>

                    <h1 class="text-6xl md:text-7xl font-display font-bold text-transparent bg-clip-text bg-gradient-to-r from-white via-white to-zinc-500 uppercase tracking-tighter drop-shadow-lg leading-tight">
                        "Threat" <br />
                        <span class="text-stroke-current text-transparent bg-clip-text bg-gradient-to-r from-brand-primary to-brand-secondary selection:text-white">"Intelligence"</span>
                    </h1>

                    <p class="max-w-2xl text-zinc-400 text-lg font-light border-l-2 border-brand-primary/50 pl-6 mt-4">
                        "Advanced cyber-surveillance telemetry. Generate exec-ready reports in seconds."
                    </p>
                </div>

                // Right side: Empty Chair Metric (Cognitive Framing)
                <div class="lg:w-1/3 w-full backdrop-blur-md bg-black/20 rounded-2xl p-6 border border-white/10 shadow-2xl relative overflow-hidden group-hover:border-brand-primary/30 transition-colors duration-700">
                    <div class="absolute top-0 right-0 w-32 h-32 bg-brand-primary/10 blur-3xl rounded-full"></div>
                    <div class="flex flex-col gap-2 relative z-10">
                        <span class="text-zinc-500 font-mono text-xs uppercase tracking-widest">"Global Impact Metric"</span>
                        <div class="flex items-end gap-2">
                            <span class="text-5xl font-black text-white group-hover:text-brand-primary transition-colors duration-500">"98.9"</span>
                            <span class="text-2xl font-bold text-brand-primary mb-1">"%"</span>
                        </div>
                        <div class="h-px w-full bg-gradient-to-r from-white/20 to-transparent my-2"></div>
                        <span class="text-sm text-zinc-300 font-medium">"Takedown Success Rate"</span>
                        <p class="text-xs text-zinc-500 tracking-wide mt-1">
                            "Across 1,400+ protected global brands."
                        </p>
                    </div>
                </div>
            </div>

            // Decorative scanline
            <div class="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-brand-primary/5 to-transparent h-[20%] w-full animate-scanline opacity-30"></div>
        </div>
    }
}
