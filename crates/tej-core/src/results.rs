use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub timestamp: DateTime<Utc>,
    pub server_location: Option<String>,
    pub latency: Option<LatencyResult>,
    pub download: Option<ThroughputResult>,
    pub upload: Option<ThroughputResult>,
    pub packet_loss: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyResult {
    /// Minimum RTT in milliseconds
    pub min_ms: f64,
    /// Average RTT in milliseconds
    pub avg_ms: f64,
    /// Maximum RTT in milliseconds
    pub max_ms: f64,
    /// Jitter in milliseconds (mean absolute difference between consecutive samples)
    pub jitter_ms: f64,
    /// All RTT samples in milliseconds
    pub samples: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputResult {
    /// Speed in bits per second
    pub bps: f64,
    /// Speed in megabits per second
    pub mbps: f64,
    /// Total bytes transferred
    pub bytes_transferred: u64,
    /// Duration of the test in seconds
    pub duration_secs: f64,
}

impl ThroughputResult {
    pub fn new(bytes: u64, duration_secs: f64) -> Self {
        let bps = if duration_secs > 0.0 {
            (bytes as f64 * 8.0) / duration_secs
        } else {
            0.0
        };
        Self {
            bps,
            mbps: bps / 1_000_000.0,
            bytes_transferred: bytes,
            duration_secs,
        }
    }
}

impl SpeedTestResult {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            server_location: None,
            latency: None,
            download: None,
            upload: None,
            packet_loss: None,
        }
    }
}

impl Default for SpeedTestResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throughput_normal() {
        let r = ThroughputResult::new(1_000_000, 1.0);
        assert!((r.bps - 8_000_000.0).abs() < 0.01);
        assert!((r.mbps - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_throughput_zero_duration() {
        let r = ThroughputResult::new(1_000_000, 0.0);
        assert_eq!(r.bps, 0.0);
        assert_eq!(r.mbps, 0.0);
    }

    #[test]
    fn test_throughput_zero_bytes() {
        let r = ThroughputResult::new(0, 1.0);
        assert_eq!(r.bps, 0.0);
        assert_eq!(r.mbps, 0.0);
    }

    #[test]
    fn test_throughput_negative_duration() {
        let r = ThroughputResult::new(1000, -1.0);
        assert_eq!(r.bps, 0.0);
    }

    #[test]
    fn test_result_serialization_roundtrip() {
        let result = SpeedTestResult {
            timestamp: Utc::now(),
            server_location: Some("SFO".to_string()),
            latency: Some(LatencyResult {
                min_ms: 5.0,
                avg_ms: 10.0,
                max_ms: 15.0,
                jitter_ms: 2.0,
                samples: vec![5.0, 10.0, 15.0],
            }),
            download: Some(ThroughputResult::new(10_000_000, 2.0)),
            upload: Some(ThroughputResult::new(5_000_000, 2.0)),
            packet_loss: Some(0.0),
        };
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: SpeedTestResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.server_location, Some("SFO".to_string()));
        assert!((deserialized.download.unwrap().mbps - 40.0).abs() < 0.01);
    }
}
