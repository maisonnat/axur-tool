# Placeholder Documentation Rule

## Rule
When developing or confirming a new placeholder for custom templates:

1. **Add the mapping** to `inject_report_data()` function in:
   `crates/core/src/report/html.rs` (line ~3562)

2. **Update GitHub Wiki** page `Placeholders-Reference.md` with:
   - Placeholder syntax (e.g., `{{placeholder_name}}`)
   - Human-readable description
   - Data source field (e.g., `data.total_tickets`)
   - API endpoint that provides the data

3. **Verify** the placeholder works by generating a report with a custom template

## Wiki Update Command
After confirming functionality, run:
```
/update-wiki
```

## Placeholder Format
All placeholders use double curly braces: `{{placeholder_name}}`

## Current Placeholders
See Wiki: `Placeholders-Reference.md`
