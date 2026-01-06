//! i18n Module - Config-Driven Translations
//!
//! This module provides a new JSON-based translation system.
//! During migration, it coexists with the legacy Dictionary trait system.
//! Use `TranslationCompat` for gradual migration with fallback support.

pub mod compat;
mod legacy;
mod loader;

// New system exports
pub use compat::TranslationCompat;
pub use loader::{TranslationError, Translations};

// Legacy system re-exports (for backwards compatibility)
pub use legacy::*;
