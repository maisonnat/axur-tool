# Architecture Brief

## Overview
Axur Web Tool generates HTML reports using a modular, Rust-based architecture.

## Core Concepts

### 1. Config-Driven i18n
- **Location**: `crates/core/translations/*.json`
- **Format**: JSON files loaded at runtime.
- **Usage**: `Translations::load("es")`
- **Legacy**: `i18n/legacy.rs` provides fallback during migration.

### 2. Plugin System (`crates/core/src/plugins/`)
- **SlidePlugin**: Generates HTML for a specific slide (e.g., `metrics`, `threats`).
- **DataPlugin**: Transforms report data before generation.
- **Registry**: `PluginRegistry` manages execution order.

### 3. Report Generation Flow
1. **Frontend**: Request -> `POST /api/report`
2. **Backend**: Fetches data (Axur API) or uses Mock Data.
3. **Core**: `PluginRegistry` executes enable plugins -> orchestrates HTML assembly.
4. **Output**: Single HTML string returned to frontend.

## Key Decisions (ADR Summary)
- **ADR-001 (No WASI)**: Plugins are compiled trait objects, not WASM modules (simplicity > isolation).
- **ADR-002 (JSON i18n)**: Runtime JSON loading over static traits for flexibility.
- **ADR-003 (Static Registry)**: Plugins registered at compile time in `registry.rs` (type safety).
- **ADR-004 (Incremental Migration)**: Legacy `html.rs` coexists until full plugin adoption.

## Directory Structure
- `crates/core/src/api`: Data structures (`PocReportData`).
- `crates/core/src/report`: HTML generation logic.
- `crates/core/src/plugins`: Modular slide logic.
- `crates/frontend/src/pages`: UI components (`dashboard.rs`).
