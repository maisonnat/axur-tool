# Directory Structure

```
assets/
  chart.min.js (3 lines)
  cover_image_base64.txt (1 lines)
  fabric.min.js (1 lines)
  tailwind.js (28 lines)
  toc_image_base64.txt (1 lines)
examples/
  probe_creds.rs (68 lines)
  probe_th.rs (133 lines)
src/
  api/
    mod.rs (7 lines)
    report.rs (1418 lines)
    retry.rs (25 lines)
  editor/
    mod.rs (3 lines)
    placeholders.rs (58 lines)
    storage.rs (24 lines)
    types.rs (100 lines)
  i18n/
    compat.rs (112 lines)
    legacy.rs (1174 lines)
    loader.rs (64 lines)
    mod.rs (5 lines)
  plugins/
    builtin/
      ai_intent.rs (48 lines)
      closing.rs (16 lines)
      comparative.rs (83 lines)
      cover.rs (26 lines)
      credentials.rs (32 lines)
      data_exposure.rs (16 lines)
      examples.rs (69 lines)
      geospatial.rs (39 lines)
      google_slides.rs (124 lines)
      heatmap.rs (111 lines)
      helpers.rs (37 lines)
      incidents.rs (33 lines)
      insights.rs (117 lines)
      intro.rs (22 lines)
      metrics.rs (43 lines)
      mod.rs (50 lines)
      poc_data.rs (18 lines)
      radar.rs (120 lines)
      roi.rs (26 lines)
      solutions.rs (18 lines)
      takedowns.rs (18 lines)
      theme.rs (49 lines)
      threat_intel.rs (21 lines)
      threats.rs (40 lines)
      timeline.rs (43 lines)
      toc.rs (30 lines)
      virality.rs (24 lines)
    mod.rs (4 lines)
    registry.rs (103 lines)
    traits.rs (71 lines)
  report/
    html.rs (800 lines)
    language_switcher.rs (42 lines)
    mod.rs (22 lines)
    render_data_exposure_slide_snippet.rs (15 lines)
    template_renderer.rs (53 lines)
  templates/
    mod.rs (61 lines)
  build_error_temp.rs (9 lines)
  error_codes.rs (114 lines)
  errors.rs (15 lines)
  lib.rs (9 lines)
  pptx_mapper.rs (151 lines)
translations/
  en.json (226 lines)
  es.json (226 lines)
  pt-br.json (226 lines)
Cargo.toml (25 lines)
debug_report.html (1357 lines)
```