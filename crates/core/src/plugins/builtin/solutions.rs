//! Solutions Slide Plugin
//!
//! Showcases Axur's complete platform capabilities.

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the solutions slide
pub struct SolutionsSlidePlugin;

impl SlidePlugin for SolutionsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.solutions"
    }
    fn name(&self) -> &'static str {
        "Solutions"
    }
    fn priority(&self) -> i32 {
        98
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let t = ctx.translations;

        let html = format!(
            r#"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-zinc-950"><div class="flex-grow h-full overflow-hidden"><div class="flex h-full w-full relative items-center"><div class="w-5/12 text-white flex flex-col justify-center pr-10"><h2 class="text-4xl font-bold text-orange-500 leading-tight mb-8">{title}</h2><div class="space-y-6 text-zinc-300 text-sm"><p>{sub1}</p><p>{sub2}</p><p>{sub3}</p></div></div><div class="w-7/12 bg-white text-zinc-800 p-8 rounded-lg shadow-2xl"><div class="grid grid-cols-12 gap-x-6 gap-y-4 w-full"><div class="col-span-8 grid grid-cols-2 gap-x-6 gap-y-4"><div class="bg-blue-500 text-white p-3 rounded-md"><h4 class="font-bold text-sm mb-1">{sol_takedown}</h4><p class="text-xs text-blue-100">Eliminación automatizada de contenido infractor.</p></div><div><h4 class="font-bold text-sm mb-1">{sol_brand}</h4><p class="text-xs text-zinc-600">Detección de abuso de marca y falsificaciones.</p></div><div><h4 class="font-bold text-sm mb-1">{sol_intel}</h4><p class="text-xs text-zinc-600">Inteligencia contextualizada sobre amenazas.</p></div><div><h4 class="font-bold text-sm mb-1">Caza de Amenazas</h4><p class="text-xs text-zinc-600">Búsqueda proactiva de amenazas ocultas.</p></div><div><h4 class="font-bold text-sm mb-1">Deep &amp; Dark Web</h4><p class="text-xs text-zinc-600">Monitoreo de foros y mercados clandestinos.</p></div><div><h4 class="font-bold text-sm mb-1">Inteligencia de Phishing</h4><p class="text-xs text-zinc-600">Análisis de campañas e infraestructura de phishing.</p></div><div><h4 class="font-bold text-sm mb-1">Antipiratería</h4><p class="text-xs text-zinc-600">Combate a la distribución no autorizada.</p></div><div><h4 class="font-bold text-sm mb-1">Protección VIP</h4><p class="text-xs text-zinc-600">Protección para ejecutivos y personas de alto perfil.</p></div><div><h4 class="font-bold text-sm mb-1">Gestión de Superficie de Ataque</h4><p class="text-xs text-zinc-600">Mapeo y monitoreo de activos digitales.</p></div><div><h4 class="font-bold text-sm mb-1">Fugas de Datos</h4><p class="text-xs text-zinc-600">Detección de credenciales y datos sensibles expuestos.</p></div></div><div class="col-span-4 border-l border-zinc-200 pl-6"><h5 class="text-xs font-semibold text-zinc-500 tracking-wider mb-4">INTEGRACIONES</h5><h4 class="font-bold text-sm mb-1">API y Conectores</h4><p class="text-xs text-zinc-600">Integre nuestros datos en su SIEM, SOAR u otras herramientas de seguridad.</p></div></div></div></div></div>{footer}</div></div>"#,
            title = t.get("solutions_title"),
            sub1 = t.get("solutions_subtitle_1"),
            sub2 = t.get("solutions_subtitle_2"),
            sub3 = t.get("solutions_subtitle_3"),
            sol_takedown = t.get("solution_takedown"),
            sol_brand = t.get("solution_brand_protection"),
            sol_intel = t.get("solution_threat_intel"),
            footer = Self::footer_dark(3, &t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "solutions".into(),
            html,
        }]
    }
}

impl SolutionsSlidePlugin {
    fn footer_dark(page: u32, footer_text: &str) -> String {
        format!(
            r#"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center"><div class="flex items-center font-black tracking-wider select-none text-white h-5"><span class="text-orange-500 text-2xl -mr-1">///</span><span class="text-xl">AXUR</span></div><div class="flex items-center text-xs text-zinc-400"><span>{}</span><span class="ml-4">{}</span></div></footer>"#,
            footer_text, page
        )
    }
}
