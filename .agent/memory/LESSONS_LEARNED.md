# 📒 Lessons Learned

> Auto-updated via ACE Protocol during session handoff.
> Inspired by praefectus "Memory-as-Code" approach.

---

## 2026-02-10

### Dead Dependencies
- **`sqlx`** was in `Cargo.toml` but never imported in any `.rs` file. Project uses **Firebase**, not Postgres/SQLite.
- **Lesson:** Before adding deps, grep for actual `use` statements. During audits, check if deps are actually imported.

### GitHub Actions — Composite Actions
- `yamadashy/repomix-action@v0.1.25` doesn't exist. The correct path for Repomix's GitHub Action is: `yamadashy/repomix/.github/actions/repomix@main`
- **Lesson:** Many tools use **composite actions** inside their main repo (path: `owner/repo/.github/actions/name@ref`), not a separate `-action` repo.

### Dependency Updates — Risk Strategy
- Transitive deps can be updated safely with `cargo update` (no Cargo.toml changes).
- Direct deps require Cargo.toml edits + verification. SemVer major bumps (e.g., `zip` 0.6→2.x) need API review.
- **Lesson:** Always categorize updates by risk tier before touching anything.

### Repomix vs Code2Prompt
- Migrated from Code2Prompt to Repomix. Repomix has better MCP integration, tree-sitter compression, and a native GitHub Action.
- **Lesson:** Repomix MCP is the preferred context engine. Code2Prompt workflows are deprecated.

### Axur API Limits
- **Tickets API** (`/stats/customer` etc.) returns **HTTP 400** if `from` -> `to` date range exceeds **90 days**.
- **Credentials API** does NOT have this limit.
- **Lesson:** When fetching data for user-selected ranges, always **clamp** specific sub-calls to their known API limits to prevent total failure. Implemented `clamp_date_range` helper in `report.rs`.
