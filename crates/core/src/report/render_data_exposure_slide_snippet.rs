fn render_data_exposure_slide(data: &PocReportData, dict: &Box<dyn Dictionary>) -> String {
    // 1. Infostealer Critical Section
    let critical_html = if !data.critical_credentials.is_empty() {
        let count = data.critical_credentials.len();
        let examples: String = data.critical_credentials.iter().take(3).map(|c| {
            let user = c.user.as_deref().unwrap_or("unknown");
            let pass = c.password.as_deref().unwrap_or("***");
            let masked_pass = if pass.len() > 4 {
                format!("{}...{}", &pass[..2], &pass[pass.len()-2..])
            } else {
                "***".to_string()
            };
            
            format!(
                r#"<div class="font-mono text-xs text-red-200 bg-red-950/40 px-2 py-1.5 rounded border border-red-500/20 flex justify-between items-center mb-1">
                    <span class="truncate pr-2 max-w-[120px]">{}</span>
                    <span class="text-red-400 font-bold">{}</span>
                </div>"#,
                user, masked_pass
            )
        }).collect::<Vec<_>>().join("");

        format!(
            r#"<div class="mb-6 p-4 bg-red-900/10 border border-red-500/30 rounded-xl animate-pulse-slow">
                <div class="flex items-start gap-3">
                    <div class="p-2 bg-red-500/10 rounded-lg flex-shrink-0 text-red-500">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                    </div>
                    <div class="flex-grow">
                        <h3 class="text-sm font-bold text-red-500 mb-1">{title}</h3>
                        <p class="text-zinc-400 mb-2 text-xs">{desc}</p>
                        <div>{examples}</div>
                    </div>
                </div>
            </div>"#,
            title = dict.stealer_critical_title(),
            desc = dict.stealer_critical_desc(count),
            examples = examples
        )
    } else {
        String::new()
    };

    format!(
        r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-8 md:p-12 shadow-lg mb-8 relative bg-zinc-950 text-white overflow-hidden">
            <!-- Background Decoration -->
            <div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle at 70% 30%, #4f46e5 0%, transparent 20%), radial-gradient(circle at 30% 70%, #ea580c 0%, transparent 20%);"></div>

            <div class="relative h-full flex flex-col z-10">
                <div class="mb-6 border-b border-zinc-800 pb-4">
                    <h2 class="text-3xl font-bold text-white mb-2">{title}</h2>
                    <p class="text-lg text-zinc-400">Total external attack surface analysis</p>
                </div>

                <div class="grid grid-cols-2 gap-8 flex-grow">
                    <!-- Left: Code Leaks (Purple/Zinc) -->
                     <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col">
                        <h3 class="text-xl font-semibold text-indigo-400 mb-6 flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                            {lbl_sub_code}
                        </h3>
                        <div class="space-y-4 flex-grow">
                            <div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg">
                                <div><p class="text-3xl font-bold text-white">{secrets}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_secrets}</p></div>
                            </div>
                            <div class="flex justify-between items-end p-4 bg-zinc-800/50 rounded-lg">
                                <div><p class="text-3xl font-bold text-white">{repos}</p><p class="text-xs text-zinc-500 uppercase tracking-wider">{lbl_repos}</p></div>
                            </div>
                            <div class="flex justify-between items-end p-4 bg-red-900/20 border border-red-500/20 rounded-lg">
                                <div><p class="text-3xl font-bold text-red-400">{prod}</p><p class="text-xs text-red-300 uppercase tracking-wider">{lbl_prod}</p></div>
                            </div>
                        </div>
                        <div class="mt-4 text-xs text-zinc-500 italic border-t border-zinc-800 pt-3">{action_code}</div>
                    </div>

                    <!-- Right: Infostealer (Orange/Red) -->
                    <div class="bg-zinc-900/50 p-6 rounded-xl border border-zinc-800 backdrop-blur-sm flex flex-col">
                        <h3 class="text-xl font-semibold text-orange-400 mb-6 flex items-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path></svg>
                            {lbl_sub_stealer}
                        </h3>
                        
                        {critical_alert}

                        <div class="grid grid-cols-2 gap-4 mb-4">
                            <div class="p-3 bg-zinc-800/50 rounded-lg">
                                <p class="text-2xl font-bold text-white">{creds}</p>
                                <p class="text-[10px] text-zinc-500 uppercase">{lbl_creds}</p>
                            </div>
                            <div class="p-3 bg-zinc-800/50 rounded-lg">
                                <p class="text-2xl font-bold text-white">{hosts}</p>
                                <p class="text-[10px] text-zinc-500 uppercase">{lbl_hosts}</p>
                            </div>
                        </div>
                        
                        <div class="mt-auto text-xs text-zinc-500 italic border-t border-zinc-800 pt-3">{action_stealer}</div>
                    </div>
                </div>
            </div>
            {footer}
        </div></div>"#,
        title = dict.exposure_title(),
        lbl_sub_code = dict.exposure_sub_code(),
        lbl_sub_stealer = dict.exposure_sub_stealer(),
        
        // Code Leak Data
        secrets = format_number(data.secrets_total),
        lbl_secrets = dict.code_leak_box_secrets(),
        repos = format_number(data.unique_repos),
        lbl_repos = dict.code_leak_box_repos(),
        prod = format_number(data.production_secrets),
        lbl_prod = dict.code_leak_box_prod(),
        action_code = dict.code_leak_action(),

        // Stealer Data
        critical_alert = critical_html,
        creds = format_number(data.credentials_total),
        lbl_creds = dict.stealer_box_creds(),
        hosts = format_number(data.unique_hosts),
        lbl_hosts = dict.stealer_box_hosts(),
        action_stealer = dict.stealer_action(), // Reusing existing action text

        footer = footer_dark(8, dict),
    )
}
