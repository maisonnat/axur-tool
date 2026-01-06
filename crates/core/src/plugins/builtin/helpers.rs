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
pub fn footer_dark(page: u32, footer_text: &str) -> String {
    format!(
        r##"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-[#FF4B00] text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-400"><span>{}</span><span class="ml-4">{}</span></div></footer>"##,
        footer_text, page
    )
}

/// Light footer (for light backgrounds)
pub fn footer_light(page: u32, footer_text: &str) -> String {
    format!(
        r##"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-zinc-800 h-5"><span class="text-[#FF4B00] text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-600"><span>{}</span><span class="ml-4">{}</span></div></footer>"##,
        footer_text, page
    )
}

/// Axur logo SVG
pub fn axur_logo() -> &'static str {
    r##"<div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-[#FF4B00] text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div>"##
}

/// Geometric pattern: Tech grid with subtle glow nodes
/// Based on Axur.com aesthetic - minimal grid with orange accent dots
pub fn geometric_pattern() -> &'static str {
    r##"<div class="absolute inset-0 overflow-hidden">
        <!-- Dark gradient background -->
        <div class="absolute inset-0 bg-gradient-to-br from-zinc-900 via-zinc-950 to-black"></div>
        
        <!-- Tech grid pattern -->
        <svg class="absolute inset-0 w-full h-full opacity-30" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <pattern id="techGrid" width="40" height="40" patternUnits="userSpaceOnUse">
                    <path d="M 40 0 L 0 0 0 40" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="0.5"/>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#techGrid)"/>
        </svg>
        
        <!-- Glowing orange accent nodes -->
        <div class="absolute top-1/4 right-1/4 w-2 h-2 bg-orange-500 rounded-full" style="box-shadow: 0 0 20px rgba(255,75,0,0.8), 0 0 40px rgba(255,75,0,0.4)"></div>
        <div class="absolute top-2/3 right-1/3 w-1.5 h-1.5 bg-orange-400 rounded-full" style="box-shadow: 0 0 15px rgba(255,75,0,0.6)"></div>
        <div class="absolute bottom-1/4 right-1/2 w-1 h-1 bg-orange-300 rounded-full" style="box-shadow: 0 0 10px rgba(255,75,0,0.5)"></div>
        
        <!-- Large diagonal slash accent -->
        <svg class="absolute inset-0 w-full h-full" viewBox="0 0 100 100" preserveAspectRatio="none">
            <line x1="70" y1="0" x2="100" y2="100" stroke="rgba(255,75,0,0.15)" stroke-width="8"/>
            <line x1="80" y1="0" x2="110" y2="100" stroke="rgba(255,75,0,0.08)" stroke-width="6"/>
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
            <path d="M80 0 L120 0 L40 200 L0 200 Z" fill="rgba(255,75,0,0.9)"/>
            <!-- White slash -->
            <path d="M55 0 L85 0 L5 200 L-25 200 Z" fill="rgba(255,255,255,0.85)"/>
            <!-- Gray slash behind -->
            <path d="M30 0 L55 0 L-25 200 L-50 200 Z" fill="rgba(128,128,128,0.5)"/>
        </svg>
        
        <!-- Subtle grid overlay -->
        <svg class="absolute inset-0 w-full h-full opacity-10">
            <defs>
                <pattern id="coverGrid" width="60" height="60" patternUnits="userSpaceOnUse">
                    <circle cx="30" cy="30" r="1" fill="rgba(255,75,0,0.5)"/>
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
