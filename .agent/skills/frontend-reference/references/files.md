# Files

## File: functions/api/[[path]].js
```javascript
export async function onRequest(context)
⋮----
// Clone the request with the new URL
```

## File: functions/health.js
```javascript
export async function onRequest(context)
```

## File: src/components/button.rs
```rust
pub fn Button(
⋮----
disabled.map(|d| d.get()).unwrap_or(false) || loading.map(|l| l.get()).unwrap_or(false)
⋮----
view! {
⋮----
pub fn SecondaryButton(
```

## File: src/components/card.rs
```rust
pub fn Card(children: Children, #[prop(optional)] class: Option<String>) -> impl IntoView {
let class = format!(
⋮----
view! {
⋮----
pub fn CardWithHeader(#[prop(into)] title: Signal<String>, children: Children) -> impl IntoView {
```

## File: src/components/combobox.rs
```rust
pub fn Combobox(
⋮----
let search_text = create_rw_signal(String::new());
let is_open = create_rw_signal(false);
let focused_index = create_rw_signal(0usize);
⋮----
let search = search_text.get().to_lowercase();
let opts = options.get();
⋮----
if search.is_empty() {
⋮----
opts.into_iter()
.filter(|(_, display)| display.to_lowercase().contains(&search))
.collect()
⋮----
let sel = selected.get();
if sel.is_empty() {
⋮----
.get()
.iter()
.find(|(key, _)| *key == sel)
.map(|(_, display)| display.clone())
.unwrap_or_default()
⋮----
create_effect(move |_| {
let display = selected_display.get();
if !display.is_empty() && !is_open.get() {
search_text.set(display);
⋮----
selected.set(key);
⋮----
is_open.set(false);
⋮----
let filtered = filtered_options.get();
let len = filtered.len();
⋮----
match ev.key().as_str() {
⋮----
ev.prevent_default();
is_open.set(true);
⋮----
focused_index.set((focused_index.get() + 1) % len);
⋮----
let current = focused_index.get();
focused_index.set(if current == 0 { len - 1 } else { current - 1 });
⋮----
if is_open.get() && len > 0 {
let idx = focused_index.get().min(len - 1);
let (key, display) = filtered[idx].clone();
select_option(key, display);
⋮----
view! {
```

## File: src/components/error.rs
```rust
pub async fn copy_to_clipboard(text: &str) -> Result<(), JsValue> {
let window = web_sys::window().expect("no window");
let navigator = window.navigator();
let clipboard = navigator.clipboard();
⋮----
let promise = clipboard.write_text(text);
⋮----
Ok(())
⋮----
pub fn ErrorDisplay(
⋮----
let copied = create_rw_signal(false);
let code = error_code.clone();
⋮----
let copy_action = create_action(move |_: &()| {
let code = code.clone();
⋮----
match copy_to_clipboard(&code).await {
⋮----
copied.set(true);
⋮----
copied.set(false);
⋮----
view! {
⋮----
pub fn ErrorBadge(
```

## File: src/components/feedback_widget.rs
```rust
use leptos::event_target_value;
use leptos::spawn_local;
⋮----
use std::time::Duration;
⋮----
use web_sys::window;
⋮----
pub fn FeedbackWidget() -> impl IntoView {
let (is_open, set_is_open) = create_signal(false);
let (message, set_message) = create_signal(String::new());
let (is_sending, set_is_sending) = create_signal(false);
let (sent_success, set_sent_success) = create_signal(false);
⋮----
let (is_annotating, set_is_annotating) = create_signal(false);
⋮----
set_is_open.set(true);
⋮----
spawn_local(async move {
match captureScreenshot().await {
⋮----
if let Some(data) = val.as_string() {
set_screenshot_data.set(Some(data));
⋮----
set_is_open.set(false);
set_is_annotating.set(false);
set_screenshot_data.set(None);
set_message.set(String::new());
⋮----
if let Some(data) = screenshot_data.get() {
open_annotation_editor(&data);
set_is_annotating.set(true);
⋮----
set_is_sending.set(true);
let msg = message.get();
⋮----
let _window = window().unwrap();
let url = _window.location().href().unwrap_or_default();
let ua = _window.navigator().user_agent().unwrap_or_default();
⋮----
let screenshot = get_annotated_screenshot().or_else(|| screenshot_data.get());
⋮----
set_is_sending.set(false);
set_sent_success.set(true);
⋮----
set_timeout(
⋮----
set_sent_success.set(false);
⋮----
view! {
```

## File: src/components/hint_bubble.rs
```rust
pub fn HintBubble(
⋮----
view! {
⋮----
pub fn use_idle_timer(idle_seconds: u32) -> RwSignal<bool> {
let is_idle = create_rw_signal(false);
⋮----
use wasm_bindgen::JsCast;
⋮----
create_effect(move |_| {
let window = web_sys::window().expect("window");
let document = window.document().expect("document");
⋮----
is_idle.set(false);
⋮----
let _ = document.add_event_listener_with_callback(
⋮----
reset_timer.as_ref().unchecked_ref(),
⋮----
.add_event_listener_with_callback("keydown", reset_timer.as_ref().unchecked_ref());
⋮----
.add_event_listener_with_callback("click", reset_timer.as_ref().unchecked_ref());
⋮----
reset_timer.forget();
⋮----
is_idle.set(true);
⋮----
let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
set_idle.as_ref().unchecked_ref(),
⋮----
set_idle.forget();
```

## File: src/components/hint_manager.rs
```rust
use crate::components::HintBubble;
use crate::onboarding::HINTS;
⋮----
pub fn HintManager() -> impl IntoView {
let state = use_context::<crate::AppState>().expect("AppState");
⋮----
let is_idle = create_rw_signal(false);
let current_hint_idx = create_rw_signal(0usize);
let hint_dismissed = create_rw_signal(false);
⋮----
use wasm_bindgen::JsCast;
⋮----
create_effect(move |_| {
let window = web_sys::window().expect("window");
let document = window.document().expect("document");
⋮----
let timer_clone = timer_id.clone();
⋮----
is_idle.set(false);
hint_dismissed.set(false);
⋮----
let id = *timer_clone.borrow();
⋮----
web_sys::window().unwrap().clear_timeout_with_handle(id);
⋮----
is_idle.set(true);
⋮----
.unwrap()
.set_timeout_with_callback_and_timeout_and_arguments_0(
set_idle.as_ref().unchecked_ref(),
⋮----
.unwrap_or(0);
⋮----
*timer_clone.borrow_mut() = new_id;
set_idle.forget();
⋮----
let _ = document.add_event_listener_with_callback(
⋮----
reset_and_restart.as_ref().unchecked_ref(),
⋮----
reset_and_restart.forget();
⋮----
if hint_dismissed.get() || !is_idle.get() {
⋮----
HINTS.get(current_hint_idx.get())
⋮----
hint_dismissed.set(true);
⋮----
let next = (current_hint_idx.get() + 1) % HINTS.len().max(1);
current_hint_idx.set(next);
⋮----
view! {
```

## File: src/components/input.rs
```rust
pub fn TextInput(
⋮----
let input_type = input_type.unwrap_or_else(|| "text".to_string());
let autocomplete = autocomplete.unwrap_or_else(|| "off".to_string());
let should_autofocus = autofocus.unwrap_or(false);
⋮----
create_effect(move |_| {
if let Some(input) = input_ref.get() {
let _ = input.focus();
⋮----
view! {
⋮----
pub fn Select(
```

## File: src/components/loading.rs
```rust
pub fn FullScreenLoader() -> impl IntoView {
view! {
⋮----
pub fn ReportLoader(#[prop(optional)] include_threat_intel: bool) -> impl IntoView {
⋮----
let current_step = create_rw_signal(0u8);
let message_index = create_rw_signal(0usize);
⋮----
vec![
⋮----
let messages = vec![
⋮----
let steps_len = steps.len();
create_effect(move |_| {
fn schedule_step_advance(current_step: RwSignal<u8>, steps_len: usize) {
set_timeout(
⋮----
if let Some(current) = current_step.try_get() {
⋮----
let _ = current_step.try_set(current + 1);
⋮----
schedule_step_advance(current_step, steps_len);
⋮----
let messages_len = messages.len();
⋮----
fn schedule_message_rotate(message_index: RwSignal<usize>, messages_len: usize) {
⋮----
if let Some(current) = message_index.try_get() {
⋮----
let _ = message_index.try_set(next);
⋮----
schedule_message_rotate(message_index, messages_len);
⋮----
pub fn LoginLoader(#[prop(into)] message: Signal<String>) -> impl IntoView {
⋮----
pub fn Spinner() -> impl IntoView {
```

## File: src/components/mod.rs
```rust
mod button;
mod card;
mod combobox;
mod error;
pub mod feedback_widget;
mod hint_bubble;
mod hint_manager;
mod input;
mod loading;
mod preview_modal;
mod progress_checklist;
pub mod queue_status;
mod sandbox_banner;
mod step_highlight;
mod sync_overlay;
mod tutorial_orchestrator;
mod tutorial_welcome;
```

## File: src/components/preview_modal.rs
```rust
pub struct PreviewData {
⋮----
impl Default for PreviewData {
fn default() -> Self {
⋮----
pub struct StreamingProgress {
⋮----
impl StreamingProgress {
pub fn progress_percentage(&self) -> f64 {
⋮----
let current_step = (self.current_index.saturating_sub(1)) * 2
⋮----
pub fn ThreatHuntingPreviewModal(
⋮----
view! {
```

## File: src/components/progress_checklist.rs
```rust
use crate::UiLanguage;
⋮----
pub fn ProgressChecklist() -> impl IntoView {
let state = use_context::<crate::AppState>().expect("AppState");
⋮----
let (is_expanded, set_is_expanded) = create_signal(false);
⋮----
let progress = create_rw_signal(storage::get_onboarding_progress());
⋮----
progress.set(storage::get_onboarding_progress());
⋮----
let unlocked_count = move || progress.get().unlocked_achievements.len();
⋮----
let total_count = ALL_ACHIEVEMENTS.len();
⋮----
let title_text = move || match ui_language.get() {
⋮----
let completed_text = move || match ui_language.get() {
⋮----
view! {
```

## File: src/components/queue_status.rs
```rust
pub struct QueueJobStatus {
⋮----
pub fn QueueStatusPanel(
⋮----
let _status = create_rw_signal(QueueJobStatus::default());
let is_visible = create_memo(move |_| job_id.get().is_some());
⋮----
create_effect(move |_| {
if let Some(id) = job_id.get() {
⋮----
let poll_code = format!(
⋮----
view! {
⋮----
pub fn QueueIndicator() -> impl IntoView {
let _queue_length = create_rw_signal(0usize);
```

## File: src/components/sandbox_banner.rs
```rust
pub fn SandboxBanner(
⋮----
let state = use_context::<crate::AppState>().expect("AppState");
⋮----
view! {
```

## File: src/components/step_highlight.rs
```rust
pub fn StepHighlight(
⋮----
let tooltip_style = create_rw_signal(String::from("bottom: 100px; left: 50%;"));
⋮----
create_effect(move |_| {
if is_visible.get() {
let selector = target_selector.get();
⋮----
tooltip_style.set(positionTooltip(&selector));
⋮----
view! {
```

## File: src/components/sync_overlay.rs
```rust
pub enum SyncState {
⋮----
pub fn GitHubSyncOverlay(
⋮----
let visible = create_memo(move |_| state.get() != SyncState::Idle);
⋮----
view! {
⋮----
pub fn ColdStartOverlay(
⋮----
let overlay_class = create_memo(move |_| {
if is_ready.get() {
⋮----
} else if is_warming.get() {
```

## File: src/components/tutorial_orchestrator.rs
```rust
pub fn TutorialOrchestrator() -> impl IntoView {
let state = use_context::<crate::AppState>().expect("AppState");
⋮----
let show_welcome = create_rw_signal(false);
let show_tutorial = create_rw_signal(false);
let current_step = create_rw_signal(0usize);
⋮----
create_effect(move |_| {
let progress = get_onboarding_progress();
⋮----
use gloo_timers::callback::Timeout;
⋮----
show_welcome.set(true);
⋮----
timeout.forget();
⋮----
let idx = current_step.get();
TUTORIAL_STEPS.get(idx)
⋮----
show_welcome.set(false);
show_tutorial.set(true);
current_step.set(0);
set_current_step(0);
⋮----
show_tutorial.set(false);
⋮----
let next = current_step.get() + 1;
if next >= TUTORIAL_STEPS.len() {
⋮----
mark_tutorial_complete();
⋮----
current_step.set(next);
set_current_step(next);
⋮----
let welcome_signal: Signal<bool> = Signal::derive(move || show_welcome.get());
let tutorial_signal: Signal<bool> = Signal::derive(move || show_tutorial.get());
⋮----
view! {
```

## File: src/components/tutorial_welcome.rs
```rust
pub fn TutorialWelcomeModal(
⋮----
let state = use_context::<crate::AppState>().expect("AppState");
⋮----
view! {
```

## File: src/onboarding/achievements.rs
```rust
use crate::UiLanguage;
⋮----
pub enum AchievementTrigger {
⋮----
/// Unlocked when visiting a specific page
    PageVisit(&'static str),
⋮----
/// A single achievement/milestone
#[derive(Clone)]
pub struct Achievement {
/// Unique identifier
    pub id: &'static str,
⋮----
/// Trigger condition
    pub trigger: AchievementTrigger,
⋮----
impl Achievement {
/// Get localized title
    pub fn title(&self, lang: UiLanguage) -> &'static str {
⋮----
pub fn title(&self, lang: UiLanguage) -> &'static str {
⋮----
pub fn description(&self, lang: UiLanguage) -> &'static str {
⋮----
/// All registered achievements (extend this list to add more)
pub static ALL_ACHIEVEMENTS: &[Achievement] = &[
⋮----
pub fn unlock_achievement(id: &str) {
use crate::onboarding::storage;
```

## File: src/onboarding/hints.rs
```rust
use crate::UiLanguage;
⋮----
pub enum HintPosition {
⋮----
pub struct Hint {
⋮----
/// CSS selector for target element
    pub target_selector: &'static str,
⋮----
impl Hint {
⋮----
pub fn message(&self, lang: UiLanguage) -> &'static str {
⋮----
// Editor canvas hint
⋮----
// Placeholder hint
⋮----
// Export hint
⋮----
// Shortcut hint
⋮----
/// Get CSS position style for hint bubble placement
    pub fn position_style(&self) -> String {
⋮----
pub fn position_style(&self) -> String {
⋮----
"bottom: 80px; left: 50%; transform: translateX(-50%);".to_string()
⋮----
"top: 80px; left: 50%; transform: translateX(-50%);".to_string()
⋮----
HintPosition::Left => "right: 20px; top: 50%; transform: translateY(-50%);".to_string(),
HintPosition::Right => "left: 20px; top: 50%; transform: translateY(-50%);".to_string(),
⋮----
/// All registered hints (extend to add more)
pub static HINTS: &[Hint] = &[
⋮----
show_after_idle_ms: 15000, // 15 seconds
```

## File: src/onboarding/mod.rs
```rust
pub mod achievements;
pub mod hints;
pub mod sandbox;
pub mod storage;
pub mod tutorial;
```

## File: src/onboarding/sandbox.rs
```rust
pub fn is_sandbox_mode() -> bool {
⋮----
pub fn set_sandbox_mode(enabled: bool) {
⋮----
pub fn get_sandbox_slides() -> Vec<SandboxSlide> {
vec![
⋮----
pub struct SandboxSlide {
⋮----
pub enum SlideElement {
⋮----
impl SlideElement {
pub fn text(
⋮----
pub fn placeholder(key: &'static str, value: &'static str, x: u32, y: u32) -> Self {
⋮----
pub fn metric_card(
⋮----
pub fn evidence_card(label: &'static str, border_color: &'static str, x: u32, y: u32) -> Self {
⋮----
pub fn recommendation(text: &'static str, color: &'static str, x: u32, y: u32) -> Self {
⋮----
pub fn tip(text: &'static str, x: u32, y: u32) -> Self {
⋮----
pub fn get_practice_activities(lang: crate::UiLanguage) -> Vec<PracticeActivity> {
⋮----
crate::UiLanguage::Es => vec![
⋮----
crate::UiLanguage::En => vec![
⋮----
crate::UiLanguage::Pt => vec![
⋮----
pub struct PracticeActivity {
⋮----
pub fn sandbox_banner_text(lang: crate::UiLanguage) -> &'static str {
⋮----
/// Get localized "Exit sandbox" button text
pub fn exit_sandbox_text(lang: crate::UiLanguage) -> &'static str {
⋮----
pub fn exit_sandbox_text(lang: crate::UiLanguage) -> &'static str {
⋮----
pub fn sandbox_welcome_title(lang: crate::UiLanguage) -> &'static str {
⋮----
/// Get localized welcome modal description
pub fn sandbox_welcome_desc(lang: crate::UiLanguage) -> &'static str {
⋮----
pub fn sandbox_welcome_desc(lang: crate::UiLanguage) -> &'static str {
⋮----
pub fn sandbox_start_tutorial(lang: crate::UiLanguage) -> &'static str {
⋮----
/// Get localized "Explore Freely" button text
pub fn sandbox_explore_freely(lang: crate::UiLanguage) -> &'static str {
⋮----
pub fn sandbox_explore_freely(lang: crate::UiLanguage) -> &'static str {
```

## File: src/onboarding/storage.rs
```rust
pub struct OnboardingProgress {
⋮----
pub fn get_onboarding_progress() -> OnboardingProgress {
LocalStorage::get(STORAGE_KEY).unwrap_or_default()
⋮----
pub fn save_onboarding_progress(progress: &OnboardingProgress) {
⋮----
pub fn mark_tutorial_complete() {
let mut progress = get_onboarding_progress();
⋮----
save_onboarding_progress(&progress);
⋮----
pub fn set_current_step(step: usize) {
⋮----
progress.current_step = Some(step);
⋮----
pub fn mark_welcome_seen() {
⋮----
pub fn mark_achievement_unlocked(id: &str) {
⋮----
if !progress.unlocked_achievements.contains(&id.to_string()) {
progress.unlocked_achievements.push(id.to_string());
⋮----
showAchievementToast(id);
⋮----
pub fn is_achievement_unlocked(id: &str) -> bool {
get_onboarding_progress()
⋮----
.contains(&id.to_string())
⋮----
pub fn dismiss_sandbox() {
⋮----
pub fn increment_shortcut_usage() {
⋮----
mark_achievement_unlocked("shortcut_master");
⋮----
pub fn reset_onboarding() {
```

## File: src/onboarding/tutorial.rs
```rust
use crate::UiLanguage;
⋮----
pub enum StepValidation {
⋮----
/// User drags something
    Drag,
/// Auto-complete after viewing for N milliseconds
    Timeout(u32),
⋮----
/// A single tutorial step
#[derive(Clone)]
pub struct TutorialStep {
/// Unique identifier (used for i18n key derivation)
    pub id: &'static str,
⋮----
/// How to validate completion
    pub validation: StepValidation,
/// Position of tooltip relative to target
    pub tooltip_position: &'static str,
⋮----
impl TutorialStep {
⋮----
pub fn title(&self, lang: UiLanguage) -> &'static str {
⋮----
// Step: edit_title
⋮----
// Step: add_placeholder
⋮----
// Step: duplicate
⋮----
// Step: export
⋮----
// Default fallback
⋮----
/// Get localized instruction using i18n key pattern: onboarding_step_{id}_instruction
    pub fn instruction(&self, lang: UiLanguage) -> &'static str {
⋮----
pub fn instruction(&self, lang: UiLanguage) -> &'static str {
```

## File: src/pages/admin_beta.rs
```rust
use gloo_net::http::Request;
⋮----
struct BetaReq {
⋮----
enum AdminTab {
⋮----
pub fn AdminBetaPage() -> impl IntoView {
let (active_tab, set_active_tab) = create_signal(AdminTab::Requests);
⋮----
let (requests, set_requests) = create_signal(Vec::<BetaReq>::new());
⋮----
let (users, set_users) = create_signal(Vec::<AllowedUser>::new());
⋮----
let (error, set_error) = create_signal(Option::<String>::None);
let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
let (loading, set_loading) = create_signal(false);
⋮----
let new_user_email = create_rw_signal(String::new());
let new_user_role = create_rw_signal("beta_tester".to_string());
let new_user_desc = create_rw_signal(String::new());
⋮----
set_loading.set(true);
spawn_local(async move {
let resp = Request::get("/api/admin/beta/requests").send().await;
⋮----
if r.ok() {
let data: Vec<BetaReq> = r.json().await.unwrap_or_default();
set_requests.set(data);
⋮----
set_error.set(Some("Failed to load requests".into()));
⋮----
Err(e) => set_error.set(Some(e.to_string())),
⋮----
set_loading.set(false);
⋮----
Ok(data) => set_users.set(data),
Err(e) => set_error.set(Some(e)),
⋮----
let _ = Request::post(&format!("/api/admin/beta/requests/{}/action", id))
.json(&serde_json::json!({ "action": "approve" }))
.unwrap()
.send()
⋮----
fetch_requests();
⋮----
.json(&serde_json::json!({ "action": "reject" }))
⋮----
let email = new_user_email.get();
let role = new_user_role.get();
let desc = new_user_desc.get();
⋮----
if email.is_empty() {
set_error.set(Some("Email required".into()));
⋮----
set_error.set(None);
set_success_msg.set(None);
⋮----
if desc.is_empty() { None } else { Some(desc) },
⋮----
set_success_msg.set(Some(format!("User {} added successfully", email)));
new_user_email.set("".into());
new_user_desc.set("".into());
fetch_users();
⋮----
let confirmed = window()
.confirm_with_message(&format!("Remove access for {}?", email))
.unwrap_or(false);
⋮----
set_success_msg.set(Some(format!("User {} removed", email)));
⋮----
create_effect(move |_| {
if active_tab.get() == AdminTab::Requests {
⋮----
view! {
```

## File: src/pages/analytics.rs
```rust
pub fn AnalyticsPage() -> impl IntoView {
let state = use_context::<AppState>().expect("AppState not found");
⋮----
let _dict = Signal::derive(move || get_ui_dict(ui_language.get()));
⋮----
let stats = create_resource(|| (), |_| async move { api::get_log_stats(Some(7)).await });
⋮----
view! {
⋮----
fn calculate_success_rate(reports: usize, errors: usize) -> f64 {
⋮----
fn KpiCard(title: &'static str, value: String, color: &'static str) -> impl IntoView {
⋮----
fn Chart(daily_stats: Vec<DailyStats>) -> impl IntoView {
⋮----
.iter()
.map(|d| d.reports.max(d.errors))
.max()
.unwrap_or(10) as f64;
```

## File: src/pages/apply.rs
```rust
use gloo_net::http::Request;
⋮----
enum Step {
⋮----
struct BetaRequestPayload {
⋮----
struct BetaRequestResponse {
⋮----
pub fn BetaApplyPage() -> impl IntoView {
let (step, set_step) = create_signal(Step::Company);
let (company, set_company) = create_signal(String::new());
let (email, set_email) = create_signal(String::new());
let (loading, set_loading) = create_signal(false);
let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
let (success_msg, set_success_msg) = create_signal(String::new());
⋮----
set_loading.set(true);
set_error_msg.set(None);
⋮----
spawn_local(async move {
⋮----
email: email.get(),
company: company.get(),
⋮----
.json(&payload)
.unwrap()
.send()
⋮----
set_loading.set(false);
⋮----
if r.ok() {
⋮----
r.json().await.unwrap_or(BetaRequestResponse {
⋮----
message: "Request received!".into(),
⋮----
set_success_msg.set(data.message);
set_step.set(Step::Success);
⋮----
let text = r.text().await.unwrap_or_default();
set_error_msg.set(Some(format!("Error: {}", text)));
⋮----
Err(e) => set_error_msg.set(Some(e.to_string())),
⋮----
let email_val = email.get();
if email_val.trim().is_empty() {
set_error_msg.set(Some("Please enter your email.".into()));
⋮----
let resp = Request::get(&format!("/api/public/beta-status?email={}", email_val))
⋮----
let status_text = r.text().await.unwrap_or_default();
⋮----
if status_text.contains("approved") {
⋮----
.set("Your access is APPROVED! You can log in now.".into());
⋮----
} else if status_text.contains("pending") {
⋮----
.set("Your request is still PENDING. We'll verify it soon.".into());
⋮----
set_error_msg.set(Some("No request found for this email.".into()));
⋮----
.set(Some("No request found or error checking status.".into()));
⋮----
let next_step = move || match step.get() {
⋮----
if !company.get().trim().is_empty() {
set_step.set(Step::Email);
⋮----
set_error_msg.set(Some("Please enter your company name.".into()));
⋮----
if !email.get().trim().is_empty() && email.get().contains('@') {
submit(());
⋮----
set_error_msg.set(Some("Please enter a valid email.".into()));
⋮----
if ev.key() == "Enter" {
next_step();
⋮----
view! {
```

## File: src/pages/dashboard.rs
```rust
struct AppError {
⋮----
impl AppError {
fn simple(message: impl Into<String>) -> Self {
⋮----
message: message.into(),
⋮----
fn from_response(resp: &GenerateReportResponse) -> Self {
⋮----
code: resp.error_code.clone(),
⋮----
.clone()
.or_else(|| Some(resp.message.clone()))
.unwrap_or_else(|| "Error desconocido".into()),
⋮----
fn slide_toggle(
⋮----
let is_enabled = move || !disabled_slides.get().contains(&plugin_id.to_string());
⋮----
disabled_slides.update(|list| {
if list.contains(&plugin_id.to_string()) {
list.retain(|id| id != plugin_id);
⋮----
list.push(plugin_id.to_string());
⋮----
view! {
⋮----
pub fn DashboardPage() -> impl IntoView {
let state = use_context::<AppState>().expect("AppState not found");
⋮----
let dict = create_memo(move |_| get_ui_dict(ui_language.get()));
⋮----
let tenants = create_rw_signal(Vec::<Tenant>::new());
let selected_tenant = create_rw_signal(String::new());
let user_templates = create_rw_signal(Vec::<TemplateListItem>::new());
let from_date = create_rw_signal(get_default_from_date());
let to_date = create_rw_signal(get_today());
⋮----
let language = create_rw_signal(ui_language.get_untracked().code().to_string());
let story_tag = create_rw_signal(String::new());
let include_threat_intel = create_rw_signal(false);
let use_user_credits = create_rw_signal(false);
let use_plugins = create_rw_signal(false);
⋮----
let plugin_theme = create_rw_signal(crate::load_theme());
let disabled_slides = create_rw_signal(crate::load_disabled_slides());
⋮----
create_effect(move |_| {
let theme = plugin_theme.get();
⋮----
let slides = disabled_slides.get();
⋮----
let loading_tenants = create_rw_signal(true);
let generating = create_rw_signal(false);
let error = create_rw_signal(Option::<AppError>::None);
let report_html = create_rw_signal(Option::<String>::None);
⋮----
let show_preview_modal = create_rw_signal(false);
let preview_loading = create_rw_signal(false);
let preview_data = create_rw_signal(PreviewData::default());
let preview_confirmed = create_rw_signal(false);
let streaming_progress = create_rw_signal(StreamingProgress::default());
⋮----
let exporting_slides = create_rw_signal(false);
⋮----
let exporting_pptx = create_rw_signal(false);
⋮----
let pending_beta_count = create_rw_signal(0i64);
⋮----
spawn_local(async move {
⋮----
tenants.set(t);
⋮----
Err(e) => error.set(Some(AppError::simple(e))),
⋮----
user_templates.set(resp.templates);
⋮----
loading_tenants.set(false);
⋮----
if is_admin.get_untracked() {
⋮----
.send()
⋮----
if resp.ok() {
⋮----
if let Some(count) = json.get("count").and_then(|v| v.as_i64()) {
pending_beta_count.set(count);
⋮----
let tenant_options = create_memo(move |_| {
⋮----
.get()
.iter()
.map(|t| (t.key.clone(), t.name.clone()))
⋮----
let generate_action = create_action(move |_: &()| async move {
if selected_tenant.get().is_empty() {
error.set(Some(AppError {
code: Some("RPT-004".into()),
message: "Selecciona un tenant para continuar".into(),
⋮----
let threat_intel = include_threat_intel.get();
let tag = story_tag.get();
⋮----
if threat_intel && !tag.trim().is_empty() && !preview_confirmed.get() {
show_preview_modal.set(true);
⋮----
streaming_progress.set(StreamingProgress {
⋮----
let tenant = selected_tenant.get();
⋮----
api::get_threat_hunting_stream_url(&tenant, &tag, use_user_credits.get());
⋮----
use wasm_bindgen::JsCast;
⋮----
init.set_with_credentials(true);
⋮----
let sp = streaming_progress.clone();
let pd = preview_data.clone();
let pl = preview_loading.clone();
let _spm = show_preview_modal.clone();
let _err = error.clone();
let es_clone = event_source.clone();
⋮----
if let Some(data) = e.data().as_string() {
⋮----
sp.update(|s| {
⋮----
pd.update(|p| p.tickets_count = total_tickets);
⋮----
pd.set(PreviewData {
⋮----
tickets_count: pd.get_untracked().tickets_count,
⋮----
pl.set(false);
es_clone.close();
⋮----
s.error_message = Some(message.clone());
⋮----
event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
onmessage.forget();
⋮----
let sp_err = streaming_progress.clone();
let pl_err = preview_loading.clone();
let es_err = event_source.clone();
⋮----
sp_err.update(|s| {
s.error_message = Some("Connection error".into());
⋮----
pl_err.set(false);
es_err.close();
⋮----
event_source.set_onerror(Some(onerror.as_ref().unchecked_ref()));
onerror.forget();
⋮----
error.set(Some(AppError::simple("Failed to create EventSource")));
show_preview_modal.set(false);
⋮----
generating.set(true);
error.set(None);
report_html.set(None);
preview_confirmed.set(false);
⋮----
let from = from_date.get();
let to = to_date.get();
let lang = language.get();
let tag_opt = if tag.trim().is_empty() {
⋮----
Some(tag.as_str())
⋮----
let template_id = selected_template.get();
⋮----
template_id.as_deref(),
use_plugins.get(),
⋮----
let report = report_html.clone();
let err = error.clone();
let gen = generating.clone();
⋮----
if let Some(data) = event.data().as_string() {
⋮----
report.set(Some(html));
gen.set(false);
⋮----
err.set(Some(AppError {
code: Some(code),
⋮----
let err_clone = error.clone();
let gen_clone = generating.clone();
⋮----
err_clone.set(Some(AppError::simple(
⋮----
gen_clone.set(false);
⋮----
error.set(Some(AppError::simple(
⋮----
generating.set(false);
⋮----
preview_confirmed.set(true);
generate_action.dispatch(());
⋮----
let logout_action = create_action(move |_: &()| async move {
⋮----
is_authenticated.set(false);
current_page.set(Page::Login);
⋮----
fn get_today() -> String {
⋮----
format!(
⋮----
fn get_default_from_date() -> String {
⋮----
date.set_time(date.get_time() - thirty_days_ms);
⋮----
fn download_html(content: &str, filename: &str) {
⋮----
let window = web_sys::window().unwrap();
let document = window.document().unwrap();
⋮----
options.set_type("text/html");
⋮----
&js_sys::Array::of1(&content.into()),
⋮----
.unwrap();
⋮----
let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
⋮----
let a = document.create_element("a").unwrap();
a.set_attribute("href", &url).unwrap();
a.set_attribute("download", filename).unwrap();
a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
⋮----
web_sys::Url::revoke_object_url(&url).unwrap();
⋮----
fn parse_html_to_slides(html: &str) -> Vec<api::ExportSlideData> {
⋮----
for line in html.lines() {
let line = line.trim();
⋮----
if line.contains("<section") || line.contains("class=\"slide\"") {
⋮----
if !current_title.is_empty() || !current_body.is_empty() {
slides.push(api::ExportSlideData {
title: if current_title.is_empty() {
format!("Slide {}", slides.len() + 1)
⋮----
current_title.clone()
⋮----
body: current_body.clone(),
layout: Some("TITLE_AND_BODY".to_string()),
⋮----
if line.contains("<h1") || line.contains("<h2") {
⋮----
if let Some(start) = line.find('>') {
if let Some(end) = line.find("</h") {
⋮----
.replace("<span>", "")
.replace("</span>", "")
.replace("<strong>", "")
.replace("</strong>", "")
.trim()
.to_string();
⋮----
// Extract paragraphs as body text
if line.contains("<p") && line.contains("</p>") {
⋮----
if let Some(end) = line.rfind("</p>") {
⋮----
.replace("<span", "")
⋮----
.replace("<em>", "")
.replace("</em>", "")
.chars()
.filter(|c| !c.is_control())
⋮----
if !clean_text.is_empty() && clean_text.len() > 5 {
current_body.push(clean_text);
⋮----
// Don't forget the last slide
⋮----
if slides.is_empty() {
⋮----
title: "Threat Intelligence Report".to_string(),
body: vec!["Report generated by Axur Web".to_string()],
layout: Some("TITLE".to_string()),
```

## File: src/pages/editor.rs
```rust
use crate::api;
use crate::storage;
⋮----
pub struct EditorSlide {
⋮----
fn sandbox_slides_to_canvas_json(slide: &crate::onboarding::SandboxSlide) -> String {
⋮----
json.to_string()
⋮----
pub fn EditorPage() -> impl IntoView {
⋮----
let slides = create_rw_signal(vec![EditorSlide {
⋮----
let current_slide_idx = create_rw_signal(0usize);
let show_placeholder_library = create_rw_signal(false);
let template_name = create_rw_signal("Untitled Template".to_string());
let is_saving = create_rw_signal(false);
let is_exporting_slides = create_rw_signal(false);
let preview_mode = create_rw_signal(false);
⋮----
let is_sandbox = create_rw_signal(crate::onboarding::is_sandbox_mode());
⋮----
let has_unsaved_changes = create_rw_signal(false);
let show_template_list = create_rw_signal(false);
let available_templates = create_rw_signal::<Vec<api::TemplateListItem>>(vec![]);
let is_loading = create_rw_signal(false);
⋮----
let clone_id = state.editor_clone_from_id.get();
let edit_id = state.editor_template_id.get();
⋮----
(Some(cid), true)
⋮----
(Some(eid), false)
⋮----
let target_id_clone = target_id.clone();
⋮----
create_effect(move |prev_ran: Option<bool>| {
⋮----
if prev_ran.unwrap_or(false) {
⋮----
let tid_opt = target_id_clone.clone();
set_timeout(
⋮----
init_fabric_canvas("editor-canvas");
⋮----
if is_sandbox.get() {
⋮----
.iter()
.enumerate()
.map(|(i, s)| EditorSlide {
id: s.id.to_string(),
name: s.name.to_string(),
canvas_json: sandbox_slides_to_canvas_json(s),
⋮----
.collect();
⋮----
if !new_slides.is_empty() {
slides.set(new_slides);
current_slide_idx.set(0);
template_name.set("Demo Template (Sandbox)".to_string());
if let Some(first) = slides.get().first() {
load_canvas_json(&first.canvas_json);
⋮----
log("Sandbox mode: Loaded demo slides");
⋮----
is_loading.set(true);
spawn_local(async move {
⋮----
template_id.set(Some(tmpl.id.clone()));
⋮----
template_name.set(tmpl.name.clone());
⋮----
.get("id")
.and_then(|v| v.as_str())
.unwrap_or(&format!("slide-{}", i + 1))
.to_string(),
⋮----
.get("name")
⋮----
.unwrap_or(&format!("Slide {}", i + 1))
⋮----
.get("canvas_json")
⋮----
.unwrap_or("{}")
⋮----
log(&format!("Loaded template: {}", tmpl.name));
⋮----
Err(e) => log(&format!("Failed to load template: {}", e)),
⋮----
is_loading.set(false);
⋮----
state.editor_template_id.set(None);
⋮----
if has_unsaved_changes.get() {
⋮----
state.current_page.set(crate::Page::Dashboard);
⋮----
slides.update(|s| {
let num = s.len() + 1;
s.push(EditorSlide {
id: format!("slide-{}", num),
name: format!("Slide {}", num),
canvas_json: "{}".to_string(),
⋮----
current_slide_idx.set(slides.get().len() - 1);
clear_canvas();
⋮----
if let Some(slide) = s.get_mut(current_slide_idx.get()) {
slide.canvas_json = get_canvas_json();
⋮----
current_slide_idx.set(idx);
if let Some(slide) = slides.get().get(idx) {
load_canvas_json(&slide.canvas_json);
⋮----
if slides.get().len() > 1 {
let idx = current_slide_idx.get();
⋮----
s.remove(idx);
⋮----
current_slide_idx.set(idx - 1);
⋮----
if let Some(slide) = slides.get().get(current_slide_idx.get()) {
⋮----
has_unsaved_changes.set(true);
⋮----
let current_slides = slides.get();
if let Some(current) = current_slides.get(idx) {
⋮----
let canvas_json = get_canvas_json();
let current_name = current.name.clone();
⋮----
s.insert(
⋮----
name: format!("{} (copy)", current_name),
⋮----
current_slide_idx.set(idx + 1);
⋮----
undo_canvas();
⋮----
redo_canvas();
⋮----
duplicate_object();
⋮----
add_text_to_canvas("Edit this text");
⋮----
add_shape_to_canvas("rectangle");
⋮----
add_shape_to_canvas("circle");
⋮----
show_placeholder_library.update(|v| *v = !*v);
⋮----
add_placeholder_to_canvas(&key, &html);
show_placeholder_library.set(false);
⋮----
is_saving.set(true);
⋮----
if let Some(tid) = template_id.get() {
⋮----
slides.get().iter().map(|s| s.canvas_json.clone()).collect();
⋮----
log(&format!("Version saved locally for template {}", tid));
⋮----
let name = template_name.get();
⋮----
.get()
⋮----
.map(|s| {
⋮----
let thumbnail = get_canvas_thumbnail();
let thumbnail_opt = if thumbnail.is_empty() {
⋮----
Some(thumbnail)
⋮----
.and_then(|w| js_sys::Reflect::get(&w, &"PPTXImporter".into()).ok())
.and_then(|imp| js_sys::Reflect::get(&imp, &"state".into()).ok())
.and_then(|st| js_sys::Reflect::get(&st, &"originalFile".into()).ok())
.and_then(|f| {
use wasm_bindgen::JsCast;
f.dyn_into::<web_sys::File>().ok()
⋮----
let existing_id = template_id.get();
let is_new = existing_id.is_none();
⋮----
existing_id.as_deref(),
⋮----
log(&format!("Template saved! ID: {:?}", resp.template_id));
⋮----
template_id.set(Some(new_id));
⋮----
has_unsaved_changes.set(false);
⋮----
show_csat();
⋮----
log(&format!("Save failed: {}", resp.message));
⋮----
log(&format!("Save error: {}", e));
⋮----
is_saving.set(false);
⋮----
view! {
⋮----
fn CollapsibleSection(
⋮----
/// Section title
    title: &'static str,
⋮----
/// Whether default is expanded
    #[prop(default = true)]
⋮----
/// Children content
    children: Children,
⋮----
// Read initial state from localStorage
let key = format!("sidebar_{}_expanded", storage_key);
⋮----
.and_then(|w| w.local_storage().ok().flatten())
.and_then(|s| s.get_item(&key).ok().flatten())
.map(|v| v == "true")
.unwrap_or(default_expanded);
⋮----
let expanded = create_rw_signal(initial_expanded);
let key_for_toggle = key.clone();
⋮----
let new_val = !expanded.get();
expanded.set(new_val);
⋮----
if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
let _ = storage.set_item(&key_for_toggle, if new_val { "true" } else { "false" });
⋮----
let children_view = children();
⋮----
fn PlaceholderLibraryModal<C, I>(on_close: C, on_insert: I) -> impl IntoView
⋮----
let category_colors = vec![
⋮----
let placeholders = vec![
⋮----
let selected_category = create_rw_signal(0usize);
⋮----
let search_query = create_rw_signal(String::new());
⋮----
let close_click = on_close.clone();
⋮----
let on_insert = on_insert.clone();
⋮----
if let Some((key, _, html)) = selected_placeholder.get() {
on_insert(key, html);
⋮----
let placeholders = placeholders.clone();
⋮----
let query = search_query.get().to_lowercase();
if query.is_empty() {
⋮----
let cat_idx = selected_category.get();
⋮----
.get(cat_idx)
.map(|(_, items)| items.clone())
.unwrap_or_default()
⋮----
.flat_map(|(_, items)| items.iter())
.filter(|(key, name, _)| {
key.to_lowercase().contains(&query) || name.to_lowercase().contains(&query)
⋮----
.cloned()
⋮----
fn PlaceholderSidePanel<I>(on_insert: I) -> impl IntoView
⋮----
// Flat placeholder data: (icon, color, key, label)
let items: Vec<(&'static str, &'static str, &'static str, &'static str)> = vec![
⋮----
let favorites = create_rw_signal(storage::load_favorites());
⋮----
fn TemplateAnalysisPanel(slides: RwSignal<Vec<EditorSlide>>) -> impl IntoView {
⋮----
let used_categories = create_rw_signal::<Vec<String>>(vec![]);
let placeholder_count = create_rw_signal(0usize);
let slide_count = create_rw_signal(0usize);
let has_analyzed = create_rw_signal(false);
⋮----
let slides_vec = slides.get();
⋮----
for slide in slides_vec.iter() {
js_array.push(&wasm_bindgen::JsValue::from_str(&slide.canvas_json));
⋮----
slide_count.set(slides_vec.len());
⋮----
let result = analyze_template_data(js_array.into());
⋮----
let used: Vec<String> = used_arr.iter().filter_map(|v| v.as_string()).collect();
used_categories.set(used);
⋮----
placeholder_count.set(placeholders_arr.length() as usize);
⋮----
has_analyzed.set(true);
⋮----
fn VersionHistoryPanel(
⋮----
let versions = create_rw_signal::<Vec<storage::TemplateVersion>>(vec![]);
let is_expanded = create_rw_signal(false);
⋮----
is_expanded.update(|v| *v = !*v);
if is_expanded.get() {
⋮----
versions.set(storage::load_template_versions(&tid));
⋮----
.map(|(i, json)| EditorSlide {
id: format!("slide-{}", i + 1),
name: format!("Slide {}", i + 1),
canvas_json: json.clone(),
⋮----
slides.set(restored_slides);
⋮----
log(&format!("Restored to version {}", version_num));
```

## File: src/pages/login.rs
```rust
use crate::api;
⋮----
enum LoginStep {
⋮----
enum LoginView {
⋮----
pub fn LoginPage() -> impl IntoView {
let state = use_context::<AppState>().expect("AppState not found");
⋮----
let email = create_rw_signal(String::new());
let password = create_rw_signal(String::new());
let tfa_code = create_rw_signal(String::new());
⋮----
let step = create_rw_signal(LoginStep::Credentials);
let loading = create_rw_signal(false);
let error = create_rw_signal(Option::<String>::None);
⋮----
let temp_token = create_rw_signal(String::new());
let correlation = create_rw_signal(Option::<String>::None);
let device_id = create_rw_signal(String::new());
⋮----
let dict = create_memo(move |_| get_ui_dict(ui_language.get()));
⋮----
let credentials_action = create_action(move |_: &()| async move {
let email_val = email.try_get().unwrap_or_default();
let password_val = password.try_get().unwrap_or_default();
let d = dict.get();
⋮----
if email_val.is_empty() || password_val.is_empty() {
let _ = error.try_set(Some(d.email_password_required.to_string()));
⋮----
let _ = loading.try_set(true);
let _ = error.try_set(None);
⋮----
temp_token.set(token);
⋮----
correlation.set(resp.correlation);
loading.set(false);
step.set(LoginStep::TwoFactor);
⋮----
error.set(Some(resp.message));
⋮----
error.set(Some(e));
⋮----
let tfa_action = create_action(move |_: &()| {
⋮----
if tfa_code.get().is_empty() {
error.set(Some(d.tfa_required.to_string()));
⋮----
loading.set(true);
error.set(None);
⋮----
let code = tfa_code.get();
let token = temp_token.get();
let corr = correlation.get();
let email_val = email.get();
let password_val = password.get();
⋮----
temp_token.set(t);
⋮----
device_id.set(did);
⋮----
step.set(LoginStep::Finalizing);
⋮----
let token_val = temp_token.get();
let corr_val = correlation.get();
let device_val = device_id.get();
⋮----
state.user_email.set(Some(email_val.clone()));
⋮----
state.is_admin.set(perms.is_admin);
state.has_log_access.set(perms.has_log_access);
⋮----
is_authenticated.set(true);
current_page.set(Page::Dashboard);
⋮----
let (view_state, set_view_state) = create_signal(LoginView::Landing);
⋮----
view! {
```

## File: src/pages/logs.rs
```rust
pub fn LogsPage() -> impl IntoView {
⋮----
let state = use_context::<crate::AppState>().expect("AppState not found");
⋮----
let dict = move || crate::get_ui_dict(state.ui_language.get());
⋮----
let logs = create_rw_signal(Vec::<LogEntry>::new());
let categories = create_rw_signal(Vec::<String>::new());
let selected_date = create_rw_signal(get_today_date());
let selected_category = create_rw_signal(String::new());
let search_query = create_rw_signal(String::new());
let loading = create_rw_signal(false);
let error = create_rw_signal(Option::<String>::None);
let total_count = create_rw_signal(0usize);
⋮----
let selected_log = create_rw_signal(Option::<LogEntry>::None);
let log_content = create_rw_signal(String::new());
let loading_content = create_rw_signal(false);
⋮----
let date = selected_date.get();
spawn_local(async move {
match api::list_log_categories(Some(&date)).await {
⋮----
categories.set(resp.categories);
⋮----
categories.set(vec![]);
⋮----
let category = selected_category.get();
⋮----
loading.set(true);
error.set(None);
⋮----
let cat_ref = if category.is_empty() {
⋮----
Some(category.as_str())
⋮----
match api::list_logs(Some(&date), cat_ref, Some(100), None).await {
⋮----
total_count.set(resp.total);
logs.set(resp.files);
⋮----
error.set(Some(resp.message));
⋮----
error.set(Some(e));
⋮----
loading.set(false);
⋮----
selected_log.set(Some(log.clone()));
loading_content.set(true);
log_content.set(String::new());
⋮----
let path = log.path.clone();
⋮----
let d = crate::get_ui_dict(state.ui_language.get_untracked());
⋮----
log_content.set(resp.content);
⋮----
log_content.set(format!("{}{}", d.logs_error_prefix, resp.content));
⋮----
log_content.set(format!("{}{}", d.logs_failed_load, e));
⋮----
loading_content.set(false);
⋮----
selected_log.set(None);
⋮----
create_effect(move |_| {
load_categories();
load_logs();
⋮----
create_effect(move |prev: Option<String>| {
⋮----
if prev.is_some() && prev.as_ref() != Some(&date) {
⋮----
let cat = selected_category.get();
if prev.is_some() {
⋮----
let query = search_query.get().to_lowercase();
if query.is_empty() {
logs.get()
⋮----
.into_iter()
.filter(|log| log.name.to_lowercase().contains(&query))
.collect()
⋮----
view! {
⋮----
// Search box
⋮----
fn get_today_date() -> String {
⋮----
let year = now.get_full_year();
let month = now.get_month() + 1;
let day = now.get_date();
format!("{}/{:02}/{:02}", year, month, day)
⋮----
fn format_size(bytes: u64) -> String {
⋮----
format!("{} B", bytes)
⋮----
format!("{:.1} KB", bytes as f64 / 1024.0)
⋮----
format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
⋮----
fn get_category_badge(name: &str) -> &'static str {
if name.contains("error") {
⋮----
} else if name.contains("request") {
⋮----
} else if name.contains("response") {
⋮----
} else if name.contains("performance") {
⋮----
} else if name.contains("feature") {
```

## File: src/pages/marketplace.rs
```rust
struct MarketplaceCard {
⋮----
pub fn MarketplacePage() -> impl IntoView {
⋮----
let dict = move || crate::get_ui_dict(state.ui_language.get());
⋮----
let templates = create_rw_signal(vec![
⋮----
let is_loading = create_rw_signal(false);
let search_query = create_rw_signal(String::new());
let filter_featured = create_rw_signal(false);
let selected_category = create_rw_signal("All".to_string());
⋮----
state.current_page.set(crate::Page::Dashboard);
⋮----
state.editor_template_id.set(None);
state.current_page.set(crate::Page::Editor);
⋮----
let d = dict();
⋮----
"All" => d.cat_all.to_string(),
"Executive" => d.cat_exec.to_string(),
"Technical" => d.cat_tech.to_string(),
"Compliance" => d.cat_comp.to_string(),
"Risk" => d.cat_risk.to_string(),
"Custom" => d.cat_custom.to_string(),
_ => cat.to_string(),
⋮----
let query = search_query.get().to_lowercase();
let featured_only = filter_featured.get();
let category = selected_category.get();
⋮----
.get()
.into_iter()
.filter(|t| {
let matches_search = query.is_empty()
|| t.name.to_lowercase().contains(&query)
⋮----
.as_ref()
.map(|d| d.to_lowercase().contains(&query))
.unwrap_or(false);
⋮----
let categories_list = vec![
⋮----
view! {
⋮----
fn TemplateCard<P>(template: MarketplaceCard, on_preview: P) -> impl IntoView
⋮----
let template_id = template.id.clone();
let category = template.category.clone();
⋮----
let author_name = template.author.clone();
⋮----
dict().mkt_author_axur
⋮----
dict().mkt_author_community
⋮----
state.editor_clone_from_id.set(Some(template_id.clone()));
```

## File: src/pages/mod.rs
```rust
mod analytics;
mod dashboard;
mod editor;
mod login;
mod logs;
mod marketplace;
⋮----
mod admin_beta;
mod apply;
mod onboarding;
⋮----
pub use admin_beta::AdminBetaPage;
pub use analytics::AnalyticsPage;
pub use apply::BetaApplyPage;
pub use dashboard::DashboardPage;
pub use editor::EditorPage;
pub use login::LoginPage;
pub use logs::LogsPage;
pub use marketplace::MarketplacePage;
pub use onboarding::OnboardingPage;
```

## File: src/pages/onboarding.rs
```rust
pub fn OnboardingPage() -> impl IntoView {
let app_state = use_context::<crate::AppState>().expect("state");
let (selected_skill, set_selected_skill) = create_signal(Option::<String>::None);
⋮----
app_state.current_page.set(crate::Page::Dashboard);
⋮----
app_state.ui_language.set(lang);
⋮----
let dict = move || crate::get_ui_dict(app_state.ui_language.get());
⋮----
let d = dict();
vec![
⋮----
view! {
```

## File: src/api.rs
```rust
use gloo_net::http::Request;
⋮----
const API_BASE: &str = match option_env!("API_BASE_URL") {
⋮----
None => "", // Empty = relative URLs, works with Cloudflare proxy
⋮----
// ========================
// REQUEST/RESPONSE TYPES
⋮----
pub struct LoginRequest {
⋮----
pub struct LoginResponse {
⋮----
pub struct TwoFactorRequest {
⋮----
pub struct TwoFactorResponse {
⋮----
pub struct FinalizeRequest {
⋮----
pub struct FinalizeResponse {
⋮----
pub struct ValidateResponse {
⋮----
pub struct Tenant {
⋮----
pub struct GenerateReportRequest {
⋮----
/// Use the new plugin-based report generation
    #[serde(default)]
⋮----
/// Theme mode: "dark", "light", or "auto"
    #[serde(default)]
⋮----
pub struct GenerateReportResponse {
⋮----
pub async fn login(email: &str, password: &str) -> Result<LoginResponse, String> {
let url = format!("{}/api/auth/login", API_BASE);
⋮----
.header("Content-Type", "application/json")
.json(&LoginRequest {
email: email.to_string(),
password: password.to_string(),
⋮----
.map_err(|e| e.to_string())?
.send()
⋮----
.map_err(|e| e.to_string())?;
⋮----
if resp.ok() {
let text = resp.text().await.map_err(|e| e.to_string())?;
serde_json::from_str(&text).map_err(|e| e.to_string())
⋮----
let status = resp.status();
let text = resp.text().await.unwrap_or_default();
⋮----
if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
return Err(msg.to_string());
⋮----
if let Some(err) = json.get("error").and_then(|v| v.as_str()) {
return Err(err.to_string());
⋮----
Err(format!("Login failed: {} - {}", status, text))
⋮----
pub async fn verify_2fa(
⋮----
let resp = Request::post(&format!("{}/api/auth/2fa", API_BASE))
⋮----
.json(&TwoFactorRequest {
code: code.to_string(),
token: token.to_string(),
⋮----
resp.json().await.map_err(|e| e.to_string())
⋮----
Err(format!("2FA failed: {}", resp.status()))
⋮----
pub async fn finalize(
⋮----
let resp = Request::post(&format!("{}/api/auth/finalize", API_BASE))
⋮----
.credentials(web_sys::RequestCredentials::Include)
.json(&FinalizeRequest {
⋮----
device_id: device_id.to_string(),
⋮----
Err(format!("Finalize failed: {}", resp.status()))
⋮----
pub async fn validate_session() -> Result<ValidateResponse, String> {
let resp = Request::get(&format!("{}/api/auth/validate", API_BASE))
⋮----
Ok(ValidateResponse {
⋮----
message: "Session invalid".into(),
⋮----
pub async fn logout() -> Result<(), String> {
Request::post(&format!("{}/api/auth/logout", API_BASE))
⋮----
Ok(())
⋮----
pub struct HealthCheckResponse {
⋮----
pub async fn health_check() -> Result<f64, String> {
⋮----
let resp = Request::get(&format!("{}/api/health", API_BASE))
⋮----
Ok(elapsed)
⋮----
Err(format!("Health check failed: {}", resp.status()))
⋮----
pub async fn list_tenants() -> Result<Vec<Tenant>, String> {
let resp = Request::get(&format!("{}/api/tenants", API_BASE))
⋮----
Err(format!("Failed to fetch tenants: {}", resp.status()))
⋮----
pub async fn generate_report(
⋮----
let resp = Request::post(&format!("{}/api/report/generate", API_BASE))
⋮----
.json(&GenerateReportRequest {
tenant_id: tenant_id.to_string(),
from_date: from_date.to_string(),
to_date: to_date.to_string(),
language: language.to_string(),
⋮----
Err(format!("Failed to generate report: {}", resp.status()))
⋮----
pub struct ThreatHuntingPreviewRequest {
⋮----
pub struct ThreatHuntingPreviewResponse {
⋮----
pub struct ThreatHuntingPreview {
⋮----
pub async fn request_threat_hunting_preview(
⋮----
let resp = Request::post(&format!("{}/api/threat-hunting/preview", API_BASE))
⋮----
.json(&ThreatHuntingPreviewRequest {
⋮----
story_tag: story_tag.to_string(),
⋮----
Err(format!("Failed to get preview: {}", resp.status()))
⋮----
pub enum ThreatHuntingStreamEvent {
⋮----
pub fn get_threat_hunting_stream_url(
⋮----
format!(
⋮----
fn urlencoding_encode(s: &str) -> String {
s.chars()
.map(|c| match c {
' ' => "%20".to_string(),
'&' => "%26".to_string(),
'=' => "%3D".to_string(),
_ => c.to_string(),
⋮----
.collect()
⋮----
pub enum ReportStreamEvent {
⋮----
pub fn get_report_stream_url(
⋮----
let mut url = format!(
⋮----
if !tag.is_empty() {
url.push_str(&format!("&story_tag={}", urlencoding_encode(tag)));
⋮----
url.push_str(&format!("&template_id={}", urlencoding_encode(tid)));
⋮----
pub struct FeedbackRequest {
⋮----
pub struct FeedbackResponse {
⋮----
pub async fn submit_feedback(
⋮----
let resp = Request::post(&format!("{}/api/feedback", API_BASE))
⋮----
.json(&FeedbackRequest {
⋮----
Err(format!("Failed to submit feedback: {}", resp.status()))
⋮----
pub struct LogEntry {
⋮----
pub struct ListLogsResponse {
⋮----
pub struct LogContentResponse {
⋮----
pub struct ListCategoriesResponse {
⋮----
pub async fn list_logs(
⋮----
let mut url = format!("{}/api/logs", API_BASE);
⋮----
params.push(format!("date={}", urlencoding_encode(d)));
⋮----
params.push(format!("category={}", urlencoding_encode(c)));
⋮----
params.push(format!("limit={}", l));
⋮----
params.push(format!("offset={}", o));
⋮----
if !params.is_empty() {
url = format!("{}?{}", url, params.join("&"));
⋮----
Err(format!("Failed to list logs: {}", resp.status()))
⋮----
pub async fn get_log_content(path: &str) -> Result<LogContentResponse, String> {
let url = format!("{}/api/logs/content/{}", API_BASE, path);
⋮----
Err(format!("Failed to get log content: {}", resp.status()))
⋮----
pub async fn list_log_categories(date: Option<&str>) -> Result<ListCategoriesResponse, String> {
let mut url = format!("{}/api/logs/categories", API_BASE);
⋮----
url = format!("{}?date={}", url, urlencoding_encode(d));
⋮----
Err(format!("Failed to list categories: {}", resp.status()))
⋮----
pub struct LogAccessResponse {
⋮----
pub async fn check_log_access(email: &str) -> Result<bool, String> {
let url = format!(
⋮----
let data: LogAccessResponse = resp.json().await.map_err(|e| e.to_string())?;
Ok(data.has_access)
⋮----
Ok(false)
⋮----
pub struct DailyStats {
⋮----
pub struct StatsResponse {
⋮----
pub async fn get_log_stats(days: Option<i64>) -> Result<StatsResponse, String> {
let mut url = format!("{}/api/logs/stats", API_BASE);
⋮----
url = format!("{}?days={}", url, d);
⋮----
Err(format!("Failed to get stats: {}", resp.status()))
⋮----
pub struct SaveTemplateRequest {
⋮----
pub struct SaveTemplateResponse {
⋮----
pub struct TemplateListItem {
⋮----
pub struct ListTemplatesResponse {
⋮----
pub struct TemplateDetail {
⋮----
pub struct GetTemplateResponse {
⋮----
pub async fn save_template(
⋮----
Some(id) => format!("{}/api/templates/{}", API_BASE, id),
None => format!("{}/api/templates", API_BASE),
⋮----
let method = if template_id.is_some() { "PUT" } else { "POST" };
⋮----
name: name.to_string(),
description: description.map(|s| s.to_string()),
⋮----
let form_data = web_sys::FormData::new().map_err(|_| "Failed to create FormData")?;
⋮----
let json_str = serde_json::to_string(&body_struct).map_err(|e| e.to_string())?;
let _ = form_data.append_with_str("data", &json_str);
⋮----
let _ = form_data.append_with_blob("file", &f);
⋮----
.body(form_data)
⋮----
Err(format!("Failed to save template: {}", resp.status()))
⋮----
pub async fn list_templates() -> Result<ListTemplatesResponse, String> {
let resp = Request::get(&format!("{}/api/templates", API_BASE))
⋮----
Err(format!("Failed to list templates: {}", resp.status()))
⋮----
pub async fn get_template(template_id: &str) -> Result<GetTemplateResponse, String> {
let resp = Request::get(&format!("{}/api/templates/{}", API_BASE, template_id))
⋮----
Err(format!("Failed to get template: {}", resp.status()))
⋮----
pub async fn delete_template(template_id: &str) -> Result<bool, String> {
let resp = Request::delete(&format!("{}/api/templates/{}", API_BASE, template_id))
⋮----
Ok(resp.ok())
⋮----
pub struct ExportSlideData {
⋮----
pub struct ExportToSlidesRequest {
⋮----
pub struct ExportToSlidesResponse {
⋮----
pub async fn export_to_slides(
⋮----
let resp = Request::post(&format!("{}/api/export/slides", API_BASE))
⋮----
.json(&ExportToSlidesRequest {
title: title.to_string(),
⋮----
Err(format!(
⋮----
use std::collections::HashMap;
⋮----
pub struct GeneratePptxResponse {
⋮----
pub struct PptxSlideEdit {
⋮----
pub async fn generate_pptx_report(
⋮----
.append_with_blob("file", &pptx_file)
.map_err(|_| "Failed to append file")?;
⋮----
let edits_json = serde_json::to_string(&edits).map_err(|e| e.to_string())?;
⋮----
.append_with_str("edits", &edits_json)
.map_err(|_| "Failed to append edits")?;
⋮----
let values_json = serde_json::to_string(&placeholder_values).map_err(|e| e.to_string())?;
⋮----
.append_with_str("placeholder_values", &values_json)
.map_err(|_| "Failed to append placeholder_values")?;
⋮----
let resp = Request::post(&format!("{}/api/export/generate-pptx", API_BASE))
⋮----
Err(format!("PPTX generation failed ({}): {}", status, text))
⋮----
pub struct GetTemplatePptxResponse {
⋮----
pub async fn get_template_pptx(template_id: &str) -> Result<GetTemplatePptxResponse, String> {
let resp = Request::get(&format!("{}/api/templates/{}/pptx", API_BASE, template_id))
⋮----
pub fn download_base64_file(base64_data: &str, filename: &str, mime_type: &str) {
use wasm_bindgen::JsCast;
⋮----
let decoded_string = match window.atob(base64_data) {
⋮----
let bytes: Vec<u8> = decoded_string.chars().map(|c| c as u8).collect();
⋮----
let array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
array.copy_from(&bytes);
⋮----
parts.push(&array.buffer());
⋮----
options.set_type(mime_type);
⋮----
if let Some(document) = window.document() {
if let Ok(element) = document.create_element("a") {
⋮----
anchor.set_href(&url);
anchor.set_download(filename);
anchor.click();
⋮----
pub struct AllowedUser {
⋮----
struct AddUserRequest {
⋮----
pub async fn list_users() -> Result<Vec<AllowedUser>, String> {
let resp = Request::get(&format!("{}/api/admin/users", API_BASE))
⋮----
Err(format!("Failed to list users: {}", resp.status()))
⋮----
pub async fn add_user(email: &str, role: &str, description: Option<String>) -> Result<(), String> {
let resp = Request::post(&format!("{}/api/admin/users", API_BASE))
⋮----
.json(&AddUserRequest {
⋮----
role: role.to_string(),
⋮----
Err(format!("Failed to add user: {} - {}", resp.status(), text))
⋮----
pub async fn remove_user(email: &str) -> Result<(), String> {
let url = format!("{}/api/admin/users/{}", API_BASE, urlencoding_encode(email));
```

## File: src/i18n.rs
```rust
pub enum UiLanguage {
⋮----
impl UiLanguage {
pub fn code(&self) -> &'static str {
⋮----
pub fn display_name(&self) -> &'static str {
⋮----
pub fn from_code(code: &str) -> Self {
match code.to_lowercase().as_str() {
⋮----
pub fn all() -> Vec<UiLanguage> {
vec![UiLanguage::Es, UiLanguage::En, UiLanguage::Pt]
⋮----
pub struct UiDict {
⋮----
// Landing page
⋮----
// 2FA
⋮----
// Finalizing
⋮----
// Common
⋮----
// Onboarding
⋮----
// Skills
⋮----
// Sandbox Mode
⋮----
// Marketplace
⋮----
// Logs
⋮----
pub fn get_ui_dict(lang: UiLanguage) -> UiDict {
```

## File: src/lib.rs
```rust
mod api;
mod components;
mod i18n;
pub mod onboarding;
mod pages;
mod storage;
⋮----
pub struct AppState {
⋮----
pub enum Page {
⋮----
impl Default for AppState {
fn default() -> Self {
⋮----
is_authenticated: create_rw_signal(false),
current_page: create_rw_signal(Page::Login),
error_message: create_rw_signal(None),
ui_language: create_rw_signal(UiLanguage::from_code(&storage::load_ui_language())),
user_email: create_rw_signal(None),
has_log_access: create_rw_signal(false),
editor_template_id: create_rw_signal(None),
editor_clone_from_id: create_rw_signal(None),
is_admin: create_rw_signal(false),
⋮----
pub fn App() -> impl IntoView {
⋮----
provide_context(state.clone());
⋮----
let is_warming = create_rw_signal(true);
let is_ready = create_rw_signal(false);
⋮----
create_effect(move |_| {
let lang = ui_lang.get();
storage::save_ui_language(lang.code());
⋮----
let is_warming = is_warming.clone();
let is_ready = is_ready.clone();
spawn_local(async move {
⋮----
is_warming.set(false);
⋮----
is_ready.set(true);
⋮----
.forget();
⋮----
state.is_authenticated.set(true);
state.is_admin.set(res.is_admin);
state.has_log_access.set(res.has_log_access);
state.current_page.set(Page::Dashboard);
⋮----
view! {
⋮----
pub fn main() {
⋮----
let window = web_sys::window().expect("no global `window` exists");
let document = window.document().expect("should have a document on window");
if let Some(loader) = document.get_element_by_id("loading-indicator") {
loader.remove();
⋮----
mount_to_body(|| view! { <App/> });
```

## File: src/storage.rs
```rust
use web_sys::window;
⋮----
fn set_item(key: &str, value: &str) {
if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
let _ = storage.set_item(key, value);
⋮----
fn get_item(key: &str) -> Option<String> {
window()
.and_then(|w| w.local_storage().ok())
.flatten()
.and_then(|s| s.get_item(key).ok())
⋮----
pub fn save_theme(theme: &str) {
set_item(THEME_KEY, theme);
⋮----
pub fn load_theme() -> String {
get_item(THEME_KEY).unwrap_or_else(|| "dark".to_string())
⋮----
pub fn save_ui_language(lang_code: &str) {
set_item(UI_LANGUAGE_KEY, lang_code);
⋮----
pub fn load_ui_language() -> String {
get_item(UI_LANGUAGE_KEY).unwrap_or_else(|| "es".to_string())
⋮----
pub fn save_disabled_slides(slides: &[String]) {
if slides.is_empty() {
⋮----
let _ = storage.remove_item(DISABLED_SLIDES_KEY);
⋮----
let value = slides.join(",");
set_item(DISABLED_SLIDES_KEY, &value);
⋮----
pub fn load_disabled_slides() -> Vec<String> {
get_item(DISABLED_SLIDES_KEY)
.map(|s| s.split(',').map(|x| x.to_string()).collect())
.unwrap_or_default()
⋮----
pub fn save_favorites(favorites: &[String]) {
if favorites.is_empty() {
⋮----
let _ = storage.remove_item(FAVORITES_KEY);
⋮----
let value = favorites.join(",");
set_item(FAVORITES_KEY, &value);
⋮----
pub fn load_favorites() -> Vec<String> {
get_item(FAVORITES_KEY)
.map(|s| {
s.split(',')
.filter(|x| !x.is_empty())
.map(|x| x.to_string())
.collect()
⋮----
pub fn toggle_favorite(key: &str) -> bool {
let mut favorites = load_favorites();
if favorites.contains(&key.to_string()) {
favorites.retain(|k| k != key);
save_favorites(&favorites);
⋮----
favorites.push(key.to_string());
⋮----
pub fn is_favorite(key: &str) -> bool {
load_favorites().contains(&key.to_string())
⋮----
pub fn add_to_recents(key: &str) {
let mut recents = load_recents();
⋮----
recents.retain(|k| k != key);
⋮----
recents.insert(0, key.to_string());
⋮----
recents.truncate(MAX_RECENTS);
let value = recents.join(",");
set_item(RECENTS_KEY, &value);
⋮----
pub fn load_recents() -> Vec<String> {
get_item(RECENTS_KEY)
⋮----
pub struct TemplateVersion {
⋮----
pub fn save_template_version(template_id: &str, slides_json: Vec<String>) {
let key = format!("{}{}", VERSIONS_KEY_PREFIX, template_id);
let mut versions = load_template_versions(template_id);
⋮----
let next_version = versions.iter().map(|v| v.version).max().unwrap_or(0) + 1;
⋮----
.to_iso_string()
.as_string()
.unwrap_or_default();
⋮----
versions.insert(
⋮----
versions.truncate(MAX_VERSIONS);
⋮----
set_item(&key, &json);
⋮----
pub fn load_template_versions(template_id: &str) -> Vec<TemplateVersion> {
⋮----
get_item(&key)
.and_then(|json| serde_json::from_str(&json).ok())
⋮----
pub fn get_template_version(template_id: &str, version: u32) -> Option<TemplateVersion> {
load_template_versions(template_id)
.into_iter()
.find(|v| v.version == version)
⋮----
pub fn get_latest_version_number(template_id: &str) -> u32 {
⋮----
.first()
.map(|v| v.version)
.unwrap_or(0)
```

## File: _redirects
```
# Cloudflare Pages Configuration
# API proxy is handled by functions/api/[[path]].js

# SPA fallback - serve index.html for all non-API routes
/* /index.html 200
```

## File: index.html
```html
<!DOCTYPE html>
<html lang="es">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Axur Web - Threat Hunting</title>
    <meta name="description" content="Axur Web - Generador de reportes de inteligencia de amenazas externas">
    <link data-trunk rel="rust" href="Cargo.toml" />
    <link data-trunk rel="copy-dir" href="assets" />


    <script src="https://cdnjs.cloudflare.com/ajax/libs/jszip/3.10.1/jszip.min.js"></script>


    <script src="https://html2canvas.hertzen.com/dist/html2canvas.min.js"></script>


    <script src="https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.min.js"></script>
    <script>
        pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js';

        // Screenshot capability
        window.captureScreenshot = async function () {
            try {
                // If in editor, try to capture valid canvas first?
                // But html2canvas captures entire DOM which is better for context (sidebar, modals)
                if (typeof html2canvas === 'undefined') return null;

                // Capture entire body
                const canvas = await html2canvas(document.body, {
                    logging: false,
                    useCORS: true,
                    allowTaint: true,
                    ignoreElements: (element) => {
                        // Ignore the feedback widget itself to avoid recursion if it's open
                        return element.classList.contains('feedback-widget-ignore');
                    }
                });

                // Return as base64 JPEG
                return canvas.toDataURL('image/jpeg', 0.7);
            } catch (e) {
                console.error('[Feedback] Screenshot failed:', e);
                return null;
            }
        };

        // ==================== ANNOTATION EDITOR ====================
        window.annotationEditor = {
            canvas: null,
            ctx: null,
            isDrawing: false,
            color: '#ef4444', // red
            lineWidth: 4,
            originalImage: null,
            annotatedData: null,

            open: function (screenshotData) {
                const overlay = document.getElementById('annotation-overlay');
                const canvas = document.getElementById('annotation-canvas');
                if (!overlay || !canvas) return;

                overlay.classList.remove('hidden');
                this.canvas = canvas;
                this.ctx = canvas.getContext('2d');

                // Load screenshot as background
                const img = new Image();
                img.onload = () => {
                    // Size canvas to fit screen
                    const maxW = window.innerWidth - 32;
                    const maxH = window.innerHeight - 100;
                    const scale = Math.min(maxW / img.width, maxH / img.height, 1);

                    canvas.width = img.width * scale;
                    canvas.height = img.height * scale;
                    this.ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
                    this.originalImage = img;
                };
                img.src = screenshotData;

                // Bind events
                this._bindEvents();
                this._bindColorButtons();
            },

            _bindEvents: function () {
                const c = this.canvas;
                c.onmousedown = (e) => this._startDraw(e);
                c.onmousemove = (e) => this._draw(e);
                c.onmouseup = () => this._endDraw();
                c.onmouseleave = () => this._endDraw();

                // Touch support
                c.ontouchstart = (e) => { e.preventDefault(); this._startDraw(e.touches[0]); };
                c.ontouchmove = (e) => { e.preventDefault(); this._draw(e.touches[0]); };
                c.ontouchend = () => this._endDraw();

                // Buttons
                document.getElementById('annotation-clear')?.addEventListener('click', () => this.clear());
                document.getElementById('annotation-done')?.addEventListener('click', () => this.done());
            },

            _bindColorButtons: function () {
                const colors = { 'red': '#ef4444', 'yellow': '#facc15', 'blue': '#3b82f6' };
                Object.keys(colors).forEach(name => {
                    const btn = document.getElementById(`annotation-color-${name}`);
                    if (btn) {
                        btn.onclick = () => {
                            this.color = colors[name];
                            // Update border
                            Object.keys(colors).forEach(n => {
                                const b = document.getElementById(`annotation-color-${n}`);
                                if (b) b.classList.toggle('border-white', n === name);
                            });
                        };
                    }
                });
            },

            _startDraw: function (e) {
                this.isDrawing = true;
                const rect = this.canvas.getBoundingClientRect();
                this.lastX = e.clientX - rect.left;
                this.lastY = e.clientY - rect.top;
            },

            _draw: function (e) {
                if (!this.isDrawing) return;
                const rect = this.canvas.getBoundingClientRect();
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;

                this.ctx.strokeStyle = this.color;
                this.ctx.lineWidth = this.lineWidth;
                this.ctx.lineCap = 'round';
                this.ctx.lineJoin = 'round';

                this.ctx.beginPath();
                this.ctx.moveTo(this.lastX, this.lastY);
                this.ctx.lineTo(x, y);
                this.ctx.stroke();

                this.lastX = x;
                this.lastY = y;
            },

            _endDraw: function () {
                this.isDrawing = false;
            },

            clear: function () {
                if (this.originalImage && this.ctx) {
                    this.ctx.drawImage(this.originalImage, 0, 0, this.canvas.width, this.canvas.height);
                }
            },

            done: function () {
                if (this.canvas) {
                    this.annotatedData = this.canvas.toDataURL('image/jpeg', 0.8);
                }
                document.getElementById('annotation-overlay')?.classList.add('hidden');
            },

            getAnnotated: function () {
                return this.annotatedData;
            }
        };

        // Global wrappers for Rust binding
        window.openAnnotationEditor = function (screenshotData) {
            window.annotationEditor.open(screenshotData);
        };
        window.getAnnotatedScreenshot = function () {
            return window.annotationEditor.getAnnotated();
        };

        // ==================== ACHIEVEMENT SYSTEM ====================
        window._achievementTitles = {
            'tutorial': '🎓 Tutorial completado',
            'first_import': '📥 Primera importación',
            'first_export': '📤 Primera exportación',
            'template_created': '✨ Template creado',
            'threat_hunting': '🔍 Threat Hunter',
            'shortcut_master': '⌨️ Maestro de atajos'
        };

        // Unlock achievement (called from JS, syncs to localStorage)
        window.unlockAchievement = function (id) {
            const key = 'axur_onboarding';
            let progress = JSON.parse(localStorage.getItem(key) || '{}');
            if (!progress.unlocked_achievements) progress.unlocked_achievements = [];
            if (!progress.unlocked_achievements.includes(id)) {
                progress.unlocked_achievements.push(id);
                localStorage.setItem(key, JSON.stringify(progress));
                window.showAchievementToast(id);
            }
        };

        // Show achievement toast notification with confetti
        window.showAchievementToast = function (id) {
            const title = window._achievementTitles[id] || id;
            const toast = document.createElement('div');
            toast.className = 'achievement-toast';
            toast.innerHTML = `
                <div style="display: flex; align-items: center; gap: 12px; padding: 16px 20px; background: linear-gradient(135deg, #18181b 0%, #27272a 100%); border: 1px solid #3f3f46; border-radius: 12px; box-shadow: 0 20px 40px rgba(0,0,0,0.4); color: white; font-family: Inter, sans-serif;">
                    <span style="font-size: 32px; animation: bounce 0.5s ease;">🏆</span>
                    <div>
                        <p style="margin: 0 0 2px; font-size: 12px; text-transform: uppercase; letter-spacing: 1px; color: #f97316;">Logro Desbloqueado</p>
                        <p style="margin: 0; font-size: 16px; font-weight: 600;">${title}</p>
                    </div>
                </div>
            `;
            toast.style.cssText = 'position: fixed; bottom: 100px; left: 20px; z-index: 10000; animation: slideUp 0.4s ease, fadeOut 0.4s ease 3s forwards;';
            document.body.appendChild(toast);

            // Trigger confetti effect
            window.launchConfetti();

            setTimeout(() => toast.remove(), 3500);
        };

        // Simple canvas confetti effect
        window.launchConfetti = function () {
            const canvas = document.createElement('canvas');
            canvas.style.cssText = 'position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 10001;';
            document.body.appendChild(canvas);
            const ctx = canvas.getContext('2d');
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;

            const particles = [];
            const colors = ['#f97316', '#fbbf24', '#22c55e', '#3b82f6', '#a855f7', '#ec4899'];

            for (let i = 0; i < 100; i++) {
                particles.push({
                    x: canvas.width / 2,
                    y: canvas.height / 2,
                    vx: (Math.random() - 0.5) * 15,
                    vy: (Math.random() - 0.5) * 15 - 5,
                    color: colors[Math.floor(Math.random() * colors.length)],
                    size: Math.random() * 8 + 4,
                    rotation: Math.random() * 360,
                    rotationSpeed: (Math.random() - 0.5) * 10
                });
            }

            let frame = 0;
            const animate = () => {
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                particles.forEach(p => {
                    p.x += p.vx;
                    p.y += p.vy;
                    p.vy += 0.3; // gravity
                    p.rotation += p.rotationSpeed;

                    ctx.save();
                    ctx.translate(p.x, p.y);
                    ctx.rotate(p.rotation * Math.PI / 180);
                    ctx.fillStyle = p.color;
                    ctx.fillRect(-p.size / 2, -p.size / 2, p.size, p.size);
                    ctx.restore();
                });

                frame++;
                if (frame < 120) requestAnimationFrame(animate);
                else canvas.remove();
            };
            animate();
        };

        // Add achievement animation styles
        const achievementStyles = document.createElement('style');
        achievementStyles.textContent = `
            @keyframes slideUp { from { transform: translateY(50px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }
            @keyframes fadeOut { to { opacity: 0; transform: translateY(-20px); } }
            @keyframes bounce { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.3); } }

            /* GitHub Sync Overlay */
            .github-sync-overlay {
                position: fixed;
                bottom: 20px;
                right: 20px;
                background: rgba(0, 0, 0, 0.85);
                backdrop-filter: blur(8px);
                padding: 12px 20px;
                border-radius: 50px;
                display: flex;
                align-items: center;
                gap: 10px;
                z-index: 9999;
                font-family: Inter, sans-serif;
                color: white;
                font-size: 14px;
                border: 1px solid rgba(255,255,255,0.1);
                box-shadow: 0 10px 30px rgba(0,0,0,0.3);
                animation: slideUp 0.3s ease;
            }
            .sync-icon { font-size: 18px; }
            .sync-icon.spinning { animation: spin 1s linear infinite; }
            .sync-icon.warning { color: #f97316; }
            .pulse-text { animation: pulse 1.5s ease-in-out infinite; }
            .fade-text { color: #22c55e; animation: fadeOut 2s ease forwards; animation-delay: 1s; }
            .warning-text { color: #f97316; }
            @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
            @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }

            /* Cold Start Overlay (Freeze to Thaw) */
            .cold-overlay {
                position: fixed;
                inset: 0;
                backdrop-filter: blur(8px);
                background: linear-gradient(135deg, rgba(165,216,255,0.7), rgba(59,130,246,0.5));
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10000;
                transition: all 1.5s ease-out;
            }
            .cold-overlay.warming {
                backdrop-filter: blur(2px);
                background: linear-gradient(135deg, rgba(249,115,22,0.3), rgba(234,88,12,0.2));
            }
            .cold-overlay.ready {
                opacity: 0;
                pointer-events: none;
            }
            .cold-content {
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 16px;
                color: white;
                font-family: Inter, sans-serif;
            }
            .cold-icon { font-size: 64px; animation: pulse 2s ease-in-out infinite; }
            .cold-text { font-size: 18px; font-weight: 500; text-shadow: 0 2px 10px rgba(0,0,0,0.3); }
        `;
        document.head.appendChild(achievementStyles);

        // Shortcut tracking (call when any shortcut is used)
        window.trackShortcutUsage = function () {
            const key = 'axur_onboarding';
            let progress = JSON.parse(localStorage.getItem(key) || '{}');
            progress.shortcuts_used = (progress.shortcuts_used || 0) + 1;
            localStorage.setItem(key, JSON.stringify(progress));
            if (progress.shortcuts_used >= 5) {
                window.unlockAchievement('shortcut_master');
            }
        };
    </script>


    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    fontFamily: {
                        sans: ['Inter', 'system-ui', 'sans-serif'],
                        mono: ['DM Mono', 'monospace'],
                    },
                    colors: {
                        axur: {
                            red: '#EF4043',
                            orange: '#f97316',
                        },
                    },
                },
            },
        };



        // ==================== PPTX IMPORTER (HYBRID) ====================

        // ==================== PPTX IMPORTER (HYBRID V2 - FIXED) ====================
        const PPTXImporter = {
            MAX_FILE_SIZE: 50 * 1024 * 1024,
            MAX_SLIDES: 50,
            PROCESSING_TIMEOUT: 30000,

            state: { zip: null, fileName: '', slides: {} },

            // --- 1. UNIT FIX: EXACT FLOAT PRECISION (Reason #2) ---
            // 914400 EMUs = 1 inch = 96 px => 1px = 9525 EMUs
            emuToPixels: function (emu) {
                if (emu === undefined || emu === null) return 0;
                return emu / 9525;
            },
            pixelsToEmu: function (pixels) {
                return Math.round(pixels * 9525);
            },

            // --- HELPER: FETCH XML ---
            async fetchXml(zip, path) {
                const f = zip.file(path);
                if (!f) return null;
                const str = await f.async('string');
                return new DOMParser().parseFromString(str, 'application/xml');
            },

            // --- HELPER: RESOLVE RELATIONSHIP ---
            async getRelTarget(zip, relsPath, typeSuffix) {
                const doc = await this.fetchXml(zip, relsPath);
                if (!doc) return null;
                // Find relationship by type ending (ignore schema date versions)
                const rel = Array.from(doc.getElementsByTagName('Relationship') || [])
                    .find(r => r.getAttribute('Type') && r.getAttribute('Type').endsWith(typeSuffix));
                if (!rel) return null;

                let target = rel.getAttribute('Target');
                // Normalize path
                if (target.startsWith('../')) target = target.replace('../', 'ppt/');
                else if (target.startsWith('/')) target = target.substring(1);
                else if (!target.startsWith('ppt/')) {
                    // Logic relative to rels folder location involves typically stepping back once
                    // ppt/slides/_rels/slide1.rels -> ppt/slides/../slideLayouts/layout.xml
                    if (relsPath.includes('slides/_rels') && !target.includes('/')) target = 'ppt/slides/' + target;
                    else if (relsPath.includes('slideLayouts/_rels') && !target.includes('/')) target = 'ppt/slideLayouts/' + target;
                }

                // Resolve ".." parts
                const parts = target.split('/');
                const stack = [];
                for (const p of parts) {
                    if (p === '..') stack.pop();
                    else stack.push(p);
                }
                return stack.join('/');
            },

            // --- PARSE COLOR ---
            parseColor: function (node) {
                if (!node) return null;
                const srgb = node.querySelector('srgbClr, a\\:srgbClr');
                if (srgb) return '#' + srgb.getAttribute('val');
                const scheme = node.querySelector('schemeClr, a\\:schemeClr');
                if (scheme) {
                    const map = {
                        'bg1': '#ffffff', 'tx1': '#000000', 'bg2': '#f3f4f6', 'tx2': '#1f2937',
                        'accent1': '#f97316', 'accent2': '#22c55e', 'accent3': '#3b82f6',
                        'accent4': '#ef4444', 'accent5': '#eab308', 'accent6': '#a855f7'
                    };
                    return map[scheme.getAttribute('val')] || '#888888';
                }
                return null;
            },

            // --- PARSE TRANSFORM ---
            parseTransform: function (spPr, ox, oy) {
                if (!spPr) return null;
                const xfrm = spPr.querySelector('xfrm, a\\:xfrm');
                if (!xfrm) return null;

                const off = xfrm.querySelector('off, a\\:off');
                const ext = xfrm.querySelector('ext, a\\:ext');

                // If missing off/ext, it's likely a placeholder inheriting transform.
                // We SKIP it for now (returning null), or we default to 0?
                // Returning null filters it out in current logic, which is cleaner than 0,0 box.
                if (!off || !ext) return null;

                return {
                    left: this.emuToPixels(parseInt(off.getAttribute('x') || 0)) + ox,
                    top: this.emuToPixels(parseInt(off.getAttribute('y') || 0)) + oy,
                    width: this.emuToPixels(parseInt(ext.getAttribute('cx') || 0)),
                    height: this.emuToPixels(parseInt(ext.getAttribute('cy') || 0)),
                    angle: parseInt(xfrm.getAttribute('rot') || 0) / 60000
                };
            },

            // --- EXTRACT SHAPES (Reason #3 partial) ---
            async extractShapes(node, zip, relsDoc, ox, oy) {
                const shapes = [];
                const children = node.childNodes;

                for (let i = 0; i < children.length; i++) {
                    const child = children[i];
                    if (child.nodeType !== 1) continue;
                    const tag = child.localName || child.nodeName.split(':').pop();

                    if (tag === 'grpSp') {
                        // Recursively handle group contents (flat for now)
                        // Group xfrm should calculate new ox, oy.
                        const grpSpPr = child.querySelector('grpSpPr, p\\:grpSpPr');
                        let newOx = ox, newOy = oy;

                        if (grpSpPr) {
                            const xfrm = grpSpPr.querySelector('xfrm, a\\:xfrm');
                            if (xfrm) {
                                const off = xfrm.querySelector('off, a\\:off');
                                const chOff = xfrm.querySelector('chOff, a\\:chOff');
                                if (off && chOff) {
                                    // Simple Group logic
                                    const gx = this.emuToPixels(parseInt(off.getAttribute('x') || 0));
                                    const gy = this.emuToPixels(parseInt(off.getAttribute('y') || 0));
                                    const cx = this.emuToPixels(parseInt(chOff.getAttribute('x') || 0));
                                    const cy = this.emuToPixels(parseInt(chOff.getAttribute('y') || 0));
                                    newOx = ox + (gx - cx);
                                    newOy = oy + (gy - cy);
                                }
                            }
                        }
                        shapes.push(...(await this.extractShapes(child, zip, relsDoc, newOx, newOy)));
                    }
                    else if (tag === 'sp') {
                        const spPr = child.querySelector('spPr, p\\:spPr');
                        const transform = this.parseTransform(spPr, ox, oy);

                        if (transform) {
                            const txBody = child.querySelector('txBody, p\\:txBody');
                            const { id, name } = { id: Date.now(), name: 'Shape' }; // Simple ID if missing

                            if (txBody) {
                                let text = '';
                                txBody.querySelectorAll('p, a\\:p').forEach(p => {
                                    p.querySelectorAll('r, a\\:r').forEach(r => {
                                        const t = r.querySelector('t, a\\:t');
                                        if (t) text += t.textContent;
                                    });
                                    text += '\n';
                                });
                                shapes.push({
                                    type: 'textbox',
                                    ...transform,
                                    text: text.trim(),
                                    fill: '#000000',
                                    fontSize: 14, fontFamily: 'Inter'
                                });
                            } else {
                                const solidFill = spPr.querySelector('solidFill, a\\:solidFill');
                                const fill = this.parseColor(solidFill) || '#3f3f46';
                                shapes.push({
                                    type: 'rect',
                                    ...transform,
                                    fill: fill
                                });
                            }
                        }
                    }
                    else if (tag === 'pic') {
                        const blipFill = child.querySelector('blipFill, p\\:blipFill');
                        if (blipFill) {
                            const blip = blipFill.querySelector('blip, a\\:blip');
                            const embedId = blip?.getAttribute('r:embed');
                            if (embedId && relsDoc) {
                                const rel = Array.from(relsDoc.getElementsByTagName('Relationship') || [])
                                    .find(r => r.getAttribute('Id') === embedId);

                                if (rel) {
                                    let target = rel.getAttribute('Target');
                                    if (target.startsWith('../')) target = target.replace('../', 'ppt/');
                                    else if (target.startsWith('media/')) target = 'ppt/media/' + target.split('/').pop();
                                    else if (!target.startsWith('ppt/')) target = 'ppt/' + target;

                                    // Fix relative path if needed
                                    if (target.indexOf('..') !== -1) {
                                        // Simplify
                                        target = target.replace(/\w+\/\.\.\//g, '');
                                    }

                                    const imgFile = zip.file(target);
                                    if (imgFile) {
                                        const data = await imgFile.async('base64');
                                        const spPr = child.querySelector('spPr, p\\:spPr');
                                        const transform = this.parseTransform(spPr, ox, oy);

                                        // Mime guess
                                        const ext = target.split('.').pop().toLowerCase();
                                        const mime = ext === 'svg' ? 'image/svg+xml' : (ext === 'png' ? 'image/png' : 'image/jpeg');

                                        if (transform) {
                                            shapes.push({
                                                type: 'image',
                                                ...transform,
                                                src: `data:${mime};base64,${data}`,
                                                selectable: true
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return shapes;
            },

            // --- HIERARCHY PIPELINE (Reason #1) ---
            async parseSlide(xml, zip, slideNum) {
                const slidesRelPath = `ppt/slides/_rels/slide${slideNum}.xml.rels`;
                const slideDoc = new DOMParser().parseFromString(xml, 'application/xml');
                const relsDoc = await this.fetchXml(zip, slidesRelPath);

                let allShapes = [];

                // 1. MASTER & LAYOUT (Reason #1: Ghost Problem)
                // Try to find Layout
                const layoutPath = await this.getRelTarget(zip, slidesRelPath, 'slideLayout');
                if (layoutPath) {
                    const layoutDoc = await this.fetchXml(zip, layoutPath);
                    if (layoutDoc) {
                        const layoutRelsPath = layoutPath.replace('slideLayouts/', 'slideLayouts/_rels/') + '.rels';
                        const layoutRelsDoc = await this.fetchXml(zip, layoutRelsPath);

                        // Try to find Master from Layout
                        const masterPath = await this.getRelTarget(zip, layoutRelsPath, 'slideMaster');
                        if (masterPath) {
                            const masterDoc = await this.fetchXml(zip, masterPath);
                            const masterRelsPath = masterPath.replace('slideMasters/', 'slideMasters/_rels/') + '.rels';
                            const masterRelsDoc = await this.fetchXml(zip, masterRelsPath);

                            // MASTER SHAPES (Backgrounds)
                            if (masterDoc) {
                                const spTree = masterDoc.getElementsByTagName('p:spTree')[0] || masterDoc.getElementsByTagName('spTree')[0];
                                if (spTree) {
                                    const mShapes = await this.extractShapes(spTree, zip, masterRelsDoc, 0, 0);
                                    // Only keep visual elements (not text placeholders)
                                    allShapes.push(...mShapes.filter(s => s.type !== 'textbox'));
                                }
                            }
                        }

                        // LAYOUT SHAPES
                        const spTree = layoutDoc.getElementsByTagName('p:spTree')[0] || layoutDoc.getElementsByTagName('spTree')[0];
                        if (spTree) {
                            const lShapes = await this.extractShapes(spTree, zip, layoutRelsDoc, 0, 0);
                            // Only keep visual elements
                            allShapes.push(...lShapes.filter(s => s.type !== 'textbox'));
                        }
                    }
                }

                // 2. SLIDE SHAPES
                const spTree = slideDoc.getElementsByTagName('p:spTree')[0] || slideDoc.getElementsByTagName('spTree')[0];
                if (spTree) {
                    const sShapes = await this.extractShapes(spTree, zip, relsDoc, 0, 0);
                    allShapes.push(...sShapes);
                }

                // Add default black background if none
                if (!allShapes.some(s => s.type === 'rect' && s.width > 1000 && s.height > 600)) {
                    allShapes.unshift({
                        type: 'rect', left: 0, top: 0, width: 1280, height: 720, fill: '#18181b'
                    });
                }

                return { objects: allShapes };
            },

            importFile: async function (file) {
                console.log("[PPTX] Importing via Google Hybrid Architecture...");
                this.state.originalFile = file; // Store for export
                this.state.fileName = file.name;

                try {
                    // 1. Upload to Backend (Google Drive/Slides)
                    const formData = new FormData();
                    formData.append('file', file);

                    console.log("[PPTX] Uploading for preview generation...");
                    const resp = await fetch('/api/import/pptx', {
                        method: 'POST',
                        body: formData
                    });

                    if (!resp.ok) {
                        const errText = await resp.text();
                        throw new Error("Preview generation failed: " + errText);
                    }

                    const result = await resp.json();
                    if (!result.success) throw new Error(result.message);

                    const imageUrls = result.slides;
                    console.log(`[PPTX] Received ${imageUrls.length} slide previews from Google.`);

                    // Update progress
                    if (window.showPptxProgress) window.showPptxProgress('processing', imageUrls.length + ' slides');

                    const slides = [];

                    // Images are now base64 data URLs from backend (no pre-fetching needed!)
                    // Backend handles rate limiting with Retry-After header support
                    console.log('[PPTX] Backend returned ' + imageUrls.length + ' embedded images (no 429 risk)');

                    for (let i = 0; i < imageUrls.length; i++) {
                        const url = imageUrls[i];
                        const slideNum = i + 1;

                        // Update progress per slide
                        if (window.showPptxProgress) {
                            window.showPptxProgress('processing', `Preparing slide ${slideNum}/${imageUrls.length}`);
                        }

                        // Note: DO NOT hardcode width/height - let Fabric.js detect from actual image
                        slides.push({
                            slideNumber: slideNum,
                            canvasJson: JSON.stringify({
                                version: "5.3.0",
                                objects: [], // Empty objects layer
                                backgroundImage: {
                                    type: 'image',
                                    version: '5.3.0',
                                    originX: 'left',
                                    originY: 'top',
                                    left: 0,
                                    top: 0,
                                    // Data URL is already embedded (no network fetch at display time)
                                    src: url,
                                    scaleX: 1,
                                    scaleY: 1,
                                    crossOrigin: 'anonymous',
                                    stroke: null,
                                    strokeWidth: 0,
                                    fill: 'rgb(0,0,0)',
                                    opacity: 1,
                                    visible: true,
                                    backgroundColor: ''
                                }
                            })
                        });
                    }
                    console.log('[PPTX] All slides prepared successfully');

                    this.state.slides = slides;
                    // Clean up URL object? No, these are remote URLs.

                    return {
                        success: true,
                        fileName: this.state.fileName,
                        slides: slides,
                        slideCount: slides.length
                    };

                } catch (e) {
                    console.error("Import failed:", e);
                    alert("Import failed: " + e.message);
                    return { success: false, slides: [] };
                }
            },

            patchSlideXML: function () { },

            exportPPTX: async function () {
                if (!this.state.originalFile) {
                    alert("No hay archivo original cargado.");
                    return;
                }

                // Sync current canvas to state using the correct global variable 'fabricCanvas'
                if (typeof fabricCanvas !== 'undefined' && fabricCanvas && window._importedSlides && window._importedSlides[window._currentImportedSlideIndex]) {
                    window._importedSlides[window._currentImportedSlideIndex].canvasJson = JSON.stringify(fabricCanvas.toJSON(['placeholderKey']));
                }

                const edits = [];
                const slides = window._importedSlides || [];

                slides.forEach((slide, idx) => {
                    const json = typeof slide.canvasJson === 'string' ? JSON.parse(slide.canvasJson) : slide.canvasJson;
                    if (json.objects) {
                        json.objects.forEach(obj => {
                            // Check if it's a Placeholder Group
                            if (obj.placeholderKey) {
                                edits.push({
                                    slide_index: idx + 1,
                                    text: '{{' + obj.placeholderKey + '}}', // Visual text
                                    placeholder_key: obj.placeholderKey,      // Internal naming
                                    x: obj.left,
                                    y: obj.top,
                                    width: obj.width * (obj.scaleX || 1),
                                    height: obj.height * (obj.scaleY || 1)
                                });
                            }
                            // Or if it's a manual Textbox
                            else if (obj.selectable !== false && (obj.type === 'textbox' || obj.type === 'i-text')) {
                                edits.push({
                                    slide_index: idx + 1,
                                    text: obj.text || "Text",
                                    x: obj.left,
                                    y: obj.top,
                                    width: obj.width * (obj.scaleX || 1),
                                    height: obj.height * (obj.scaleY || 1)
                                });
                            }
                        });
                    }
                });

                console.log("[PPTX] Exporting edits:", edits);

                const formData = new FormData();
                formData.append('file', this.state.originalFile);
                formData.append('edits', JSON.stringify(edits));

                try {
                    const resp = await fetch('/api/export/inject', { method: 'POST', body: formData });
                    if (!resp.ok) throw new Error("Backend error: " + resp.statusText);

                    const blob = await resp.blob();
                    const url = window.URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = url;
                    a.download = "injected_" + this.state.originalFile.name;
                    document.body.appendChild(a);
                    a.click();
                    document.body.removeChild(a);
                } catch (e) {
                    alert("Export failed: " + e.message);
                }
            }
        };
        window.PPTXImporter = PPTXImporter;
        window.importPPTX = (file) => PPTXImporter.importFile(file);

        // Handle PPTX import from file input (called via setTimeout to avoid WASM closure issues)
        window.handlePptxImport = async function () {
            try {
                const input = document.getElementById('pptx-input');
                if (!input.files || input.files.length === 0) return;

                const file = input.files[0];
                console.log('[PPTX] Selected file:', file.name);

                const result = await window.importPPTX(file);
                input.value = '';

                if (result.success && result.slides.length > 0) {
                    console.log('[PPTX] Loading ' + result.slides.length + ' slides with editable elements...');

                    // Use parsed slides directly (editable elements!)
                    const editableSlides = result.slides.map((slide, i) => ({
                        id: 'imported-' + (i + 1),
                        name: 'Slide ' + (i + 1),
                        canvasJson: slide.canvasJson // Keep original parsed JSON
                    }));

                    // Store for navigation
                    window._importedSlides = editableSlides;
                    window._importedFileName = result.fileName.replace('.pptx', '');
                    window._currentImportedSlideIndex = 0;

                    // Load first slide into canvas
                    window.loadCanvasJSON(editableSlides[0].canvasJson);

                    // Show guidance modal (Reusing Editable guide but tailored in function)
                    window.showImportGuideEditable(result.slideCount, result.fileName, function () {
                        console.log('[PPTX] User ready to edit ' + editableSlides.length + ' slides');
                        window.updatePptxIndicator();
                        // Unlock first_import achievement
                        if (window.unlockAchievement) window.unlockAchievement('first_import');
                    });
                } else {
                    alert('❌ No se encontraron slides en el archivo');
                }
            } catch (e) {
                console.error('[PPTX] Import error:', e);
                alert('❌ Error al importar: ' + e.message);
            }
        };

        // ==================== PPTX IMPORT PROGRESS OVERLAY ====================
        window._pptxOverlay = null;

        window.showPptxProgress = function (stage, detail) {
            if (!window._pptxOverlay) {
                window._pptxOverlay = document.createElement('div');
                window._pptxOverlay.id = 'pptx-progress-overlay';
                window._pptxOverlay.innerHTML = `
                    <div style="position: fixed; inset: 0; background: rgba(0,0,0,0.85); display: flex; align-items: center; justify-content: center; z-index: 9999;">
                        <div style="background: #18181b; border: 1px solid #27272a; border-radius: 16px; padding: 32px; max-width: 400px; width: 90%; color: white; font-family: Inter, sans-serif; text-align: center;">
                            <div style="width: 48px; height: 48px; border: 3px solid #3f3f46; border-top-color: #f97316; border-radius: 50%; margin: 0 auto 20px; animation: pptx-spin 1s linear infinite;"></div>
                            <h3 id="pptx-stage" style="margin: 0 0 8px; font-size: 18px; font-weight: 600;">Importando...</h3>
                            <p id="pptx-detail" style="margin: 0; color: #a1a1aa; font-size: 14px;"></p>
                            <div style="margin-top: 20px; height: 4px; background: #27272a; border-radius: 2px; overflow: hidden;">
                                <div id="pptx-progress-bar" style="height: 100%; background: linear-gradient(90deg, #f97316, #fbbf24); width: 0%; transition: width 0.3s ease;"></div>
                            </div>
                        </div>
                    </div>
                    <style>
                        @keyframes pptx-spin { to { transform: rotate(360deg); } }
                    </style>
                `;
                document.body.appendChild(window._pptxOverlay);
            }

            const stageEl = document.getElementById('pptx-stage');
            const detailEl = document.getElementById('pptx-detail');
            const progressBar = document.getElementById('pptx-progress-bar');

            const stages = {
                'uploading': { text: '📤 Subiendo archivo...', progress: 20 },
                'processing': { text: '⚙️ Procesando slides...', progress: 50 },
                'loading': { text: '🎨 Cargando editor...', progress: 80 },
                'complete': { text: '✅ Completado', progress: 100 }
            };

            const s = stages[stage] || { text: stage, progress: 50 };
            if (stageEl) stageEl.textContent = s.text;
            if (detailEl) detailEl.textContent = detail || '';
            if (progressBar) progressBar.style.width = s.progress + '%';
        };

        window.hidePptxProgress = function () {
            if (window._pptxOverlay) {
                document.body.removeChild(window._pptxOverlay);
                window._pptxOverlay = null;
            }
        };

        // Handle PPTX import from stored global file (works with setTimeout)
        window.handlePptxImportFromGlobal = async function () {
            try {
                const file = window._pendingPptxFile;
                if (!file) {
                    console.warn('[PPTX] No pending file to import');
                    return;
                }

                // Clear the pending file
                window._pendingPptxFile = null;

                // Also clear the input
                const input = document.getElementById('pptx-input');
                if (input) input.value = '';

                console.log('[PPTX] Selected file:', file.name);

                // Show progress overlay
                window.showPptxProgress('uploading', file.name);

                const result = await window.importPPTX(file);

                window.showPptxProgress('loading', result.slides.length + ' slides');

                if (result.success && result.slides.length > 0) {
                    console.log('[PPTX] Loading ' + result.slides.length + ' slides with editable elements...');

                    // Use parsed slides directly (editable elements!)
                    const editableSlides = result.slides.map((slide, i) => ({
                        id: 'imported-' + (i + 1),
                        name: 'Slide ' + (i + 1),
                        canvasJson: slide.canvasJson
                    }));

                    // Store for navigation
                    window._importedSlides = editableSlides;
                    window._importedFileName = result.fileName.replace('.pptx', '');
                    window._currentImportedSlideIndex = 0;

                    // Load first slide into canvas
                    window.loadCanvasJSON(editableSlides[0].canvasJson);

                    // Hide progress and show guidance modal
                    window.hidePptxProgress();
                    window.showImportGuideEditable(result.slideCount, result.fileName, function () {
                        console.log('[PPTX] User ready to edit ' + editableSlides.length + ' slides');
                        window.updatePptxIndicator();
                    });
                } else {
                    window.hidePptxProgress();
                    alert('❌ No se encontraron slides en el archivo');
                }
            } catch (e) {
                window.hidePptxProgress();
                console.error('[PPTX] Import error:', e);
                alert('❌ Error al importar: ' + e.message);
            }
        };

        // Convert parsed slide to background image for maximum fidelity
        window.convertToBackgroundImage = async function (canvasJson) {
            return new Promise((resolve, reject) => {
                try {
                    // Create temporary canvas
                    const tempCanvasEl = document.createElement('canvas');
                    tempCanvasEl.width = 1280;
                    tempCanvasEl.height = 720;
                    tempCanvasEl.style.display = 'none';
                    document.body.appendChild(tempCanvasEl);

                    // Create temporary Fabric canvas
                    const tempCanvas = new fabric.Canvas(tempCanvasEl, {
                        width: 1280,
                        height: 720,
                        backgroundColor: '#18181b'
                    });

                    // Load the parsed JSON
                    const data = JSON.parse(canvasJson);
                    tempCanvas.loadFromJSON(data, function () {
                        tempCanvas.renderAll();

                        // Export as PNG data URL
                        const dataUrl = tempCanvas.toDataURL({
                            format: 'png',
                            quality: 1,
                            multiplier: 1
                        });

                        // Create new JSON with image as background
                        const backgroundJson = {
                            version: '5.3.0',
                            objects: [{
                                type: 'image',
                                left: 0,
                                top: 0,
                                width: 1280,
                                height: 720,
                                src: dataUrl,
                                selectable: false,
                                evented: false,
                                lockMovementX: true,
                                lockMovementY: true,
                                lockRotation: true,
                                lockScalingX: true,
                                lockScalingY: true,
                                hasControls: false,
                                hasBorders: false,
                                isBackground: true
                            }]
                        };

                        // Cleanup
                        tempCanvas.dispose();
                        document.body.removeChild(tempCanvasEl);

                        resolve(JSON.stringify(backgroundJson));
                    });
                } catch (e) {
                    console.error('[PPTX] Failed to convert to background:', e);
                    reject(e);
                }
            });
        };

        // Show import guidance modal
        window.showImportGuide = function (slideCount, fileName, onContinue) {
            // Retired legacy modal
            if (onContinue) onContinue();
        };

        // Show import guidance modal for EDITABLE elements
        window.showImportGuideEditable = function (slideCount, fileName, onContinue) {
            const modal = document.createElement('div');
            modal.id = 'import-guide-modal';
            modal.innerHTML = `
        <div
            style="position: fixed; inset: 0; background: rgba(0,0,0,0.8); display: flex; align-items: center; justify-content: center; z-index: 9999;">
            <div
                style="background: #18181b; border: 1px solid #27272a; border-radius: 16px; padding: 32px; max-width: 560px; color: white; font-family: Inter, sans-serif;">
                <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 24px;">
                    <span style="font-size: 32px;">✅</span>
                    <div>
                        <h2 style="margin: 0; font-size: 20px; font-weight: 600;">Importación Overlay Exitosa</h2>
                        <p style="margin: 4px 0 0; color: #a1a1aa; font-size: 14px;">${slideCount} slide(s) de
                            "${fileName}"</p>
                    </div>
                </div>

                <div style="background: #27272a; border-radius: 12px; padding: 20px; margin-bottom: 24px;">
                    <h3 style="margin: 0 0 12px; font-size: 16px; color: #22c55e;">✨ Modo de Edición Overlay:</h3>
                    <ul style="margin: 0; padding-left: 20px; color: #d4d4d8; font-size: 14px; line-height: 1.8;">
                        <li>El contenido original es un <strong>Fondo Estático</strong> (100% fidelidad).</li>
                        <li>Agrega <strong>Placeholders</strong> encima usando el menú "📦 Placeholders".</li>
                        <li>Al exportar, se inyectarán los placeholders en el archivo original.</li>
                    </ul>
                </div>

                <div style="display: flex; gap: 12px; justify-content: flex-end;">
                    <button id="guide-continue-btn2"
                        style="padding: 12px 24px; background: #22c55e; border: none; border-radius: 8px; color: white; font-weight: 600; cursor: pointer; font-size: 14px;">
                        Comenzar a editar →
                    </button>
                </div>
            </div>
        </div>
        `;
            document.body.appendChild(modal);

            document.getElementById('guide-continue-btn2').onclick = function () {
                document.body.removeChild(modal);
                if (onContinue) onContinue();
            };
        };

        // Get imported slides for editor panel
        window.getImportedSlides = function () {
            return window._importedSlides || [];
        };

        // Get imported file name
        window.getImportedFileName = function () {
            return window._importedFileName || '';
        };

        // Check if there are imported slides waiting
        window.hasImportedSlides = function () {
            return window._importedSlides && window._importedSlides.length > 0;
        };

        // Clear imported slides after they've been loaded into editor
        window.clearImportedSlides = function () {
            window._importedSlides = null;
            window._importedFileName = '';
        };

        // Sync imported slides to editor panel (updates DOM directly)
        window.syncImportedSlidesToPanel = function () {
            const slides = window._importedSlides;
            if (!slides || slides.length === 0) return;

            // Find the slide panel container
            const slidePanel = document.querySelector('.slide-panel, [data-slide-panel], .space-y-2');
            if (!slidePanel) {
                console.log('[PPTX] Slide panel not found, slides stored in window._importedSlides');
                return;
            }

            // Clear existing slides (except any system slides)
            // Note: Leptos will manage this, we just log for debugging
            console.log('[PPTX] ' + slides.length + ' slides ready in window._importedSlides');
            console.log('[PPTX] Use getImportedSlides() to retrieve them');
        };

        // Make slide navigation easier by exposing current slide index
        window._currentImportedSlideIndex = 0;

        window.navigateImportedSlide = function (index) {
            const slides = window._importedSlides;
            if (!slides || index < 0 || index >= slides.length) return;

            window._currentImportedSlideIndex = index;
            window.loadCanvasJSON(slides[index].canvasJson);

            // Update slide indicator in toolbar
            const indicator = document.getElementById('pptx-slide-indicator');
            if (indicator) {
                indicator.textContent = 'PPTX: ' + (index + 1) + '/' + slides.length;
            }

            console.log('[PPTX] Loaded slide ' + (index + 1) + ' of ' + slides.length);
        };

        // Update indicator when slides are first imported
        window.updatePptxIndicator = function () {
            const slides = window._importedSlides;
            const indicator = document.getElementById('pptx-slide-indicator');
            if (indicator && slides && slides.length > 0) {
                indicator.textContent = 'PPTX: ' + (window._currentImportedSlideIndex + 1) + '/' + slides.length;
            }
            // Also render thumbnails in sidebar
            window.renderPptxThumbnails();
        };

        // Render thumbnails for imported slides in the sidebar
        window.renderPptxThumbnails = function () {
            const slides = window._importedSlides;
            const panel = document.getElementById('pptx-slides-panel');
            const container = document.getElementById('pptx-thumbnails');

            if (!slides || slides.length === 0 || !panel || !container) {
                if (panel) panel.style.display = 'none';
                return;
            }

            // Show panel
            panel.style.display = 'block';

            // Clear existing thumbnails
            container.innerHTML = '';

            // Drag state
            let dragIndex = null;

            // Create thumbnail buttons with real previews and drag-drop
            slides.forEach((slide, idx) => {
                const btn = document.createElement('button');
                btn.className = 'w-full aspect-video rounded cursor-grab border-2 transition overflow-hidden relative ' +
                    (idx === window._currentImportedSlideIndex ? 'border-orange-500' : 'border-zinc-700 hover:border-zinc-500');
                btn.title = 'Slide ' + (idx + 1) + ' - drag to reorder';
                btn.draggable = true;
                btn.dataset.index = idx;

                // Slide number badge
                const badge = document.createElement('span');
                badge.className = 'absolute bottom-1 right-1 bg-black/70 text-orange-400 text-xs px-1 rounded';
                badge.textContent = (idx + 1);
                btn.appendChild(badge);

                // Extract background image URL from canvas JSON and show as thumbnail
                try {
                    const jsonData = typeof slide.canvasJson === 'string' ?
                        JSON.parse(slide.canvasJson) : slide.canvasJson;

                    const bgSrc = jsonData.backgroundImage?.src;
                    if (bgSrc) {
                        // Use simple img element instead of Fabric canvas for thumbnails
                        const thumbImg = document.createElement('img');
                        thumbImg.src = bgSrc;
                        thumbImg.className = 'w-full h-full object-cover';
                        thumbImg.crossOrigin = 'anonymous';
                        thumbImg.style.pointerEvents = 'none';
                        thumbImg.onerror = function () {
                            // Fallback on error
                            thumbImg.remove();
                            btn.style.background = '#27272a';
                        };
                        btn.appendChild(thumbImg);
                    } else {
                        // No background image, show placeholder
                        btn.style.background = '#27272a';
                    }
                } catch (e) {
                    // Fallback: show number if preview fails
                    btn.innerHTML = '<span class="text-orange-400 text-2xl">' + (idx + 1) + '</span>';
                    btn.className += ' bg-zinc-800 flex items-center justify-center';
                }

                // Click to navigate
                btn.onclick = function () {
                    window.navigateImportedSlide(idx);
                    window.renderPptxThumbnails();
                };

                // Drag start
                btn.ondragstart = function (e) {
                    dragIndex = idx;
                    btn.style.opacity = '0.5';
                    e.dataTransfer.effectAllowed = 'move';
                    e.dataTransfer.setData('text/plain', idx);
                };

                // Drag end
                btn.ondragend = function () {
                    btn.style.opacity = '1';
                    dragIndex = null;
                    // Remove all drag-over styles
                    container.querySelectorAll('.border-orange-400').forEach(el => {
                        el.classList.remove('border-orange-400');
                    });
                };

                // Drag over
                btn.ondragover = function (e) {
                    e.preventDefault();
                    e.dataTransfer.dropEffect = 'move';
                    if (dragIndex !== null && dragIndex !== idx) {
                        btn.classList.add('border-orange-400');
                    }
                };

                // Drag leave
                btn.ondragleave = function () {
                    btn.classList.remove('border-orange-400');
                };

                // Drop
                btn.ondrop = function (e) {
                    e.preventDefault();
                    btn.classList.remove('border-orange-400');
                    const fromIdx = parseInt(e.dataTransfer.getData('text/plain'));
                    if (!isNaN(fromIdx) && fromIdx !== idx) {
                        window.reorderSlide(fromIdx, idx);
                    }
                };

                container.appendChild(btn);
            });
        };

        // ============= PPTX Slide Management =============

        // Reorder slides
        window.reorderSlide = function (fromIndex, toIndex) {
            const slides = window._importedSlides;
            if (!slides || fromIndex < 0 || toIndex < 0 || fromIndex >= slides.length || toIndex >= slides.length) {
                console.warn('[PPTX] Invalid reorder indices');
                return false;
            }
            const [slide] = slides.splice(fromIndex, 1);
            slides.splice(toIndex, 0, slide);
            if (window._currentImportedSlideIndex === fromIndex) {
                window._currentImportedSlideIndex = toIndex;
            } else if (fromIndex < window._currentImportedSlideIndex && toIndex >= window._currentImportedSlideIndex) {
                window._currentImportedSlideIndex--;
            } else if (fromIndex > window._currentImportedSlideIndex && toIndex <= window._currentImportedSlideIndex) {
                window._currentImportedSlideIndex++;
            }
            console.log('[PPTX] Reordered slide from ' + (fromIndex + 1) + ' to ' + (toIndex + 1));
            window.updatePptxIndicator();
            return true;
        };

        // Duplicate slide
        window.duplicateSlide = function (index) {
            const slides = window._importedSlides;
            if (!slides) return null;
            const idx = index !== undefined ? index : window._currentImportedSlideIndex;
            if (idx < 0 || idx >= slides.length) return null;
            const original = slides[idx];
            const duplicate = JSON.parse(JSON.stringify(original));
            duplicate.name = original.name + ' (Copy)';
            slides.splice(idx + 1, 0, duplicate);
            console.log('[PPTX] Duplicated slide ' + (idx + 1) + ', now ' + slides.length + ' slides');
            window.updatePptxIndicator();
            return idx + 1;
        };

        // Delete slide
        window.deleteSlide = function (index) {
            const slides = window._importedSlides;
            if (!slides || slides.length <= 1) {
                console.warn('[PPTX] Cannot delete: need at least 1 slide');
                return false;
            }
            const idx = index !== undefined ? index : window._currentImportedSlideIndex;
            if (idx < 0 || idx >= slides.length) return false;
            slides.splice(idx, 1);
            if (window._currentImportedSlideIndex >= slides.length) {
                window._currentImportedSlideIndex = slides.length - 1;
            }
            console.log('[PPTX] Deleted slide ' + (idx + 1) + ', remaining: ' + slides.length);
            window.updatePptxIndicator();
            // Reload current slide if we deleted the active one
            if (idx === window._currentImportedSlideIndex || window._currentImportedSlideIndex === -1) {
                if (window._currentImportedSlideIndex < 0) window._currentImportedSlideIndex = 0;
                window.loadCanvasJSON(slides[window._currentImportedSlideIndex].canvasJson);
            }
            return true;
        };

        // Export current canvas as single PNG
        window.exportCurrentSlide = function () {
            const dataUrl = (window.fabricCanvas) ?
                window.fabricCanvas.toDataURL({ format: 'png', quality: 1.0 }) : null;
            if (dataUrl) {
                const link = document.createElement('a');
                link.download = 'slide_' + (window._currentImportedSlideIndex + 1) + '.png';
                link.href = dataUrl;
                link.click();
                console.log('[PPTX Export] Exported current slide as PNG');
            } else {
                alert('Canvas not available');
            }
        };

        // Local template storage
        window.localTemplates = {
            STORAGE_KEY: 'axur_user_templates',
            save: function (template) {
                const templates = this.loadAll();
                template.id = 'local_' + Date.now();
                template.savedAt = new Date().toISOString();
                templates.push(template);
                localStorage.setItem(this.STORAGE_KEY, JSON.stringify(templates));
                console.log('[Templates] Saved locally:', template.id);
                return template.id;
            },
            loadAll: function () {
                try {
                    return JSON.parse(localStorage.getItem(this.STORAGE_KEY) || '[]');
                } catch (e) {
                    return [];
                }
            },
            load: function (id) {
                return this.loadAll().find(t => t.id === id);
            },
            delete: function (id) {
                const templates = this.loadAll().filter(t => t.id !== id);
                localStorage.setItem(this.STORAGE_KEY, JSON.stringify(templates));
                console.log('[Templates] Deleted:', id);
            }
        };

        // ==================== CSAT SYSTEM ====================
        window.csatSystem = {
            show: function () {
                if (document.getElementById('csat-toast')) return;

                const div = document.createElement('div');
                div.id = 'csat-toast';
                div.className = 'fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-zinc-800 border border-zinc-700 rounded-full px-4 py-2 shadow-2xl flex items-center gap-3 animate-in slide-in-from-bottom-5 fade-in duration-300';
                div.innerHTML = `
                    <span class="text-sm text-white font-medium">¿Qué tal funcionó?</span>
                    <div class="flex gap-2">
                        <button onclick="window.csatSystem.vote(5)" class="text-xl hover:scale-125 transition" title="Bien">😀</button>
                        <button onclick="window.csatSystem.vote(3)" class="text-xl hover:scale-125 transition" title="Regular">😐</button>
                        <button onclick="window.csatSystem.vote(1)" class="text-xl hover:scale-125 transition" title="Mal">☹️</button>
                    </div>
                    <button onclick="window.csatSystem.close()" class="ml-2 text-zinc-500 hover:text-white text-lg leading-none">&times;</button>
                `;
                document.body.appendChild(div);

                // Auto close after 10s
                setTimeout(() => this.close(), 10000);
            },

            close: function () {
                const el = document.getElementById('csat-toast');
                if (el) {
                    el.classList.add('animate-out', 'fade-out', 'slide-out-to-bottom-5');
                    setTimeout(() => el.remove(), 300);
                }
            },

            vote: async function (score) {
                this.close();

                if (score >= 4) {
                    const toast = document.createElement('div');
                    toast.className = 'fixed bottom-20 left-1/2 -translate-x-1/2 z-50 bg-green-600/90 backdrop-blur text-white px-4 py-1.5 rounded-full text-sm font-medium animate-in fade-in zoom-in duration-300';
                    toast.textContent = '¡Gracias por tu feedback!';
                    document.body.appendChild(toast);
                    setTimeout(() => {
                        toast.classList.add('animate-out', 'fade-out');
                        setTimeout(() => toast.remove(), 300);
                    }, 2000);
                } else if (score <= 3) {
                    // Encourage detailed feedback
                    const toast = document.createElement('div');
                    toast.className = 'fixed bottom-20 left-1/2 -translate-x-1/2 z-50 bg-zinc-800 text-zinc-300 px-4 py-1.5 rounded-full text-sm font-medium border border-zinc-600';
                    toast.innerHTML = '¿Algo salió mal? <button onclick="document.querySelector(\'[title=\\\'Enviar Feedback\\\']\').click()" class="text-orange-400 font-bold hover:underline ml-1">Repórtalo aquí</button>';
                    document.body.appendChild(toast);
                    setTimeout(() => toast.remove(), 5000);
                }

                try {
                    const apiBase = window.API_BASE_URL || '';
                    await fetch(`${apiBase}/api/feedback`, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            message: `[CSAT Vote] Score: ${score}/5`,
                            url: window.location.href,
                            user_agent: navigator.userAgent
                        })
                    });
                } catch (e) { console.error('[CSAT] Failed to log:', e); }
            }
        };

        // Expose globally for simpler Rust binding
        window.showCsat = function () { if (window.csatSystem) window.csatSystem.show(); };

        // ==================== CLOUD AUTO-SAVE ====================
        window.cloudAutoSave = {
            INTERVAL_MS: 30000, // 30 seconds
            _intervalId: null,
            _lastContent: null,
            _templateName: null,
            _isSaving: false,

            // Start auto-save (call after importing PPTX or loading template)
            start: function (templateName) {
                this._templateName = templateName || 'Untitled Template';
                if (this._intervalId) clearInterval(this._intervalId);
                this._intervalId = setInterval(() => this.save(), this.INTERVAL_MS);
                console.log('[CloudAutoSave] Started with name:', this._templateName);
            },

            stop: function () {
                if (this._intervalId) {
                    clearInterval(this._intervalId);
                    this._intervalId = null;
                }
                console.log('[CloudAutoSave] Stopped');
            },

            // Save current slides to cloud (if changed)
            save: async function () {
                if (this._isSaving) return;

                const slides = window._importedSlides;
                if (!slides || slides.length === 0) return;

                // Sync current canvas to slide first
                if (window.fabricCanvas && slides[window._currentImportedSlideIndex]) {
                    slides[window._currentImportedSlideIndex].canvasJson =
                        JSON.stringify(window.fabricCanvas.toJSON(['placeholderKey']));
                }

                // Check if content changed
                const currentContent = JSON.stringify(slides);
                if (currentContent === this._lastContent) {
                    console.log('[CloudAutoSave] No changes, skipping');
                    return;
                }

                this._isSaving = true;
                this._updateIndicator('saving');

                try {
                    const apiBase = window.API_BASE_URL || '';
                    const response = await fetch(`${apiBase}/api/templates/quick-save`, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        credentials: 'include',
                        body: JSON.stringify({
                            name: this._templateName,
                            description: 'Auto-saved template',
                            slides: slides.map((s, i) => ({
                                id: s.id || `slide-${i + 1}`,
                                name: s.name || `Slide ${i + 1}`,
                                canvas_json: s.canvasJson
                            }))
                        })
                    });

                    const result = await response.json();
                    if (result.success) {
                        this._lastContent = currentContent;
                        this._updateIndicator('saved');
                        console.log('[CloudAutoSave] Saved:', result.id, result.saved_at);
                    } else {
                        console.error('[CloudAutoSave] Failed:', result.message);
                        this._updateIndicator('error');
                    }
                } catch (e) {
                    console.error('[CloudAutoSave] Error:', e);
                    this._updateIndicator('error');
                }

                this._isSaving = false;
            },

            // Update save indicator in UI
            _updateIndicator: function (status) {
                const indicator = document.getElementById('autosave-indicator');
                if (!indicator) return;

                switch (status) {
                    case 'saving':
                        indicator.textContent = '☁️ Guardando...';
                        indicator.className = 'text-yellow-400 text-xs';
                        break;
                    case 'saved':
                        indicator.textContent = '☁️ Guardado';
                        indicator.className = 'text-green-400 text-xs';
                        setTimeout(() => {
                            if (indicator.textContent === '☁️ Guardado') {
                                indicator.textContent = '';
                            }
                        }, 3000);
                        break;
                    case 'error':
                        indicator.textContent = '⚠️ Error';
                        indicator.className = 'text-red-400 text-xs';
                        break;
                }
            }
        };

        // Auto-start when PPTX is imported
        const originalImport = window.importPptxFile;
        if (originalImport) {
            window.importPptxFile = async function (file) {
                const result = await originalImport.call(this, file);
                if (result && result.success) {
                    window.cloudAutoSave.start(file.name.replace('.pptx', ''));
                }
                return result;
            };
        }
    </script>

    <!-- Fabric.js for canvas editing -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/fabric.js/5.3.1/fabric.min.js"></script>
    <script>
        // FIX: Patch Fabric.js 5.3.1 for Chrome 130+ textBaseline issue
        if (typeof fabric !== 'undefined') {
            if (fabric.Text) fabric.Text.prototype.textBaseline = 'alphabetic';
            if (fabric.IText) fabric.IText.prototype.textBaseline = 'alphabetic';
        }
    </script>
    <script>
        // Global canvas instance
        let fabricCanvas = null;

        // Undo/Redo history
        let canvasHistory = [];
        let historyIndex = -1;
        const MAX_HISTORY = 50;

        // Save canvas state to history
        window.saveCanvasState = function () {
            if (!fabricCanvas) return;
            if (historyIndex < canvasHistory.length - 1) {
                canvasHistory = canvasHistory.slice(0, historyIndex + 1);
            }
            const json = JSON.stringify(fabricCanvas.toJSON(['placeholderKey', 'id'])); // Include ID via option?
            // Ideally fabricCanvas.toJSON() captures most props. We need ID for hybrid sync.
            // Fabric by default includes 'id' if property exists.
            canvasHistory.push(json);
            historyIndex = canvasHistory.length - 1;
            if (canvasHistory.length > MAX_HISTORY) {
                canvasHistory.shift();
                historyIndex--;
            }
        };

        // Undo last action
        window.undoCanvas = function () {
            if (!fabricCanvas || historyIndex <= 0) return false;
            historyIndex--;
            fabricCanvas.loadFromJSON(JSON.parse(canvasHistory[historyIndex]), function () {
                fabricCanvas.renderAll();
            });
            return true;
        };

        // Redo last undone action
        window.redoCanvas = function () {
            if (!fabricCanvas || historyIndex >= canvasHistory.length - 1) return false;
            historyIndex++;
            fabricCanvas.loadFromJSON(JSON.parse(canvasHistory[historyIndex]), function () {
                fabricCanvas.renderAll();
            });
            return true;
        };

        window.canUndo = function () { return historyIndex > 0; };
        window.canRedo = function () { return historyIndex < canvasHistory.length - 1; };

        // ==================== KEYBOARD UX ACTIONS ====================
        window._clipboard = null;

        window.copyObject = function () {
            if (!fabricCanvas) return;
            const activeObj = fabricCanvas.getActiveObject();
            if (activeObj) {
                activeObj.clone(function (cloned) {
                    window._clipboard = cloned;
                });
                console.log('[Editor] Copied object');
            }
        };

        window.pasteObject = function () {
            if (!fabricCanvas || !window._clipboard) return;
            window._clipboard.clone(function (clonedObj) {
                fabricCanvas.discardActiveObject();
                clonedObj.set({
                    left: clonedObj.left + 20,
                    top: clonedObj.top + 20,
                    evented: true,
                });
                if (clonedObj.type === 'activeSelection') {
                    // special handling for multiple objects
                    clonedObj.canvas = fabricCanvas;
                    clonedObj.forEachObject(function (obj) {
                        fabricCanvas.add(obj);
                    });
                    clonedObj.setCoords();
                } else {
                    fabricCanvas.add(clonedObj);
                }
                window._clipboard = clonedObj; // Enable multiple pastes "staircase" effect
                fabricCanvas.setActiveObject(clonedObj);
                fabricCanvas.requestRenderAll();
                window.saveCanvasState();
                console.log('[Editor] Pasted object');
            });
        };

        window.duplicateObject = function () {
            if (!fabricCanvas) return;
            const activeObj = fabricCanvas.getActiveObject();
            if (activeObj) {
                activeObj.clone(function (clonedObj) {
                    fabricCanvas.discardActiveObject();
                    clonedObj.set({
                        left: activeObj.left + 20,
                        top: activeObj.top + 20,
                        evented: true,
                    });
                    if (clonedObj.type === 'activeSelection') {
                        clonedObj.canvas = fabricCanvas;
                        clonedObj.forEachObject(function (obj) {
                            fabricCanvas.add(obj);
                        });
                        clonedObj.setCoords();
                    } else {
                        fabricCanvas.add(clonedObj);
                    }
                    fabricCanvas.setActiveObject(clonedObj);
                    fabricCanvas.requestRenderAll();
                    window.saveCanvasState();
                    console.log('[Editor] Duplicated object');
                });
            }
        };

        // Keyboard Shortcuts Listener
        document.addEventListener('keydown', function (e) {
            // Ignorar si estamos escribiendo en un input
            if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA' || e.target.isContentEditable) return;

            // Ctrl+Z (Undo)
            if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
                e.preventDefault();
                window.undoCanvas();
            }
            // Ctrl+Shift+Z or Ctrl+Y (Redo)
            if ((e.ctrlKey || e.metaKey) && ((e.shiftKey && e.key === 'z') || e.key === 'y')) {
                e.preventDefault();
                window.redoCanvas();
            }
            // Ctrl+C (Copy)
            if ((e.ctrlKey || e.metaKey) && e.key === 'c') {
                e.preventDefault();
                window.copyObject();
            }
            // Ctrl+V (Paste)
            if ((e.ctrlKey || e.metaKey) && e.key === 'v') {
                e.preventDefault();
                window.pasteObject();
            }
            // Ctrl+D (Duplicate)
            if ((e.ctrlKey || e.metaKey) && e.key === 'd') {
                e.preventDefault();
                window.duplicateObject();
            }
            // Delete / Backspace
            if (e.key === 'Delete' || e.key === 'Backspace') {
                if (fabricCanvas && fabricCanvas.getActiveObject()) {
                    e.preventDefault();
                    const activeObjects = fabricCanvas.getActiveObjects();
                    if (activeObjects.length) {
                        fabricCanvas.discardActiveObject();
                        // Iterate reversed copy to avoid index issues
                        activeObjects.forEach(function (obj) {
                            fabricCanvas.remove(obj);
                        });
                    } else {
                        fabricCanvas.remove(fabricCanvas.getActiveObject());
                    }
                    window.saveCanvasState();
                }
            }
        });

        // Initialize Fabric.js canvas
        window.initFabricCanvas = function (canvasId) {
            console.log('[Fabric] Initializing canvas:', canvasId);
            const el = document.getElementById(canvasId);
            if (!el) {
                console.error('[Fabric] Canvas element not found:', canvasId);
                return null;
            }
            fabricCanvas = new fabric.Canvas(canvasId, {
                backgroundColor: '#1e293b',
                selection: true,
            });

            canvasHistory = [];
            historyIndex = -1;
            window.saveCanvasState();

            fabricCanvas.on('object:modified', window.saveCanvasState);
            fabricCanvas.on('object:added', window.saveCanvasState);
            fabricCanvas.on('object:removed', window.saveCanvasState);

            // HYBRID SYNC: Update XML on change
            fabricCanvas.on('object:modified', function (e) {
                const obj = e.target;
                if (obj.id && window.PPTXImporter && window.PPTXImporter.patchSlideXML) {
                    const slideNum = (window._currentImportedSlideIndex || 0) + 1;
                    window.PPTXImporter.patchSlideXML(slideNum, [{
                        id: obj.id,
                        left: obj.left,
                        top: obj.top
                    }]);
                }
            });

            // Add grid overlay for visual guidance
            window.toggleCanvasGrid(window._gridEnabled !== false); // Default ON

            console.log('[Fabric] Canvas initialized with history, hybrid sync, and grid');
            return fabricCanvas;
        };

        // Grid state
        window._gridEnabled = true;
        window._gridLines = [];

        // Toggle canvas grid on/off
        window.toggleCanvasGrid = function (enabled) {
            if (!fabricCanvas) return;
            window._gridEnabled = enabled;

            // Remove existing grid lines
            window._gridLines.forEach(line => fabricCanvas.remove(line));
            window._gridLines = [];

            if (!enabled) {
                fabricCanvas.renderAll();
                return;
            }

            const gridSize = 60; // 60px grid cells
            const canvasWidth = fabricCanvas.width || 960;
            const canvasHeight = fabricCanvas.height || 540;
            const gridColor = 'rgba(255,255,255,0.05)';

            // Vertical lines
            for (let x = gridSize; x < canvasWidth; x += gridSize) {
                const line = new fabric.Line([x, 0, x, canvasHeight], {
                    stroke: gridColor,
                    selectable: false,
                    evented: false,
                    strokeWidth: 1,
                    excludeFromExport: true
                });
                window._gridLines.push(line);
                fabricCanvas.add(line);
                fabricCanvas.sendToBack(line);
            }

            // Horizontal lines
            for (let y = gridSize; y < canvasHeight; y += gridSize) {
                const line = new fabric.Line([0, y, canvasWidth, y], {
                    stroke: gridColor,
                    selectable: false,
                    evented: false,
                    strokeWidth: 1,
                    excludeFromExport: true
                });
                window._gridLines.push(line);
                fabricCanvas.add(line);
                fabricCanvas.sendToBack(line);
            }

            // Center lines (slightly more visible)
            const centerX = canvasWidth / 2;
            const centerY = canvasHeight / 2;
            const centerColor = 'rgba(99,102,241,0.15)'; // Indigo tint

            const centerVLine = new fabric.Line([centerX, 0, centerX, canvasHeight], {
                stroke: centerColor,
                selectable: false,
                evented: false,
                strokeWidth: 1,
                excludeFromExport: true
            });
            window._gridLines.push(centerVLine);
            fabricCanvas.add(centerVLine);
            fabricCanvas.sendToBack(centerVLine);

            const centerHLine = new fabric.Line([0, centerY, canvasWidth, centerY], {
                stroke: centerColor,
                selectable: false,
                evented: false,
                strokeWidth: 1,
                excludeFromExport: true
            });
            window._gridLines.push(centerHLine);
            fabricCanvas.add(centerHLine);
            fabricCanvas.sendToBack(centerHLine);

            fabricCanvas.renderAll();
            console.log('[Fabric] Grid ' + (enabled ? 'enabled' : 'disabled'));
        };
        // ==================== KEYBOARD SHORTCUTS ====================
        document.addEventListener('keydown', function (e) {
            if (!fabricCanvas) return;

            // Ignore if typing in input
            if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

            // Ctrl+Z = Undo
            if (e.ctrlKey && e.key === 'z' && !e.shiftKey) {
                e.preventDefault();
                window.undoCanvas();
            }
            // Ctrl+Y or Ctrl+Shift+Z = Redo
            if ((e.ctrlKey && e.key === 'y') || (e.ctrlKey && e.shiftKey && e.key === 'z')) {
                e.preventDefault();
                window.redoCanvas();
            }
            // Delete = Remove selected
            if (e.key === 'Delete' || e.key === 'Backspace') {
                const active = fabricCanvas.getActiveObjects();
                if (active.length > 0) {
                    e.preventDefault();
                    active.forEach(obj => fabricCanvas.remove(obj));
                    fabricCanvas.discardActiveObject();
                    fabricCanvas.renderAll();
                }
            }
            // Ctrl+D = Duplicate selected
            if (e.ctrlKey && e.key === 'd') {
                e.preventDefault();
                window.duplicateSelected();
            }
            // Ctrl+A = Select all
            if (e.ctrlKey && e.key === 'a') {
                e.preventDefault();
                fabricCanvas.discardActiveObject();
                const sel = new fabric.ActiveSelection(fabricCanvas.getObjects(), { canvas: fabricCanvas });
                fabricCanvas.setActiveObject(sel);
                fabricCanvas.renderAll();
            }
        });

        // ==================== ZOOM CONTROLS ====================
        let currentZoom = 1;

        window.setCanvasZoom = function (zoom) {
            if (!fabricCanvas) return;
            currentZoom = Math.max(0.25, Math.min(2, zoom));
            fabricCanvas.setZoom(currentZoom);
            fabricCanvas.renderAll();
            return currentZoom;
        };

        window.zoomIn = function () { return window.setCanvasZoom(currentZoom + 0.1); };
        window.zoomOut = function () { return window.setCanvasZoom(currentZoom - 0.1); };
        window.zoomReset = function () { return window.setCanvasZoom(1); };
        window.getZoom = function () { return currentZoom; };

        // ==================== ALIGNMENT TOOLS ====================
        window.alignSelected = function (direction) {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObjects();
            if (active.length < 2) return;

            const bounds = fabricCanvas.getActiveObject().getBoundingRect();

            active.forEach(obj => {
                switch (direction) {
                    case 'left': obj.set('left', bounds.left); break;
                    case 'center': obj.set('left', bounds.left + (bounds.width - obj.width * obj.scaleX) / 2); break;
                    case 'right': obj.set('left', bounds.left + bounds.width - obj.width * obj.scaleX); break;
                    case 'top': obj.set('top', bounds.top); break;
                    case 'middle': obj.set('top', bounds.top + (bounds.height - obj.height * obj.scaleY) / 2); break;
                    case 'bottom': obj.set('top', bounds.top + bounds.height - obj.height * obj.scaleY); break;
                }
                obj.setCoords();
            });
            fabricCanvas.renderAll();
        };

        // ==================== LAYER CONTROLS ====================
        window.bringToFront = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObject();
            if (active) {
                fabricCanvas.bringToFront(active);
                fabricCanvas.renderAll();
            }
        };

        window.sendToBack = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObject();
            if (active) {
                fabricCanvas.sendToBack(active);
                fabricCanvas.renderAll();
            }
        };

        window.bringForward = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObject();
            if (active) {
                fabricCanvas.bringForward(active);
                fabricCanvas.renderAll();
            }
        };

        window.sendBackward = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObject();
            if (active) {
                fabricCanvas.sendBackwards(active);
                fabricCanvas.renderAll();
            }
        };

        // ==================== DELETE & DUPLICATE ====================
        window.deleteSelected = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObjects();
            active.forEach(obj => fabricCanvas.remove(obj));
            fabricCanvas.discardActiveObject();
            fabricCanvas.renderAll();
        };

        window.duplicateSelected = function () {
            if (!fabricCanvas) return;
            const active = fabricCanvas.getActiveObject();
            if (!active) return;

            active.clone(function (cloned) {
                cloned.set({
                    left: cloned.left + 20,
                    top: cloned.top + 20,
                });
                fabricCanvas.add(cloned);
                fabricCanvas.setActiveObject(cloned);
                fabricCanvas.renderAll();
            });
        };

        // ==================== GET OBJECTS LIST (for layers panel) ====================
        window.getCanvasObjects = function () {
            if (!fabricCanvas) return '[]';
            const objects = fabricCanvas.getObjects().map((obj, idx) => ({
                index: idx,
                type: obj.type,
                name: obj.placeholderKey || obj.text || obj.type,
                visible: obj.visible !== false,
                locked: obj.lockMovementX && obj.lockMovementY
            }));
            return JSON.stringify(objects);
        };

        window.selectObjectByIndex = function (idx) {
            if (!fabricCanvas) return;
            const objects = fabricCanvas.getObjects();
            if (idx >= 0 && idx < objects.length) {
                fabricCanvas.setActiveObject(objects[idx]);
                fabricCanvas.renderAll();
            }
        };

        // Add text element
        window.addTextToCanvas = function (text) {
            if (!fabricCanvas) return;
            const textObj = new fabric.IText(text, {
                left: 100,
                top: 100,
                fill: '#f8fafc',
                fontSize: 24,
                fontFamily: 'Inter, sans-serif',
            });
            fabricCanvas.add(textObj);
            fabricCanvas.setActiveObject(textObj);
            fabricCanvas.renderAll();
        };

        // Add placeholder (as group with border)
        window.addPlaceholderToCanvas = function (key, html) {
            if (!fabricCanvas) return;
            // Create placeholder as a styled rect with label
            const rect = new fabric.Rect({
                width: 200,
                height: 80,
                fill: 'rgba(99, 102, 241, 0.2)',
                stroke: '#6366f1',
                strokeWidth: 2,
                strokeDashArray: [5, 5],
                rx: 8,
                ry: 8,
            });
            const label = new fabric.Text('{{' + key + '}}', {
                fontSize: 14,
                fill: '#6366f1',
                fontFamily: 'DM Mono, monospace',
            });
            // Center label in rect
            label.set({
                left: (rect.width - label.width) / 2,
                top: (rect.height - label.height) / 2,
            });
            const group = new fabric.Group([rect, label], {
                left: 150,
                top: 150,
            });
            group.set('placeholderKey', key);
            fabricCanvas.add(group);
            fabricCanvas.setActiveObject(group);
            fabricCanvas.renderAll();
        };

        // === Preview Mode (Real-time placeholder rendering) ===
        window.previewModeEnabled = false;

        // Mock data for placeholders (sample values for preview)
        window.placeholderMockData = {
            // General
            'company_name': 'Acme Corporation',
            'date_range': 'Ene 2026 - Dic 2026',
            'tlp_level': 'TLP:AMBER',
            // Metrics
            'total_tickets': '1,247',
            'total_threats': '892',
            'hours_saved': '156h',
            // Threats
            'top_threat_1_name': 'Phishing',
            'top_threat_1_count': '234',
            'threats_by_type': '🎯 Phishing 45% | 🔐 Creds 30%',
            // Credentials
            'credentials_total': '3,456',
            'credentials_critical': '127',
            'stealer_log_count': '89',
            // Takedowns
            'takedown_total': '284',
            'takedown_success_rate': '94.2%',
            'takedown_resolved': '267',
            // Risk
            'risk_score_value': '72',
            'risk_score_label': 'Moderado',
        };

        // Toggle preview mode on all placeholders
        window.togglePreviewMode = function (enabled) {
            if (!fabricCanvas) return false;
            window.previewModeEnabled = enabled;

            fabricCanvas.getObjects().forEach(function (obj) {
                const key = obj.get('placeholderKey');
                if (!key) return;

                // Groups contain rect + text
                if (obj._objects && obj._objects.length > 1) {
                    const textObj = obj._objects.find(o => o.type === 'text');
                    if (textObj) {
                        if (enabled) {
                            // Store original and show mock
                            // Check if this is a conditional placeholder
                            let mockValue;
                            if (window.conditionalPlaceholders && window.conditionalPlaceholders[key]) {
                                mockValue = window.evaluateConditional(key);
                            } else {
                                mockValue = window.placeholderMockData[key] || '(sin datos)';
                            }
                            textObj.set('originalText', textObj.text);
                            textObj.set('text', mockValue);
                            // Use cyan for conditional, green for regular
                            const isConditional = window.conditionalPlaceholders && window.conditionalPlaceholders[key];
                            textObj.set('fill', isConditional ? '#06b6d4' : '#22c55e');
                        } else {
                            // Restore original placeholder
                            const original = textObj.get('originalText') || '{{' + key + '}}';
                            textObj.set('text', original);
                            textObj.set('fill', '#6366f1'); // Indigo for placeholder
                        }
                    }
                }
            });

            fabricCanvas.renderAll();
            console.log('[Preview] Mode:', enabled ? 'ON' : 'OFF');
            return enabled;
        };

        // Get current preview mode state
        window.getPreviewMode = function () {
            return window.previewModeEnabled;
        };

        // === Smart Templates: Placeholder Categories ===

        // Map placeholder keys to data categories
        window.placeholderCategories = {
            // General
            'company_name': 'General',
            'date_range': 'General',
            'tlp_level': 'General',
            // Metrics
            'total_tickets': 'Metrics',
            'total_threats': 'Metrics',
            'hours_saved': 'Metrics',
            // Threats
            'top_threat_1_name': 'Threats',
            'top_threat_1_count': 'Threats',
            'top_threat_2_name': 'Threats',
            'top_threat_3_name': 'Threats',
            'threats_by_type': 'Threats',
            // Credentials
            'credentials_total': 'Credentials',
            'credentials_critical': 'Credentials',
            'stealer_log_count': 'Credentials',
            // Takedowns
            'takedown_total': 'Takedowns',
            'takedown_success_rate': 'Takedowns',
            'takedown_resolved': 'Takedowns',
            // Risk
            'risk_score_value': 'Risk',
            'risk_score_label': 'Risk',
        };

        // All possible categories with descriptions
        window.dataCategories = {
            'General': { icon: '📋', name: 'Información General', color: '#6366f1' },
            'Metrics': { icon: '📊', name: 'Métricas', color: '#f97316' },
            'Threats': { icon: '🎯', name: 'Amenazas', color: '#ef4444' },
            'Credentials': { icon: '🔐', name: 'Credenciales', color: '#a855f7' },
            'Takedowns': { icon: '✅', name: 'Takedowns', color: '#22c55e' },
            'Risk': { icon: '⚠️', name: 'Risk Score', color: '#f59e0b' },
            'Condicional': { icon: '🔀', name: 'Condicionales', color: '#06b6d4' },
        };

        // === Smart Placeholders: Conditional Logic ===

        // Pre-defined conditional placeholders
        window.conditionalPlaceholders = {
            'risk_status': {
                label: 'Estado de Riesgo',
                condition: 'risk_score_value',
                rules: [
                    { op: '>', value: 70, result: '🔴 CRÍTICO: Acción inmediata requerida' },
                    { op: '>', value: 40, result: '🟡 ALERTA: Monitorear situación' },
                    { op: '>=', value: 0, result: '🟢 Estado normal' }
                ]
            },
            'threat_level': {
                label: 'Nivel de Amenazas',
                condition: 'total_threats',
                rules: [
                    { op: '>', value: 500, result: '⚠️ Nivel Alto - Requiere atención prioritaria' },
                    { op: '>', value: 100, result: '⚡ Nivel Medio - Monitoreo continuo' },
                    { op: '>=', value: 0, result: '✅ Nivel Bajo - Bajo control' }
                ]
            },
            'credential_alert': {
                label: 'Alerta Credenciales',
                condition: 'credentials_critical',
                rules: [
                    { op: '>', value: 100, result: '🚨 ALERTA CRÍTICA: +100 credenciales expuestas' },
                    { op: '>', value: 50, result: '⚠️ Alerta: Más de 50 credenciales críticas' },
                    { op: '>=', value: 0, result: 'ℹ️ Credenciales bajo control' }
                ]
            },
            'takedown_efficiency': {
                label: 'Eficiencia Takedowns',
                condition: 'takedown_success_rate',
                rules: [
                    { op: '>=', value: 90, result: '🏆 Excelente: Eficiencia superior al 90%' },
                    { op: '>=', value: 70, result: '👍 Buena: Eficiencia sobre 70%' },
                    { op: '>=', value: 0, result: '📈 Oportunidad de mejora' }
                ]
            }
        };

        // Evaluate a conditional placeholder
        window.evaluateConditional = function (placeholderKey) {
            const config = window.conditionalPlaceholders[placeholderKey];
            if (!config) return `{{${placeholderKey}}}`;

            // Get the value of the condition variable
            const conditionVar = config.condition;
            let rawValue = window.placeholderMockData[conditionVar] || '0';
            // Parse numeric value (remove % and other non-numeric chars)
            let numValue = parseFloat(rawValue.replace(/[^0-9.-]/g, ''));
            if (isNaN(numValue)) numValue = 0;

            // Evaluate rules in order
            for (const rule of config.rules) {
                let match = false;
                switch (rule.op) {
                    case '>': match = numValue > rule.value; break;
                    case '<': match = numValue < rule.value; break;
                    case '>=': match = numValue >= rule.value; break;
                    case '<=': match = numValue <= rule.value; break;
                    case '==': match = numValue === rule.value; break;
                    case '!=': match = numValue !== rule.value; break;
                }
                if (match) return rule.result;
            }
            return config.rules[config.rules.length - 1]?.result || '';
        };

        // Add conditional placeholder category mappings
        window.placeholderCategories['risk_status'] = 'Condicional';
        window.placeholderCategories['threat_level'] = 'Condicional';
        window.placeholderCategories['credential_alert'] = 'Condicional';
        window.placeholderCategories['takedown_efficiency'] = 'Condicional';

        // Analyze template to find which categories are used
        // If slidesJsonArray is provided, also parse those (for full project analysis)
        window.analyzeTemplateData = function (slidesJsonArray) {
            const usedCategories = new Set();
            const usedPlaceholders = [];

            // 1. Analyze current canvas
            if (fabricCanvas) {
                fabricCanvas.getObjects().forEach(function (obj) {
                    const key = obj.get('placeholderKey');
                    if (key) {
                        usedPlaceholders.push(key);
                        const category = window.placeholderCategories[key];
                        if (category) {
                            usedCategories.add(category);
                        }
                    }
                });
            }

            // 2. Analyze all slides from stored JSON (if provided)
            if (slidesJsonArray && Array.isArray(slidesJsonArray)) {
                slidesJsonArray.forEach(function (jsonStr) {
                    try {
                        const canvasData = JSON.parse(jsonStr);
                        if (canvasData.objects && Array.isArray(canvasData.objects)) {
                            canvasData.objects.forEach(function (obj) {
                                // Check for placeholderKey in object properties
                                const key = obj.placeholderKey;
                                if (key && !usedPlaceholders.includes(key)) {
                                    usedPlaceholders.push(key);
                                    const category = window.placeholderCategories[key];
                                    if (category) {
                                        usedCategories.add(category);
                                    }
                                }
                                // Also check in group objects
                                if (obj.objects && Array.isArray(obj.objects)) {
                                    obj.objects.forEach(function (child) {
                                        const childKey = child.placeholderKey;
                                        if (childKey && !usedPlaceholders.includes(childKey)) {
                                            usedPlaceholders.push(childKey);
                                            const cat = window.placeholderCategories[childKey];
                                            if (cat) usedCategories.add(cat);
                                        }
                                    });
                                }
                            });
                        }
                    } catch (e) {
                        console.warn('[SmartTemplates] Failed to parse slide JSON:', e);
                    }
                });
            }

            const allCategories = Object.keys(window.dataCategories);
            const used = allCategories.filter(c => usedCategories.has(c));
            const unused = allCategories.filter(c => !usedCategories.has(c));

            console.log('[SmartTemplates] Full Project Analysis:', {
                slides: slidesJsonArray ? slidesJsonArray.length : 0,
                placeholders: usedPlaceholders.length,
                usedCategories: used.length,
                unusedCategories: unused.length
            });

            return {
                used: used,
                unused: unused,
                placeholders: usedPlaceholders
            };
        };

        // Get category details for UI
        window.getCategoryDetails = function () {
            return window.dataCategories;
        };

        // Add shape
        window.addShapeToCanvas = function (shapeType) {
            if (!fabricCanvas) return;
            let shape;
            if (shapeType === 'rectangle') {
                shape = new fabric.Rect({
                    left: 100,
                    top: 100,
                    width: 150,
                    height: 100,
                    fill: '#3f3f46',
                    stroke: '#52525b',
                    strokeWidth: 1,
                    rx: 4,
                    ry: 4,
                });
            } else if (shapeType === 'circle') {
                shape = new fabric.Circle({
                    left: 100,
                    top: 100,
                    radius: 50,
                    fill: '#3f3f46',
                    stroke: '#52525b',
                    strokeWidth: 1,
                });
            }
            if (shape) {
                fabricCanvas.add(shape);
                fabricCanvas.setActiveObject(shape);
                fabricCanvas.renderAll();
            }
        };

        // Get canvas JSON (with error handling for styles)
        window.getCanvasJSON = function () {
            if (!fabricCanvas) return '{}';
            try {
                // Ensure all text objects have styles property to avoid stylesToArray error
                fabricCanvas.getObjects().forEach(function (obj) {
                    if (obj.type === 'text' || obj.type === 'i-text' || obj.type === 'textbox') {
                        if (!obj.styles) obj.styles = {};
                    }
                });
                return JSON.stringify(fabricCanvas.toJSON(['placeholderKey']));
            } catch (e) {
                console.error('[Fabric] Error serializing canvas:', e);
                return '{}';
            }
        };

        // Load canvas JSON
        window.loadCanvasJSON = function (json) {
            if (!fabricCanvas) return;
            try {
                const data = JSON.parse(json);
                // Initialize styles property for all text objects in the data
                if (data.objects) {
                    data.objects.forEach(function (obj) {
                        if (obj.type === 'text' || obj.type === 'i-text' || obj.type === 'textbox') {
                            if (!obj.styles) obj.styles = {};
                        }
                    });
                }
                fabricCanvas.loadFromJSON(data, function () {
                    // Ensure styles after loading too
                    fabricCanvas.getObjects().forEach(function (obj) {
                        if (obj.type === 'text' || obj.type === 'i-text' || obj.type === 'textbox') {
                            if (!obj.styles) obj.styles = {};
                        }
                    });

                    // Scale background image to fit canvas if needed
                    const bgImg = fabricCanvas.backgroundImage;
                    if (bgImg && bgImg.width && bgImg.height) {
                        const canvasW = fabricCanvas.width;
                        const canvasH = fabricCanvas.height;
                        const imgW = bgImg.width;
                        const imgH = bgImg.height;

                        // Calculate scale to FIT canvas (show entire image, maintain aspect ratio)
                        const scaleX = canvasW / imgW;
                        const scaleY = canvasH / imgH;
                        const scale = Math.min(scaleX, scaleY); // Use min to show entire image

                        bgImg.scaleX = scale;
                        bgImg.scaleY = scale;
                        bgImg.set({ left: 0, top: 0, originX: 'left', originY: 'top' });

                        console.log('[Fabric] Background scaled from ' + imgW + 'x' + imgH + ' to fit canvas');
                    }

                    fabricCanvas.renderAll();
                    console.log('[Fabric] Canvas loaded successfully');
                });
            } catch (e) {
                console.error('[Fabric] Failed to load JSON:', e);
            }
        };

        // Clear canvas
        window.clearCanvas = function () {
            if (!fabricCanvas) return;
            fabricCanvas.clear();
            fabricCanvas.backgroundColor = '#1e293b';
            fabricCanvas.renderAll();
        };

        // Add image to canvas
        window.addImageToCanvas = function (dataUrl) {
            if (!fabricCanvas) return;
            fabric.Image.fromURL(dataUrl, function (img) {
                // Scale image to fit canvas if too large
                const maxWidth = fabricCanvas.width * 0.8;
                const maxHeight = fabricCanvas.height * 0.8;
                if (img.width > maxWidth || img.height > maxHeight) {
                    const scale = Math.min(maxWidth / img.width, maxHeight / img.height);
                    img.scale(scale);
                }
                img.set({
                    left: 50,
                    top: 50,
                });
                fabricCanvas.add(img);
                fabricCanvas.setActiveObject(img);
                fabricCanvas.renderAll();
            });
        };

        // Get canvas thumbnail as base64
        window.getCanvasThumbnail = function () {
            if (!fabricCanvas) return '';
            try {
                return fabricCanvas.toDataURL({
                    format: 'png',
                    quality: 0.5,
                    multiplier: 0.25  // 1/4 size thumbnail
                });
            } catch (e) {
                console.error('[Fabric] Failed to get thumbnail:', e);
                return '';
            }
        };

        // Trigger image file picker
        window.triggerImageUpload = function () {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = 'image/*';
            input.onchange = function (e) {
                const file = e.target.files[0];
                if (file) {
                    const reader = new FileReader();
                    reader.onload = function (event) {
                        window.addImageToCanvas(event.target.result);
                    };
                    reader.readAsDataURL(file);
                }
            };
            input.click();
        };

        // Setup drag and drop for canvas
        window.setupCanvasDragDrop = function () {
            const canvasContainer = document.querySelector('.canvas-drop-zone');
            if (!canvasContainer) {
                console.log('[DragDrop] Canvas container not found, using body');
                return;
            }

            canvasContainer.addEventListener('dragover', function (e) {
                e.preventDefault();
                e.stopPropagation();
                canvasContainer.classList.add('drag-over');
            });

            canvasContainer.addEventListener('dragleave', function (e) {
                e.preventDefault();
                e.stopPropagation();
                canvasContainer.classList.remove('drag-over');
            });

            canvasContainer.addEventListener('drop', function (e) {
                e.preventDefault();
                e.stopPropagation();
                canvasContainer.classList.remove('drag-over');

                // Check for text/plain data first (placeholder drops from modal)
                const textData = e.dataTransfer.getData('text/plain');
                if (textData && textData.startsWith('PLACEHOLDER:')) {
                    // Format: PLACEHOLDER:key:html
                    const parts = textData.split(':');
                    if (parts.length >= 3) {
                        const key = parts[1];
                        const html = parts.slice(2).join(':'); // Rejoin in case HTML had colons
                        console.log('[DragDrop] Placeholder dropped:', key);
                        if (window.addPlaceholderToCanvas) {
                            window.addPlaceholderToCanvas(key, html);
                        }
                    }
                    return; // Handled placeholder drop
                }

                // Otherwise handle file drops (images, PPTX)
                const files = e.dataTransfer.files;
                if (files.length > 0) {
                    const file = files[0];
                    if (file.type.startsWith('image/')) {
                        const reader = new FileReader();
                        reader.onload = function (event) {
                            window.addImageToCanvas(event.target.result);
                        };
                        reader.readAsDataURL(file);
                    } else if (file.name.endsWith('.pptx')) {
                        window.handlePptxImport(file);
                    }
                }
            });

            console.log('[DragDrop] Canvas drag & drop enabled (files + placeholders)');
        };

        // Trigger PPTX file picker
        window.triggerPptxUpload = function () {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = '.pptx,.ppt';
            input.onchange = function (e) {
                const file = e.target.files[0];
                if (file) window.handlePptxImport(file);
            };
            input.click();
        };
    </script>
</head>

<body class="bg-zinc-950 text-zinc-100">

    <div id="loading-indicator" class="min-h-screen flex items-center justify-center">
        <div class="text-center">
            <div class="inline-flex items-center gap-2 mb-6">
                <span class="text-orange-500 text-4xl font-black italic">///</span>
                <span class="text-white text-3xl font-bold tracking-widest">AXUR</span>
            </div>
            <div class="flex items-center justify-center gap-2 text-zinc-400">
                <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                    </path>
                </svg>
                <span>Cargando aplicación...</span>
            </div>
        </div>
    </div>


</body>

</html>
```

## File: Trunk.toml
```toml
[build]
target = "index.html"

[serve]
address = "127.0.0.1"
port = 8080
open = false

[[proxy]]
rewrite = "/api/"
backend = "http://localhost:3001/api/"
```

## File: Cargo.toml
```toml
[package]
name = "axur-frontend"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# Leptos framework
leptos = { version = "0.6", features = ["csr"] }

# HTTP client for WASM
gloo-net = "0.6"
gloo-storage = "0.3"
gloo-timers = { version = "0.3", features = ["futures"] }

# Serialization
serde.workspace = true
serde_json.workspace = true

# Utilities
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "Window", "Document", "HtmlElement", "console",
    "Blob", "BlobPropertyBag", "Url", "RequestCredentials",
    "Navigator", "Clipboard", "EventSource", "MessageEvent",
    "EventSourceInit", "Event", "File", "FormData",
    "DragEvent", "DataTransfer", "HtmlAnchorElement"
] }
console_error_panic_hook = "0.1"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```