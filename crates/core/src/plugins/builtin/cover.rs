//! Cover Slide Plugin
//!
//! The main cover/title slide for the report.
//! Features the iconic orange smoke image on the right.

use super::theme;
use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Base64 encoded cover image (person with orange smoke)
const COVER_IMAGE_BASE64: &str = include_str!("../../../assets/cover_image_base64.txt");

/// Plugin that generates the cover slide
pub struct CoverSlidePlugin;

impl SlidePlugin for CoverSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.cover"
    }
    fn name(&self) -> &'static str {
        "Cover Slide"
    }
    fn priority(&self) -> i32 {
        100
    } // First slide

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        let partner_html = data.partner_name.as_ref().map(|p| format!(
            r##"<div class="mt-4"><p class="text-[#FF4B00] font-semibold uppercase tracking-wider text-sm">{}</p><p class="text-2xl font-bold">{}</p></div>"##,
            t.get("label_partner"), p
        )).unwrap_or_default();

        let title = if data.is_dynamic_window {
            t.get("cover_title_dynamic")
        } else {
            t.get("cover_title_static")
        };

        // Global CSS is now injected by html.rs
        // let brand_css = theme::BRAND_CSS; (Deprecated)

        let html = format!(
            r##"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex shadow-lg mb-8 relative bg-black text-white overflow-hidden">
<!-- Left Panel - Content -->
<div class="w-5/12 h-full flex flex-col p-14 z-10 bg-black/80 backdrop-blur-md border-r border-white/10">
  <!-- TLP Badge -->
  <div>
    <div class="inline-block">
      <div class="inline-flex items-center gap-2 px-4 py-2 bg-[#FF4B00] text-white shadow-[0_0_15px_rgba(255,75,0,0.3)]">
        <span class="font-bold text-lg tracking-wider">{tlp_lbl}{tlp}</span>
      </div>
    </div>
    <p class="mt-2 text-xs text-zinc-400 max-w-xs">{tlp_desc}</p>
  </div>
  
  <!-- Title -->
  <div class="flex-grow flex flex-col justify-center">
    <h1 class="text-6xl font-black leading-tight display-text uppercase">{title}</h1>
    <div class="mt-8 space-y-4">
      <div>
        <p class="text-[#FF4B00] font-semibold uppercase tracking-wider text-sm mb-1">{company_lbl}</p>
        <p class="text-3xl font-bold display-text text-white">{company}</p>
      </div>
      {partner}
    </div>
  </div>
  
  <!-- Logo -->
  {logo}
</div>

<!-- Right Panel - Orange Smoke Image -->
<div class="w-7/12 h-full relative">
  <img src="data:image/png;base64,{image}" alt="Cover" class="absolute inset-0 w-full h-full object-cover object-center" style="filter: contrast(1.1) brightness(0.8)"/>
  <!-- Mesh overlay for text readability -->
  <div class="absolute inset-0 bg-gradient-to-r from-black via-black/40 to-transparent"></div>
  <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent"></div>
</div>
</div></div>"##,
            tlp_lbl = t.get("label_tlp"),
            tlp = data.tlp_level,
            tlp_desc = t.get("label_tlp_desc"),
            company_lbl = t.get("label_company"),
            company = data.company_name,
            partner = partner_html,
            logo = theme::axur_logo_styled("md"),
            image = COVER_IMAGE_BASE64.trim(),
            title = title,
        );

        vec![SlideOutput {
            id: "cover".into(),
            html,
        }]
    }
}
