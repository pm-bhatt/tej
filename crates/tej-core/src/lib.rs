pub mod config;
pub mod data;
pub mod download;
pub mod error;
pub mod jitter;
pub mod latency;
pub mod packet_loss;
pub mod progress;
pub mod results;
pub mod runner;
pub mod upload;

pub use config::TestConfig;
pub use error::{Result, SpeedTestError};
pub use progress::{ProgressCallback, ProgressUpdate, TestPhase};
pub use results::{LatencyResult, SpeedTestResult, ThroughputResult};
pub use runner::run_speed_test;
