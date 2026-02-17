# ADR-001: Data Storytelling Strategy (Deterministic vs. Generative AI)

**Status:** Accepted  
**Date:** 2026-02-17  
**Context:**  
The project requires generating "Executive Summaries" and narrative insights for report slides (e.g., Virality, Threats) to transform raw data into actionable intelligence. The user inquired about using external AI services (specifically mentioning "Google Stitch MCP", interpreted as a Generative AI integration) versus our current custom implementation.

**Options Considered:**

1.  **Deterministic Logic (Current Implementation)**
    -   *Mechanism:* Rust `match` statements and format strings based on hardcoded thresholds.
    -   *Example:* `if chat_shares > social_mentions { "Focus on Private Channels" }`

2.  **Generative AI (MCP / External API)**
    -   *Mechanism:* Sending report data to an LLM (e.g., Gemini, GPT-4) via MCP to generate unique, context-aware narratives.
    -   *Example:* Prompt: "Analyze this threat data and write a 2-sentence executive summary focusing on brand impact."

**Decision:**
We will proceed with **Deterministic Logic (Option 1)** for the **V1 / MVP** release of the Senior Design System.

**Rationale:**

1.  **Reliability & Trust**: Security reports require 100% accuracy. Deterministic logic guarantees that a specific data pattern *always* produces the same valid insight. Generative AI carries a non-zero risk of "hallucination" (e.g., inventing a trend that doesn't exist), which could damage trust in the security dashboard.
2.  **Performance**: Rust-based logic executes in microseconds. Calling an external LLM API adds 500ms - 2s of latency per slide, significantly slowing down report generation.
3.  **Privacy & Compliance**: Keeping data processing local avoids sending sensitive threat intelligence metrics to external third-party APIs.
4.  **Cost**: Deterministic logic is free. API calls scale linearly with usage.

**Future Considerations (V2):**
We acknowledge that Option 1 has limited "creativity" and can become repetitive. For **V2**, we will evaluate a **Hybrid Approach**:
-   Use Deterministic Logic for *critical* alerts and "facts".
-   Use a Local LLM or a strictly guarded Prompt Chain (via MCP) for "flavor text" or "stylistic variations" where accuracy is less critical than engagement.

**Consequences:**
*   **Positive**: Fast generation, zero marginal cost, high reliability, no external dependencies.
*   **Negative**: Finite set of narrative templates; insights limited to what developers explicitly code for.
