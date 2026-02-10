---
name: dev_component_scaffold
description: Generates a robust Leptos component with prop validation and consistent styling.
version: 1.0.0
---

# Dev Component Scaffold

**Objective**: Create high-quality, reusable frontend components without boilerplate fatigue.

## Usage
Run this skill when adding a new UI element.
Ex: `Implement a 'UserCard' component using dev_component_scaffold`

## Inputs
- **ComponentName**: PascalCase name (e.g., `UserCard`)
- **Props**: List of properties (e.g., `name: String`, `is_active: bool`)

## Procedure

1.  **Read Template**: Read `.agent/skills/dev_component_scaffold/templates/component.rs`.
2.  **Generate File**: Create `crates/frontend/src/components/<component_name_snake_case>.rs`.
3.  **Register Module**: Add `mod <component_name_snake_case>;` to `crates/frontend/src/components/mod.rs` (if it exists) or `lib.rs`.

## Template Rules
- ALWAYS use `#[component]` macro.
- ALWAYS use `#[prop(into)]` for flexibility.
- ALWAYS include a docstring.
- Style using Tailwind CSS classes.
