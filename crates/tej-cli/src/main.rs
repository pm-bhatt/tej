mod display;
mod output;

use std::sync::Arc;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use tej_core::{ProgressUpdate, TestConfig, TestPhase};

#[derive(Parser)]
#[command(name = "tej", about = "Tej - Honest internet speed test using Cloudflare CDN")]
struct Args {
    /// Output format
    #[arg(long, default_value = "text", value_parser = ["text", "json"])]
    format: String,

    /// Number of parallel connections (1-32)
    #[arg(short = 'c', long, default_value_t = 6, value_parser = clap::value_parser!(u64).range(1..=32))]
    connections: u64,

    /// Skip download test
    #[arg(long)]
    no_download: bool,

    /// Skip upload test
    #[arg(long)]
    no_upload: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = TestConfig {
        parallel_connections: args.connections as usize,
        skip_download: args.no_download,
        skip_upload: args.no_upload,
        ..TestConfig::default()
    };

    let is_json = args.format == "json";

    let progress_cb = if is_json {
        None
    } else {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} {msg} [{bar:30.cyan/dim}] {pos}%")
                .expect("valid template")
                .progress_chars("=> "),
        );
        pb.set_message("Starting...");

        let callback = move |update: ProgressUpdate| {
            let msg = match update.phase {
                TestPhase::Latency => {
                    if let Some(ms) = update.latency_ms {
                        format!("Measuring latency... {:.1} ms", ms)
                    } else {
                        "Measuring latency...".to_string()
                    }
                }
                TestPhase::Download => {
                    if let Some(speed) = update.speed_mbps {
                        format!("Download: {:.2} Mbps", speed)
                    } else {
                        "Measuring download...".to_string()
                    }
                }
                TestPhase::Upload => {
                    if let Some(speed) = update.speed_mbps {
                        format!("Upload: {:.2} Mbps", speed)
                    } else {
                        "Measuring upload...".to_string()
                    }
                }
                TestPhase::PacketLoss => "Measuring packet loss...".to_string(),
                TestPhase::Done => "Done!".to_string(),
            };
            pb.set_message(msg);
            pb.set_position((update.progress * 100.0) as u64);

            if update.phase == TestPhase::Done {
                pb.finish_and_clear();
            }
        };

        Some(Arc::new(callback) as tej_core::ProgressCallback)
    };

    if !is_json {
        println!("Tej - Honest Speed Test");
        println!("Testing with {} parallel connections...", config.parallel_connections);
        println!();
    }

    match tej_core::run_speed_test(&config, progress_cb).await {
        Ok(result) => {
            if is_json {
                output::print_json(&result);
            } else {
                display::print_results(&result);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
