use crate::api::TemplateListItem;
use crate::components::*;
use crate::AppState;
use leptos::*;

#[component]
pub fn ReportGeneratorForm(
    #[prop(into)] ui_dict: Signal<crate::i18n::UiDict>,
    loading_tenants: ReadSignal<bool>,
    #[prop(into)] tenant_options: Signal<Vec<(String, String)>>,
    selected_tenant: RwSignal<String>,
    from_date: RwSignal<String>,
    to_date: RwSignal<String>,
    story_tag: RwSignal<String>,
    language: RwSignal<String>,
    user_templates: ReadSignal<Vec<TemplateListItem>>,
    selected_template: RwSignal<Option<String>>,
    include_threat_intel: RwSignal<bool>,
    debug_mode: RwSignal<bool>,
    use_plugins: RwSignal<bool>,
    plugin_theme: RwSignal<String>,
    disabled_slides: RwSignal<Vec<String>>,
    use_mock_data: RwSignal<bool>,
    generating: ReadSignal<bool>,
    generate_action: Action<(), ()>,
) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    // Helper to create a slide toggle checkbox
    let slide_toggle = {
        let disabled_slides = disabled_slides;
        move |plugin_id: &'static str, label: &'static str| {
            let is_enabled = move || !disabled_slides.get().contains(&plugin_id.to_string());

            let toggle = move |_| {
                disabled_slides.update(|list| {
                    if list.contains(&plugin_id.to_string()) {
                        list.retain(|id| id != plugin_id);
                    } else {
                        list.push(plugin_id.to_string());
                    }
                });
            };

            view! {
                <label class="flex items-center gap-2 text-zinc-400 cursor-pointer hover:text-zinc-300 transition-colors">
                    <input
                        type="checkbox"
                        class="w-3.5 h-3.5 rounded border-white/20 bg-surface-base/50 text-brand-primary focus:ring-brand-primary focus:ring-offset-zinc-900 cursor-pointer"
                        prop:checked=is_enabled
                        on:change=toggle
                    />
                    {label}
                </label>
            }
        }
    };

    let try_submit = create_rw_signal(false); // Track if user tried to submit to show validation errors

    view! {
        // Main Container - Interactive Glass Panel with animated gradient background
        <div class="glass-panel rounded-3xl p-1 overflow-hidden group shadow-[0_0_50px_rgba(0,0,0,0.5)] relative">
            // Animated background layer
            <div class="absolute inset-0 bg-gradient-to-br from-brand-primary/10 via-brand-secondary/5 to-purple-900/10 opacity-50 group-hover:opacity-100 transition-opacity duration-700 pointer-events-none"></div>

            <div class="bg-surface-base/95 p-10 rounded-[1.4rem] space-y-10 backdrop-blur-3xl relative z-10 border border-white/5">

                // --- BLOCK 1: TARGET DEFINITION ---
                <div class="space-y-5">
                    <div class="flex items-center gap-3 pb-2 border-b border-white/5">
                        <div class="w-8 h-8 rounded-xl bg-brand-primary/10 flex items-center justify-center text-brand-primary border border-brand-primary/30 shadow-[0_0_10px_rgba(255,103,49,0.2)] animate-[pulse_3s_ease-in-out_infinite]">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
                        </div>
                        <h2 class="text-sm font-black uppercase tracking-widest text-[#FFF1E5] font-mono drop-shadow-[0_0_8px_rgba(255,103,49,0.3)]">
                            "1. Target Definition"
                        </h2>
                    </div>

                    // Section border wrapper with subtle pulsing glow
                    <div class="space-y-4 bg-surface-base/40 p-5 rounded-[1.2rem] border border-brand-primary/10 shadow-[0_0_15px_rgba(255,103,49,0.05)] transition-all duration-1000">
                        <Show
                            when=move || !loading_tenants.get()
                            fallback=|| view! {
                                <div class="space-y-3">
                                    <div class="h-3 w-16 bg-white/5 rounded animate-pulse"></div>
                                    <div class="w-full h-12 bg-white/5 rounded-xl animate-pulse"></div>
                                </div>
                            }
                        >
                            <div class="group/field relative transition-all duration-300 rounded-xl"
                                class=("border-red-500", move || try_submit.get() && selected_tenant.get().is_empty())
                                class=("shadow-[0_0_15px_rgba(239,68,68,0.2)]", move || try_submit.get() && selected_tenant.get().is_empty())
                            >
                                <Combobox
                                    label=Signal::derive(move || ui_dict.get().tenant_label.to_string())
                                    options=tenant_options
                                    selected=selected_tenant
                                    placeholder=Signal::derive(move || ui_dict.get().tenant_placeholder.to_string())
                                />
                                // Subtle glow effect on the field
                                <div class="absolute -inset-0.5 bg-gradient-to-r from-brand-primary to-orange-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                            </div>
                            // Validation message
                            <Show when=move || try_submit.get() && selected_tenant.get().is_empty()>
                                <p class="text-red-400 text-xs mt-1 ml-1 animate-pulse">Debes seleccionar un Tenant</p>
                            </Show>
                        </Show>

                        <div class="grid grid-cols-2 gap-4">
                            <div class="space-y-1.5 group/field relative">
                                <label class="text-xs font-bold text-zinc-300 uppercase tracking-widest ml-1">{move || ui_dict.get().from_date}</label>
                                <input
                                    type="date"
                                    class="w-full bg-zinc-900/50 border border-white/10 hover:border-brand-primary/50 focus:border-brand-primary text-zinc-100 rounded-xl py-3 px-4 outline-none focus:shadow-[0_0_15px_rgba(255,103,49,0.3)] focus:bg-zinc-800 transition-all duration-300 placeholder-zinc-400 shadow-inner [color-scheme:dark]"
                                    prop:value=move || from_date.get()
                                    on:input=move |ev| from_date.set(event_target_value(&ev))
                                />
                                <div class="absolute -inset-0.5 bg-gradient-to-r from-brand-primary to-orange-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                            </div>

                            <div class="space-y-1.5 group/field relative">
                                <label class="text-xs font-bold text-zinc-300 uppercase tracking-widest ml-1">{move || ui_dict.get().to_date}</label>
                                <input
                                    type="date"
                                    class="w-full bg-zinc-900/50 border border-white/10 hover:border-brand-primary/50 focus:border-brand-primary text-zinc-100 rounded-xl py-3 px-4 outline-none focus:shadow-[0_0_15px_rgba(255,103,49,0.3)] focus:bg-zinc-800 transition-all duration-300 placeholder-zinc-400 shadow-inner [color-scheme:dark]"
                                    prop:value=move || to_date.get()
                                    on:input=move |ev| to_date.set(event_target_value(&ev))
                                />
                                <div class="absolute -inset-0.5 bg-gradient-to-r from-brand-primary to-orange-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                            </div>
                        </div>

                        <div class="space-y-1.5 group/field relative">
                            <label class="text-xs font-bold text-zinc-300 uppercase tracking-widest ml-1">"Story Tag (Optional)"</label>
                            <div class="relative group/input">
                                <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-zinc-400 group-focus-within/input:text-brand-primary transition-colors">
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path></svg>
                                </div>
                                <input
                                    type="text"
                                    class="w-full bg-zinc-900/50 border border-white/10 hover:border-brand-primary/50 focus:border-brand-primary text-zinc-100 rounded-xl py-3 pl-11 pr-4 outline-none focus:shadow-[0_0_15px_rgba(255,103,49,0.3)] focus:bg-zinc-800 transition-all duration-300 placeholder-zinc-500 font-mono text-sm shadow-inner"
                                    placeholder="tag:campaign-name"
                                    prop:value=move || story_tag.get()
                                    on:input=move |ev| story_tag.set(event_target_value(&ev))
                                />
                            </div>
                            <div class="absolute -inset-0.5 bg-gradient-to-r from-brand-primary to-orange-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                        </div>
                    </div>
                </div>

                // --- BLOCK 2: ENGINE CONFIGURATION ---
                <div class="space-y-5">
                    <div class="flex items-center gap-3 pb-2 border-b border-white/5">
                        <div class="w-8 h-8 rounded-xl bg-purple-500/10 flex items-center justify-center text-purple-400 border border-purple-500/30 shadow-[0_0_10px_rgba(168,85,247,0.2)] animate-[pulse_4s_ease-in-out_infinite_0.5s]">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                        </div>
                        <h2 class="text-sm font-black uppercase tracking-widest text-[#F3E8FF] font-mono drop-shadow-[0_0_8px_rgba(168,85,247,0.3)]">
                            "2. Engine Config"
                        </h2>
                    </div>

                    // Section border wrapper with subtle pulsing glow
                    <div class="space-y-4 bg-surface-base/40 p-5 rounded-[1.2rem] border border-purple-500/10 shadow-[0_0_15px_rgba(168,85,247,0.05)] transition-all duration-1000">
                        <div class="grid grid-cols-2 gap-4">
                            <div class="space-y-1.5 group/field relative">
                                <label class="text-xs font-bold text-zinc-300 uppercase tracking-widest ml-1">{move || ui_dict.get().language_label}</label>
                                <div class="relative">
                                    <select
                                        class="w-full bg-zinc-900/50 border border-white/10 hover:border-purple-500/50 focus:border-purple-500 text-zinc-100 rounded-xl py-3 px-4 outline-none cursor-pointer focus:shadow-[0_0_15px_rgba(168,85,247,0.3)] focus:bg-zinc-800 transition-all appearance-none shadow-inner [color-scheme:dark]"
                                        on:change=move |ev| language.set(event_target_value(&ev))
                                    >
                                        <option value="es" selected>"Español"</option>
                                        <option value="en">"English"</option>
                                        <option value="pt">"Português"</option>
                                    </select>
                                    <div class="absolute inset-y-0 right-0 flex items-center px-4 pointer-events-none text-zinc-500">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l4-4 4 4m0 6l-4 4-4-4"></path></svg>
                                    </div>
                                </div>
                                <div class="absolute -inset-0.5 bg-gradient-to-r from-purple-500 to-fuchsia-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                            </div>

                            <div class="space-y-1.5 group/field relative">
                                <label class="text-xs font-bold text-zinc-300 uppercase tracking-widest ml-1">"Template"</label>
                                <div class="relative flex items-center">
                                    <select
                                        class="w-full bg-zinc-900/50 border border-white/10 hover:border-purple-500/50 focus:border-purple-500 text-zinc-100 rounded-xl py-3 pl-4 pr-10 outline-none cursor-pointer focus:shadow-[0_0_15px_rgba(168,85,247,0.3)] focus:bg-zinc-800 transition-all appearance-none shadow-inner [color-scheme:dark]"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            if val == "default" {
                                                selected_template.set(None);
                                            } else {
                                                selected_template.set(Some(val));
                                            }
                                        }
                                    >
                                        <optgroup label="System Templates">
                                            <option value="default" selected>"⭐ Axur Default"</option>
                                            <option value="executive">"📊 Executive Summary"</option>
                                            <option value="technical">"🔧 Technical Deep Dive"</option>
                                            <option value="compliance">"📋 Compliance Report"</option>
                                        </optgroup>
                                        <optgroup label="My Templates">
                                            <Show when=move || !user_templates.get().is_empty()
                                                fallback=|| view! { <option disabled>"No templates yet"</option> }
                                            >
                                                <For
                                                    each=move || user_templates.get()
                                                    key=|t| t.id.clone()
                                                    children=move |t| view! {
                                                        <option value={t.id.clone()}>{t.name}</option>
                                                    }
                                                />
                                            </Show>
                                        </optgroup>
                                        <optgroup label="Options">
                                            <option value="custom">"✏️ Browse Custom..."</option>
                                        </optgroup>
                                    </select>
                                    <button
                                        on:click=move |_| state.current_page.set(crate::Page::Marketplace)
                                        class="absolute h-8 right-2 px-3 bg-zinc-800 hover:bg-purple-900/50 text-zinc-300 hover:text-white border border-white/10 hover:border-purple-500/80 hover:shadow-[0_0_10px_rgba(168,85,247,0.5)] rounded-lg transition-all flex items-center justify-center pointer-events-auto"
                                        title="Open Template Marketplace"
                                    >
                                        "🛒"
                                    </button>
                                </div>
                                <div class="absolute -inset-0.5 bg-gradient-to-r from-purple-500 to-fuchsia-400 rounded-xl opacity-0 group-focus-within/field:opacity-40 group-hover/field:opacity-20 transition duration-500 blur -z-10"></div>
                            </div>
                        </div>

                        // Threat Hunting Toggle
                        <ThreatHuntingToggle
                            include_threat_intel=include_threat_intel
                            debug_mode=debug_mode
                        />
                    </div>
                </div>

                // --- BLOCK 3: ADVANCED OPTIONS ---
                <div class="space-y-4 pt-2 group/advanced_container overflow-hidden">
                    <details class="group/advanced">
                        <summary class="flex items-center gap-2 cursor-pointer outline-none">
                            <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-white/10 flex items-center justify-center text-zinc-300 group-open/advanced:text-brand-primary group-open/advanced:border-brand-primary/50 group-open/advanced:shadow-[0_0_8px_rgba(255,103,49,0.3)] transition-all">
                                <svg class="w-3 h-3 transition-transform duration-300 group-open/advanced:rotate-90" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
                            </div>
                            <span class="text-xs font-bold uppercase tracking-widest text-[#FFF1E5] group-hover/advanced:text-white group-hover/advanced:drop-shadow-[0_0_5px_rgba(255,255,255,0.5)] transition-all">"Advanced Options"</span>
                        </summary>

                        <div class="mt-4 space-y-4 pt-2 group-open/advanced:animate-fade-in">
                            // Plugin System Toggle (Experimental)
                            <div class="p-5 rounded-2xl bg-gradient-to-br from-green-900/10 to-teal-900/10 border border-green-700/30 backdrop-blur-md transition-all hover:border-green-500/40">
                                <div class="flex items-start justify-between">
                                    <div class="space-y-1">
                                        <label class="flex items-center gap-3 cursor-pointer">
                                            <input
                                                type="checkbox"
                                                class="w-4 h-4 rounded border-green-600/50 bg-black/50 text-green-500 focus:ring-green-500 focus:ring-offset-black cursor-pointer shadow-inner"
                                                prop:checked=move || use_plugins.get()
                                                on:change=move |ev| use_plugins.set(event_target_checked(&ev))
                                            />
                                            <span class="text-emerald-400 font-bold tracking-wide">"Neuro-Design Plugins (Beta)"</span>
                                        </label>
                                        <p class="text-green-500/60 text-xs ml-7">"Modular architecture with dynamic AI slide generation."</p>
                                    </div>
                                    <div class="px-2 py-1 rounded bg-green-500/10 border border-green-500/20 text-[10px] text-green-400 font-mono font-bold uppercase">
                                        "BETA"
                                    </div>
                                </div>

                                // Plugin Options - only show when enabled
                                <Show when=move || use_plugins.get()>
                                    <div class="mt-5 pt-5 border-t border-green-700/20 grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-[10px] font-bold text-emerald-500/70 uppercase tracking-widest mb-1.5 ml-1">"Industry Template"</label>
                                            <select
                                                class="w-full bg-black/40 backdrop-blur-md border border-green-500/20 hover:border-green-500/40 text-green-100 rounded-xl py-2 px-3 outline-none focus:border-green-500/70 focus:shadow-[0_0_15px_rgba(16,185,129,0.2)] transition-all text-sm cursor-pointer appearance-none shadow-inner"
                                                on:change=move |ev| {
                                                    let template = event_target_value(&ev);
                                                    match template.as_str() {
                                                        "fintech" => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec!["builtin.geospatial".to_string()]);
                                                        },
                                                        "retail" => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec![]);
                                                        },
                                                        "healthcare" => {
                                                            plugin_theme.set("light".to_string());
                                                            disabled_slides.set(vec!["builtin.evidence".to_string()]);
                                                        },
                                                        _ => {
                                                            plugin_theme.set("dark".to_string());
                                                            disabled_slides.set(vec![]);
                                                        }
                                                    }
                                                }
                                            >
                                                <option value="general">"🎯 General"</option>
                                                <option value="fintech">"🏦 Fintech"</option>
                                                <option value="retail">"🛒 Retail"</option>
                                                <option value="healthcare">"🏥 Healthcare"</option>
                                            </select>
                                        </div>

                                        <div>
                                            <label class="block text-[10px] font-bold text-emerald-500/70 uppercase tracking-widest mb-1.5 ml-1">"Theme"</label>
                                            <select
                                                class="w-full bg-black/40 backdrop-blur-md border border-green-500/20 hover:border-green-500/40 text-green-100 rounded-xl py-2 px-3 outline-none focus:border-green-500/70 focus:shadow-[0_0_15px_rgba(16,185,129,0.2)] transition-all text-sm cursor-pointer appearance-none shadow-inner"
                                                on:change=move |ev| plugin_theme.set(event_target_value(&ev))
                                            >
                                                <option value="dark" selected=move || plugin_theme.get() == "dark">"🌙 Cyber Noir (Axur)"</option>
                                                <option value="light" selected=move || plugin_theme.get() == "light">"☀️ Light"</option>
                                                <option value="auto" selected=move || plugin_theme.get() == "auto">"🔄 Auto"</option>
                                            </select>
                                        </div>

                                        <div class="col-span-2 mt-2 p-3 bg-black/30 rounded-xl border border-white/5">
                                            <div class="text-[10px] font-bold text-emerald-500/70 uppercase tracking-widest mb-2 ml-1">"Include Slides"</div>
                                            <div class="grid grid-cols-2 gap-y-2 gap-x-4 text-xs">
                                                {slide_toggle("builtin.cover", "Cover & Intro")}
                                                {slide_toggle("builtin.toc", "Table of Contents")}
                                                {slide_toggle("builtin.metrics", "General Metrics")}
                                                {slide_toggle("builtin.threats", "Threats Analysis")}
                                                {slide_toggle("builtin.takedowns", "Takedowns & ROI")}
                                                {slide_toggle("builtin.exposure", "Data Exposure")}
                                                {slide_toggle("builtin.geospatial", "Geospatial Map")}
                                                {slide_toggle("builtin.evidence", "Evidence & Evidence")}
                                            </div>
                                        </div>
                                    </div>
                                </Show>
                            </div>

                            // Mock Mode (Dev)
                            <div class="p-4 rounded-xl bg-indigo-900/10 border border-indigo-500/20 backdrop-blur-sm flex justify-between items-center transition-colors hover:border-indigo-500/40 hover:bg-indigo-900/20">
                                <span class="text-indigo-300 font-bold text-xs uppercase tracking-wider pl-1">"Dev Mock Mode"</span>
                                <label class="relative inline-flex items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        class="sr-only peer"
                                        prop:checked=move || use_mock_data.get()
                                        on:change=move |ev| use_mock_data.set(event_target_checked(&ev))
                                    />
                                    <div class="w-11 h-6 bg-surface-elevated peer-focus:outline-none rounded-full peer border border-indigo-500/30 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-zinc-400 peer-checked:after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                                </label>
                            </div>
                        </div>
                    </details>
                </div>

                <div class="pt-4 mt-8 border-t border-white/5 relative">
                    <button
                        class="w-full relative overflow-hidden group rounded-2xl p-[2px] shadow-[0_0_20px_rgba(255,103,49,0.2)] hover:shadow-[0_0_40px_rgba(255,103,49,0.5)] transition-all duration-500 hover:scale-[1.01]"
                        disabled=move || generating.get()
                        on:click=move |_| {
                            try_submit.set(true);
                            if !selected_tenant.get().is_empty() {
                                generate_action.dispatch(());
                            }
                        }
                    >
                        // Button Border Gradient Animation - Converted to a more active, moving beam
                        <span class="absolute inset-0 bg-gradient-to-r from-brand-primary via-purple-500 to-brand-primary rounded-2xl opacity-100 bg-[length:200%_auto] animate-[gradient-x_3s_linear_infinite]"></span>

                        <div class="relative bg-zinc-900 group-hover:bg-transparent px-6 py-4 rounded-2xl transition-all duration-300 flex items-center justify-center gap-3 w-full h-full backdrop-blur-md">
                            // Inner breathing gradient for extra energy
                            <div class="absolute inset-0 bg-gradient-to-r from-brand-primary/20 to-purple-500/20 opacity-0 group-hover:opacity-100 transition-opacity animate-[pulse_2s_ease-in-out_infinite] rounded-2xl"></div>

                            {move || if generating.get() {
                                view! {
                                    <div class="relative z-10 flex items-center gap-3 text-white">
                                        <div class="relative flex items-center justify-center w-6 h-6 drop-shadow-[0_0_5px_rgba(255,255,255,0.5)]">
                                            <div class="absolute inset-0 rounded-full border-t-2 border-white animate-spin"></div>
                                            <div class="absolute inset-1 rounded-full border-b-2 border-white/50 animate-spin-reverse delay-150"></div>
                                        </div>
                                        <span class="font-bold tracking-widest uppercase font-mono text-sm drop-shadow-md">"ENGAGING AI..."</span>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="relative z-10 flex items-center gap-3 group-hover:scale-[1.03] transition-transform duration-300">
                                        <svg class="w-5 h-5 text-brand-primary group-hover:text-white transition-colors drop-shadow-[0_0_5px_rgba(255,103,49,0.8)]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                                        <span class="text-zinc-100 group-hover:text-white font-black tracking-widest uppercase font-mono text-sm transition-colors drop-shadow-[0_2px_4px_rgba(0,0,0,0.5)]">{move || ui_dict.get().generate_btn}</span>
                                    </div>
                                }.into_view()
                            }}
                        </div>
                    </button>
                    <p class="text-center text-[10px] uppercase tracking-widest font-mono text-zinc-500 mt-5 opacity-40 hover:opacity-100 transition-opacity">"Axur Intelligence Engine v3.0"</p>
                </div>
            </div>
        </div>
    }
}
