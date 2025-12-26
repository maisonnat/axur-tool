//! Structured error codes for the Axur Tool
//!
//! This module provides a standardized error code system that:
//! - Maps internal errors to user-friendly codes (e.g., `AUTH-001`)
//! - Enables quick debugging without reading full logs
//! - Supports i18n for error messages

use serde::{Deserialize, Serialize};

/// Error code prefixes by module
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ErrorModule {
    /// Authentication errors (login, 2FA, session)
    Auth,
    /// Axur API connection errors
    Api,
    /// Report generation errors
    Rpt,
    /// Threat Intelligence errors
    Ti,
    /// Network errors (CORS, timeout)
    Net,
    /// System/internal errors
    Sys,
}

impl ErrorModule {
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Auth => "AUTH",
            Self::Api => "API",
            Self::Rpt => "RPT",
            Self::Ti => "TI",
            Self::Net => "NET",
            Self::Sys => "SYS",
        }
    }
}

/// Structured error code with module and number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    pub module: ErrorModule,
    pub number: u16,
    /// Internal context (not shown to user, for dev debugging)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl ErrorCode {
    pub fn new(module: ErrorModule, number: u16) -> Self {
        Self {
            module,
            number,
            context: None,
        }
    }

    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Returns the formatted code (e.g., "AUTH-001")
    pub fn code(&self) -> String {
        format!("{}-{:03}", self.module.prefix(), self.number)
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

// ============================================================================
// Predefined Error Codes
// ============================================================================

/// Authentication Errors
pub mod auth {
    use super::*;

    /// AUTH-001: Invalid email or password
    pub fn invalid_credentials() -> ErrorCode {
        ErrorCode::new(ErrorModule::Auth, 1)
    }

    /// AUTH-002: Invalid 2FA code
    pub fn invalid_2fa() -> ErrorCode {
        ErrorCode::new(ErrorModule::Auth, 2)
    }

    /// AUTH-003: Session expired
    pub fn session_expired() -> ErrorCode {
        ErrorCode::new(ErrorModule::Auth, 3)
    }

    /// AUTH-004: Missing authentication token
    pub fn missing_token() -> ErrorCode {
        ErrorCode::new(ErrorModule::Auth, 4)
    }
}

/// API Connection Errors
pub mod api {
    use super::*;

    /// API-001: Axur API token expired
    pub fn token_expired() -> ErrorCode {
        ErrorCode::new(ErrorModule::Api, 1)
    }

    /// API-002: Tenant not found
    pub fn tenant_not_found() -> ErrorCode {
        ErrorCode::new(ErrorModule::Api, 2)
    }

    /// API-003: Rate limit exceeded
    pub fn rate_limited() -> ErrorCode {
        ErrorCode::new(ErrorModule::Api, 3)
    }

    /// API-004: Invalid API response
    pub fn invalid_response() -> ErrorCode {
        ErrorCode::new(ErrorModule::Api, 4)
    }

    /// API-005: API endpoint not available
    pub fn endpoint_unavailable() -> ErrorCode {
        ErrorCode::new(ErrorModule::Api, 5)
    }
}

/// Report Generation Errors
pub mod report {
    use super::*;

    /// RPT-001: No data in selected period
    pub fn no_data_in_period() -> ErrorCode {
        ErrorCode::new(ErrorModule::Rpt, 1)
    }

    /// RPT-002: No incidents for evidence slide
    pub fn no_incidents_for_evidence() -> ErrorCode {
        ErrorCode::new(ErrorModule::Rpt, 2)
    }

    /// RPT-003: Failed to render slide
    pub fn slide_render_failed() -> ErrorCode {
        ErrorCode::new(ErrorModule::Rpt, 3)
    }

    /// RPT-004: Invalid date range
    pub fn invalid_date_range() -> ErrorCode {
        ErrorCode::new(ErrorModule::Rpt, 4)
    }
}

/// Threat Intelligence Errors
pub mod threat_intel {
    use super::*;

    /// TI-001: Dark Web search timeout
    pub fn dark_web_timeout() -> ErrorCode {
        ErrorCode::new(ErrorModule::Ti, 1)
    }

    /// TI-002: Threat Hunting API unavailable
    pub fn hunting_api_unavailable() -> ErrorCode {
        ErrorCode::new(ErrorModule::Ti, 2)
    }

    /// TI-003: No credential results found
    pub fn no_credentials_found() -> ErrorCode {
        ErrorCode::new(ErrorModule::Ti, 3)
    }

    /// TI-004: Search cancelled by user
    pub fn search_cancelled() -> ErrorCode {
        ErrorCode::new(ErrorModule::Ti, 4)
    }
}

/// Network Errors
pub mod network {
    use super::*;

    /// NET-001: CORS blocked
    pub fn cors_blocked() -> ErrorCode {
        ErrorCode::new(ErrorModule::Net, 1)
    }

    /// NET-002: Connection timeout
    pub fn connection_timeout() -> ErrorCode {
        ErrorCode::new(ErrorModule::Net, 2)
    }

    /// NET-003: DNS resolution failed
    pub fn dns_failed() -> ErrorCode {
        ErrorCode::new(ErrorModule::Net, 3)
    }

    /// NET-004: SSL/TLS error
    pub fn ssl_error() -> ErrorCode {
        ErrorCode::new(ErrorModule::Net, 4)
    }
}

/// System Errors
pub mod system {
    use super::*;

    /// SYS-001: Internal server error
    pub fn internal_error() -> ErrorCode {
        ErrorCode::new(ErrorModule::Sys, 1)
    }

    /// SYS-002: Configuration error
    pub fn config_error() -> ErrorCode {
        ErrorCode::new(ErrorModule::Sys, 2)
    }

    /// SYS-003: Resource exhausted
    pub fn resource_exhausted() -> ErrorCode {
        ErrorCode::new(ErrorModule::Sys, 3)
    }
}

// ============================================================================
// Progress Tracking
// ============================================================================

/// Report generation step for real-time progress tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStep {
    /// Connecting to APIs
    Connecting,
    /// Fetching metrics data
    FetchingMetrics,
    /// Analyzing credentials
    AnalyzingCredentials,
    /// Searching dark web (Threat Intel)
    SearchingDarkWeb,
    /// Tracking virality (Threat Intel)
    TrackingVirality,
    /// Detecting malicious ads (Threat Intel)
    DetectingAds,
    /// Rendering slides
    RenderingSlides,
    /// Complete
    Complete,
}

impl ReportStep {
    /// Returns progress percentage (0-100)
    pub fn progress(&self, include_threat_intel: bool) -> u8 {
        if include_threat_intel {
            match self {
                Self::Connecting => 10,
                Self::FetchingMetrics => 20,
                Self::AnalyzingCredentials => 35,
                Self::SearchingDarkWeb => 50,
                Self::TrackingVirality => 65,
                Self::DetectingAds => 80,
                Self::RenderingSlides => 95,
                Self::Complete => 100,
            }
        } else {
            match self {
                Self::Connecting => 15,
                Self::FetchingMetrics => 35,
                Self::AnalyzingCredentials => 55,
                Self::RenderingSlides => 85,
                Self::Complete => 100,
                // Threat Intel steps not used
                _ => 0,
            }
        }
    }

    /// i18n key for UI display
    pub fn i18n_key(&self) -> &'static str {
        match self {
            Self::Connecting => "step_connecting",
            Self::FetchingMetrics => "step_fetching_metrics",
            Self::AnalyzingCredentials => "step_analyzing_credentials",
            Self::SearchingDarkWeb => "step_searching_dark_web",
            Self::TrackingVirality => "step_tracking_virality",
            Self::DetectingAds => "step_detecting_ads",
            Self::RenderingSlides => "step_rendering_slides",
            Self::Complete => "step_complete",
        }
    }
}

/// Progress event sent via SSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProgressEvent {
    /// Step progress update
    Step { step: ReportStep, progress: u8 },
    /// Error occurred
    Error { code: ErrorCode, message: String },
    /// Report generation complete
    Complete {
        /// Serialized report data
        report_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_format() {
        let code = auth::invalid_credentials();
        assert_eq!(code.code(), "AUTH-001");

        let code = threat_intel::dark_web_timeout();
        assert_eq!(code.code(), "TI-001");
    }

    #[test]
    fn test_error_with_context() {
        let code = api::token_expired().with_context("Token TTL exceeded by 300s");
        assert_eq!(code.code(), "API-001");
        assert!(code.context.is_some());
    }

    #[test]
    fn test_progress_percentage() {
        assert_eq!(ReportStep::Connecting.progress(true), 10);
        assert_eq!(ReportStep::SearchingDarkWeb.progress(true), 50);
        assert_eq!(ReportStep::Complete.progress(true), 100);
    }
}
