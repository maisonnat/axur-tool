# Project Rules

## Axur API Documentation Policy
**Effective Date:** 2025-12-24
**Related Folder:** `axur-api-doc/`

### Rule
When investigating improvements, new functions, or debugging issues related to the consumption of the Axur API, the developer **MUST** thoroughly investigate the documentation located in the `axur-api-doc/` directory.

### Procedure
1. **Check Local Docs First:** Before implementing any new API call, check `axur-api-doc/` for relevant markdown or PDF files (e.g., `TH API.md`, `doc api axur*.md`).
2. **Verify Endpoints:** Use the local documentation to verify URL paths, required headers, and payload structures.
3. **Update Docs:** If new behavior is discovered that contradicts the docs, update the local documentation or add notes.

### Ignored Status
The `axur-api-doc/` folder is gitignored to protect sensitive or proprietary documentation. Do not commit files from this directory.
