use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum TestPhase {
    Latency,
    Download,
    Upload,
    PacketLoss,
    Done,
}

#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub phase: TestPhase,
    /// Current speed in Mbps (for download/upload phases)
    pub speed_mbps: Option<f64>,
    /// Progress from 0.0 to 1.0
    pub progress: f64,
    /// Current latency sample in ms (for latency phase)
    pub latency_ms: Option<f64>,
}

pub type ProgressCallback = Arc<dyn Fn(ProgressUpdate) + Send + Sync>;
