---
name: qa_browser_smoke
description: Browser-based E2E smoke tests using browser_subagent for visual validation.
version: 1.0.0
status: active
---

# Browser Smoke Test

**Objective**: Verify the frontend loads correctly and critical UI elements are functional.

## When to Use
- After frontend changes
- Before promoting to production
- When debugging visual/render issues

## Prerequisites
- Local dev environment running (`.\dev.ps1`)
- Frontend accessible at `http://localhost:8080`

## Procedure

### 1. Basic Load Test
Use `browser_subagent` to:
```
Task: Navigate to http://localhost:8080 and verify:
1. Page loads without errors
2. Main heading is visible
3. No JavaScript console errors
4. Take a screenshot for validation
Return: Success/failure with any error messages found
```

### 2. Critical Path Test
```
Task: On http://localhost:8080:
1. Look for the main navigation or key buttons
2. Click on primary action button (if exists)
3. Verify the expected response/navigation occurs
4. Check for loading states and error messages
Return: Description of what was tested and results
```

### 3. Form Validation Test (if applicable)
```
Task: On http://localhost:8080:
1. Find any input forms
2. Try submitting empty form (should show validation)
3. Try submitting with invalid data
4. Verify error messages appear correctly
Return: Form validation status
```

## Smoke Test Checklist

| Check | Expected | Command |
|-------|----------|---------|
| Page loads | 200 OK, content visible | browser_subagent navigate |
| No JS errors | Console clean | browser_subagent check console |
| Key elements | Visible and clickable | browser_subagent interact |
| Responsive | Works at different widths | browser_subagent resize |

## Recording
The `browser_subagent` automatically records actions as WebP videos in artifacts directory for review.

## Quick Test Example
```
browser_subagent Task:
"Navigate to http://localhost:8080, wait for page to fully load, 
take a screenshot, check for any visible error messages, 
and report if the main application container is visible."
```

## Common Issues Detected

| Issue | Symptom | Cause |
|-------|---------|-------|
| Blank page | White screen | WASM load failure |
| Console errors | Red text in devtools | JS/WASM runtime error |
| Missing elements | Expected UI not found | Render/hydration issue |
| Broken styles | Unstyled content | CSS not loaded |
