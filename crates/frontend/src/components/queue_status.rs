//! Queue Status Component
//!
//! Shows real-time queue position and ETA using Server-Sent Events

use leptos::*;

/// Queue job status from backend
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct QueueJobStatus {
    pub job_id: String,
    pub status: String, // "queued", "processing", "completed", "failed"
    pub position: Option<usize>,
    pub eta_seconds: Option<u64>,
    pub error: Option<String>,
}

/// Queue Status Panel Component
/// Shows current position in queue and estimated wait time
#[component]
#[allow(non_snake_case)]
pub fn QueueStatusPanel(
    /// The job ID to track
    job_id: Signal<Option<String>>,
    /// Callback when job completes
    #[prop(optional)]
    _on_complete: Option<Callback<()>>,
) -> impl IntoView {
    let _status = create_rw_signal(QueueJobStatus::default());
    let is_visible = create_memo(move |_| job_id.get().is_some());

    // Start SSE connection when job_id is set
    create_effect(move |_| {
        if let Some(id) = job_id.get() {
            // Poll status via JavaScript (SSE alternative for WASM)
            let poll_code = format!(
                r#"
                (async function() {{
                    const jobId = '{}';
                    let attempts = 0;
                    const maxAttempts = 150; // 5 minutes at 2s intervals
                    
                    const poll = async () => {{
                        if (attempts >= maxAttempts) return;
                        attempts++;
                        
                        try {{
                            const resp = await fetch('/api/queue/status/' + jobId);
                            const data = await resp.json();
                            
                            // Update DOM elements
                            const panel = document.getElementById('queue-status-panel');
                            if (panel) {{
                                const posEl = panel.querySelector('.queue-position');
                                const etaEl = panel.querySelector('.queue-eta');
                                const statusEl = panel.querySelector('.queue-status-text');
                                
                                if (data.position !== undefined && posEl) {{
                                    posEl.textContent = 'Position: ' + (data.position + 1);
                                }}
                                if (data.eta_seconds !== undefined && etaEl) {{
                                    const mins = Math.ceil(data.eta_seconds / 60);
                                    etaEl.textContent = 'ETA: ~' + mins + ' min';
                                }}
                                if (statusEl) {{
                                    statusEl.textContent = data.status;
                                }}
                                
                                if (data.status === 'completed' || data.status === 'failed') {{
                                    panel.classList.add('queue-done');
                                    window.dispatchEvent(new CustomEvent('queue-job-done', {{
                                        detail: {{ jobId, status: data.status, result: data.result }}
                                    }}));
                                    return; // Stop polling
                                }}
                            }}
                            
                            // Continue polling
                            setTimeout(poll, 2000);
                        }} catch (e) {{
                            console.error('[Queue] Poll error:', e);
                            setTimeout(poll, 5000); // Retry slower on error
                        }}
                    }};
                    
                    poll();
                }})();
                "#,
                id
            );
            let _ = js_sys::eval(&poll_code);
        }
    });

    view! {
        <Show when=move || is_visible.get()>
            <div
                id="queue-status-panel"
                class="fixed bottom-4 right-4 bg-zinc-900 border border-zinc-700 rounded-lg p-4 shadow-xl z-50 min-w-64"
            >
                <div class="flex items-center gap-3 mb-3">
                    <div class="w-3 h-3 bg-orange-500 rounded-full animate-pulse"></div>
                    <span class="text-white font-medium">"Processing Request"</span>
                </div>

                <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                        <span class="text-zinc-400">"Status:"</span>
                        <span class="queue-status-text text-orange-400 font-medium">"queued"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-zinc-400">"Queue:"</span>
                        <span class="queue-position text-white">"Position: 1"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-zinc-400">"Wait:"</span>
                        <span class="queue-eta text-zinc-300">"ETA: calculating..."</span>
                    </div>
                </div>

                <div class="mt-3 h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                    <div class="h-full bg-gradient-to-r from-orange-500 to-yellow-500 rounded-full animate-pulse" style="width: 30%"></div>
                </div>

                <p class="text-xs text-zinc-500 mt-2">
                    "Your request is being processed. This panel will update automatically."
                </p>
            </div>
        </Show>
    }
}

/// Compact inline queue indicator for toolbars
#[component]
#[allow(non_snake_case)]
pub fn QueueIndicator() -> impl IntoView {
    let _queue_length = create_rw_signal(0usize);

    // Poll queue length periodically
    create_effect(move |_| {
        let poll_code = r#"
            (async function pollQueue() {
                try {
                    const resp = await fetch('/api/queue/length');
                    const data = await resp.json();
                    const el = document.getElementById('queue-length-indicator');
                    if (el && data.length !== undefined) {
                        el.textContent = data.length;
                        el.parentElement.style.display = data.length > 0 ? 'flex' : 'none';
                    }
                } catch (e) {}
                setTimeout(pollQueue, 10000); // Poll every 10s
            })();
        "#;
        let _ = js_sys::eval(poll_code);
    });

    view! {
        <div
            class="hidden items-center gap-1.5 px-2 py-1 bg-orange-500/20 text-orange-400 rounded text-xs"
            title="Jobs in queue"
        >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <span id="queue-length-indicator">"0"</span>
            " in queue"
        </div>
    }
}
