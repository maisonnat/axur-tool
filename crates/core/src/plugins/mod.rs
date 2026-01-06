//! Plugin System
//!
//! Extensible architecture for report generation.
//! Allows adding slides, data transformers, and export formats
//! without modifying core code.
//!
//! # Example
//!
//! ```rust,ignore
//! use axur_core::plugins::{PluginRegistry, SlidePlugin, PluginContext, SlideOutput};
//!
//! struct MyCustomSlide;
//!
//! impl SlidePlugin for MyCustomSlide {
//!     fn id(&self) -> &'static str { "custom.my-slide" }
//!     fn name(&self) -> &'static str { "My Custom Slide" }
//!     
//!     fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
//!         vec![SlideOutput {
//!             id: "custom".into(),
//!             html: "<section>Custom content</section>".into(),
//!         }]
//!     }
//! }
//!
//! let mut registry = PluginRegistry::new();
//! registry.register_slide(Box::new(MyCustomSlide));
//! ```

mod registry;
mod traits;

pub mod builtin;

pub use traits::{DataPlugin, ExportPlugin, PluginConfig, PluginContext, SlideOutput, SlidePlugin};

pub use registry::{PluginRegistry, RegistryStats};
