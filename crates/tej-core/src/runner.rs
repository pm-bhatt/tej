use std::time::Duration;

use crate::config::TestConfig;
use crate::download::measure_download;
use crate::error::Result;
use crate::latency::measure_latency;
use crate::packet_loss::measure_packet_loss;
use crate::progress::{ProgressCallback, ProgressUpdate, TestPhase};
use crate::results::SpeedTestResult;
use crate::upload::measure_upload;

pub async fn run_speed_test(
    config: &TestConfig,
    progress: Option<ProgressCallback>,
) -> Result<SpeedTestResult> {
    let client = reqwest::Client::builder()
        .timeout(config.timeout)
        .pool_max_idle_per_host(config.parallel_connections)
        .tcp_nodelay(true)
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    let mut result = SpeedTestResult::new();
    let progress_ref = progress.as_ref();

    // Phase 1: Latency + Jitter
    let (latency_result, server_location) =
        measure_latency(&client, config, progress_ref).await?;
    result.latency = Some(latency_result);
    result.server_location = server_location;

    // Phase 2: Download
    if !config.skip_download {
        let download_result = measure_download(&client, config, progress_ref).await?;
        result.download = Some(download_result);
    }

    // Phase 3: Upload
    if !config.skip_upload {
        let upload_result = measure_upload(&client, config, progress_ref).await?;
        result.upload = Some(upload_result);
    }

    // Phase 4: Packet Loss
    let packet_loss = measure_packet_loss(&client, config, progress_ref).await?;
    result.packet_loss = Some(packet_loss);

    // Signal completion
    if let Some(cb) = progress_ref {
        cb(ProgressUpdate {
            phase: TestPhase::Done,
            speed_mbps: None,
            progress: 1.0,
            latency_ms: None,
        });
    }

    Ok(result)
}
