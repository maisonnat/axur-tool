//! Page components

mod analytics;
mod dashboard;
mod editor;
mod login;
mod logs;
mod marketplace;

mod admin_beta;
mod apply;
mod onboarding;

pub use admin_beta::AdminBetaPage;
pub use analytics::AnalyticsPage;
pub use apply::BetaApplyPage;
pub use dashboard::DashboardPage;
pub use editor::EditorPage;
pub use login::LoginPage;
pub use logs::LogsPage;
pub use marketplace::MarketplacePage;
pub use onboarding::OnboardingPage;
