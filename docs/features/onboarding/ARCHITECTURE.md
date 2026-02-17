# Onboarding System Architecture

## Overview

The Axur Academy onboarding system provides an interactive learning experience for new users. It consists of four main components:

1. **Tutorial Engine** - Step-by-step guided walkthroughs
2. **Achievement System** - Gamified progress tracking
3. **Sandbox Mode** - Safe practice environment
4. **Hints System** - Passive contextual help

---

## Module Structure

```
crates/frontend/src/onboarding/
├── mod.rs           # Module exports and documentation
├── tutorial.rs      # TutorialStep trait and step registry
├── achievements.rs  # Achievement struct and trigger system
├── hints.rs         # Passive hint tooltips
├── sandbox.rs       # Demo template and safe mode
└── storage.rs       # localStorage persistence
```

---

## Adding New Tutorial Steps

1. Open `tutorial.rs`
2. Add your step to `TUTORIAL_STEPS`:

```rust
TutorialStep {
    id: "my_step",
    target_selector: "#my-element",
    validation: StepValidation::Click,
    tooltip_position: "bottom",
}
```

3. Add i18n translations in the `title()` and `instruction()` methods:

```rust
("my_step", UiLanguage::Es) => "Mi título",
("my_step", UiLanguage::En) => "My title",
("my_step", UiLanguage::Pt) => "Meu título",
```

---

## Adding New Achievements

1. Open `achievements.rs`
2. Add to `ALL_ACHIEVEMENTS`:

```rust
Achievement {
    id: "my_achievement",
    icon: "🏆",
    trigger: AchievementTrigger::ManualAction("my_action_id"),
}
```

3. Add translations in `title()` and `description()` methods.

4. Trigger from anywhere:

```rust
use crate::onboarding::unlock_achievement;
unlock_achievement("my_achievement");
```

---

## Adding New Hints

1. Open `hints.rs`
2. Add to `HINTS`:

```rust
Hint {
    id: "my_hint",
    target_selector: "#my-element",
    show_after_idle_ms: 15000, // 15 seconds
    position: HintPosition::Top,
}
```

3. Add translations in the `message()` method.

---

## Storage Schema

Progress is stored in `localStorage` under key `axur_onboarding`:

```json
{
  "tutorial_completed": true,
  "current_step": null,
  "unlocked_achievements": ["tutorial", "first_import"],
  "sandbox_dismissed": false,
  "welcome_seen": true,
  "shortcuts_used": 7
}
```

---

## Achievement Triggers

| Trigger Type | Description | Example |
|--------------|-------------|---------|
| `TutorialComplete` | When tutorial finishes | Automatic |
| `ManualAction(id)` | Explicit call with ID | `unlock_achievement("import_pptx")` |
| `PageVisit(path)` | When visiting a page | `/editor` |
| `Shortcut(keys)` | When using a shortcut | `Ctrl+D` |

---

## i18n Pattern

All text uses the pattern:
- Method-based localization in each struct
- Match on `(id, UiLanguage::Es|En|Pt)`
- 3 languages: Spanish (default), English, Portuguese
