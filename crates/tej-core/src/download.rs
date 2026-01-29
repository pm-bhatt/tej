use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use futures::stream::StreamExt;
use tokio::task::JoinSet;

use crate::config::TestConfig;
use crate::error::Result;
use crate::progress::{ProgressCallback, ProgressUpdate, TestPhase};
use crate::results::ThroughputResult;

pub async fn measure_download(
    client: &reqwest::Client,
    config: &TestConfig,
    progress: Option<&ProgressCallback>,
) -> Result<ThroughputResult> {
    // Warmup with small download to estimate speed
    let warmup_size = config.download_sizes[0];
    let warmup_start = Instant::now();
    let resp = client
        .get(format!("{}?bytes={}", config.download_url, warmup_size))
        .send()
        .await?;
    let mut warmup_bytes = 0u64;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        warmup_bytes += chunk.len() as u64;
    }
    let warmup_elapsed = warmup_start.elapsed().as_secs_f64();
    let warmup_speed_bps = (warmup_bytes as f64 * 8.0) / warmup_elapsed;

    // Select test size based on warmup speed
    let test_size = if warmup_speed_bps > 100_000_000.0 {
        // >100 Mbps: use largest
        *config.download_sizes.last().unwrap_or(&25_000_000)
    } else if warmup_speed_bps > 10_000_000.0 {
        // >10 Mbps: use medium
        config.download_sizes.get(2).copied().unwrap_or(10_000_000)
    } else {
        // Slow connection: use small
        config.download_sizes.get(1).copied().unwrap_or(1_000_000)
    };

    // Run parallel downloads
    let total_bytes = Arc::new(AtomicU64::new(0));
    let start = Instant::now();
    let mut tasks = JoinSet::new();

    for _ in 0..config.parallel_connections {
        let client = client.clone();
        let url = format!("{}?bytes={}", config.download_url, test_size);
        let total = total_bytes.clone();

        tasks.spawn(async move {
            let resp = client.get(&url).send().await?;
            let mut stream = resp.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                total.fetch_add(chunk.len() as u64, Ordering::Release);
            }
            Ok::<(), crate::error::SpeedTestError>(())
        });
    }

    // Progress reporting loop
    let progress_total = total_bytes.clone();
    let expected_total = (test_size * config.parallel_connections) as f64;
    let progress_handle = if let Some(cb) = progress {
        let cb = cb.clone();
        Some(tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                let bytes = progress_total.load(Ordering::Acquire);
                let elapsed = start.elapsed().as_secs_f64();
                let speed_bps = (bytes as f64 * 8.0) / elapsed;
                cb(ProgressUpdate {
                    phase: TestPhase::Download,
                    speed_mbps: Some(speed_bps / 1_000_000.0),
                    progress: (bytes as f64 / expected_total).min(1.0),
                    latency_ms: None,
                });
            }
        }))
    } else {
        None
    };

    // Wait for all downloads to complete
    while let Some(result) = tasks.join_next().await {
        result.map_err(|e| crate::error::SpeedTestError::Other(e.to_string()))??;
    }

    if let Some(handle) = progress_handle {
        handle.abort();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let bytes = total_bytes.load(Ordering::Acquire);

    // Final progress update
    if let Some(cb) = progress {
        cb(ProgressUpdate {
            phase: TestPhase::Download,
            speed_mbps: Some((bytes as f64 * 8.0) / elapsed / 1_000_000.0),
            progress: 1.0,
            latency_ms: None,
        });
    }

    Ok(ThroughputResult::new(bytes, elapsed))
}
