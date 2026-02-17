//! Builtin Plugins
//!
//! Default plugins that ship with the core library.

pub mod ai_intent;
pub mod closing;
pub mod comparative;
pub mod cover;
pub mod credentials;
pub mod data_exposure;
pub mod examples;
pub mod geospatial;
pub mod google_slides; // Cloud export
pub mod heatmap;
pub mod helpers;
pub mod incidents;
pub mod insights;
pub mod intro;
pub mod metrics;
pub mod poc_data;
pub mod radar;
pub mod roi;
pub mod solutions;
pub mod style_showcase;
pub mod takedowns;
pub mod theme; // Axur brand theme
pub mod threat_intel;
pub mod threats;
pub mod timeline;
pub mod toc;
pub mod virality;

pub use ai_intent::AiIntentSlidePlugin;
pub use closing::ClosingSlidePlugin;
pub use comparative::ComparativeSlidePlugin;
pub use cover::CoverSlidePlugin;
pub use credentials::CredentialsSlidePlugin;
pub use data_exposure::DataExposureSlidePlugin;
pub use examples::{PocExamplesSlidePlugin, TakedownExamplesSlidePlugin};
pub use geospatial::GeospatialSlidePlugin;
pub use heatmap::HeatmapSlidePlugin;
pub use incidents::IncidentsSlidePlugin;
pub use insights::InsightsSlidePlugin;
pub use intro::IntroSlidePlugin;
pub use metrics::MetricsSlidePlugin;
pub use poc_data::PocDataSlidePlugin;
pub use radar::RadarSlidePlugin;
pub use roi::RoiSlidePlugin;
pub use solutions::SolutionsSlidePlugin;
pub use style_showcase::StyleShowcasePlugin;
pub use takedowns::TakedownsSlidePlugin;
pub use threat_intel::ThreatIntelSlidePlugin;
pub use threats::ThreatsSlidePlugin;
pub use timeline::TimelineSlidePlugin;
pub use toc::TocSlidePlugin;
pub use virality::ViralitySlidePlugin;
