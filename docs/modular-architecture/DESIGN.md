# Technical Design: Modular Architecture

## Part 1: Config-Driven i18n

### Estado Actual (Legacy)

```rust
// core/src/i18n.rs - ~2000 líneas
pub trait Dictionary: Send + Sync {
    fn welcome_message(&self) -> String;
    fn login_prompt_email(&self) -> String;
    // ... ~100 métodos más
}

impl Dictionary for Spanish {
    fn welcome_message(&self) -> String {
        "Bienvenido a Axur CLI".to_string()
    }
    // ... ~100 implementaciones
}
```

**Problemas:**
- Agregar idioma = copiar ~600 líneas de código Rust
- Cambiar texto requiere recompilación
- No-developers no pueden contribuir traducciones

### Estado Objetivo

```
translations/
├── en.json
├── es.json
├── pt-br.json
└── schema.json
```

```json
// translations/es.json
{
  "welcome_message": "Bienvenido a Axur CLI",
  "login_prompt_email": "Correo electrónico",
  "metrics_title": "Métricas Generales",
  "threats_desc": "Se detectaron <strong>{total}</strong> amenazas.",
  "_meta": {
    "language": "es",
    "name": "Español",
    "version": "1.0.0"
  }
}
```

### Nuevo Módulo i18n

```rust
// core/src/i18n/mod.rs
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Translations {
    #[serde(flatten)]
    data: HashMap<String, serde_json::Value>,
}

impl Translations {
    pub fn load(lang: &str) -> Result<Self, TranslationError> {
        let json = match lang {
            "en" => include_str!("../../translations/en.json"),
            "es" => include_str!("../../translations/es.json"),
            "pt-br" => include_str!("../../translations/pt-br.json"),
            _ => return Err(TranslationError::UnknownLanguage(lang.into())),
        };
        serde_json::from_str(json).map_err(TranslationError::ParseError)
    }
    
    pub fn get(&self, key: &str) -> String {
        self.data.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("[MISSING: {}]", key))
    }
    
    pub fn format(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.get(key);
        for (k, v) in args {
            result = result.replace(&format!("{{{}}}", k), v);
        }
        result
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("Unknown language: {0}")]
    UnknownLanguage(String),
    #[error("Failed to parse translations: {0}")]
    ParseError(#[from] serde_json::Error),
}
```

### Compatibilidad (Transición)

Durante la migración, el sistema legacy coexiste:

```rust
// Wrapper que intenta nuevo sistema, fallback a legacy
pub fn get_text(key: &str, lang: Language) -> String {
    // Nuevo sistema
    if let Ok(trans) = Translations::load(&lang.to_code()) {
        let value = trans.get(key);
        if !value.starts_with("[MISSING:") {
            return value;
        }
    }
    // Fallback a legacy
    get_dictionary(lang).get_by_key(key)
}
```

---

## Part 2: Plugin System

### Traits Principales

```rust
// core/src/plugins/traits.rs

/// Plugin que genera slides HTML
pub trait SlidePlugin: Send + Sync {
    /// Identificador único (e.g., "builtin.metrics")
    fn id(&self) -> &'static str;
    
    /// Nombre legible
    fn name(&self) -> &'static str;
    
    /// Genera slides basadas en datos del reporte
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput>;
    
    /// Orden de ejecución (mayor = primero)
    fn priority(&self) -> i32 { 0 }
    
    /// Si el plugin está habilitado para este reporte
    fn is_enabled(&self, _ctx: &PluginContext) -> bool { true }
}

/// Plugin que transforma datos antes de generar slides
pub trait DataPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn transform(&self, data: &mut ReportData);
}

/// Contexto disponible para plugins
pub struct PluginContext<'a> {
    pub data: &'a ReportData,
    pub translations: &'a Translations,
    pub config: &'a ReportConfig,
    pub tenant_name: &'a str,
}

/// Output de un slide plugin
pub struct SlideOutput {
    pub id: String,
    pub html: String,
}
```

### Registry

```rust
// core/src/plugins/registry.rs

pub struct PluginRegistry {
    slide_plugins: Vec<Box<dyn SlidePlugin>>,
    data_plugins: Vec<Box<dyn DataPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        // Registrar plugins builtin
        registry.register_slide(Box::new(CoverSlidePlugin));
        registry.register_slide(Box::new(MetricsSlidePlugin));
        registry.register_slide(Box::new(ThreatsChartPlugin));
        // ... más plugins
        registry
    }
    
    pub fn register_slide(&mut self, plugin: Box<dyn SlidePlugin>) {
        self.slide_plugins.push(plugin);
        self.slide_plugins.sort_by_key(|p| -p.priority());
    }
    
    pub fn generate_report(&self, ctx: &PluginContext) -> String {
        let slides: Vec<String> = self.slide_plugins.iter()
            .filter(|p| p.is_enabled(ctx))
            .flat_map(|p| p.generate_slides(ctx))
            .map(|s| s.html)
            .collect();
        
        slides.join("\n")
    }
}
```

### Ejemplo de Plugin Builtin

```rust
// core/src/plugins/builtin/metrics.rs

pub struct MetricsSlidePlugin;

impl SlidePlugin for MetricsSlidePlugin {
    fn id(&self) -> &'static str { "builtin.metrics" }
    fn name(&self) -> &'static str { "General Metrics" }
    fn priority(&self) -> i32 { 90 }
    
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let t = &ctx.translations;
        let d = &ctx.data;
        
        vec![SlideOutput {
            id: "metrics".into(),
            html: format!(r#"
                <section class="slide" id="metrics">
                    <h2>{title}</h2>
                    <div class="metrics-grid">
                        <div class="metric">
                            <span class="value">{total}</span>
                            <span class="label">{label}</span>
                        </div>
                    </div>
                </section>
            "#,
                title = t.get("metrics_title"),
                total = d.total_tickets,
                label = t.get("metrics_total_tickets"),
            ),
        }]
    }
}
```

---

## Estructura de Directorios Final

```
crates/core/
├── src/
│   ├── i18n/
│   │   ├── mod.rs          # Nuevo loader
│   │   └── legacy.rs       # Compat layer (temporal)
│   ├── plugins/
│   │   ├── mod.rs          # Exports
│   │   ├── traits.rs       # SlidePlugin, DataPlugin
│   │   ├── registry.rs     # PluginRegistry
│   │   └── builtin/
│   │       ├── mod.rs
│   │       ├── cover.rs
│   │       ├── metrics.rs
│   │       └── threats.rs
│   └── ...
├── translations/
│   ├── en.json
│   ├── es.json
│   └── pt-br.json
└── Cargo.toml
```

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_translations() {
        let trans = Translations::load("en").unwrap();
        assert!(!trans.get("welcome_message").starts_with("[MISSING"));
    }
    
    #[test]
    fn test_format_interpolation() {
        let trans = Translations::load("en").unwrap();
        let result = trans.format("threats_desc", &[("total", "42")]);
        assert!(result.contains("42"));
    }
    
    #[test]
    fn test_plugin_registry() {
        let registry = PluginRegistry::with_builtins();
        assert!(registry.slide_plugins.len() > 0);
    }
}
```
