use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tej_core::{ProgressUpdate, TestConfig, TestPhase};

use crate::state::AppState;

#[derive(Clone, Serialize)]
struct ProgressEvent {
    phase: String,
    speed_mbps: Option<f64>,
    progress: f64,
    latency_ms: Option<f64>,
}

#[tauri::command]
pub async fn start_speed_test(
    app: AppHandle,
    state: State<'_, AppState>,
    connections: Option<usize>,
) -> Result<tej_core::SpeedTestResult, String> {
    let mut config = TestConfig::default();
    if let Some(c) = connections {
        if c == 0 || c > 32 {
            return Err("connections must be between 1 and 32".to_string());
        }
        config.parallel_connections = c;
    }

    let app_handle = app.clone();
    let progress: tej_core::ProgressCallback = Arc::new(move |update: ProgressUpdate| {
        let phase = match update.phase {
            TestPhase::Latency => "latency",
            TestPhase::Download => "download",
            TestPhase::Upload => "upload",
            TestPhase::PacketLoss => "packet_loss",
            TestPhase::Done => "done",
        };

        let _ = app_handle.emit(
            "speed-test-progress",
            ProgressEvent {
                phase: phase.to_string(),
                speed_mbps: update.speed_mbps,
                progress: update.progress,
                latency_ms: update.latency_ms,
            },
        );
    });

    let result = tej_core::run_speed_test(&config, Some(progress))
        .await
        .map_err(|e| e.to_string())?;

    state.save_result(result.clone());

    Ok(result)
}

#[tauri::command]
pub fn get_history(state: State<'_, AppState>) -> Vec<tej_core::SpeedTestResult> {
    state.get_history()
}
