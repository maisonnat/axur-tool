//! Onboarding System - Axur Academy
//!
//! Modular onboarding with tutorial steps, achievements, sandbox mode, and hints.
//!
//! ## Extensibility
//!
//! ### Adding a new tutorial step:
//! ```rust
//! // In tutorial/steps.rs, add to TUTORIAL_STEPS:
//! TutorialStep {
//!     id: "my_step",
//!     target_selector: "#my-button",
//!     validation: StepValidation::Click,
//!     // i18n keys are auto-derived as: onboarding_step_{id}_title, onboarding_step_{id}_instruction
//! }
//! ```
//!
//! ### Adding a new achievement:
//! ```rust
//! // In achievements/registry.rs, add to ALL_ACHIEVEMENTS:
//! Achievement {
//!     id: "my_achievement",
//!     icon: "🏆",
//!     trigger: AchievementTrigger::ManualAction("my_action"),
//!     // i18n keys: onboarding_achievement_{id}_title, onboarding_achievement_{id}_desc
//! }
//!
//! // Trigger it from anywhere:
//! unlock_achievement("my_achievement");
//! ```

pub mod achievements;
pub mod hints;
pub mod sandbox;
pub mod storage;
pub mod tutorial;

pub use achievements::{unlock_achievement, Achievement, AchievementTrigger, ALL_ACHIEVEMENTS};
pub use hints::{Hint, HintPosition, HINTS};
pub use sandbox::{
    get_practice_activities, get_sandbox_slides, is_sandbox_mode, set_sandbox_mode,
    PracticeActivity, SandboxSlide, SlideElement,
};
pub use storage::{get_onboarding_progress, OnboardingProgress};
pub use tutorial::{StepValidation, TutorialStep, TUTORIAL_STEPS};
