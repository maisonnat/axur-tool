//! Builtin Plugins
//!
//! Default plugins that ship with the core library.

pub mod ai_intent;
pub mod closing;
pub mod cover;
pub mod credentials;
pub mod data_exposure;
pub mod examples;
pub mod geospatial;
pub mod helpers;
pub mod incidents;
pub mod intro;
pub mod metrics;
pub mod poc_data;
pub mod roi;
pub mod solutions;
pub mod takedowns;
pub mod theme; // Axur brand theme
pub mod threat_intel;
pub mod threats;
pub mod timeline;
pub mod toc;
pub mod virality;

pub use ai_intent::AiIntentSlidePlugin;
pub use closing::ClosingSlidePlugin;
pub use cover::CoverSlidePlugin;
pub use credentials::CredentialsSlidePlugin;
pub use data_exposure::DataExposureSlidePlugin;
pub use examples::{PocExamplesSlidePlugin, TakedownExamplesSlidePlugin};
pub use geospatial::GeospatialSlidePlugin;
pub use incidents::IncidentsSlidePlugin;
pub use intro::IntroSlidePlugin;
pub use metrics::MetricsSlidePlugin;
pub use poc_data::PocDataSlidePlugin;
pub use roi::RoiSlidePlugin;
pub use solutions::SolutionsSlidePlugin;
pub use takedowns::TakedownsSlidePlugin;
pub use threat_intel::ThreatIntelSlidePlugin;
pub use threats::ThreatsSlidePlugin;
pub use timeline::TimelineSlidePlugin;
pub use toc::TocSlidePlugin;
pub use virality::ViralitySlidePlugin;
