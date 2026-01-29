use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use bytes::Bytes;
use tokio::task::JoinSet;

use crate::config::TestConfig;
use crate::data::random_payload;
use crate::error::Result;
use crate::progress::{ProgressCallback, ProgressUpdate, TestPhase};
use crate::results::ThroughputResult;

pub async fn measure_upload(
    client: &reqwest::Client,
    config: &TestConfig,
    progress: Option<&ProgressCallback>,
) -> Result<ThroughputResult> {
    let upload_size = config.upload_size;
    let total_bytes = Arc::new(AtomicU64::new(0));
    let start = Instant::now();
    let mut tasks = JoinSet::new();

    // Pre-generate payload once, share via zero-copy Bytes across connections
    let payload: Bytes = random_payload(upload_size).into();

    for _ in 0..config.parallel_connections {
        let client = client.clone();
        let url = config.upload_url.clone();
        let total = total_bytes.clone();
        let data = payload.clone(); // Bytes::clone is O(1) ref-count bump

        tasks.spawn(async move {
            let size = data.len() as u64;
            client
                .post(&url)
                .header("Content-Type", "application/octet-stream")
                .body(data)
                .send()
                .await?
                .error_for_status()?;
            total.fetch_add(size, Ordering::Release);
            Ok::<(), crate::error::SpeedTestError>(())
        });
    }

    // Progress reporting loop
    let progress_total = total_bytes.clone();
    let expected_total = (upload_size * config.parallel_connections) as f64;
    let progress_handle = if let Some(cb) = progress {
        let cb = cb.clone();
        Some(tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                let bytes = progress_total.load(Ordering::Acquire);
                let elapsed = start.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let speed_bps = (bytes as f64 * 8.0) / elapsed;
                    cb(ProgressUpdate {
                        phase: TestPhase::Upload,
                        speed_mbps: Some(speed_bps / 1_000_000.0),
                        progress: (bytes as f64 / expected_total).min(1.0),
                        latency_ms: None,
                    });
                }
            }
        }))
    } else {
        None
    };

    while let Some(result) = tasks.join_next().await {
        result.map_err(|e| crate::error::SpeedTestError::Other(e.to_string()))??;
    }

    if let Some(handle) = progress_handle {
        handle.abort();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let bytes = total_bytes.load(Ordering::Acquire);

    if let Some(cb) = progress {
        cb(ProgressUpdate {
            phase: TestPhase::Upload,
            speed_mbps: Some((bytes as f64 * 8.0) / elapsed / 1_000_000.0),
            progress: 1.0,
            latency_ms: None,
        });
    }

    Ok(ThroughputResult::new(bytes, elapsed))
}
