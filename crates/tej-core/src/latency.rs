use std::time::Instant;

use crate::config::TestConfig;
use crate::error::Result;
use crate::progress::{ProgressCallback, ProgressUpdate, TestPhase};
use crate::results::LatencyResult;

pub async fn measure_latency(
    client: &reqwest::Client,
    config: &TestConfig,
    progress: Option<&ProgressCallback>,
) -> Result<(LatencyResult, Option<String>)> {
    let url = format!("{}?bytes=0", config.latency_url);
    let total_samples = config.latency_samples;
    let mut all_samples = Vec::with_capacity(total_samples);
    let mut server_location = None;

    for i in 0..total_samples {
        let start = Instant::now();
        let resp = client.get(&url).send().await?;
        let rtt = start.elapsed().as_secs_f64() * 1000.0;

        // Extract server location from cf-ray header on first response
        if server_location.is_none() {
            if let Some(ray) = resp.headers().get("cf-ray") {
                if let Ok(ray_str) = ray.to_str() {
                    // cf-ray format: "hex-LOCATION"
                    if let Some(loc) = ray_str.rsplit('-').next() {
                        server_location = Some(loc.to_string());
                    }
                }
            }
        }

        // Consume response body
        let _ = resp.bytes().await?;
        all_samples.push(rtt);

        if let Some(cb) = progress {
            cb(ProgressUpdate {
                phase: TestPhase::Latency,
                speed_mbps: None,
                progress: (i + 1) as f64 / total_samples as f64,
                latency_ms: Some(rtt),
            });
        }
    }

    // Discard warmup samples
    let samples: Vec<f64> = all_samples
        .iter()
        .skip(config.latency_warmup)
        .copied()
        .collect();

    if samples.is_empty() {
        return Err(crate::error::SpeedTestError::InvalidResponse(
            "No valid latency samples after discarding warmup".to_string(),
        ));
    }

    let min_ms = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max_ms = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let avg_ms = samples.iter().sum::<f64>() / samples.len() as f64;
    let jitter_ms = crate::jitter::calculate_jitter(&samples);

    Ok((
        LatencyResult {
            min_ms,
            avg_ms,
            max_ms,
            jitter_ms,
            samples,
        },
        server_location,
    ))
}
