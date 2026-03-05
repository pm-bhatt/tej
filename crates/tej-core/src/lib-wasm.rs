// Conditional compilation for WASM vs native targets

// WASM-only module
#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Native-only modules (require tokio/reqwest)
#[cfg(not(target_arch = "wasm32"))]
pub mod config;
#[cfg(not(target_arch = "wasm32"))]
pub mod data;
#[cfg(not(target_arch = "wasm32"))]
pub mod download;
#[cfg(not(target_arch = "wasm32"))]
pub mod error;
#[cfg(not(target_arch = "wasm32"))]
pub mod jitter;
#[cfg(not(target_arch = "wasm32"))]
pub mod latency;
#[cfg(not(target_arch = "wasm32"))]
pub mod packet_loss;
#[cfg(not(target_arch = "wasm32"))]
pub mod progress;
#[cfg(not(target_arch = "wasm32"))]
pub mod results;
#[cfg(not(target_arch = "wasm32"))]
pub mod runner;
#[cfg(not(target_arch = "wasm32"))]
pub mod upload;

// Re-exports for native builds
#[cfg(not(target_arch = "wasm32"))]
pub use config::TestConfig;
#[cfg(not(target_arch = "wasm32"))]
pub use error::{Result, SpeedTestError};
#[cfg(not(target_arch = "wasm32"))]
pub use progress::{ProgressCallback, ProgressUpdate, TestPhase};
#[cfg(not(target_arch = "wasm32"))]
pub use results::{LatencyResult, SpeedTestResult, ThroughputResult};
#[cfg(not(target_arch = "wasm32"))]
pub use runner::run_speed_test;

// Re-export WASM function for WASM builds
#[cfg(target_arch = "wasm32")]
pub use wasm::run_speed_test_wasm;
