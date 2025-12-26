---
description: Workflow for updating or creating new feature tutorials (Offline HTML)
---

This workflow standardizes the process of creating visual, offline-viewable tutorials for the Axur Tool.

1. **Preparation & Mocking**
   - If the feature relies on external APIs or specific states, implement temporary **mocks** in `backend/src/routes` or `core/src/api`.
   - Ensure the application is running locally.

2. **Recording**
   - Use `browser_subagent` to perform the user flow.
   - naming convention: `tutorial_[feature_name]`.
   - request the subagent to perform slowly and clearly.

3. **Screenshot Extraction**
   - Identify key frames from the session. The agent typically saves click feedback in hidden system directories.
   - Copy relevant images to the artifact directory using `run_command`.
   - Rename images descriptively (e.g., `login_step.png`, `feature_toggle.png`).

4. **Draft Content**
   - Create a Markdown file (e.g., `feature_tutorial.md`) in the artifact directory.
   - Write step-by-step instructions.
   - Embed images using standard syntax: `![Alt](image.png)`.

5. **Generate Offline HTML**
   - The user requires **offline** files with embedded images.
   - Use a PowerShell/Python script to:
     - Read the images and convert them to **Base64**.
     - Inject the Base64 strings into `src="data:image/png;base64,..."` attributes.
     - Wrap in a clean HTML template (Segoe UI font, responsive container).

6. **Localization**
   - Duplicate the HTML logic for required languages (ES, EN, PT-BR).
   - Translate the text content while keeping the same images.

7. **Standard Footer & Contact**
   - Ensure every HTML file includes the footer:
   - `Axur Tool v1.0 | Para reportar problemas o mejoras, por favor contacte a <a href='mailto:alejandro.maisonnat@axur.com'>alejandro.maisonnat@axur.com</a>` (localized).

8. **Cleanup**
   - **Critical:** Revert any temporary mock code in the codebase.
   - Delete intermediate `.b64` or temp image files from artifacts if no longer needed.
