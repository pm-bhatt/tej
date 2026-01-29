use std::time::Duration;

use crate::config::TestConfig;
use crate::error::Result;
use crate::progress::{ProgressCallback, ProgressUpdate, TestPhase};

pub async fn measure_packet_loss(
    client: &reqwest::Client,
    config: &TestConfig,
    progress: Option<&ProgressCallback>,
) -> Result<f64> {
    let url = format!("{}?bytes=0", config.latency_url);
    let count = config.packet_loss_count;
    let timeout = config.packet_loss_timeout;
    let mut failures = 0u32;

    for i in 0..count {
        let result = tokio::time::timeout(
            timeout,
            client.get(&url).send(),
        )
        .await;

        match result {
            Ok(Ok(resp)) => {
                // Also count HTTP errors as failures
                if !resp.status().is_success() {
                    failures += 1;
                }
            }
            _ => {
                failures += 1;
            }
        }

        // Small delay between requests to avoid bursting
        tokio::time::sleep(Duration::from_millis(50)).await;

        if let Some(cb) = progress {
            cb(ProgressUpdate {
                phase: TestPhase::PacketLoss,
                speed_mbps: None,
                progress: (i + 1) as f64 / count as f64,
                latency_ms: None,
            });
        }
    }

    Ok(failures as f64 / count as f64 * 100.0)
}
