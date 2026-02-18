//! Shared helpers for builtin plugins

/// Formats a number with thousand separators
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result.chars().rev().collect()
}

/// Dark footer (for dark backgrounds)
pub fn footer_dark(page: usize, footer_text: &str) -> String {
    format!(
        r##"<footer class="absolute bottom-12 left-12 right-12 flex justify-between items-center text-[10px] text-zinc-600 font-mono tracking-widest uppercase opacity-60">
            <div class="footer-brand flex items-center font-black tracking-wider select-none h-5">
                <span class="text-[#FF671F] text-2xl -mr-1">///</span>
                <span class="text-xl text-white">AXUR</span>
                <span class="mx-3 text-zinc-700">|</span>
                <span class="uppercase tracking-widest text-[10px]">Digital Experiences Made Safe</span>
            </div>
            <div class="flex items-center gap-6">
                <span class="uppercase tracking-widest">{text}</span>
                <span class="font-mono text-zinc-600">0{page}</span>
            </div>
        </footer>"##,
        text = footer_text,
        page = page
    )
}

/// Light footer (for light backgrounds)
pub fn footer_light(page: u32, footer_text: &str) -> String {
    format!(
        r##"<footer class="footer-premium absolute bottom-8 left-14 right-14 flex justify-between items-center text-zinc-500 text-xs tracking-wide border-t border-zinc-200 pt-3">
            <div class="footer-brand flex items-center font-black tracking-wider select-none h-5">
                <span class="text-[#FF671F] text-2xl -mr-1">///</span>
                <span class="text-xl text-zinc-900">AXUR</span>
                <span class="mx-3 text-zinc-300">|</span>
                <span class="uppercase tracking-widest text-[10px] text-zinc-400">Digital Experiences Made Safe</span>
            </div>
            <div class="flex items-center gap-6">
                <span class="uppercase tracking-widest">{text}</span>
                <span class="font-mono text-zinc-400">0{page}</span>
            </div>
        </footer>"##,
        text = footer_text,
        page = page
    )
}

/// Axur logo SVG
pub fn axur_logo() -> &'static str {
    r##"<div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-[#FF671F] text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div>"##
}

/// Geometric pattern: Tech grid with subtle glow nodes
/// Based on Axur.com aesthetic - minimal grid with orange accent dots
pub fn geometric_pattern() -> &'static str {
    r##"<div class="absolute inset-0 overflow-hidden pointer-events-none">
        <div class="bg-orb-orange top-0 left-1/4 w-[500px] h-[500px] opacity-20 animate-[pulse-orange_8s_ease-in-out_infinite]"></div>
        <div class="bg-orb-purple bottom-0 right-1/4 w-[400px] h-[400px] opacity-10"></div>

        <div class="absolute top-1/4 left-1/5 w-1 h-1 bg-white rounded-full opacity-20 animate-[float_6s_ease-in-out_infinite]"></div>
        <div class="absolute top-2/3 right-1/4 w-1.5 h-1.5 bg-orange-500 rounded-full opacity-40 animate-[float_8s_ease-in-out_infinite_1s]"></div>
        <div class="absolute bottom-1/5 left-1/3 w-1 h-1 bg-purple-400 rounded-full opacity-30 animate-[float_7s_ease-in-out_infinite_2s]"></div>

        <svg class="absolute inset-0 w-full h-full opacity-[0.15]" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <pattern id="techGrid" width="60" height="60" patternUnits="userSpaceOnUse">
                    <path d="M 60 0 L 0 0 0 60" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="0.5"/>
                    <rect x="0" y="0" width="1" height="1" fill="rgba(255,255,255,0.2)"/>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#techGrid)"/>
        </svg>
        
        <div class="absolute top-1/4 right-1/4 w-2 h-2 bg-orange-500 rounded-full" style="box-shadow: 0 0 20px rgba(255,103,31,0.8), 0 0 40px rgba(255,103,31,0.4)"></div>
        <div class="absolute top-2/3 right-1/3 w-1.5 h-1.5 bg-orange-400 rounded-full" style="box-shadow: 0 0 15px rgba(255,103,31,0.6)"></div>
        
        <svg class="absolute inset-0 w-full h-full opacity-20" viewBox="0 0 100 100" preserveAspectRatio="none">
            <line x1="60" y1="0" x2="100" y2="100" stroke="url(#slashGrad)" stroke-width="1"/>
            <defs>
                <linearGradient id="slashGrad" x1="0" y1="0" x2="1" y2="1">
                    <stop offset="0%" stop-color="transparent"/>
                    <stop offset="50%" stop-color="rgba(255,103,31,0.3)"/>
                    <stop offset="100%" stop-color="transparent"/>
                </linearGradient>
            </defs>
        </svg>
    </div>"##
}

/// Cover pattern: Portal-style gradient with Axur signature elements
/// Uses the /// slash motif and cyber gradient aesthetic
pub fn cover_pattern() -> &'static str {
    r##"<div class="absolute inset-0 overflow-hidden">
        <!-- Base gradient - cyber portal feel -->
        <div class="absolute inset-0 bg-gradient-to-r from-transparent via-zinc-900/50 to-black"></div>
        
        <!-- Radial glow from right -->
        <div class="absolute -right-20 top-1/2 -translate-y-1/2 w-96 h-96 bg-orange-500 rounded-full opacity-20 blur-3xl"></div>
        
        <!-- Triple slash signature motif -->
        <svg class="absolute right-8 top-1/2 -translate-y-1/2 h-3/4" viewBox="0 0 120 200" fill="none">
            <!-- Main orange slash -->
            <path d="M80 0 L120 0 L40 200 L0 200 Z" fill="rgba(255,103,31,0.9)"/>
            <!-- White slash -->
            <path d="M55 0 L85 0 L5 200 L-25 200 Z" fill="rgba(255,255,255,0.85)"/>
            <!-- Gray slash behind -->
            <path d="M30 0 L55 0 L-25 200 L-50 200 Z" fill="rgba(128,128,128,0.5)"/>
        </svg>
        
        <!-- Subtle grid overlay -->
        <svg class="absolute inset-0 w-full h-full opacity-10">
            <defs>
                <pattern id="coverGrid" width="60" height="60" patternUnits="userSpaceOnUse">
                    <circle cx="30" cy="30" r="1" fill="rgba(255,103,31,0.5)"/>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#coverGrid)"/>
        </svg>
    </div>"##
}

/// Format a chrono DateTime to a display string
pub fn format_date(date: &Option<chrono::DateTime<chrono::Utc>>) -> String {
    date.map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "N/A".to_string())
}
