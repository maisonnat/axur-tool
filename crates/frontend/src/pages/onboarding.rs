use leptos::*;

#[component]
pub fn OnboardingPage() -> impl IntoView {
    let app_state = use_context::<crate::AppState>().expect("state");
    let (selected_skill, set_selected_skill) = create_signal(Option::<String>::None);

    let enter_dashboard = move |_| {
        app_state.current_page.set(crate::Page::Dashboard);
    };

    let set_lang = move |lang: crate::i18n::UiLanguage| {
        app_state.ui_language.set(lang);
    };

    // Derived dictionary based on current language
    let dict = move || crate::get_ui_dict(app_state.ui_language.get());

    let skills = move || {
        let d = dict();
        vec![
            (
                d.skill_th_title,
                d.skill_th_desc,
                "🎯",
                "tutorial_threat_hunting.webp",
            ),
            (
                d.skill_templates_title,
                d.skill_templates_desc,
                "📦",
                "tutorial_templates.webp",
            ),
            (
                d.skill_reports_title,
                d.skill_reports_desc,
                "📊",
                "tutorial_reports.webp",
            ),
            (
                d.skill_editor_title,
                d.skill_editor_desc,
                "✏️",
                "tutorial_editor.webp", // Fixed filename from previous rename
            ),
        ]
    };

    view! {
        <div class="min-h-screen bg-black text-white relative flex flex-col items-center justify-center p-8 overflow-hidden">
             // Ambient Background
            <div class="absolute top-0 left-0 w-full h-[500px] bg-gradient-to-b from-zinc-900 to-transparent pointer-events-none"></div>

            // Language Selector
            <div class="absolute top-6 right-8 flex gap-2 z-20">
                <button
                    class="px-3 py-1 rounded-full text-sm font-medium transition-colors hover:bg-white/10"
                    class:bg-white={move || app_state.ui_language.get() == crate::i18n::UiLanguage::En}
                    class:text-black={move || app_state.ui_language.get() == crate::i18n::UiLanguage::En}
                    on:click=move |_| set_lang(crate::i18n::UiLanguage::En)
                >
                    "EN"
                </button>
                <button
                    class="px-3 py-1 rounded-full text-sm font-medium transition-colors hover:bg-white/10"
                    class:bg-white={move || app_state.ui_language.get() == crate::i18n::UiLanguage::Es}
                    class:text-black={move || app_state.ui_language.get() == crate::i18n::UiLanguage::Es}
                    on:click=move |_| set_lang(crate::i18n::UiLanguage::Es)
                >
                    "ES"
                </button>
                <button
                    class="px-3 py-1 rounded-full text-sm font-medium transition-colors hover:bg-white/10"
                    class:bg-white={move || app_state.ui_language.get() == crate::i18n::UiLanguage::Pt}
                    class:text-black={move || app_state.ui_language.get() == crate::i18n::UiLanguage::Pt}
                    on:click=move |_| set_lang(crate::i18n::UiLanguage::Pt)
                >
                    "PT"
                </button>
            </div>

            <div class="z-10 max-w-4xl w-full mt-12">
                // Hero
                <div class="text-center mb-16 animate-in fade-in slide-in-from-bottom-8 duration-700">
                    <h1 class="text-6xl font-bold mb-6 tracking-tighter bg-clip-text text-transparent bg-gradient-to-r from-white to-zinc-500">
                        {move || dict().onboarding_title}
                    </h1>
                    <p class="text-xl text-zinc-400 max-w-2xl mx-auto">
                        {move || dict().onboarding_subtitle}
                    </p>
                </div>

                // Two Paths
                <div class="grid md:grid-cols-2 gap-8 mb-16">
                    // Path 1: Jump In
                    <div
                        class="group bg-zinc-900/50 hover:bg-zinc-900 border border-zinc-800 hover:border-zinc-700 p-8 rounded-2xl transition-all cursor-pointer flex flex-col items-start justify-between h-[250px]"
                        on:click=enter_dashboard
                    >
                        <div>
                            <div class="bg-white/10 w-12 h-12 rounded-lg flex items-center justify-center mb-4 text-2xl">"🚀"</div>
                            <h2 class="text-2xl font-bold mb-2">{move || dict().jump_in_title}</h2>
                            <p class="text-zinc-500">{move || dict().jump_in_desc}</p>
                        </div>
                         <button class="flex items-center gap-2 text-white font-medium group-hover:translate-x-1 transition-transform">
                            {move || dict().enter_dashboard}
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3" /></svg>
                        </button>
                    </div>

                    // Path 2: Learning Hub
                    <div class="bg-gradient-to-br from-zinc-800 to-zinc-900 border border-zinc-700 p-8 rounded-2xl relative overflow-hidden">
                        <div class="absolute top-0 right-0 p-32 bg-blue-500/10 blur-3xl rounded-full"></div>

                        <div class="relative z-10">
                            <div class="bg-blue-500/20 w-12 h-12 rounded-lg flex items-center justify-center mb-4 text-2xl text-blue-400">"🎓"</div>
                            <h2 class="text-2xl font-bold mb-2">{move || dict().learning_hub_title}</h2>
                             <p class="text-zinc-400 mb-6">{move || dict().learning_hub_desc}</p>

                            <div class="grid grid-cols-2 gap-3">
                                {move || skills().into_iter().map(|(title, _desc, icon, _img)| {
                                    let title_clone = title.to_string();
                                    view! {
                                        <button
                                            class="text-left p-3 rounded-lg bg-black/40 hover:bg-white/10 transition-colors border border-white/5 hover:border-white/20"
                                            on:click=move |_| set_selected_skill.set(Some(title_clone.clone()))
                                        >
                                            <div class="mr-2 text-lg">{icon}</div>
                                            <div class="font-medium text-sm">{title}</div>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Skill Overlay (Mock)
            {move || selected_skill.get().map(|skill_title| {
                let current_skills = skills(); // Get current state
                let skill_data = current_skills.iter().find(|(t, ..)| *t == skill_title).cloned();

                if let Some((title, desc, _icon, image)) = skill_data {
                    view! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-md animate-in fade-in duration-200">
                            <div class="bg-zinc-900 border border-zinc-700 rounded-2xl max-w-2xl w-full p-8 relative shadow-2xl">
                                <button
                                    class="absolute top-4 right-4 text-zinc-500 hover:text-white"
                                    on:click=move |_| set_selected_skill.set(None)
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /></svg>
                                </button>

                                <div class="text-center py-6">
                                    <h2 class="text-3xl font-bold mb-4">"Learning: "{title}</h2>
                                    <div class="w-full aspect-video bg-zinc-800 rounded-xl flex items-center justify-center mb-6 overflow-hidden border border-zinc-700 shadow-lg">
                                        <img
                                            src=format!("assets/{}", image)
                                            alt=title
                                            class="w-full h-full object-cover"
                                            on:error=move |ev| {
                                                // Fallback if image not found
                                                let img = event_target::<web_sys::HtmlImageElement>(&ev);
                                                img.set_src("https://placehold.co/600x400/18181b/52525b?text=Tutorial+Coming+Soon");
                                            }
                                        />
                                    </div>
                                    <p class="text-zinc-400">{desc}</p>

                                    // Disclaimer
                                    <div class="mt-4 p-3 bg-blue-500/10 border border-blue-500/20 rounded-lg">
                                        <p class="text-blue-200 text-xs italic">
                                            {move || dict().tutorial_disclaimer}
                                        </p>
                                    </div>

                                    <button
                                        class="mt-8 bg-white text-black px-6 py-2 rounded-full font-medium hover:scale-105 transition-transform"
                                        on:click=move |_| set_selected_skill.set(None)
                                    >
                                        "Finish Module"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            })}
        </div>
    }
}
