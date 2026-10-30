use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTelemetryEvent {
    pub id: usize,
    pub timestamp: String,
    pub client_tool: String,
    pub target_file: String,
    pub slop_found: usize,
    pub score: usize,
    pub latency_ms: u128,
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryState {
    pub total_scans: usize,
    pub total_slop_blocked: usize,
    pub current_hygiene_score: usize,
    pub avg_latency_ms: f64,
    pub events: Vec<McpTelemetryEvent>,
}
impl Default for TelemetryState {
    fn default() -> Self {
        Self {
            total_scans: 0,
            total_slop_blocked: 0,
            current_hygiene_score: 100,
            avg_latency_ms: 2.1,
            events: Vec::new(),
        }
    }
}
pub type SharedTelemetry = Arc<Mutex<TelemetryState>>;
pub fn create_shared_telemetry() -> SharedTelemetry {
    Arc::new(Mutex::new(TelemetryState::default()))
}
fn get_current_time_str() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let hours = (secs / 3600) % 24;
    let mins = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, s)
}
pub fn record_event(shared: &SharedTelemetry, target_root: &Path, tool: &str, file: &str, slop: usize, score: usize, latency: u128, msg: &str) {
    if let Ok(mut state) = shared.lock() {
        state.total_scans += 1;
        state.total_slop_blocked += slop;
        state.current_hygiene_score = score;
        let total_lat = (state.avg_latency_ms * (state.total_scans - 1) as f64) + latency as f64;
        state.avg_latency_ms = (total_lat / state.total_scans as f64 * 10.0).round() / 10.0;
        let event = McpTelemetryEvent {
            id: state.total_scans,
            timestamp: get_current_time_str(),
            client_tool: tool.to_string(),
            target_file: file.to_string(),
            slop_found: slop,
            score,
            latency_ms: latency,
            message: msg.to_string(),
        };
        state.events.insert(0, event);
        if state.events.len() > 100 {
            state.events.truncate(100);
        }
        let dot_dir = target_root.join(".slopguard");
        let _ = fs::create_dir_all(&dot_dir);
        let log_file = dot_dir.join("telemetry.json");
        if let Ok(serialized) = serde_json::to_string(&*state) {
            let _ = fs::write(log_file, serialized);
        }
    }
}
pub fn load_persisted_telemetry(target_root: &Path) -> TelemetryState {
    let log_file = target_root.join(".slopguard").join("telemetry.json");
    if log_file.exists() {
        if let Ok(content) = fs::read_to_string(log_file) {
            if let Ok(parsed) = serde_json::from_str::<TelemetryState>(&content) {
                return parsed;
            }
        }
    }
    TelemetryState::default()
}